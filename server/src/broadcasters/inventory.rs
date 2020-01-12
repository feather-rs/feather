//! Broadcasting of inventory-related events.

use crate::entity::{EntityId, EntitySendEvent};
use crate::network::Network;
use crate::p_inventory::{EntityInventory, Equipment, InventoryUpdateEvent};
use crate::state::State;
use feather_core::inventory::{SlotIndex, SLOT_HOTBAR_OFFSET};
use feather_core::network::packet::implementation::{EntityEquipment, SetSlot};
use legion::query::Read;
use num_traits::ToPrimitive;
use tonks::{PreparedWorld, Query};

/// System for broadcasting equipment updates.
#[event_handler]
fn broadcast_equipment_updates(
    event: &InventoryUpdateEvent,
    state: &State,
    _query: &mut Query<(Read<EntityId>, Read<EntityInventory>)>,
    world: &mut PreparedWorld,
) {
    let inv = world
        .get_component::<EntityInventory>(event.player)
        .unwrap();

    for slot in &event.slots {
        // Skip this slot if it is not an equipment update.
        if let Ok(equipment) = is_equipment_update(&inv, *slot) {
            let slot = equipment.slot_index(inv.held_item);
            let item = inv.item_at(slot).cloned();

            let packet = EntityEquipment {
                entity_id: world.get_component::<EntityId>(event.player).unwrap().0,
                slot: equipment.to_i32().unwrap(),
                item,
            };

            state.broadcast_entity_update(event.player, packet, Some(event.player));
        }
    }
}

/// System which listens to `EntitySendEvent`s and
/// sends entity equipment alongside.
#[event_handler]
fn send_entity_equipment(
    event: &EntitySendEvent,
    _query: &mut Query<(Read<EntityId>, Read<EntityInventory>, Read<Network>)>,
    world: &mut PreparedWorld,
) {
    let network = world.get_component::<Network>(event.to).unwrap();
    let inventory = match world.get_component::<EntityInventory>(event.entity) {
        Some(inv) => inv,
        None => return,
    };

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
            let slot = equipment.slot_index(inventory.held_item);
            inventory.item_at(slot).cloned()
        };

        let equipment_slot = equipment.to_i32().unwrap();

        let packet = EntityEquipment {
            entity_id: world.get_component::<EntityId>(event.entity).unwrap().0,
            slot: equipment_slot,
            item,
        };
        network.send(packet);
    }
}

/// System for sending the Set Slot packet
/// when a player's inventory is updated.
#[event_handler]
fn send_set_slot(
    event: &InventoryUpdateEvent,
    _query: &mut Query<(Read<EntityInventory>, Read<Network>)>,
    world: &mut PreparedWorld,
) {
    let inv = world
        .get_component::<EntityInventory>(event.player)
        .unwrap();
    let network = world.get_component::<Network>(event.player).unwrap();

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
fn is_equipment_update(inv: &EntityInventory, slot: SlotIndex) -> Result<Equipment, ()> {
    if slot >= SLOT_HOTBAR_OFFSET && slot - SLOT_HOTBAR_OFFSET == inv.held_item {
        Ok(Equipment::MainHand)
    } else if let Some(equipment) = Equipment::from_slot_index(slot) {
        Ok(equipment)
    } else {
        Err(())
    }
}
