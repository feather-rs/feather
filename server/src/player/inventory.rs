use crate::disconnect_player;
use crate::entity::PlayerComponent;
use crate::joinhandler::PlayerJoinEvent;
use crate::network::{send_packet_to_player, NetworkComponent, PacketQueue};
use crate::player::digging::PlayerItemDropEvent;
use crate::util::Util;
use feather_core::inventory::{
    Inventory, InventoryType, SlotIndex, HOTBAR_SIZE, SLOT_ARMOR_CHEST, SLOT_ARMOR_FEET,
    SLOT_ARMOR_HEAD, SLOT_ARMOR_LEGS, SLOT_HOTBAR_OFFSET, SLOT_OFFHAND,
};
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    CreativeInventoryAction, EntityEquipment, HeldItemChangeServerbound, SetSlot,
};
use feather_core::network::packet::PacketType;
use feather_core::{Gamemode, ItemStack};
use num_traits::ToPrimitive;
use shrev::EventChannel;
use smallvec::SmallVec;
use specs::{
    Component, Entities, Join, LazyUpdate, Read, ReadStorage, ReaderId, World, WriteStorage,
};
use specs::{DenseVecStorage, SystemData};
use specs::{Entity, System, Write};
use std::ops::{Deref, DerefMut};

/// Component for storing a player's inventory.
#[derive(Clone, Debug)]
pub struct InventoryComponent {
    inventory: Inventory,
    /// The player's held item.
    /// This is stored as an index in the range 0..9.
    pub held_item: SlotIndex,
}

impl InventoryComponent {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(InventoryType::Player, 46),
            held_item: 0,
        }
    }

    /// Returns the item in this inventory's
    /// main hand.
    pub fn item_in_main_hand(&self) -> Option<&ItemStack> {
        self.inventory.item_at(SLOT_HOTBAR_OFFSET + self.held_item)
    }

    /// Sets the item in this inventory's main hand.
    pub fn set_item_in_main_hand(&mut self, item: ItemStack) {
        self.inventory
            .set_item_at(SLOT_HOTBAR_OFFSET + self.held_item, item);
    }
}

impl Default for InventoryComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for InventoryComponent {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.inventory
    }
}

impl DerefMut for InventoryComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inventory
    }
}

impl Component for InventoryComponent {
    type Storage = DenseVecStorage<Self>;
}

/// An equipment slot, with variants
/// listed in the order of the Entity Equipment
/// IDs to allow for easy conversion using `ToPrimitive`/`FromPrimitive`.
#[derive(Debug, Clone, Copy, ToPrimitive, FromPrimitive, PartialEq, Eq, Hash)]
pub enum Equipment {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}

impl Equipment {
    pub fn from_slot_index(index: SlotIndex) -> Option<Self> {
        match index {
            SLOT_OFFHAND => Some(Equipment::OffHand),
            SLOT_ARMOR_FEET => Some(Equipment::Boots),
            SLOT_ARMOR_LEGS => Some(Equipment::Leggings),
            SLOT_ARMOR_CHEST => Some(Equipment::Chestplate),
            SLOT_ARMOR_HEAD => Some(Equipment::Helmet),
            _ => None,
        }
    }

    pub fn slot_index(self, held_item: SlotIndex) -> SlotIndex {
        match self {
            Equipment::MainHand => held_item + SLOT_HOTBAR_OFFSET,
            Equipment::OffHand => SLOT_OFFHAND,
            Equipment::Boots => SLOT_ARMOR_FEET,
            Equipment::Leggings => SLOT_ARMOR_LEGS,
            Equipment::Chestplate => SLOT_ARMOR_CHEST,
            Equipment::Helmet => SLOT_ARMOR_HEAD,
        }
    }
}

/// Event which is triggered when a player
/// updates their inventory.
///
/// This event could also be triggered when the player
/// changes their held item.
#[derive(Debug, Clone)]
pub struct InventoryUpdateEvent {
    /// The slot(s) affected by the update.
    ///
    /// Multiple slots could be affected when, for
    /// example, a player uses the "drag" inventory interaction.
    pub slots: SmallVec<[SlotIndex; 2]>,
    /// The player owning the updated inventory.
    pub player: Entity,
}

/// System for handling Creative Inventory Action packets.
pub struct CreativeInventorySystem;

