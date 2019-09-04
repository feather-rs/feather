//! Implements a broadcaster, used to lazily broadcast
//! packets to players able to see a given entity.

use crate::chunk_logic::ChunkHolders;
use crate::entity::PositionComponent;
use crate::network::{send_packet_boxed_to_player, NetworkComponent};
use crate::util::Util;
use crossbeam::queue::SegQueue;
use feather_core::{ChunkPosition, Packet};
use specs::{Entities, Entity, Read, ReadStorage, System};

/// Broadcaster used to lazily broadcast packets.
#[derive(Default)]
pub struct Broadcaster {
    /// Internal queue of broadcasts to send.
    queue: SegQueue<BroadcastRequest>,
}

impl Broadcaster {
    /// Lazily broadcasts a packet to all players
    /// able to see a given entity.
    pub fn broadcast_entity_update<P>(&self, entity: Entity, packet: P, neq: Option<Entity>)
    where
        P: Packet + 'static,
    {
        self.queue.push(BroadcastRequest {
            condition: BroadcastCondition::Entity(entity),
            packet: Box::new(packet),
            neq,
        });
    }

    /// Lazily broadcasts a packet to all players
    /// able to see a given chunk.
    pub fn broadcast_chunk_update<P>(&self, chunk: ChunkPosition, packet: P, neq: Option<Entity>)
    where
        P: Packet + 'static,
    {
        self.queue.push(BroadcastRequest {
            condition: BroadcastCondition::Chunk(chunk),
            packet: Box::new(packet),
            neq,
        });
    }
}

/// A broadcast request.
struct BroadcastRequest {
    /// Packet will only be sent to players able to see
    /// this entity or chunk.
    condition: BroadcastCondition,
    /// The packet to broadcast.
    packet: Box<dyn Packet>,
    /// Optional entity not to send to.
    neq: Option<Entity>,
}

#[derive(Debug, Clone, Copy)]
enum BroadcastCondition {
    Entity(Entity),
    Chunk(ChunkPosition),
}

/// System for flushing the `Broadcaster` queue and broadcasting
/// the necessary packets.
pub struct BroadcasterSystem;

impl<'a> System<'a> for BroadcasterSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, Util>,
        Read<'a, ChunkHolders>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, networks, util, chunk_holders, entities) = data;

        let broadcaster = &util.broadcaster;

        while let Ok(request) = broadcaster.queue.pop() {
            // Broadcast packet.
            // Iterate over entities in the chunk_holders
            // entry for the chunk. If they are a player,
            // send the packet.
            // This works because any player able to see
            // a chunk will always have a chunk holder on the chunk.

            let chunk = match request.condition {
                BroadcastCondition::Entity(entity) => {
                    // Prevents a panic if the entity was destroyed.
                    if !entities.is_alive(entity) {
                        continue;
                    }

                    positions.get(entity).unwrap().current.chunk_pos()
                }
                BroadcastCondition::Chunk(chunk) => chunk,
            };
            if let Some(holders) = chunk_holders.holders_for(chunk) {
                for holder in holders {
                    if let Some(neq) = request.neq.as_ref() {
                        if *holder == *neq {
                            continue;
                        }
                    }

                    if let Some(network) = networks.get(*holder) {
                        send_packet_boxed_to_player(network, request.packet.box_clone());
                    }
                }
            }
        }
    }
}
