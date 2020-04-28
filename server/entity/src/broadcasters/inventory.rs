//! Broadcasting of inventory-related events.

use crate::inventory::Equipment;
use feather_core::inventory::{Inventory, SlotIndex, SLOT_HOTBAR_OFFSET};
use feather_core::network::packets::{EntityEquipment, SetSlot};
use feather_server_types::{
    EntityId, EntitySendEvent, Game, HeldItem, InventoryUpdateEvent, Network,
};
use fecs::World;
use num_traits::ToPrimitive;

/// System for broadcasting equipment updates.
#[fecs::event_handler]
pub fn on_inventory_update_broadcast_equipment_update(
    event: &InventoryUpdateEvent,
    game: &mut Game,
    world: &mut World,
) {
    let inv = world.get::<Inventory>(event.player);
    let held_item = world.get::<HeldItem>(event.player);

    for slot in &event.slots {
        // Skip this slot if it is not an equipment update.
        if let Ok(equipment) = is_equipment_update(held_item.0, *slot) {
            let slot = equipment.slot_index(held_item.0);
            let item = inv.item_at(slot).cloned();

            let packet = EntityEquipment {
                entity_id: world.get::<EntityId>(event.player).0,
                slot: equipment.to_i32().unwrap(),
                item,
            };

            game.broadcast_entity_update(world, packet, event.player, Some(event.player));
        }
    }
}

/// System to send an entity's equipment when the
/// entity is sent to a client.
#[fecs::event_handler]
pub fn on_entity_send_send_equipment(event: &EntitySendEvent, world: &mut World) {
    let client = event.client;
    let entity = event.entity;
    if !world.is_alive(client) || !world.is_alive(entity) {
        return;
    }

    let network = world.get::<Network>(client);
    let inventory = match world.try_get::<Inventory>(entity) {
        Some(inv) => inv,
        None => return, // no equipment to send
    };
    let held_item = world.get::<HeldItem>(entity);

    let equipments = [
        Equipment::MainHand,
        Equipment::Boots,
        Equipment::Leggings,
        Equipment::Chestplate,
        Equipment::Helmet,
        Equipment::OffHand,
    ];

    for equipment in equipments.iter() {
        let item = {
            let slot = equipment.slot_index(held_item.0);
            inventory.item_at(slot).cloned()
        };

        let equipment_slot = equipment.to_i32().unwrap();

        let packet = EntityEquipment {
            entity_id: world.get::<EntityId>(entity).0,
            slot: equipment_slot,
            item,
        };
        network.send(packet);
    }
}

/// System for sending the Set Slot packet
/// when a player's inventory is updated.
#[fecs::event_handler]
pub fn on_inventory_update_send_set_slot(event: &InventoryUpdateEvent, world: &mut World) {
    let inv = world.get::<Inventory>(event.player);
    let network = world.get::<Network>(event.player);

    for slot in &event.slots {
        let packet = SetSlot {
            window_id: 0,
            slot: *slot as i16,
            slot_data: inv.item_at(*slot as usize).cloned(),
        };

        network.send(packet);
    }
}

/// Returns whether the given update to an inventory
/// is an equipment update.
fn is_equipment_update(held_item: SlotIndex, slot: SlotIndex) -> Result<Equipment, ()> {
    if slot >= SLOT_HOTBAR_OFFSET && slot - SLOT_HOTBAR_OFFSET == held_item {
        Ok(Equipment::MainHand)
    } else if let Some(equipment) = Equipment::from_slot_index(slot) {
        Ok(equipment)
    } else {
        Err(())
    }
}
