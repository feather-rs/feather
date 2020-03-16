//! Handling of player block placement packets.

use crate::block::BlockUpdateCause;
use crate::network::PacketQueue;
use crate::p_inventory::{EntityInventory, InventoryUpdateEvent};
use crate::state::State;
use crate::util::disconnect_player;
use feather_core::inventory::SLOT_HOTBAR_OFFSET;
use feather_core::network::packet::implementation::PlayerBlockPlacement;
use feather_core::{Block, Gamemode, ItemStack};
use feather_item_block::ItemToBlock;
use legion::query::{Read, Write};
use tonks::{PreparedWorld, Query, Trigger};
use crate::game::Game;
use fecs::World;

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
#[system]
fn handle_player_block_placement(
    game: &mut Game,
    world: &mut World,
) {
    let packets = game.received::<PlayerBlockPlacement>();

    for (player, packet) in packets {
        // TODO: handle slabs, blocks with directions, etc.
        let gamemode = *world.get::<Gamemode>(player);
        let mut inventory = world.get::<EntityInventory>(player).unwrap();

        let item = match inventory.item_in_main_hand() {
            Some(item) => item,
            None => continue, // No block to place
        };

        let block = match item.ty.to_block() {
            Some(block) => block,
            None => continue, // Item is not a block
        };

        let placed_on = match state.block_at(packet.location) {
            Some(block) => block,
            None => {
                disconnect_player(state, player, "Attempted to place block in unloaded chunk");
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

        state.set_block_at(pos, block, BlockUpdateCause::Player(player));

        // Update player's inventory if in survival
        if gamemode == Gamemode::Survival {
            if item.amount == 0 {
                disconnect_player(
                    state,
                    player,
                    "Attempted to place block with 0-sized item stack",
                );
            }

            let item = ItemStack::new(item.ty, item.amount - 1);
            inventory.set_item_in_main_hand(item);

            let event = InventoryUpdateEvent {
                slots: smallvec![SLOT_HOTBAR_OFFSET + inventory.held_item],
                player,
            };
            inventory_update_events.trigger(event);
        }
    }
}
