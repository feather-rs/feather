//! Handling of inventory update packets.
//! This currently includes Creative Inventory Action and Held Item Change.

use feather_core::inventory::{Inventory, HOTBAR_SIZE, SLOT_HOTBAR_OFFSET};
use feather_core::network::packets::{CreativeInventoryAction, HeldItemChangeServerbound};
use feather_core::util::Gamemode;
use feather_server_types::{Game, HeldItem, InventoryUpdateEvent, ItemDropEvent, PacketBuffers};
use fecs::World;
use std::sync::Arc;

/// System for handling Creative Inventory Action packets.
#[fecs::system]
pub fn handle_creative_inventory_action(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    let packets = packet_buffers.received::<CreativeInventoryAction>();

    for (player, packet) in packets {
        // Creative Inventory Action can only be used in creative
        // mode.
        let gamemode = *world.get::<Gamemode>(player);
        if gamemode != Gamemode::Creative {
            game.disconnect(
                player,
                world,
                "attempted to use Creative Inventory Action outside of creative mode",
            );
            continue;
        }

        // Slot -1 means that the user clicked outside the window,
        // dropping the item.
        if packet.slot == -1 {
            match &packet.clicked_item {
                Some(stack) => {
                    // Cause item to be dropped
                    let event = ItemDropEvent {
                        slot: None,
                        stack: *stack,
                        player,
                    };
                    game.handle(world, event);

                    // No need to update inventory
                    continue;
                }
                None => (),
            }
        }

        let inventory = world.get::<Inventory>(player);
        let slot_count = inventory.slot_count() as i16;
        drop(inventory);

        if packet.slot >= slot_count || packet.slot < -1 {
            game.disconnect(player, world, "Slot index out of bounds");
            continue;
        }

        let mut inventory = world.get_mut::<Inventory>(player);

        match packet.clicked_item.as_ref() {
            Some(item) => {
                inventory.set_item_at(packet.slot as usize, item.clone());
            }
            None => {
                inventory.clear_item_at(packet.slot as usize);
            }
        }

        // Trigger inventory update event
        let event = InventoryUpdateEvent {
            slots: std::iter::once(packet.slot as usize).collect(),
            player,
        };
        drop(inventory);
        game.handle(world, event);
    }
}

/// System for handling Held Item Change packets.
#[fecs::system]
pub fn handle_held_item_change(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    let packets = packet_buffers.received::<HeldItemChangeServerbound>();

    for (player, packet) in packets {
        if packet.slot as usize >= HOTBAR_SIZE {
            game.disconnect(player, world, "Hotbar index out of bounds");
            continue;
        }

        let mut held_item = world.get_mut::<HeldItem>(player);
        held_item.0 = packet.slot as usize;

        // Trigger event
        let event = InventoryUpdateEvent {
            slots: std::iter::once(held_item.0 as usize + SLOT_HOTBAR_OFFSET).collect(),
            player,
        };
        drop(held_item);
        game.handle(world, event);
    }
}
