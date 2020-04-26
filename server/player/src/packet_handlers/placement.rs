//! Handling of player block placement packets.

use feather_core::blocks::BlockKind;
use feather_core::inventory::{Inventory, SLOT_HOTBAR_OFFSET};
use feather_core::item_block::ItemToBlock;
use feather_core::items::ItemStack;
use feather_core::network::packets::PlayerBlockPlacement;
use feather_core::util::Gamemode;
use feather_server_types::{Game, HeldItem, InventoryUpdateEvent, PacketBuffers};
use fecs::World;
use std::sync::Arc;

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
#[fecs::system]
pub fn handle_player_block_placement(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    let packets = packet_buffers.received::<PlayerBlockPlacement>();

    for (player, packet) in packets {
        // TODO: handle slabs, blocks with directions, etc.
        let gamemode = *world.get::<Gamemode>(player);
        let inventory = world.get::<Inventory>(player);

        let item = match inventory.item_at(world.get::<HeldItem>(player).0) {
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
        let pos = match placed_on.kind() {
            BlockKind::Grass | BlockKind::TallGrass | BlockKind::Water | BlockKind::Lava => {
                packet.location
            }
            _ => packet.location + packet.face.placement_offset(),
        };

        game.set_block_at(world, pos, block);

        let held_item = world.get::<HeldItem>(player).0;
        let mut inventory = world.get_mut::<Inventory>(player);

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
            inventory.set_item_at(held_item, item);

            let event = InventoryUpdateEvent {
                slots: std::iter::once(SLOT_HOTBAR_OFFSET + held_item).collect(),
                player,
            };
            drop(inventory);
            game.handle(world, event);
        }
    }
}
