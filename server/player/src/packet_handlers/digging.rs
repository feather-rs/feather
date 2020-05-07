//! This module handles the monolithic Player Digging packet.
//!
//! The packet's name is rather misleading, as it is also sent
//! for actions mostly unrelated to digging including eating, shooting bows,
//! swapping items out to the offhand, and dropping items.

use crate::{ItemTimedUse, IteratorExt};
use entity::InventoryExt;
use feather_core::blocks::BlockId;
use feather_core::inventory::{Inventory, SlotIndex, SLOT_HOTBAR_OFFSET, SLOT_OFFHAND};
use feather_core::items::{Item, ItemStack};
use feather_core::network::packets::{PlayerDigging, PlayerDiggingStatus};
use feather_core::util::{BlockPosition, Position};
use feather_server_types::{
    BlockUpdateCause, CanBreak, CanInstaBreak, EntitySpawnEvent, Game, HeldItem,
    InventoryUpdateEvent, ItemDropEvent, PacketBuffers, Velocity, PLAYER_EYE_HEIGHT,
};
use feather_server_util::{charge_from_ticks_held, compute_projectile_velocity};
use fecs::{Entity, World};
use smallvec::smallvec;
use std::sync::Arc;

/// Stores the "digging status" of a player.
///
/// If this component exists for an entity,
/// then it is currently digging a block. The
/// corresponding animation must be displayed.
#[derive(Copy, Clone, Debug)]
pub struct Digging {
    /// The position of the block being dug
    pos: BlockPosition,
}

/// System responsible for polling for PlayerDigging
/// packets and writing the corresponding events.
#[fecs::system]
pub fn handle_player_digging(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    use PlayerDiggingStatus::*;

    packet_buffers
        .received::<PlayerDigging>()
        .for_each_valid(world, |world, (player, packet)| match packet.status {
            StartedDigging | FinishedDigging | CancelledDigging => {
                handle_digging(game, world, player, packet)
            }
            DropItem | DropItemStack => handle_drop_item_stack(game, world, player, packet),
            ConsumeItem => handle_consume_item(game, world, player, packet),
            status => log::warn!("Unhandled Player Digging status {:?}", status),
        });
}

fn handle_digging(game: &mut Game, world: &mut World, player: Entity, packet: PlayerDigging) {
    if !world.has::<CanBreak>(player) {
        log::trace!(
            "Player cannot break blocks but sent player digging status {:?}",
            packet.status
        );
        return;
    }

    match packet.status {
        PlayerDiggingStatus::StartedDigging => handle_started_digging(game, world, player, packet),
        PlayerDiggingStatus::CancelledDigging => handle_cancelled_digging(world, player),
        PlayerDiggingStatus::FinishedDigging => {
            handle_finished_digging(game, world, player, packet)
        }
        _ => unreachable!(),
    }
}

const MAX_DIG_RADIUS_SQUARED: f64 = 36.0;

fn handle_started_digging(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: PlayerDigging,
) {
    // Delete old `Digging`, if it exists
    let _ = world.remove::<Digging>(player);

    // Check the distance isn't too far.
    if packet
        .location
        .position()
        .distance_squared_to(*world.get::<Position>(player))
        > MAX_DIG_RADIUS_SQUARED
    {
        // Ignore the packet.
        log::trace!("player {:?} tried to dig too far", player);
        return;
    }

    // If the player can insta-break, then they can already break the block.
    if world.has::<CanInstaBreak>(player) {
        dig(game, world, player, packet.location);
    } else {
        // Insert new `Digging`.
        world
            .add(
                player,
                Digging {
                    pos: packet.location,
                },
            )
            .unwrap();
    }
}

fn handle_cancelled_digging(world: &mut World, player: Entity) {
    let _ = world.remove::<Digging>(player);
}

fn handle_finished_digging(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: PlayerDigging,
) {
    let digging = match world.try_get::<Digging>(player) {
        Some(digging) => *digging,
        None => {
            if world.has::<CanInstaBreak>(player) {
                // Can insta-break - no `StartedDigging` needed
                Digging {
                    pos: packet.location,
                }
            } else {
                // Player can't insta-break and has
                // not sent StartedDigging.
                // They cannot finish.
                return;
            }
        }
    };

    let _ = world.remove::<Digging>(player);

    if digging.pos != packet.location {
        return;
    }

    // Attempt to break the block
    dig(game, world, player, digging.pos);
}

fn dig(game: &mut Game, world: &mut World, player: Entity, pos: BlockPosition) {
    if !game.set_block_at(world, pos, BlockId::air(), BlockUpdateCause::Entity(player)) {
        game.disconnect(
            player,
            world,
            format!(
                "Attempted to break block in unloaded chunk (position: {:?})",
                pos
            ),
        );
    }
}

fn handle_drop_item_stack(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: PlayerDigging,
) {
    assert!(
        packet.status == PlayerDiggingStatus::DropItem
            || packet.status == PlayerDiggingStatus::DropItemStack
    );

    let held_item = world.get::<HeldItem>(player).0;
    let mut inventory = world.get_mut::<Inventory>(player);

    let slot = held_item + SLOT_HOTBAR_OFFSET;

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

    drop(inventory);

    let inv_update = InventoryUpdateEvent {
        slots: smallvec![slot],
        player,
    };
    game.handle(world, inv_update);

    if amnt != 0 {
        let item_drop = ItemDropEvent {
            slot: Some(slot),
            stack: ItemStack::new(stack.ty, amnt),
            player,
        };
        game.handle(world, item_drop);
    }
}

/// Handles food consumption and shooting arrows.
fn handle_consume_item(game: &mut Game, world: &mut World, player: Entity, packet: PlayerDigging) {
    assert_eq!(packet.status, PlayerDiggingStatus::ConsumeItem);

    // TODO: Fallback to off-hand if main-hand is not a consumable
    let inventory = world.get::<Inventory>(player);
    let used_item = inventory.item_in_main_hand(player, world);

    if let Some(item) = used_item {
        if item.ty == Item::Bow {
            drop(inventory);
            handle_shoot_bow(game, world, player);
        }
        // TODO: Food, potions
    }
}

fn handle_shoot_bow(game: &mut Game, world: &mut World, player: Entity) {
    let inventory = world.get::<Inventory>(player);
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
    log::trace!("Held for {} ticks. Force of {}", time_held, charge_force);

    let init_position = *world.get::<Position>(player) + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);

    let direction = init_position.direction();

    let arrow_velocity = compute_projectile_velocity(
        glm::vec3(direction.x, direction.y, direction.z),
        charge_force as f64,
        0.0,
        &mut *game.rng(),
    );
    log::trace!(
        "Computed exit velocity: {}. Velocity is norm {}",
        arrow_velocity,
        arrow_velocity.norm()
    );

    drop(timed_use);

    world.remove::<ItemTimedUse>(player).unwrap();

    log::trace!("Spawning arrow entity.");
    let entity = entity::arrow::create()
        .with(init_position)
        .with(Velocity(arrow_velocity))
        .build()
        .spawn_in(world);
    game.handle(world, EntitySpawnEvent { entity });
}

fn find_arrow(inventory: &Inventory) -> Option<(SlotIndex, ItemStack)> {
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
