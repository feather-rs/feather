use crate::disconnect_player;
use crate::network::PacketQueue;
use crate::player::{BlockUpdateCause, BlockUpdateEvent, InventoryComponent, InventoryUpdateEvent};
use feather_core::inventory::SLOT_HOTBAR_OFFSET;
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::PlayerBlockPlacement;
use feather_core::world::ChunkMap;
use feather_core::{Block, ItemStack, PacketType};
use feather_item_block::ItemToBlock;
use shrev::EventChannel;
use specs::{LazyUpdate, Read, System, Write, WriteStorage};

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
pub struct BlockPlacementSystem;

impl<'a> System<'a> for BlockPlacementSystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        Write<'a, ChunkMap>,
        Write<'a, EventChannel<BlockUpdateEvent>>,
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut inventories,
            mut chunk_map,
            mut block_update_events,
            mut inventory_update_events,
            packet_queue,
            lazy,
        ) = data;

        let packets = packet_queue.for_packet(PacketType::PlayerBlockPlacement);

        for (player, packet) in packets {
            let packet = cast_packet::<PlayerBlockPlacement>(&*packet);

            // TODO: handle slabs, blocks with directions, etc.
            let inventory = inventories.get_mut(player).unwrap();

            let item = continue_if_none!(inventory.item_in_main_hand());

            let block = continue_if_none!(item.ty.to_block());

            let placed_on = match chunk_map.block_at(packet.location) {
                Some(block) => block,
                None => {
                    disconnect_player(
                        player,
                        String::from("Attempted to place block in unloaded chunk"),
                        &lazy,
                    );
                    continue;
                }
            };

            // TODO: waterlogged blocks, more
            debug!("placed_on: {:?}", placed_on);
            debug!("location: {:?}", packet.location);
            let pos = match placed_on {
                Block::Grass | Block::TallGrass(_) | Block::Water(_) | Block::Lava(_) => {
                    packet.location
                }
                _ => packet.location + packet.face.placement_offset(),
            };

            let old = match chunk_map.block_at(pos) {
                Some(block) => block,
                None => {
                    disconnect_player(
                        player,
                        String::from("Attempted to place block in unloaded chunk"),
                        &lazy,
                    );
                    continue;
                }
            };

            chunk_map.set_block_at(pos, block).unwrap();

            let event = BlockUpdateEvent {
                cause: BlockUpdateCause::Player(player),
                pos,
                old_block: old,
                new_block: block,
            };

            block_update_events.single_write(event);

            // Update player's inventory
            let item = ItemStack::new(item.ty, item.amount - 1);
            inventory.set_item_in_main_hand(item);

            let event = InventoryUpdateEvent {
                slots: smallvec![SLOT_HOTBAR_OFFSET + inventory.held_item],
                player,
            };
            inventory_update_events.single_write(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::inventory::SLOT_HOTBAR_OFFSET;
    use feather_core::network::packet::implementation::Face;
    use feather_core::{Block, BlockPosition, Item, ItemStack};
    use specs::WorldExt;

    #[test]
    fn test_block_placement_system() {
        let (mut w, mut d) = t::builder().with(BlockPlacementSystem, "").build();

        t::populate_with_air(&mut w);

        let player = t::add_player(&mut w);

        {
            let mut inventories = w.write_component::<InventoryComponent>();
            inventories
                .get_mut(player.entity)
                .unwrap()
                .set_item_at(SLOT_HOTBAR_OFFSET, ItemStack::new(Item::Cobblestone, 1));
        }

        let pos = BlockPosition::new(10, 20, 30);

        let packet = PlayerBlockPlacement {
            location: pos,
            face: Face::Top,
            hand: 0,
            cursor_position_x: 0.0,
            cursor_position_y: 0.0,
            cursor_position_z: 0.0,
        };
        t::receive_packet(&player, &w, packet);

        let mut reader = t::reader(&w);

        d.dispatch(&w);
        w.maintain();

        let events = t::triggered_events::<BlockUpdateEvent>(&w, &mut reader);
        let first = events.first().unwrap();

        assert_eq!(first.cause, BlockUpdateCause::Player(player.entity));
        assert_eq!(first.old_block, Block::Air);
        assert_eq!(first.new_block, Block::Cobblestone);
        assert_eq!(first.pos, pos + BlockPosition::new(0, 1, 0));

        let inventory = w
            .read_component::<InventoryComponent>()
            .get(player.entity)
            .unwrap()
            .clone();
        assert_eq!(inventory.item_in_main_hand(), None);
    }

    #[test]
    fn test_block_placement_system_unloaded_chunk() {
        let (mut w, mut d) = t::builder().with(BlockPlacementSystem, "").build();

        t::populate_with_air(&mut w);

        let player = t::add_player(&mut w);

        {
            let mut inventories = w.write_component::<InventoryComponent>();
            inventories
                .get_mut(player.entity)
                .unwrap()
                .set_item_at(SLOT_HOTBAR_OFFSET, ItemStack::new(Item::Cobblestone, 1));
        }

        let pos = BlockPosition::new(1000, 100, 2000);

        let packet = PlayerBlockPlacement {
            location: pos,
            face: Face::Top,
            hand: 0,
            cursor_position_x: 0.0,
            cursor_position_y: 0.0,
            cursor_position_z: 0.0,
        };
        t::receive_packet(&player, &w, packet);

        let mut reader = t::reader(&w);

        d.dispatch(&w);
        w.maintain();

        t::assert_disconnected(&player);

        assert!(t::triggered_events::<BlockUpdateEvent>(&w, &mut reader).is_empty());
    }
}