impl<'a> System<'a> for CreativeInventorySystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        ReadStorage<'a, PlayerComponent>,
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        Write<'a, EventChannel<PlayerItemDropEvent>>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut inventories, players, mut update_events, mut drop_events, packet_queue, lazy) =
            data;

        let packets = packet_queue.for_packet(PacketType::CreativeInventoryAction);

        for (player, packet) in packets {
            // Creative Inventory Action can only be used in creative
            // mode.
            let player_comp = players.get(player).unwrap();
            if player_comp.gamemode != Gamemode::Creative {
                disconnect_player(
                    player,
                    "Attempted to use Creative Inventory Action while not in creative mode"
                        .to_string(),
                    &lazy,
                );
                continue;
            }

            let packet = cast_packet::<CreativeInventoryAction>(&*packet);

            let inventory = inventories.get_mut(player).unwrap();

            // Slot -1 means that the user clicked outside the window,
            // dropping the item.
            if packet.slot == -1 {
                match &packet.clicked_item {
                    Some(stack) => {
                        let event = PlayerItemDropEvent {
                            slot: None,
                            stack: stack.clone(),
                            player,
                        };
                        drop_events.single_write(event);

                        // No need to update inventory
                        continue;
                    }
                    None => (),
                }
            }

            if packet.slot >= inventory.slot_count() as i16 || packet.slot < -1 {
                disconnect_player(player, "Slot index out of bounds".to_string(), &lazy);
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
            update_events.single_write(event);
        }
    }
}

/// System for handling Held Item Change packets.
pub struct HeldItemChangeSystem;

impl<'a> System<'a> for HeldItemChangeSystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut inventories, mut events, packet_queue, lazy) = data;

        let packets = packet_queue.for_packet(PacketType::HeldItemChangeServerbound);

        for (player, packet) in packets {
            let packet = cast_packet::<HeldItemChangeServerbound>(&*packet);

            if packet.slot as usize >= HOTBAR_SIZE {
                disconnect_player(player, "Hotbar index out of bounds".to_string(), &lazy);
                continue;
            }

            let inventory = inventories.get_mut(player).unwrap();
            inventory.held_item = packet.slot as usize;

            // Trigger event
            let event = InventoryUpdateEvent {
                slots: smallvec![inventory.held_item as usize + SLOT_HOTBAR_OFFSET],
                player,
            };
            events.single_write(event);
        }
    }
}

/// System for broadcasting equipment updates.
#[derive(Default)]
pub struct HeldItemBroadcastSystem {
    reader: Option<ReaderId<InventoryUpdateEvent>>,
}

impl<'a> System<'a> for HeldItemBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, InventoryComponent>,
        Read<'a, EventChannel<InventoryUpdateEvent>>,
        Read<'a, Util>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (inventories, events, util) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let inv = inventories.get(event.player).unwrap();

            for slot in &event.slots {
                // Skip this slot if it is not an equipment update.
                if let Ok(equipment) = is_equipment_update(&inv, *slot) {
                    let slot = equipment.slot_index(inv.held_item);
                    let item = inv.item_at(slot).cloned();

                    let packet = EntityEquipment::new(
                        event.player.id() as i32,
                        equipment.to_i32().unwrap(),
                        item,
                    );

                    util.broadcast_entity_update(event.player, packet, Some(event.player));
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<InventoryUpdateEvent>>()
                .register_reader(),
        );
    }
}

/// System which sends other players' equipment
/// to players who have just joined.
#[derive(Default)]
pub struct EquipmentSendSystem {
    reader: Option<ReaderId<PlayerJoinEvent>>,
}

