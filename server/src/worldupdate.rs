//! This module handles packets relating to world updates,
//! including block update packets.

use crate::entity::{check_player_joined, PlayerComponent};
use crate::network::PacketQueue;
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{PlayerDigging, PlayerDiggingStatus};
use feather_core::network::packet::PacketType;
use feather_core::world::{BlockPosition, ChunkMap};
use feather_core::Gamemode;
use specs::{LazyUpdate, Read, ReadStorage, System, Write};

/// System responsible for polling for PlayerDigging
/// packets and the like and handling them accordingly.
pub struct WorldUpdateSystem;

impl<'a> System<'a> for WorldUpdateSystem {
    type SystemData = (
        Write<'a, ChunkMap>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, PacketQueue>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (chunk_map, pcomps, packet_queue, lazy) = data;

        let mut packets = vec![];
        packets.push(packet_queue.for_packet(PacketType::PlayerDigging));

        // Handle packets
        for (player, _packet) in packets {
            if !check_player_joined(player, &pcomps, &lazy) {
                continue;
            }
            let pcomp = pcomps.get(player).unwrap();

            let packet = cast_packet::<PlayerDigging>(&_packet);
            match packet.status {
                PlayerDiggingStatus::StartedDigging => if pcomp.gamemode == Gamemode::Creative {},
            }
        }
    }
}

/// Handles a broken block by a player.
fn handle_block_break(chunk_map: &mut ChunkMap, pos: BlockPosition) {}
