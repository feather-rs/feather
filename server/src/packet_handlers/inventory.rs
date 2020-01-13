//! Handling of inventory update packets.
//! This currently includes Creative Inventory Action and Held Item Change.

use crate::entity::item::ItemDropEvent;
use crate::network::PacketQueue;
use crate::p_inventory::{EntityInventory, InventoryUpdateEvent};
use crate::state::State;
use crate::util::disconnect_player;
use feather_core::inventory::{HOTBAR_SIZE, SLOT_HOTBAR_OFFSET};
use feather_core::network::packet::implementation::{
    CreativeInventoryAction, HeldItemChangeServerbound,
};
use feather_core::Gamemode;
use legion::prelude::Read;
use legion::query::Write;
use tonks::{PreparedWorld, Query, Trigger};

/// System for handling Creative Inventory Action packets.
#[system]
fn handle_creative_inventory_action(
    state: &State,
    queue: &PacketQueue,
    _query: &mut Query<(Read<Gamemode>, Write<EntityInventory>)>,
    world: &mut PreparedWorld,
    trigger_inventory: &mut Trigger<InventoryUpdateEvent>,
    trigger_drop: &mut Trigger<ItemDropEvent>,
) {
    let packets = queue.received::<CreativeInventoryAction>();

    for (player, packet) in packets {
        // Creative Inventory Action can only be used in creative
        // mode.
        let gamemode = *world.get_component::<Gamemode>(player).unwrap();
        if gamemode != Gamemode::Creative {
            disconnect_player(
                state,
                player,
                "Attempted to use Creative Inventory Action while not in creative mode",
            );
            continue;
        }

        let mut inventory = world.get_component_mut::<EntityInventory>(player).unwrap();

        // Slot -1 means that the user clicked outside the window,
        // dropping the item.
        if packet.slot == -1 {
            match &packet.clicked_item {
                Some(stack) => {
                    // Cause item to be dropped
                    let event = ItemDropEvent {
                        slot: None,
                        stack: stack.clone(),
                        player,
                    };
                    trigger_drop.trigger(event);

                    // No need to update inventory
                    continue;
                }
                None => (),
            }
        }

        if packet.slot >= inventory.slot_count() as i16 || packet.slot < -1 {
            disconnect_player(state, player, "Slot index out of bounds");
            continue;
        }

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
        trigger_inventory.trigger(event);
    }
}

/// System for handling Held Item Change packets.
#[system]
fn handle_held_item_change(
    state: &State,
    queue: &PacketQueue,
    _query: &mut Query<Write<EntityInventory>>,
    world: &mut PreparedWorld,
    trigger: &mut Trigger<InventoryUpdateEvent>,
) {
    let packets = queue.received::<HeldItemChangeServerbound>();

    for (player, packet) in packets {
        if packet.slot as usize >= HOTBAR_SIZE {
            disconnect_player(state, player, "Hotbar index out of bounds");
            continue;
        }

        let mut inventory = world.get_component_mut::<EntityInventory>(player).unwrap();
        inventory.held_item = packet.slot as usize;

        // Trigger event
        let event = InventoryUpdateEvent {
            slots: smallvec![inventory.held_item as usize + SLOT_HOTBAR_OFFSET],
            player,
        };
        trigger.trigger(event);
    }
}
