use crate::{ItemTimedUse, IteratorExt};
use entity::InventoryExt;
use feather_core::inventory::Inventory;
use feather_core::items::Item;
use feather_core::network::packets::UseItem;
use feather_core::util::Hand;
use feather_server_types::{Game, Name, PacketBuffers};
use fecs::{Entity, World};
use std::sync::Arc;

#[fecs::system]
pub fn handle_player_use_item(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<UseItem>()
        .for_each_valid(world, |world, (player, packet)| {
            handle_use_item(game, world, player, packet)
        });
}

fn handle_use_item(game: &mut Game, world: &mut World, player: Entity, packet: UseItem) {
    let hand = match packet.hand {
        0 => Hand::Main,
        _ => Hand::Off,
    };

    if hand != Hand::Main {
        return;
    }

    let item_in_main_hand = world
        .get::<Inventory>(player)
        .item_in_main_hand(player, world);

    if let Some(item_in_main_hand) = item_in_main_hand {
        if item_in_main_hand.ty != Item::Bow {
            //TODO: Handle other used items
            return;
        }
        world
            .add(
                player,
                ItemTimedUse {
                    tick_start: game.tick_count,
                },
            )
            .unwrap();
        let player_name = world.get::<Name>(player);
        log::trace!("Added ItemTimedUse to player {}.", player_name.0);
    }
}
