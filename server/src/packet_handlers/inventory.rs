//! Handling of inventory update packets.
//! This currently includes Creative Inventory Action and Held Item Change.

use crate::entity::item::ItemDropEvent;
use crate::game::Game;
use crate::p_inventory::{EntityInventory, InventoryUpdateEvent};
use crate::packet_buffer::PacketBuffers;
use feather_core::inventory::{HOTBAR_SIZE, SLOT_HOTBAR_OFFSET};
use feather_core::network::packet::implementation::{
    CreativeInventoryAction, HeldItemChangeServerbound,
};
use feather_core::Gamemode;
use fecs::World;
use std::sync::Arc;

/// System for handling Creative Inventory Action packets.
#[system]
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
                    game.on_item_drop(world, event);

                    // No need to update inventory
                    continue;
                }
                None => (),
            }
        }

        let inventory = world.get::<EntityInventory>(player);
        let slot_count = inventory.slot_count() as i16;
        drop(inventory);

        if packet.slot >= slot_count || packet.slot < -1 {
            game.disconnect(player, world, "Slot index out of bounds");
            continue;
        }

        let mut inventory = world.get_mut::<EntityInventory>(player);

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
            slots: smallvec![packet.slot as usize],
            player,
        };
        drop(inventory);
        game.on_inventory_update(world, event);
    }
}

/// System for handling Held Item Change packets.
#[system]
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

        let mut inventory = world.get_mut::<EntityInventory>(player);
        inventory.held_item = packet.slot as usize;

        // Trigger event
        let event = InventoryUpdateEvent {
            slots: smallvec![inventory.held_item as usize + SLOT_HOTBAR_OFFSET],
            player,
        };
        drop(inventory);
        game.on_inventory_update(world, event);
    }
}
