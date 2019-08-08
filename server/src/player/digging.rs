//! This module handles the monolithic Player Digging packet.
//!
//! The packet's name is rather misleading, as it is also sent
//! for completely unrelated actions, including eating, shooting bows,
//! swapping items out the the offhand, and dropping items.

use specs::{Entity, LazyUpdate, Read, ReadStorage, System, Write, WriteStorage};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{PlayerDigging, PlayerDiggingStatus};
use feather_core::network::packet::PacketType;
use feather_core::world::block::Block;
use feather_core::world::{BlockPosition, ChunkMap};
use feather_core::Gamemode;

use crate::disconnect_player;
use crate::entity::PlayerComponent;
use crate::network::PacketQueue;
use crate::player::{InventoryComponent, InventoryUpdateEvent};
use feather_core::inventory::{ItemStack, SlotIndex, SLOT_HOTBAR_OFFSET};
use shrev::EventChannel;

/// Event triggered when a block is broken.
///
/// This event is triggered *after* the block is updated
/// in the chunk map.
#[derive(Debug, Clone)]
pub struct BlockBreakEvent {
    /// The cause of this block break event.
    pub cause: BlockBreakCause,
    /// The location of the block which was broken.
    pub pos: BlockPosition,
    /// The block which was previously at the position.
    pub block: Block,
}

/// The possible causes of a block break event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockBreakCause {
    /// Indicates that a player broke the block.
    Player(Entity),
}

/// Event triggered when a player drops an item.
///
/// Before this event is triggered, the item
/// is removed from the player's inventory.
#[derive(Debug, Clone)]
pub struct PlayerItemDropEvent {
    /// The slot from which the item was dropped.
    pub slot: SlotIndex,
    /// The item stack which was dropped.
    pub stack: ItemStack,
    /// The player who dropped the item.
    pub player: Entity,
}

/// System responsible for polling for PlayerDigging
/// packets and writing the corresponding events.
pub struct PlayerDiggingSystem;

impl<'a> System<'a> for PlayerDiggingSystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        ReadStorage<'a, PlayerComponent>, // For gamemodes
        Write<'a, EventChannel<BlockBreakEvent>>,
        Write<'a, EventChannel<PlayerItemDropEvent>>,
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        Write<'a, ChunkMap>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use PlayerDiggingStatus::*;

        let (
            mut inventories,
            players,
            mut block_breaks,
            mut item_drops,
            mut inventory_updates,
            mut chunk_map,
            packet_queue,
            lazy,
        ) = data;

        let packets = packet_queue.for_packet(PacketType::PlayerDigging);

        for (player, packet) in packets {
            let packet = cast_packet::<PlayerDigging>(&*packet);

            match packet.status {
                StartedDigging | FinishedDigging | CancelledDigging => handle_digging(
                    packet,
                    players.get(player).unwrap(),
                    player,
                    &mut block_breaks,
                    &mut chunk_map,
                    &lazy,
                ),
                DropItem | DropItemStack => handle_drop_item_stack(
                    packet,
                    player,
                    &mut inventory_updates,
                    &mut item_drops,
                    inventories.get_mut(player).unwrap(),
                ),
                status => warn!("Unhandled Player Digging status {:?}", status),
            }
        }
    }
}

fn handle_digging(
    packet: &PlayerDigging,
    player: &PlayerComponent,
    entity: Entity,
    events: &mut EventChannel<BlockBreakEvent>,
    chunk_map: &mut ChunkMap,
    lazy: &LazyUpdate,
) {
    // Return early if needed
    match packet.status {
        PlayerDiggingStatus::StartedDigging => {
            if player.gamemode != Gamemode::Creative {
                return;
            }
        }
        PlayerDiggingStatus::CancelledDigging => return,
        _ => (),
    }

    let old = chunk_map.block_at(packet.location);

    if chunk_map.set_block_at(packet.location, Block::Air).is_err() {
        disconnect_player(
            entity,
            "Attempted to break block in unloaded chunk".to_string(),
            lazy,
        );
        return;
    }

    let event = BlockBreakEvent {
        cause: BlockBreakCause::Player(entity),
        pos: packet.location,
        block: old.unwrap(), // We checked that the location was valid above
    };

    events.single_write(event);
}

