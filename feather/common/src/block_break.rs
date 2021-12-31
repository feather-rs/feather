use base::{ItemStack, ValidBlockPosition};
use ecs::{EntityBuilder, SysResult, SystemExecutor};
use libcraft_items::EnchantmentKind;
use quill_common::entity_init::EntityInit;

pub struct DestroyStateChange(pub ValidBlockPosition, pub u8);

use crate::{Game, World};

pub type BlockBreaker = Option<ActiveBlockBreaker>;
#[derive(Clone, Copy)]
pub struct ActiveBlockBreaker {
    pub position: ValidBlockPosition,
    pub drop_item: bool,
    pub total_ticks: u32,
    pub ticks_remaining: u32,
}
impl ActiveBlockBreaker {
    pub fn tick(&mut self) -> (bool, bool) {
        let before = self.destroy_stage();
        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
        let after = self.destroy_stage();
        (self.ticks_remaining == 0, before != after)
    }
    pub fn break_block(self, game: &mut Game) -> SysResult {
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
    pub fn new_player(
        world: &mut World,
        block_pos: ValidBlockPosition,
        main_hand: Option<&ItemStack>,
    ) -> Option<Self> {
        let block = world.block_at(block_pos)?.kind();
        if !block.diggable() {
            return None;
        }
        let harvestable = match (block.harvest_tools(), main_hand) {
            (None, None | Some(_)) => true,
            (Some(_), None) => false,
            (Some(tools), Some(tool)) => tools.contains(&tool.item()),
        };
        let dig_multiplier = block
            .dig_multipliers()
            .iter()
            .find_map(|(item, speed)| {
                main_hand
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
        let effi_level = main_hand
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
            (1.0 / damage).ceil() as u32 - 4
        };
        Some(Self {
            position: block_pos,
            drop_item: true,
            total_ticks: ticks,
            ticks_remaining: ticks,
        })
    }
    pub fn destroy_stage(&self) -> u8 {
        9 - (self.ticks_remaining as f32 / self.total_ticks as f32 * 9.0).round() as u8
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(process_block_breaking);
}

fn process_block_breaking(game: &mut Game) -> SysResult {
    let mut break_queue = vec![];
    let mut update_queue = vec![];
    for (entity, breaker) in game.ecs.query::<&mut BlockBreaker>().iter() {
        if let Some(active) = breaker {
            let (break_block, update_stage) = active.tick();
            if update_stage {
                update_queue.push(entity);
            }
            if break_block {
                break_queue.push(entity);
            }
        }
    }
    for entity in update_queue {
        let breaker = { game.ecs.get_mut::<BlockBreaker>(entity).unwrap().unwrap() };
        game.ecs.insert_entity_event(
            entity,
            DestroyStateChange(breaker.position, breaker.destroy_stage()),
        )?;
    }
    for entity in break_queue.into_iter() {
        // Set block breakers to None
        let breaker = {
            game.ecs
                .get_mut::<BlockBreaker>(entity)
                .unwrap()
                .take()
                .unwrap()
        };
        game.ecs
            .insert_entity_event(entity, DestroyStateChange(breaker.position, 10))
            .unwrap();
        breaker.break_block(game)?;
    }
    Ok(())
}
