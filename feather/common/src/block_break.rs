use std::mem;

use base::{BlockKind, ItemStack, ValidBlockPosition};
use ecs::{EntityBuilder, SysResult, SystemExecutor};
use libcraft_items::EnchantmentKind;
use quill_common::{entities::Player, entity_init::EntityInit};

pub struct DestroyStateChange(pub ValidBlockPosition, pub u8);

use crate::{Game, World};

// Comparing to 0.7 ensures good feeling in the client
pub const BREAK_THRESHOLD: f32 = 0.7;

#[derive(Clone)]
pub enum BlockBreaker {
    Active(ActiveBreaker),
    Finished(FinishedBreaker),
    Inactive,
}
impl BlockBreaker {
    /// Create a new active instance pointing to `block_pos`. Calculates the time needed using `world.block_at(block_pos)` and `equipped_item`.
    pub fn new(
        world: &mut World,
        block_pos: ValidBlockPosition,
        equipped_item: Option<&ItemStack>,
    ) -> Self {
        ActiveBreaker::new(world, block_pos, equipped_item)
            .map(Self::Active)
            .unwrap_or(Self::Inactive)
    }
    /// If active, produces a `DestroyStateChange` event for the adequate position.
    pub fn destroy_change_event(&self) -> Option<DestroyStateChange> {
        Some(DestroyStateChange(self.position()?, self.destroy_stage()))
    }
    /// If active or finished, returns the pointed to position.
    pub fn position(&self) -> Option<ValidBlockPosition> {
        match self {
            BlockBreaker::Active(a) => Some(a.position),
            BlockBreaker::Finished(f) => Some(f.position),
            BlockBreaker::Inactive => None,
        }
    }
    /// If active, returns the underlying `ActiveBreaker`.
    pub fn active(&self) -> Option<&ActiveBreaker> {
        match self {
            Self::Active(a) => Some(a),
            _ => None,
        }
    }
    /// If finished, returns the underlying `FinishedBreaker`.
    pub fn finished(&self) -> Option<&FinishedBreaker> {
        match self {
            Self::Finished(f) => Some(f),
            _ => None,
        }
    }
    /// Progresses block breaking. Returns a (newly_finished, do_destry_state_change) tuple.
    /// If this operation finishes block breaking, this turns `self` into `Self::Finished` with the same position.
    pub fn tick(&mut self) -> (bool, bool) {
        let (block_break, stage_update) = if let Self::Active(breaker) = self {
            breaker.tick()
        } else {
            (false, false)
        };
        if block_break {
            let fin = match mem::take(self) {
                Self::Active(a) => a.finish(),
                _ => unreachable!(),
            };
            *self = Self::Finished(fin);
        }
        (block_break, stage_update)
    }
    /// Returns the block destroying progress in a range of 0 - 9. When inactive or finished, returns 10.
    pub fn destroy_stage(&self) -> u8 {
        match self {
            BlockBreaker::Active(a) => a.destroy_stage(),
            _ => 10,
        }
    }
    /// Set `self` to `Self::Inactive`.
    pub fn cancel(&mut self) {
        *self = Self::Inactive;
    }
    /// Check if the breaker points to `pos`. Returns `true` when `self` is `Self::Inactive`.
    pub fn matches_position(&self, pos: ValidBlockPosition) -> bool {
        match self {
            BlockBreaker::Active(a) => a.position == pos,
            BlockBreaker::Finished(f) => f.position == pos,
            BlockBreaker::Inactive => true,
        }
    }
    /// Attempts to finish breaking the target block, optionally turning `self` into `Self::Finished`.
    pub fn try_finish(&mut self) -> Option<FinishedBreaker> {
        let this = self.clone();
        match this {
            BlockBreaker::Active(a) => {
                if a.can_break() {
                    let fin = a.finish();
                    *self = Self::Finished(fin.clone());
                    Some(fin)
                } else {
                    None
                }
            }
            BlockBreaker::Finished(f) => Some(f),
            BlockBreaker::Inactive => None,
        }
    }
}
impl Default for BlockBreaker {
    fn default() -> Self {
        Self::Inactive
    }
}
#[derive(Clone)]
pub struct FinishedBreaker {
    pub position: ValidBlockPosition,
    pub drop_item: bool,
    pub fake_finished: bool,
}
impl FinishedBreaker {
    /// Breaks the targeted block and spawns its drops. TODO: make drops work.
    pub fn break_block(&self, game: &mut Game) -> SysResult {
        let target_block = match game.block(self.position) {
            Some(b) => b,
            None => anyhow::bail!("cannot break unloaded block"),
        };
        game.break_block(self.position);
        if let Some(_item_drop) = base::Item::from_name(target_block.kind().name()) {
            if !self.drop_item {
                return Ok(());
            }
            let mut item_entity = EntityBuilder::new();
            crate::entities::item::build_default(&mut item_entity);
            let builder = game.create_entity_builder(self.position.position(), EntityInit::Item);
            game.spawn_entity(builder);
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct ActiveBreaker {
    pub position: ValidBlockPosition,
    pub drop_item: bool,
    pub fake_finished: bool,
    pub progress: f32,
    pub damage: f32,
}
impl ActiveBreaker {
    pub fn tick(&mut self) -> (bool, bool) {
        let before = self.destroy_stage();
        self.progress += self.damage;
        let after = self.destroy_stage();
        let break_block = self.can_break();
        let change_stage = before != after || break_block;
        (break_block, change_stage)
    }
    /// Check if the block has been damaged enough to break.
    pub fn can_break(&self) -> bool {
        self.progress >= BREAK_THRESHOLD - self.damage / 2.0
    }
    pub fn new(
        world: &mut World,
        block_pos: ValidBlockPosition,
        equipped_item: Option<&ItemStack>,
    ) -> Option<Self> {
        let block = world.block_at(block_pos)?.kind();
        if !block.diggable() || block == BlockKind::Air {
            return None;
        }
        let harvestable = match (block.harvest_tools(), equipped_item) {
            (None, None | Some(_)) => true,
            (Some(_), None) => false,
            (Some(tools), Some(tool)) => tools.contains(&tool.item()),
        };
        let dig_multiplier = block
            .dig_multipliers()
            .iter()
            .find_map(|(item, speed)| {
                equipped_item
                    .and_then(|e| {
                        bool::then_some(e.item() == *item, *speed)
                    })
            })
            .unwrap_or(1.0);
        let effi_level = equipped_item
            .and_then(ItemStack::metadata)
            .and_then(|meta| {
                meta.get_enchantment_level(EnchantmentKind::Efficiency)
            });
        let effi_speed = effi_level.map(|level| level * level + 1).unwrap_or(0) as f32;
        let damage = if harvestable {
            (dig_multiplier + effi_speed) / block.hardness() / 30.0
        } else {
            1.0 / block.hardness() / 100.0
        };
        Some(Self {
            position: block_pos,
            drop_item: true,
            fake_finished: false,
            progress: 0.0,
            damage,
        })
    }
    /// Get the destroying progress.
    pub fn destroy_stage(&self) -> u8 {
        if self.fake_finished {
            10
        } else {
            (self.progress / BREAK_THRESHOLD * 9.0).round() as u8
        }
    }
    pub fn finish(self) -> FinishedBreaker {
        FinishedBreaker {
            position: self.position,
            drop_item: self.drop_item,
            fake_finished: self.fake_finished,
        }
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(process_block_breaking);
}

fn process_block_breaking(game: &mut Game) -> SysResult {
    let mut break_queue = vec![];
    let mut update_queue = vec![];
    for (entity, breaker) in game.ecs.query::<&mut BlockBreaker>().iter() {
        let (break_block, update_stage) = breaker.tick();
        if update_stage {
            update_queue.push(entity);
        }
        // Break block when client requests to finish in order to prevent desyncs
        if break_block && breaker.finished().unwrap().fake_finished
            || game.ecs.get::<Player>(entity).is_err()
        {
            break_queue.push(entity);
        }
    }
    for entity in update_queue {
        let event = game
            .ecs
            .get_mut::<BlockBreaker>(entity)?
            .destroy_change_event()
            .unwrap();
        game.ecs.insert_entity_event(entity, event)?;
    }
    for entity in break_queue.into_iter() {
        let breaker = game
            .ecs
            .get::<BlockBreaker>(entity)?
            .finished()
            .unwrap()
            .clone();
        breaker.break_block(game)?;
    }
    Ok(())
}