fn handle_drop_item_stack(
    packet: &PlayerDigging,
    entity: Entity,
    inventory_updates: &mut EventChannel<InventoryUpdateEvent>,
    item_drops: &mut EventChannel<PlayerItemDropEvent>,
    inventory: &mut InventoryComponent,
) {
    assert!(
        packet.status == PlayerDiggingStatus::DropItem
            || packet.status == PlayerDiggingStatus::DropItemStack
    );

    let stack = {
        if let Some(item) = inventory.item_at(inventory.held_item) {
            item.clone()
        } else {
            // Silently fail - no item stack to drop
            return;
        }
    };

    let slot = inventory.held_item + SLOT_HOTBAR_OFFSET;

    let amnt = match packet.status {
        PlayerDiggingStatus::DropItem => {
            if stack.amount == 0 {
                inventory.clear_item_at(slot);
                0
            } else if stack.amount == 1 {
                inventory.clear_item_at(slot);
                1
            } else {
                inventory.set_item_at(slot, ItemStack::new(stack.ty, stack.amount - 1));
                1
            }
        }
        PlayerDiggingStatus::DropItemStack => {
            inventory.clear_item_at(slot);
            stack.amount
        }
        _ => unreachable!(), // Assertion above
    };

    if amnt != 0 {
        let item_drop = PlayerItemDropEvent {
            slot: inventory.held_item,
            stack,
            player: entity,
        };
        item_drops.single_write(item_drop);

        let inv_update = InventoryUpdateEvent {
            slots: smallvec![slot],
            player: entity,
        };
        inventory_updates.single_write(inv_update);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::world::chunk::Chunk;
    use feather_core::world::ChunkPosition;
    use specs::WorldExt;

    #[test]
    fn test_started_digging() {
        let (mut w, mut d) = t::init_world();

        let cpos = ChunkPosition::new(0, 0);
        let bpos = BlockPosition::new(0, 0, 0);
        let mut chunk = Chunk::new(cpos);
        chunk.set_block_at(0, 0, 0, Block::Stone);
        w.fetch_mut::<ChunkMap>().set_chunk_at(cpos, chunk);

        let mut event_reader = t::reader(&w);

        // Creative mode

        let player = t::add_player(&mut w);

        let packet = PlayerDigging::new(PlayerDiggingStatus::StartedDigging, bpos, 0);

        t::receive_packet(&player, &w, packet.clone());

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        {
            let mut chunk_map = w.fetch_mut::<ChunkMap>();

            assert_eq!(chunk_map.block_at(bpos).unwrap(), Block::Air);

            chunk_map.set_block_at(bpos, Block::Stone).unwrap();

            let channel = w.fetch_mut::<EventChannel<BlockBreakEvent>>();
            let events = channel.read(&mut event_reader).collect::<Vec<_>>();
            assert_eq!(events.len(), 1);

            let first = events.first().unwrap();
            assert_eq!(first.block, Block::Stone);
            assert_eq!(first.cause, BlockBreakCause::Player(player.entity));
            assert_eq!(first.pos, bpos);
        }

        // Survival mode
        let player = t::add_player(&mut w);
        w.write_component::<PlayerComponent>()
            .get_mut(player.entity)
            .unwrap()
            .gamemode = Gamemode::Survival;

        t::receive_packet(&player, &w, packet.clone());

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let chunk_map = w.fetch::<ChunkMap>();
        assert_eq!(chunk_map.block_at(bpos).unwrap(), Block::Stone);

        let channel = w.fetch_mut::<EventChannel<BlockBreakEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert_eq!(events.len(), 0);
    }

    // This should be a no-op.
    #[test]
    fn test_cancelled_digging() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut event_reader = t::reader(&w);

        let packet = PlayerDigging::new(
            PlayerDiggingStatus::CancelledDigging,
            BlockPosition::default(),
            0,
        );
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let channel = w.fetch::<EventChannel<BlockBreakEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert!(events.is_empty());
    }

    #[test]
    fn test_finished_digging() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        w.write_component::<PlayerComponent>()
            .get_mut(player.entity)
            .unwrap()
            .gamemode = Gamemode::Survival;

        let mut event_reader = t::reader(&w);

        let bpos = BlockPosition::new(0, 0, 0);
        let cpos = bpos.chunk_pos();

        let mut chunk = Chunk::new(cpos);
        chunk.set_block_at(0, 0, 0, Block::Stone);

        w.fetch_mut::<ChunkMap>().set_chunk_at(cpos, chunk);

        let packet = PlayerDigging::new(PlayerDiggingStatus::FinishedDigging, bpos, 0);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_disconnected(&player);

        let chunk_map = w.fetch::<ChunkMap>();

        assert_eq!(chunk_map.block_at(bpos).unwrap(), Block::Air);

        let channel = w.fetch_mut::<EventChannel<BlockBreakEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.block, Block::Stone);
        assert_eq!(first.cause, BlockBreakCause::Player(player.entity));
        assert_eq!(first.pos, bpos);
    }

    #[test]
    fn test_block_break_in_unloaded_chunk() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut event_reader = t::reader(&w);

        let bpos = BlockPosition::new(1000, 25, 1000);

        let packet = PlayerDigging::new(PlayerDiggingStatus::FinishedDigging, bpos, 0);
        t::receive_packet(&player, &w, packet);

        d.dispatch(&w);
        w.maintain();

        t::assert_disconnected(&player);

        let channel = w.fetch::<EventChannel<BlockBreakEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert!(events.is_empty());
    }
}