impl<'a> System<'a> for EquipmentSendSystem {
    type SystemData = (
        ReadStorage<'a, InventoryComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<PlayerJoinEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (inventories, networks, events, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let network = networks.get(event.player).unwrap();

            // Send all other players' equipment to this player.
            for (inventory, entity) in (&inventories, &entities).join() {
                // Don't send player their own equipment
                if entity == event.player {
                    continue;
                }

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

                    let packet = EntityEquipment::new(entity.id() as i32, equipment_slot, item);
                    send_packet_to_player(network, packet);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

/// System for sending the Set Slot packet
/// when a player's inventory is updated.
#[derive(Default)]
pub struct SetSlotSystem {
    reader: Option<ReaderId<InventoryUpdateEvent>>,
}

impl<'a> System<'a> for SetSlotSystem {
    type SystemData = (
        ReadStorage<'a, InventoryComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<InventoryUpdateEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (inventories, networks, events) = data;

        for event in events.read(self.reader.as_mut().unwrap()) {
            let inv = inventories.get(event.player).unwrap();
            let network = networks.get(event.player).unwrap();

            for slot in &event.slots {
                let packet = SetSlot {
                    window_id: 0,
                    slot: *slot as i16,
                    slot_data: inv.item_at(*slot as usize).cloned(),
                };

                send_packet_to_player(&network, packet);
            }
        }
    }

    setup_impl!(reader);
}

/// Returns whether the given update to an inventory
/// is an equipment update.
fn is_equipment_update(inv: &InventoryComponent, slot: SlotIndex) -> Result<Equipment, ()> {
    if slot >= SLOT_HOTBAR_OFFSET && slot - SLOT_HOTBAR_OFFSET == inv.held_item {
        Ok(Equipment::MainHand)
    } else if let Some(equipment) = Equipment::from_slot_index(slot) {
        Ok(equipment)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::inventory::{ItemStack, SLOT_ENTITY_EQUIPMENT_MAIN_HAND};
    use feather_core::item::Item;
    use specs::WorldExt;

    #[test]
    fn test_creative_inventory_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let packet = CreativeInventoryAction::new(
            SLOT_HOTBAR_OFFSET as i16,
            Some(ItemStack::new(Item::IronSword, 1)),
        );

        t::receive_packet(&player, &w, packet);

        let mut update_reader = t::reader(&w);
        let mut drop_reader = t::reader(&w);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let inv_storage = w.read_component::<InventoryComponent>();
        let inv = inv_storage.get(player.entity).unwrap();
        assert_eq!(
            inv.item_at(SLOT_HOTBAR_OFFSET).unwrap(),
            &ItemStack::new(Item::IronSword, 1)
        );

        // Confirm that event was triggered
        {
            let channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();
            let events = channel.read(&mut update_reader).collect::<Vec<_>>();
            assert_eq!(events.len(), 1);
            let first = events.first().unwrap();
            assert_eq!(first.player, player.entity);
            assert_eq!(first.slots.as_slice(), &[SLOT_HOTBAR_OFFSET]);

            assert!(w
                .fetch::<EventChannel<PlayerItemDropEvent>>()
                .read(&mut drop_reader)
                .next()
                .is_none());
        }

        drop(inv_storage);

        let packet = CreativeInventoryAction::new(0, None);

        t::receive_packet(&player, &w, packet.clone());

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let inv_storage = w.read_component::<InventoryComponent>();
        let inv = inv_storage.get(player.entity).unwrap();
        assert_eq!(inv.item_at(0), None);

        drop(inv_storage);

        // Now with a survival mode player...
        w.write_component::<PlayerComponent>()
            .get_mut(player.entity)
            .unwrap()
            .gamemode = Gamemode::Survival;

        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_disconnected(&player);
    }

    #[test]
    fn test_creative_inventory_slot_out_of_bounds() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let packet = CreativeInventoryAction::new(46, Some(ItemStack::new(Item::IronSword, 1)));
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_disconnected(&player);
    }

    #[test]
    fn test_creative_inventory_armor() {
        let equipments = [
            Equipment::OffHand,
            Equipment::Boots,
            Equipment::Leggings,
            Equipment::Chestplate,
            Equipment::Helmet,
        ];

        for equipment in equipments.iter() {
            let (mut w, mut d) = t::init_world();

            let player = t::add_player(&mut w);

            let mut event_reader = t::reader(&w);

            let packet = CreativeInventoryAction::new(
                equipment.slot_index(0) as i16,
                Some(ItemStack::new(Item::IronSword, 1)),
            );
            t::receive_packet(&player, &w, packet);

            d.dispatch(&w);
            w.maintain();

            let ch = w.fetch::<EventChannel<InventoryUpdateEvent>>();
            let events = ch.read(&mut event_reader).collect::<Vec<_>>();

            assert_eq!(events.len(), 1);
            let first = events.first().unwrap();

            assert_eq!(first.slots.as_slice(), &[equipment.slot_index(0)]);
            assert_eq!(first.player, player.entity);
        }
    }

    #[test]
    fn test_creative_inventory_system_drop_item() {
        // Drop item - slot index -1
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let stack = ItemStack::new(Item::CookedBeef, 1);

        let mut drop_reader = t::reader(&w);
        let mut update_reader = t::reader(&w);

        let packet = CreativeInventoryAction {
            slot: -1,
            clicked_item: Some(stack.clone()),
        };
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();
        let events = channel.read(&mut update_reader).collect::<Vec<_>>();
        assert!(events.is_empty());

        let channel = w.fetch::<EventChannel<PlayerItemDropEvent>>();
        let events = channel.read(&mut drop_reader).collect::<Vec<_>>();

        assert_eq!(events.len(), 1);
        let first = events.first().unwrap();
        assert_eq!(first.stack, stack);
        assert_eq!(first.player, player.entity);
        assert_eq!(first.slot, None);
    }

    #[test]
    fn test_held_item_change_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let slot = 4;

        let mut event_reader = t::reader(&w);

        let packet = HeldItemChangeServerbound::new(slot);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        let channel = w.fetch::<EventChannel<InventoryUpdateEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();

        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(
            first.slots.as_slice(),
            &[slot as usize + SLOT_HOTBAR_OFFSET]
        );

        let inventories = w.read_component::<InventoryComponent>();
        let inv = inventories.get(player.entity).unwrap();

        assert_eq!(inv.held_item, slot as usize);
    }

    #[test]
    fn test_held_item_change_system_out_of_bounds() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let slot = 9;

        let packet = HeldItemChangeServerbound::new(slot);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_disconnected(&player); // Slot out of bounds
    }

