//! This module handles packets relating to world updates,
//! including block update packets.

use crate::disconnect_player;
use crate::entity::{check_player_joined, PlayerComponent};
use crate::network::{send_packet_to_player, NetworkComponent, PacketQueue};
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::BlockChange;
use feather_core::network::packet::implementation::{PlayerDigging, PlayerDiggingStatus};
use feather_core::network::packet::PacketType;
use feather_core::world::block::{Block, BlockToId};
use feather_core::world::{BlockPosition, ChunkMap};
use feather_core::Gamemode;
use specs::{Join, LazyUpdate, Read, ReadStorage, System, Write};

/// System responsible for polling for PlayerDigging
/// packets and the like and handling them accordingly.
pub struct WorldUpdateSystem;

impl<'a> System<'a> for WorldUpdateSystem {
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
            if !check_player_joined(player, &pcomps, &lazy) {
                continue;
            }

            let pcomp = pcomps.get(player).unwrap();

            let packet = cast_packet::<PlayerDigging>(&_packet);
            match packet.status {
                PlayerDiggingStatus::StartedDigging => {
                    if pcomp.gamemode == Gamemode::Creative {
                        if handle_block_break(&mut chunk_map, packet.location, &netcomps, &pcomps)
                            .is_err()
                        {
                            disconnect_player(
                                player,
                                "Attempted to break block in unloaded chunk",
                                &lazy,
                            );
                        }
                    }
                }
                PlayerDiggingStatus::FinishedDigging => {
                    if handle_block_break(&mut chunk_map, packet.location, &netcomps, &pcomps)
                        .is_err()
                    {
                        disconnect_player(
                            player,
                            "Attempted to break block in unloaded chunk",
                            &lazy,
                        );
                    }
                }
                status => warn!("Unhandled Player Digging status: {:?}", status),
            }
        }
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
        let block_update = BlockChange::new(pos, new_block.block_state_id() as i32);
        send_packet_to_player(net, block_update);
    }
}
