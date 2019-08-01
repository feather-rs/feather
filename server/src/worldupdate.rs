//! This module handles packets relating to world updates,
//! including block update packets.

use specs::{Entity, Join, LazyUpdate, Read, ReadStorage, System, Write};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::BlockChange;
use feather_core::network::packet::implementation::{PlayerDigging, PlayerDiggingStatus};
use feather_core::network::packet::PacketType;
use feather_core::world::block::{Block, BlockExt};
use feather_core::world::{BlockPosition, ChunkMap};
use feather_core::Gamemode;

use crate::disconnect_player;
use crate::entity::PlayerComponent;
use crate::network::{send_packet_to_player, NetworkComponent, PacketQueue};

/// System responsible for polling for PlayerDigging
/// packets and handling them accordingly.
pub struct PlayerDiggingSystem;

impl<'a> System<'a> for PlayerDiggingSystem {
    type SystemData = (
        Write<'a, ChunkMap>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunk_map, pcomps, netcomps, packet_queue, lazy) = data;

        let mut packets = vec![];
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerDigging));

        // Handle packets
        for (player, _packet) in packets {
            let packet = cast_packet::<PlayerDigging>(&_packet);
            handle_player_digging(&mut chunk_map, packet, player, &netcomps, &pcomps, &lazy);
        }
    }
}

/// Handles a Player Digging packet.
fn handle_player_digging(
    chunk_map: &mut ChunkMap,
    packet: &PlayerDigging,
    player: Entity,
    netcomps: &ReadStorage<NetworkComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    lazy: &LazyUpdate,
) {
    let pcomp = pcomps.get(player).unwrap();
    match packet.status {
        PlayerDiggingStatus::StartedDigging => {
            if pcomp.gamemode == Gamemode::Creative {
                if handle_block_break(chunk_map, packet.location, &netcomps, &pcomps).is_err() {
                    disconnect_player(
                        player,
                        format!(
                            "Attempted to break block in unloaded chunk ({:?})",
                            packet.location
                        ),
                        &lazy,
                    );
                }
            }
        }
        PlayerDiggingStatus::FinishedDigging => {
            if handle_block_break(chunk_map, packet.location, &netcomps, &pcomps).is_err() {
                disconnect_player(
                    player,
                    format!(
                        "Attempted to break block in unloaded chunk ({:?})",
                        packet.location
                    ),
                    &lazy,
                );
            }
        }
        status => warn!("Unhandled Player Digging status: {:?}", status),
    }
}

/// Handles a broken block by a player.
fn handle_block_break(
    chunk_map: &mut ChunkMap,
    pos: BlockPosition,
    netcomps: &ReadStorage<NetworkComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
) -> Result<(), ()> {
    chunk_map.set_block_at(pos, Block::Air)?;

    broadcast_block_update(pos, Block::Air, netcomps, pcomps);

    Ok(())
}

/// Broadcasts a block update to all joined players.
fn broadcast_block_update(
    pos: BlockPosition,
    new_block: Block,
    netcomps: &ReadStorage<NetworkComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
) {
    for (net, _) in (netcomps, pcomps).join() {
        let block_update = BlockChange::new(pos, new_block.native_state_id() as i32);
        send_packet_to_player(net, block_update);
    }
}

#[cfg(test)]
mod tests {
    use specs::{RunNow, WorldExt};

    use feather_core::world::chunk::Chunk;
    use feather_core::world::ChunkPosition;

    use crate::testframework as t;

    use super::*;

    #[test]
    fn test_system() {
        let (mut w, _) = t::init_world();
        let player = t::add_player(&mut w);

        t::receive_packet(
            &player,
            &w,
            PlayerDigging::new(
                PlayerDiggingStatus::StartedDigging,
                BlockPosition::new(0, 0, 0),
                0,
            ),
        );

        let mut system = PlayerDiggingSystem;
        system.run_now(&w);
    }

    #[test]
    fn test_handle_player_digging() {
        let (mut w, _) = t::init_world();

        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        chunk.set_block_at(0, 0, 0, Block::Stone);
        w.write_resource::<ChunkMap>()
            .set_chunk_at(ChunkPosition::new(0, 0), chunk);

        let pos = BlockPosition::new(0, 0, 0);

        // Test with creative mode player
        let player = t::add_player(&mut w);

        let packet = PlayerDigging::new(PlayerDiggingStatus::StartedDigging, pos, 0);
        handle_player_digging(
            &mut w.fetch_mut(),
            &packet,
            player.entity,
            &w.read_component(),
            &w.read_component(),
            &w.read_resource(),
        );

        // Call lazily updated disconnect
        w.maintain();

        // Make sure player wasn't disconnected
        let packets = t::received_packets(&player, None);
        packets
            .iter()
            .for_each(|packet| assert_ne!(packet.ty(), PacketType::DisconnectPlay));

        // Make sure player was notified of block update
        let _block_change = packets
            .iter()
            .find(|packet| packet.ty() == PacketType::BlockChange)
            .unwrap();
        let block_change = cast_packet::<BlockChange>(&_block_change);
        assert_eq!(block_change.location, pos);
        assert_eq!(block_change.block_id, Block::Air.native_state_id() as i32);

        // Make sure block was actually updated
        assert_eq!(
            w.read_resource::<ChunkMap>().block_at(pos).unwrap(),
            Block::Air
        );
    }

    #[test]
    fn test_handle_block_break() {
        let (mut w, _) = t::init_world();

        // Confirm that breaking block in unloaded chunk fails
        let _player = t::add_player(&mut w);
        assert!(handle_block_break(
            &mut w.fetch_mut(),
            BlockPosition::new(1024, 9, 1024),
            &w.read_component(),
            &w.read_component(),
        )
        .is_err());

        // Break block in known chunk
        let _player = t::add_player(&mut w);

        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);
        chunk.set_block_at(0, 0, 0, Block::Stone);
        w.write_resource::<ChunkMap>().set_chunk_at(pos, chunk);

        assert!(handle_block_break(
            &mut w.write_resource(),
            BlockPosition::new(0, 0, 0),
            &w.read_component(),
            &w.read_component(),
        )
        .is_ok());
        assert_eq!(
            w.read_resource::<ChunkMap>()
                .block_at(BlockPosition::new(0, 0, 0))
                .unwrap(),
            Block::Air
        );
    }

    #[test]
    fn test_broadcast_block_update() {
        let (mut w, _) = t::init_world();

        let player = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);
        w.write_component::<PlayerComponent>()
            .remove(player2.entity)
            .unwrap();

        broadcast_block_update(
            BlockPosition::new(0, 0, 0),
            Block::Sand,
            &w.read_component(),
            &w.read_component(),
        );

        // Check that the joined player received block update but the unjoined
        // player did not
        let packet = t::assert_packet_received(&player, PacketType::BlockChange);

        let packet = cast_packet::<BlockChange>(&packet);
        assert_eq!(packet.location, BlockPosition::new(0, 0, 0));
        assert_eq!(packet.block_id, Block::Sand.native_state_id() as i32);

        let p2_packets = t::received_packets(&player2, None);
        assert_eq!(p2_packets.len(), 0);
    }
}