    #[test]
    fn test_held_item_broadcast_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        {
            let mut invs = w.write_component::<InventoryComponent>();
            let inv = invs.get_mut(player.entity).unwrap();
            inv.held_item = 0;
            inv.set_item_at(SLOT_HOTBAR_OFFSET, ItemStack::new(Item::IronSword, 1));
        }

        let event = InventoryUpdateEvent {
            player: player.entity,
            slots: smallvec![SLOT_HOTBAR_OFFSET],
        };

        w.fetch_mut::<EventChannel<InventoryUpdateEvent>>()
            .single_write(event);

        d.dispatch(&w);
        w.maintain();

        let packet = t::assert_packet_received(&player2, PacketType::EntityEquipment);

        let packet = cast_packet::<EntityEquipment>(&*packet);
        assert_eq!(packet.slot, SLOT_ENTITY_EQUIPMENT_MAIN_HAND as i32);
        assert_eq!(packet.entity_id, player.entity.id() as i32);
        assert_eq!(packet.item, Some(ItemStack::new(Item::IronSword, 1)));

        t::assert_packet_not_received(&player, PacketType::EntityEquipment);
    }

    #[test]
    fn test_equipment_send_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        let event = PlayerJoinEvent {
            player: player.entity,
        };

        w.fetch_mut::<EventChannel<_>>().single_write(event);

        {
            let mut invs = w.write_component::<InventoryComponent>();
            let inv = invs.get_mut(player2.entity).unwrap();

            inv.held_item = 1;
            inv.set_item_at(SLOT_HOTBAR_OFFSET + 1, ItemStack::new(Item::IronSword, 1));
            inv.set_item_at(SLOT_ARMOR_HEAD, ItemStack::new(Item::DiamondHelmet, 1));
        }

        d.dispatch(&w);
        w.maintain();

        let packets = t::received_packets(&player, None);

        let packets = packets
            .into_iter()
            .filter(|packet| packet.ty() == PacketType::EntityEquipment)
            .collect::<Vec<_>>();

        assert_eq!(packets.len(), 6);

        for packet in packets {
            let packet = cast_packet::<EntityEquipment>(&*packet);
            assert_eq!(packet.entity_id, player2.entity.id() as i32);
        }
    }

    #[test]
    fn test_set_slot_system() {
        let (mut w, mut d) = t::builder().with(SetSlotSystem::default(), "").build();

        let player = t::add_player(&mut w);
        let stack = ItemStack::new(Item::EnderPearl, 8);
        {
            let mut inventories = w.write_component::<InventoryComponent>();
            inventories
                .get_mut(player.entity)
                .unwrap()
                .set_item_at(0, stack.clone());

            let event = InventoryUpdateEvent {
                slots: smallvec![0],
                player: player.entity,
            };
            t::trigger_event(&w, event);
        }

        d.dispatch(&w);
        w.maintain();

        let packet = t::assert_packet_received(&player, PacketType::SetSlot);
        let packet = cast_packet::<SetSlot>(&*packet);

        assert_eq!(packet.window_id, 0);
        assert_eq!(packet.slot_data, Some(stack));
        assert_eq!(packet.slot, 0);
    }

    #[test]
    fn test_is_equipment_update() {
        let mut inv = InventoryComponent::default();
        inv.held_item = 0;

        assert!(is_equipment_update(&inv, 21).is_err());
        assert_eq!(
            is_equipment_update(&inv, SLOT_HOTBAR_OFFSET),
            Ok(Equipment::MainHand)
        );
        assert_eq!(
            is_equipment_update(&inv, SLOT_ARMOR_HEAD),
            Ok(Equipment::Helmet)
        );
        assert_eq!(
            is_equipment_update(&inv, SLOT_ARMOR_CHEST),
            Ok(Equipment::Chestplate)
        );
        assert_eq!(
            is_equipment_update(&inv, SLOT_ARMOR_LEGS),
            Ok(Equipment::Leggings)
        );
        assert_eq!(
            is_equipment_update(&inv, SLOT_ARMOR_FEET),
            Ok(Equipment::Boots)
        );
        assert_eq!(
            is_equipment_update(&inv, SLOT_OFFHAND),
            Ok(Equipment::OffHand)
        );
    }
}
