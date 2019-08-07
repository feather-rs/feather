use crate::disconnect_player;
use crate::entity::PlayerComponent;
use crate::network::{send_packet_to_all_players, NetworkComponent, PacketQueue};
use feather_core::inventory::{
    Inventory, InventoryType, SlotIndex, HOTBAR_SIZE, SLOT_ENTITY_EQUIPMENT_MAIN_HAND,
    SLOT_ENTITY_EQUIPMENT_OFF_HAND, SLOT_HOTBAR_OFFSET, SLOT_OFFHAND,
};
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    CreativeInventoryAction, EntityEquipment, HeldItemChangeServerbound,
};
use feather_core::network::packet::PacketType;
use feather_core::{Gamemode, Hand};
use shrev::EventChannel;
use specs::storage::BTreeStorage;
use specs::SystemData;
use specs::{Component, Entities, LazyUpdate, Read, ReadStorage, ReaderId, World, WriteStorage};
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
    type Storage = BTreeStorage<Self>;
}

/// Event which is triggered when a player updates
/// their held item.
pub struct HeldItemUpdateEvent {
    /// The hand of the held item update.
    pub hand: Hand,
    pub player: Entity,
}

/// System for handling Creative Inventory Action packets.
pub struct CreativeInventorySystem;

impl<'a> System<'a> for CreativeInventorySystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        ReadStorage<'a, PlayerComponent>,
        Write<'a, EventChannel<HeldItemUpdateEvent>>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut inventories, players, mut events, packet_queue, lazy) = data;

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

            if packet.slot >= inventory.slot_count() {
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

            // If the updated slot was the player's held item,
            // we need to broadcast the equipment update
            if packet.slot as usize >= SLOT_HOTBAR_OFFSET
                && inventory.held_item == packet.slot as usize - SLOT_HOTBAR_OFFSET
            {
                let event = HeldItemUpdateEvent {
                    hand: Hand::Main,
                    player,
                };
                events.single_write(event);
            }
            // The same goes for the offhand.
            else if inventory.held_item == SLOT_OFFHAND {
                let event = HeldItemUpdateEvent {
                    hand: Hand::Off,
                    player,
                };
                events.single_write(event);
            }
        }
    }
}

/// System for handling Held Item Change packets.
pub struct HeldItemChangeSystem;

impl<'a> System<'a> for HeldItemChangeSystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        Write<'a, EventChannel<HeldItemUpdateEvent>>,
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
            let event = HeldItemUpdateEvent {
                hand: Hand::Main,
                player,
            };
            events.single_write(event);
        }
    }
}

/// System for broadcasting held item updates.
#[derive(Default)]
pub struct HeldItemBroadcastSystem {
    reader: Option<ReaderId<HeldItemUpdateEvent>>,
}

impl<'a> System<'a> for HeldItemBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, InventoryComponent>,
        Read<'a, EventChannel<HeldItemUpdateEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (networks, inventories, events, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let inv = inventories.get(event.player).unwrap();

            let (slot, item) = match event.hand {
                Hand::Main => {
                    let item = inv.item_at(SLOT_HOTBAR_OFFSET + inv.held_item).cloned();
                    (SLOT_ENTITY_EQUIPMENT_MAIN_HAND, item)
                }
                Hand::Off => {
                    let item = inv.item_at(SLOT_OFFHAND).cloned();
                    (SLOT_ENTITY_EQUIPMENT_OFF_HAND, item)
                }
            };

            let packet = EntityEquipment::new(event.player.id() as i32, slot as i32, item);

            send_packet_to_all_players(&networks, &entities, packet, Some(event.player));
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<HeldItemUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::inventory::ItemStack;
    use feather_core::item::Item;
    use specs::WorldExt;

    #[test]
    fn test_creative_inventory_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let packet = CreativeInventoryAction::new(
            SLOT_HOTBAR_OFFSET as u16,
            Some(ItemStack::new(Item::IronSword, 1)),
        );

        t::receive_packet(&player, &w, packet);

        let mut event_reader = t::reader(&w);

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
            let channel = w.fetch::<EventChannel<HeldItemUpdateEvent>>();
            let events = channel.read(&mut event_reader).collect::<Vec<_>>();
            assert_eq!(events.len(), 1);
            let first = events.first().unwrap();
            assert_eq!(first.player, player.entity);
            assert_eq!(first.hand, Hand::Main);
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
    fn test_held_item_change_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let slot = 4;

        let mut event_reader = t::reader(&w);

        let packet = HeldItemChangeServerbound::new(slot);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        let channel = w.fetch::<EventChannel<HeldItemUpdateEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();

        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.player, player.entity);
        assert_eq!(first.hand, Hand::Main);

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

        let event = HeldItemUpdateEvent {
            player: player.entity,
            hand: Hand::Main,
        };

        w.fetch_mut::<EventChannel<HeldItemUpdateEvent>>()
            .single_write(event);

        d.dispatch(&w);
        w.maintain();

        t::assert_packet_received(&player2, PacketType::EntityEquipment);
        t::assert_packet_not_received(&player, PacketType::EntityEquipment);
    }
}
