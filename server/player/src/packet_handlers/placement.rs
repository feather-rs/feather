//! Handling of player block placement packets.

use super::block_interaction::*;
use crate::IteratorExt;
use entity::InventoryExt;
use feather_core::blocks::BlockKind;
use feather_core::inventory::{slot, Area, Inventory};
use feather_core::item_block::ItemToBlock;
use feather_core::items::ItemStack;
use feather_core::network::packets::PlayerBlockPlacement;
use feather_core::util::Gamemode;
use feather_server_types::{BlockUpdateCause, Game, HeldItem, InventoryUpdateEvent, PacketBuffers};
use fecs::{Entity, World};
use once_cell::sync::Lazy;
use smallvec::smallvec;
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
static INTERACTION_HANDLERS: Lazy<HashMap<BlockKind, Box<dyn InteractionHandler>>> =
    Lazy::new(|| {
        let mut handlers_hashmap: HashMap<BlockKind, Box<dyn InteractionHandler>> = HashMap::new();

        handlers_hashmap.insert(BlockKind::CraftingTable, Box::new(CraftingTableInteraction));

        handlers_hashmap
    });

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
///
/// Also handles block interactions because they are handled with the same packet.
#[fecs::system]
pub fn handle_player_block_placement(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<PlayerBlockPlacement>()
        .for_each_valid(world, |world, (player, packet)| {
            let target_block = match game.block_at(packet.location) {
                Some(block) => block,
                None => {
                    game.disconnect(
                        player,
                        world,
                        "Attempted to interact with block in unloaded chunk",
                    );
                    return;
                }
            };

            // Decide whether the player should place a block or interact with the block they are targeting
            // TODO: Maybe player shifting may need to be taken into account (shift click on interactable block)
            if let Some(interaction_handler) = INTERACTION_HANDLERS.get(&target_block.kind()) {
                // Interact with the block
                interaction_handler.handle_interaction(
                    game,
                    world,
                    player,
                    target_block.kind(),
                    packet,
                );
            } else {
                // Try to place a block
                handle_block_placement(game, world, player, target_block.kind(), packet);
            }
        });
}

pub fn handle_block_placement(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    target_block_kind: BlockKind,
    packet: PlayerBlockPlacement,
) {
    let gamemode = *world.get::<Gamemode>(player);

    let inventory = world.get::<Inventory>(player);

    let item = match inventory.item_in_main_hand(player, world) {
        Some(item) => item,
        None => return, // No block to place
    };

    drop(inventory);

    let block = match item.ty.to_block() {
        Some(block) => block,
        None => return, // Item is not a block
    };

    // TODO: waterlogged blocks, more
    let pos = match target_block_kind {
        BlockKind::Grass | BlockKind::TallGrass | BlockKind::Water | BlockKind::Lava => {
            packet.location
        }
        _ => packet.location + packet.face.placement_offset(),
    };

    game.set_block_at(world, pos, block, BlockUpdateCause::Entity(player));

    let held_item = world.get::<HeldItem>(player).0;
    let inventory = world.get::<Inventory>(player);

    // Update player's inventory if in survival
    if gamemode == Gamemode::Survival {
        if item.amount == 0 {
            drop(inventory);
            game.disconnect(
                player,
                world,
                "attempted to place block with zero-sized item stack",
            );
            return;
        }

        let item = ItemStack::new(item.ty, item.amount - 1);
        inventory
            .set_item_at(Area::Hotbar, held_item, item)
            .unwrap();

        let event = InventoryUpdateEvent {
            slots: smallvec![slot(Area::Hotbar, held_item)],
            player,
        };
        drop(inventory);
        game.handle(world, event);
    }
}

/// Handles interaction with blocks that have an inventory
pub fn handle_block_interaction(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    target_block_kind: BlockKind,
    packet: PlayerBlockPlacement,
) {
    let gamemode = *world.get::<Gamemode>(player);
}
