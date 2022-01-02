use base::{BlockKind, ItemStack, ValidBlockPosition};
use ecs::{EntityBuilder, SysResult, SystemExecutor};
use libcraft_items::EnchantmentKind;
use quill_common::{entities::Player, entity_init::EntityInit};

pub struct DestroyStateChange(pub ValidBlockPosition, pub u8);

use crate::{Game, World};

#[derive(Clone)]
pub enum BlockBreaker {
    Active(ActiveBreaker),
    Finished(FinishedBreaker),
    Inactive,
}
impl BlockBreaker {
    pub fn new(
        world: &mut World,
        block_pos: ValidBlockPosition,
        equipped_item: Option<&ItemStack>,
    ) -> Self {
        ActiveBreaker::new(world, block_pos, equipped_item)
            .map(Self::Active)
            .unwrap_or(Self::Inactive)
    }
    pub fn destroy_change_event(&self) -> Option<DestroyStateChange> {
        Some(DestroyStateChange(self.position()?, self.destroy_stage()))
    }
    pub fn position(&self) -> Option<ValidBlockPosition> {
        match self {
            BlockBreaker::Active(a) => Some(a.position),
            BlockBreaker::Finished(f) => Some(f.position),
            BlockBreaker::Inactive => None,
        }
    }
    pub fn active(&self) -> Option<&ActiveBreaker> {
        match self {
            Self::Active(a) => Some(a),
            _ => None,
        }
    }
    pub fn finished(&self) -> Option<&FinishedBreaker> {
        match self {
            Self::Finished(f) => Some(f),
            _ => None,
        }
    }
    pub fn tick(&mut self) -> (bool, bool) {
        let (block_break, stage_update) = if let Self::Active(breaker) = self {
            breaker.tick()
        } else {
            (false, false)
        };
        if block_break {
            let fin = match self {
                Self::Active(a) => a.clone().finish(),
                _ => unreachable!(),
            };
            *self = Self::Finished(fin);
        }
        (block_break, stage_update)
    }
    pub fn destroy_stage(&self) -> u8 {
        match self {
            BlockBreaker::Active(a) => a.destroy_stage(),
            _ => 10,
        }
    }
    pub fn cancel(&mut self) {
        *self = Self::Inactive;
    }
    pub fn matches_position(&self, pos: ValidBlockPosition) -> bool {
        match self {
            BlockBreaker::Active(a) => a.position == pos,
            BlockBreaker::Finished(f) => f.position == pos,
            BlockBreaker::Inactive => true,
        }
    }
    pub fn try_finish(&mut self) -> Option<FinishedBreaker> {
        let this = self.clone();
        match this {
            BlockBreaker::Active(a) => {
                if a.ticks_remaining == 1 {
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
#[derive(Clone)]
pub struct FinishedBreaker {
    pub position: ValidBlockPosition,
    pub drop_item: bool,
    pub fake_finished: bool,
}
impl FinishedBreaker {
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
    pub total_ticks: u32,
    pub ticks_remaining: u32,
}
impl ActiveBreaker {
    pub fn tick(&mut self) -> (bool, bool) {
        let before = self.destroy_stage();
        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
        let after = self.destroy_stage();
        let break_block = self.ticks_remaining == 0;
        let change_stage = before != after || break_block;
        (break_block, change_stage)
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
                    .map(|e| {
                        if e.item() == *item {
                            Some(*speed)
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .unwrap_or(1.0);
        let effi_level = equipped_item
            .map(ItemStack::metadata)
            .flatten()
            .map(|meta| {
                meta.enchantments().iter().find_map(|ench| {
                    if ench.kind() == EnchantmentKind::Efficiency {
                        Some(ench.level())
                    } else {
                        None
                    }
                })
            })
            .flatten();
        let effi_speed = effi_level.map(|level| level * level + 1).unwrap_or(0) as f32;
        let damage = if harvestable {
            (dig_multiplier + effi_speed) / block.hardness() / 30.0
        } else {
            1.0 / block.hardness() / 100.0
        };
        let ticks = if damage > 1.0 {
            0
        } else {
            (1.0 / damage / 1.2).ceil() as u32
        };
        println!(
            "Mining {} with {} takes {} ticks",
            block.display_name(),
            equipped_item
                .map(|e| e.get_item().item().display_name())
                .unwrap_or("bare hands"),
            ticks
        );
        Some(Self {
            position: block_pos,
            drop_item: true,
            fake_finished: false,
            total_ticks: ticks,
            ticks_remaining: ticks,
        })
    }
    pub fn destroy_stage(&self) -> u8 {
        if self.fake_finished {
            10
        } else {
            9 - (self.ticks_remaining as f32 / self.total_ticks as f32 * 9.0).round() as u8
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
