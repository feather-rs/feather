use crate::disconnect_player;
use crate::entity::PlayerComponent;
use crate::network::PacketQueue;
use feather_core::inventory::{Inventory, InventoryType};
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::CreativeInventoryAction;
use feather_core::network::packet::PacketType;
use feather_core::Gamemode;
use specs::storage::BTreeStorage;
use specs::System;
use specs::{Component, LazyUpdate, Read, ReadStorage, WriteStorage};
use std::ops::{Deref, DerefMut};

/// Component for storing a player's inventory.
#[derive(Clone, Debug)]
pub struct InventoryComponent {
    inventory: Inventory,
}

impl InventoryComponent {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(InventoryType::Player, 46),
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

/// System for handling Creative Inventory Action packets.
pub struct CreativeInventorySystem;

impl<'a> System<'a> for CreativeInventorySystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut inventories, players, packet_queue, lazy) = data;

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
        }
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

        let packet = CreativeInventoryAction::new(0, Some(ItemStack::new(Item::IronSword, 1)));

        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let inv_storage = w.read_component::<InventoryComponent>();
        let inv = inv_storage.get(player.entity).unwrap();
        assert_eq!(inv.item_at(0).unwrap(), &ItemStack::new(Item::IronSword, 1));

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
}
