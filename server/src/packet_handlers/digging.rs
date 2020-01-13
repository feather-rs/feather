//! This module handles the monolithic Player Digging packet.
//!
//! The packet's name is rather misleading, as it is also sent
//! for actions mostly unrelated to digging including eating, shooting bows,
//! swapping items out to the offhand, and dropping items.

use crate::block::BlockUpdateCause;
use crate::network::PacketQueue;
use crate::p_inventory::EntityInventory;
use crate::state::State;
use crate::util::disconnect_player;
use feather_core::network::packet::implementation::{PlayerDigging, PlayerDiggingStatus};
use feather_core::{Block, Gamemode, Item, ItemStack, Position};
use legion::entity::Entity;
use legion::query::{Read, Write};
use tonks::{PreparedWorld, Query};

/// System responsible for polling for PlayerDigging
/// packets and writing the corresponding events.
#[system]
fn handle_player_digging(
    state: &State,
    queue: &PacketQueue,
    _query: &mut Query<(Write<EntityInventory>, Read<Position>, Read<Gamemode>)>,
    world: &mut PreparedWorld,
) {
    use PlayerDiggingStatus::*;

    let packets = queue.received::<PlayerDigging>();

    for (player, packet) in packets {
        let gamemode = *world.get_component::<Gamemode>(player).unwrap();
        let inventory = world.get_component_mut::<EntityInventory>(player).unwrap();

        match packet.status {
            StartedDigging | FinishedDigging | CancelledDigging => handle_digging(
                packet,
                state,
                player,
                gamemode,
                inventory.item_in_main_hand(),
            ),
            /*
            DropItem | DropItemStack => handle_drop_item_stack(
                packet,
                player,
                &mut inventory_updates,
                &mut item_drops,
                inventories.get_mut(player).unwrap(),
            ),
            ConsumeItem => handle_consume_item(
                packet,
                players.get(player).unwrap(),
                player,
                inventories.get_mut(player).unwrap(),
                &mut inventory_updates,
                positions.get(player).unwrap().current,
                &mut shoot_arrow_events,
            ),
            */
            status => warn!("Unhandled Player Digging status {:?}", status),
        }
    }
}

fn handle_digging(
    packet: PlayerDigging,
    state: &State,
    player: Entity,
    gamemode: Gamemode,
    item_in_main_hand: Option<&ItemStack>,
) {
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

    // Don't break block if player is holding a sword in creative mode.
    if gamemode == Gamemode::Creative {
        if let Some(item_in_main_hand) = item_in_main_hand {
            match item_in_main_hand.ty {
                Item::WoodenSword
                | Item::StoneSword
                | Item::GoldenSword
                | Item::IronSword
                | Item::DiamondSword => return,
                _ => (),
            }
        }
    }

    if !state.set_block_at(
        packet.location,
        Block::Air,
        BlockUpdateCause::Player(player),
    ) {
        disconnect_player(state, player, "Attempted to break block in unloaded chunk");
        return;
    }
}

/*
fn handle_drop_item_stack(
    packet: &PlayerDigging,
    entity: Entity,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    item_drops: &mut EventChannel<PlayerItemDropEvent>,
    inventory: &mut InventoryComponent,
) {
    assert!(
        packet.status == PlayerDiggingStatus::DropItem
            || packet.status == PlayerDiggingStatus::DropItemStack
    );

    let slot = inventory.held_item + SLOT_HOTBAR_OFFSET;

    let stack = {
        if let Some(item) = inventory.item_at(slot) {
            item.clone()
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
    inventory_updates.single_write(inv_update);

    if amnt != 0 {
        let item_drop = PlayerItemDropEvent {
            slot: Some(slot),
            stack: ItemStack::new(stack.ty, amnt),
            player: entity,
        };
        item_drops.single_write(item_drop);
    }
}

/// Handles food consumption and shooting arrows.
fn handle_consume_item(
    packet: &PlayerDigging,
    player: &PlayerComponent,
    entity: Entity,
    inventory: &mut InventoryComponent,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    position: Position,
    shoot_arrow_events: &mut EventChannel<ShootArrowEvent>,
) {
    assert_eq!(packet.status, PlayerDiggingStatus::ConsumeItem);

    // TODO: Fallback to off-hand if main-hand is not a consumable
    let used_item = inventory.item_in_main_hand();

    if let Some(item) = used_item {
        if item.ty == Item::Bow {
            handle_shoot_bow(
                player,
                entity,
                inventory,
                inventory_updates,
                position,
                shoot_arrow_events,
            );
        }
        // TODO: Food, potions
    }
}

fn handle_shoot_bow(
    player: &PlayerComponent,
    entity: Entity,
    inventory: &mut InventoryComponent,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    position: Position,
    shoot_arrow_events: &mut EventChannel<ShootArrowEvent>,
) {
    let arrow_to_consume: Option<(SlotIndex, ItemStack)> = find_arrow(&inventory);
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

    let arrow_type: Item = match arrow_to_consume {
        None => Item::Arrow, // Default to generic arrow in creative mode with none in inventory
        Some((_, arrow_stack)) => arrow_stack.ty,
    };

    shoot_arrow_events.single_write(ShootArrowEvent {
        shooter: Some(entity),
        position,
        arrow_type,
        critical: false, // TODO: Determine critical based on how long bow was pulled back
    });
}

fn find_arrow(inventory: &InventoryComponent) -> Option<(SlotIndex, ItemStack)> {
    // Order of priority is: off-hand, hotbar (0 to 8), rest of inventory

    if let Some(offhand) = inventory.item_at(SLOT_OFFHAND) {
        if is_arrow_item(offhand.ty) {
            return Some((SLOT_OFFHAND, offhand.clone()));
        }
    }

    for hotbar_slot in 0..9 {
        if let Some(hotbar_stack) = inventory.item_at(SLOT_HOTBAR_OFFSET + hotbar_slot) {
            if is_arrow_item(hotbar_stack.ty) {
                return Some((SLOT_HOTBAR_OFFSET + hotbar_slot, hotbar_stack.clone()));
            }
        }
    }

    for inv_slot in 9..=35 {
        if let Some(inv_stack) = inventory.item_at(inv_slot) {
            if is_arrow_item(inv_stack.ty) {
                return Some((inv_slot, inv_stack.clone()));
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
*/
