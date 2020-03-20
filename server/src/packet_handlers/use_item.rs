use crate::game::Game;
use fecs::{World, Entity};
use feather_core::network::packet::implementation::UseItem;
use feather_core::{Hand, Item};
use crate::p_inventory::EntityInventory;
use crate::player::ItemTimedUse;
use crate::entity::Name;
use crate::packet_buffer::PacketBuffers;
use std::sync::Arc;

#[system]
pub fn handle_player_use_item(game: &mut Game, world: &mut World,
packet_buffers: &Arc<PacketBuffers>) {
    let packets = packet_buffers.received::<UseItem>();

    for (player, packet) in packets {
        handle_use_item(game, world, player, packet);
    }
}

fn handle_use_item(game: &mut Game, world: &mut World, player: Entity, packet: UseItem) {
    let hand = match packet.hand {
        0 => Hand::Main,
        _ => Hand::Off,
    };

    if hand != Hand::Main {
        return;
    }

    let item_in_main_hand = world.get::<EntityInventory>(player)
        .item_in_main_hand()
        .copied();

    if let Some(item_in_main_hand) = item_in_main_hand {
        if item_in_main_hand.ty != Item::Bow { //TODO: Handle other used items
            return;
        }
        world.add(player, ItemTimedUse { tick_start: game.tick_count });
        let player_name = world.get::<Name>(player);
        info!("Added ItemTimedUse to player {}.", player_name.0);
    }
}
