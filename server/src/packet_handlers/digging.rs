//! This module handles the monolithic Player Digging packet.
//!
//! The packet's name is rather misleading, as it is also sent
//! for actions mostly unrelated to digging including eating, shooting bows,
//! swapping items out to the offhand, and dropping items.

use crate::entity::arrow;
use crate::game::Game;
use crate::p_inventory::EntityInventory;
use crate::packet_buffer::PacketBuffers;
use crate::physics::{charge_from_ticks_held, compute_projectile_velocity};
use crate::player::{ItemTimedUse, PLAYER_EYE_HEIGHT};
use feather_core::inventory::{SlotIndex, SLOT_HOTBAR_OFFSET, SLOT_OFFHAND};
use feather_core::network::packet::implementation::{PlayerDigging, PlayerDiggingStatus};
use feather_core::{Block, Gamemode, Item, ItemStack, Position};
use fecs::{Entity, World};
use std::sync::Arc;

/// System responsible for polling for PlayerDigging
/// packets and writing the corresponding events.
#[system]
pub fn handle_player_digging(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    use PlayerDiggingStatus::*;

    let packets = packet_buffers.received::<PlayerDigging>();

    for (player, packet) in packets {
        match packet.status {
            StartedDigging | FinishedDigging | CancelledDigging => {
                handle_digging(game, world, player, packet)
            }
            /*DropItem | DropItemStack => handle_drop_item_stack(
                packet,
                player,
                inventory_updates,
                item_drops,
                &mut inventory,
            ),*/
            ConsumeItem => handle_consume_item(game, world, player, packet),

            status => warn!("Unhandled Player Digging status {:?}", status),
        }
    }
}

fn handle_digging(game: &mut Game, world: &mut World, player: Entity, packet: PlayerDigging) {
    let gamemode = *world.get::<Gamemode>(player);
    // Return early if needed
    match packet.status {
        PlayerDiggingStatus::StartedDigging => {
            if gamemode != Gamemode::Creative {
                return;
            }
        }
        PlayerDiggingStatus::CancelledDigging => return,
        _ => (),
    }

    let item_in_main_hand = world
        .get::<EntityInventory>(player)
        .item_in_main_hand()
        .copied();

    // Don't break block if player is holding a sword in creative mode.
    if gamemode == Gamemode::Creative {
        if let Some(item_in_main_hand) = item_in_main_hand {
            match item_in_main_hand.ty {
                Item::WoodenSword
                | Item::StoneSword
                | Item::GoldenSword
                | Item::IronSword
                | Item::DiamondSword => return, // creative mode: don't break block with swords
                _ => (),
            }
        }
    }

    if !game.set_block_at(world, packet.location, Block::Air) {
        game.disconnect(player, world, "attempted to break block in unloaded chunk");
        return;
    }
}

/*
fn handle_drop_item_stack(
    packet: PlayerDigging,
    entity: Entity,
    inventory_updates: &mut Trigger<InventoryUpdateEvent>,
    item_drops: &mut Trigger<ItemDropEvent>,
    inventory: &mut EntityInventory,
) {
    assert!(
        packet.status == PlayerDiggingStatus::DropItem
            || packet.status == PlayerDiggingStatus::DropItemStack
    );

    let slot = inventory.held_item + SLOT_HOTBAR_OFFSET;

    let stack = {
        if let Some(item) = inventory.item_at(slot) {
            *item
        } else {
            // Silently fail - no item stack to drop
            return;
        }
    };

    let amnt = match packet.status {
        PlayerDiggingStatus::DropItem => {
            if stack.amount == 0 {
                inventory.clear_item_at(slot);
                0
            } else if stack.amount == 1 {
                inventory.clear_item_at(slot);
                1
            } else {
                inventory.set_item_at(slot, ItemStack::new(stack.ty, stack.amount - 1));
                1
            }
        }
        PlayerDiggingStatus::DropItemStack => {
            inventory.clear_item_at(slot);
            stack.amount
        }
        _ => unreachable!(), // Assertion above
    };

    let inv_update = InventoryUpdateEvent {
        slots: smallvec![slot],
        player: entity,
    };
    inventory_updates.trigger(inv_update);

    if amnt != 0 {
        let item_drop = ItemDropEvent {
            slot: Some(slot),
            stack: ItemStack::new(stack.ty, amnt),
            player: entity,
        };
        item_drops.trigger(item_drop);
    }
}
*/

