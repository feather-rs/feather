use anyhow::Context;
use base::{inventory::SLOT_HOTBAR_OFFSET, BlockKind, ItemStack, ValidBlockPosition};
use ecs::{Entity, SysResult, SystemExecutor};
use libcraft_items::EnchantmentKind;
use quill_common::components::Instabreak;

pub struct DestroyStateChange(pub ValidBlockPosition, pub u8);

use crate::{entities::player::HotbarSlot, Game, Window, World};

pub const BREAK_THRESHOLD: f32 = 0.7;

/// Handle a player's request to start digging the block at `position`. Adds an `ActiveBreaker` component to `player`.
pub fn start_digging(
    game: &mut Game,
    player: Entity,
    position: ValidBlockPosition,
) -> anyhow::Result<bool> {
    if game.ecs.get::<Instabreak>(player)?.0 {
        game.break_block(position);
    } else {
        let breaker = {
            let window = game.ecs.get::<Window>(player)?;
            let hotbar_slot = game.ecs.get::<HotbarSlot>(player)?.get();
            let main_hand = window.item(SLOT_HOTBAR_OFFSET + hotbar_slot)?;
            ActiveBreaker::new(&mut game.world, position, main_hand.option_ref())
                .context("Cannot mine this block")?
        };
        game.ecs.insert(player, breaker)?;
    }
    Ok(true)
}
/// Handle a player's request to stop digging the block at `position`. Removes `ActiveBreaker` from `player`.
pub fn cancel_digging(
    game: &mut Game,
    player: Entity,
    position: ValidBlockPosition,
) -> anyhow::Result<bool> {
    if game.ecs.get::<ActiveBreaker>(player).is_err() {
        return Ok(false);
    }
    game.ecs.remove::<ActiveBreaker>(player)?;
    game.ecs
        .insert_entity_event(player, DestroyStateChange(position, 10))?;
    Ok(true)
}
/// Handle a player's request to finish digging the block at `position`.
/// This is called when the block breaking finishes at the player's side.
///
/// Will return `false` if the system hasn't finished breaking the block yet.
pub fn finish_digging(
    game: &mut Game,
    player: Entity,
    position: ValidBlockPosition,
) -> anyhow::Result<bool> {
    if game.ecs.get::<Instabreak>(player)?.0 {
        return Ok(true);
    }
    let success = if let Ok(breaker) = game.ecs.get::<ActiveBreaker>(player) {
        breaker.can_break()
    } else {
        false
    };
    if success {
        let pos = game.ecs.get::<ActiveBreaker>(player)?.position;
        game.break_block(pos); // TODO: drop an item
        game.ecs.remove::<ActiveBreaker>(player)?;
    }
    game.ecs
        .insert_entity_event(player, DestroyStateChange(position, 10))?;
    Ok(success)
}
/// Main component for the block breaking system.
/// Tracks the position of the block being mined, whether it will drop an item and the current progress.
#[derive(Clone)]
pub struct ActiveBreaker {
    pub position: ValidBlockPosition,
    pub drop_item: bool,
    pub progress: f32,
    pub damage: f32,
}
impl ActiveBreaker {
    /// Advance block breaking by one tick.
    pub fn tick(&mut self) -> (bool, bool) {
        let before = self.destroy_stage();
        self.progress += self.damage;
        let after = self.destroy_stage();
        let break_block = self.can_break();
        let change_stage = break_block || before != after;
        (break_block, change_stage)
    }
    /// Check if the block has been damaged enough to break.
    pub fn can_break(&self) -> bool {
        self.progress >= BREAK_THRESHOLD - self.damage / 2.0
    }
    /// Start breaking a block. Returns an error if the block at `block_pos` is unloaded or not diggable.
    pub fn new(
        world: &mut World,
        block_pos: ValidBlockPosition,
        equipped_item: Option<&ItemStack>,
    ) -> anyhow::Result<Self> {
        let block = world
            .block_at(block_pos)
            .context("Block is not loaded")?
            .kind();
        if !block.diggable() || block == BlockKind::Air {
            anyhow::bail!("Block is not diggable")
        }
        let harvestable = match (block.harvest_tools(), equipped_item) {
            (None, None | Some(_)) => true,
            (Some(_), None) => false,
            (Some(valid_tools), Some(equipped)) => valid_tools.contains(&equipped.item()),
        };
        let dig_multiplier = block // TODO: calculate with Haste effect
            .dig_multipliers()
            .iter()
            .find_map(|(item, speed)| {
                equipped_item.and_then(|e| bool::then_some(e.item() == *item, *speed))
            })
            .unwrap_or(1.0);
        let effi_level = equipped_item
            .and_then(ItemStack::metadata)
            .and_then(|meta| meta.get_enchantment_level(EnchantmentKind::Efficiency));
        let effi_speed = effi_level.map(|level| level * level + 1).unwrap_or(0) as f32;
        let damage = if harvestable {
            (dig_multiplier + effi_speed) / block.hardness() / 30.0
        } else {
            1.0 / block.hardness() / 100.0
        };
        Ok(Self {
            position: block_pos,
            drop_item: harvestable,
            progress: damage,
            damage,
        })
    }
    /// Get the destroying progress.
    pub fn destroy_stage(&self) -> u8 {
        (self.progress * 9.0).round() as u8
    }
    pub fn destroy_change_event(&self) -> DestroyStateChange {
        DestroyStateChange(self.position, self.destroy_stage())
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(process_block_breaking);
}

fn process_block_breaking(game: &mut Game) -> SysResult {
    let mut update_queue = vec![];
    for (entity, breaker) in game.ecs.query::<&mut ActiveBreaker>().iter() {
        let (_, update_stage) = breaker.tick();
        if update_stage {
            update_queue.push(entity);
        }
    }
    for entity in update_queue {
        let event = game
            .ecs
            .get_mut::<ActiveBreaker>(entity)?
            .destroy_change_event();
        game.ecs.insert_entity_event(entity, event)?;
    }
    Ok(())
}
