//! Handling of player block placement packets.

use crate::game::Game;
use crate::p_inventory::{EntityInventory, InventoryUpdateEvent};
use crate::packet_buffer::PacketBuffers;
use feather_core::inventory::SLOT_HOTBAR_OFFSET;
use feather_core::network::packet::implementation::PlayerBlockPlacement;
use feather_core::{Block, Gamemode, ItemStack};
use feather_item_block::ItemToBlock;
use fecs::World;
use std::sync::Arc;

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
#[system]
pub fn handle_player_block_placement(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    let packets = packet_buffers.received::<PlayerBlockPlacement>();

    for (player, packet) in packets {
        // TODO: handle slabs, blocks with directions, etc.
        let gamemode = *world.get::<Gamemode>(player);
        let inventory = world.get::<EntityInventory>(player);

        let item = match inventory.item_in_main_hand() {
            Some(item) => *item,
            None => continue, // No block to place
        };

        drop(inventory);

        let block = match item.ty.to_block() {
            Some(block) => block,
            None => continue, // Item is not a block
        };

        let placed_on = match game.block_at(packet.location) {
            Some(block) => block,
            None => {
                game.disconnect(player, world, "attempted to place block in unloaded chunk");
                continue;
            }
        };

        // TODO: waterlogged blocks, more
        let pos = match placed_on {
            Block::Grass | Block::TallGrass(_) | Block::Water(_) | Block::Lava(_) => {
                packet.location
            }
            _ => packet.location + packet.face.placement_offset(),
        };

        game.set_block_at(world, pos, block);

        let mut inventory = world.get_mut::<EntityInventory>(player);

        // Update player's inventory if in survival
        if gamemode == Gamemode::Survival {
            if item.amount == 0 {
                drop(inventory);
                game.disconnect(
                    player,
                    world,
                    "attempted to place block with zero-sized item stack",
                );
                continue;
            }

            let item = ItemStack::new(item.ty, item.amount - 1);
            inventory.set_item_in_main_hand(item);

            let event = InventoryUpdateEvent {
                slots: smallvec![SLOT_HOTBAR_OFFSET + inventory.held_item],
                player,
            };
            drop(inventory);
            game.on_inventory_update(world, event);
        }
    }
}