/// Handles food consumption and shooting arrows.
fn handle_consume_item(game: &mut Game, world: &mut World, player: Entity, packet: PlayerDigging) {
    assert_eq!(packet.status, PlayerDiggingStatus::ConsumeItem);

    // TODO: Fallback to off-hand if main-hand is not a consumable
    let inventory = world.get::<EntityInventory>(player);
    let used_item = inventory.item_in_main_hand();

    if let Some(item) = used_item {
        if item.ty == Item::Bow {
            drop(inventory);
            handle_shoot_bow(game, world, player);
        }
        // TODO: Food, potions
    }
}

fn handle_shoot_bow(game: &mut Game, world: &mut World, player: Entity) {
    let inventory = world.get::<EntityInventory>(player);
    let arrow_to_consume: Option<(SlotIndex, ItemStack)> = find_arrow(&inventory);
    // Unnecessary until more gamemodes are supported
    /*
    if player.gamemode == Gamemode::Survival || player.gamemode == Gamemode::Adventure {
        // If no arrow was found, don't shoot
        let arrow_to_consume = arrow_to_consume.clone();
        if arrow_to_consume.is_none() {
            debug!("Tried to shoot bow with no arrows.");
            return;
        }

        // Consume arrow
        let (arrow_slot, arrow_stack) = arrow_to_consume.unwrap();
        let mut arrow_stack: ItemStack = arrow_stack;
        arrow_stack.amount -= 1;

        inventory.set_item_at(arrow_slot, arrow_stack);
        inventory_updates.single_write(InventoryUpdateEvent {
            slots: smallvec![arrow_slot],
            player: entity,
        });
    }
    */

    drop(inventory); // Inventory no longer used.

    let _arrow_type: Item = match arrow_to_consume {
        None => Item::Arrow, // Default to generic arrow in creative mode with none in inventory
        Some((_, arrow_stack)) => arrow_stack.ty,
    };

    let timed_use = world.try_get::<ItemTimedUse>(player);

    // Spam clicking can lead to a scenario where this system is called before the UseItem system adds the component
    // In that case just return.
    if timed_use.is_none() {
        return;
    }

    let timed_use = timed_use.unwrap();

    let mut time_held = game.tick_count - timed_use.tick_start;

    if time_held > 20 {
        time_held = 20;
    }

    let charge_force = charge_from_ticks_held(time_held as u32);
    trace!("Held for {} ticks. Force of {}", time_held, charge_force);

    let init_position = *world.get::<Position>(player) + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);

    let direction = init_position.direction();

    let arrow_velocity = compute_projectile_velocity(
        glm::vec3(direction.x, direction.y, direction.z),
        charge_force as f64,
        0.0,
        &mut *game.rng(),
    );
    trace!(
        "Computed exit velocity: {}. Velocity is norm {}",
        arrow_velocity,
        arrow_velocity.norm()
    );

    drop(timed_use);

    world.remove::<ItemTimedUse>(player).unwrap();

    trace!("Spawning arrow entity.");
    arrow::spawn_arrow(game, world, arrow_velocity, init_position);
}

fn find_arrow(inventory: &EntityInventory) -> Option<(SlotIndex, ItemStack)> {
    // Order of priority is: off-hand, hotbar (0 to 8), rest of inventory

    if let Some(offhand) = inventory.item_at(SLOT_OFFHAND) {
        if is_arrow_item(offhand.ty) {
            return Some((SLOT_OFFHAND, *offhand));
        }
    }

    for hotbar_slot in 0..9 {
        if let Some(hotbar_stack) = inventory.item_at(SLOT_HOTBAR_OFFSET + hotbar_slot) {
            if is_arrow_item(hotbar_stack.ty) {
                return Some((SLOT_HOTBAR_OFFSET + hotbar_slot, *hotbar_stack));
            }
        }
    }

    for inv_slot in 9..=35 {
        if let Some(inv_stack) = inventory.item_at(inv_slot) {
            if is_arrow_item(inv_stack.ty) {
                return Some((inv_slot, *inv_stack));
            }
        }
    }
    None
}

fn is_arrow_item(item: Item) -> bool {
    match item {
        Item::Arrow | Item::SpectralArrow | Item::TippedArrow => true,
        _ => false,
    }
}
