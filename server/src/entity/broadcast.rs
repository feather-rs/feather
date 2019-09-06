//! Module for broadcasting when an entity comes within
//! range of a player. Also handles sending the correct
//! packet to spawn entities on the client.
//!
//! Sending entities to a client is handled lazily:
//! an internal queue is kept of entities to send to players,
//! and each tick, a system flushes this queue and sends
//! the correct packet. This is done because of the number
//! of components which need to be accessed to send an entity
//! to a player.

use crate::chunk_logic::ChunkHolders;
use crate::entity::movement::degrees_to_stops;
use crate::entity::{EntityType, VelocityComponent};
use crate::entity::{Metadata, NamedComponent, PositionComponent};
use crate::network::{send_packet_boxed_to_player, send_packet_to_player, NetworkComponent};
use crate::util::protocol_velocity;
use crossbeam::queue::SegQueue;
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::network::packet::implementation::{PacketEntityMetadata, SpawnPlayer};
use feather_core::Packet;
use shrev::EventChannel;
use specs::{Entity, Read, ReadStorage, ReaderId, System, Write, WriteStorage};
use uuid::Uuid;

/// Handles lazy sending of entities to a client.
#[derive(Debug, Default)]
pub struct EntitySender {
    /// A queue of entities to lazily send.
    queue: SegQueue<SendRequest>,
}

impl EntitySender {
    /// Lazily sends an entity to a client.
    pub fn send_entity_to_player(&self, player: Entity, entity: Entity) {
        self.queue.push(SendRequest { player, entity })
    }
}

/// An entity send request, containing
/// the player to send to and the entity
/// to send.
#[derive(Debug)]
struct SendRequest {
    player: Entity,
    entity: Entity,
}

/// Event which is triggered when an entity
/// is sent to a client. This can be used to send
/// associated information, such as entity equipment.
#[derive(Debug, Clone)]
pub struct EntitySendEvent {
    /// The player for which this event was triggered.
    pub player: Entity,
    /// The entity which was sent to the player.
    pub entity: Entity,
}

/// System for flushing the `EntitySender` queue
/// and sending the correct packets for the given
/// entities.
pub struct EntitySendSystem;

impl<'a> System<'a> for EntitySendSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, NamedComponent>,
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, VelocityComponent>,
        ReadStorage<'a, EntityType>,
        WriteStorage<'a, Metadata>,
        Write<'a, EventChannel<EntitySendEvent>>,
        Read<'a, EntitySender>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            positions,
            nameds,
            networks,
            velocities,
            types,
            mut metadatas,
            mut send_events,
            entity_sender,
        ) = data;

        while let Ok(request) = entity_sender.queue.pop() {
            let ty = types.get(request.entity).unwrap();
            let metadata = metadatas.get_mut(request.entity).unwrap();
            let position = positions.get(request.entity).unwrap();
            let velocity = velocities.get(request.entity);
            let named = nameds.get(request.entity);

            let network = networks.get(request.player).unwrap();

            // Send corresponding packet to player.
            let packet =
                packet_to_spawn_entity(request.entity, *ty, &position, metadata, velocity, named);
            send_packet_boxed_to_player(&network, packet);

            // Send metadata.
            let entity_metadata = PacketEntityMetadata {
                entity_id: request.entity.id() as i32,
                metadata: metadata.to_full_raw_metadata(),
            };

            send_packet_to_player(network, entity_metadata);

            // Trigger event.
            let event = EntitySendEvent {
                player: request.player,
                entity: request.entity,
            };
            send_events.single_write(event);
        }
    }
}

/// Event triggered when an entity of any
/// type is spawned.
#[derive(Debug, Clone)]
pub struct EntitySpawnEvent {
    /// The spawned entity.
    pub entity: Entity,
    /// The type of the spawned entity.
    pub ty: EntityType,
}

/// System for broadcasting when an entity is spawned.
///
/// Broadcasts are lazily queued for sending
/// and are sent by `EntitySendSystem`.
///
/// This system listens to `EntitySpawnEvent`s.
#[derive(Default)]
pub struct EntityBroadcastSystem {
    reader: Option<ReaderId<EntitySpawnEvent>>,
}

impl<'a> System<'a> for EntityBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, ChunkHolders>,
        Read<'a, EntitySender>,
        Read<'a, EventChannel<EntitySpawnEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, networks, chunk_holders, entity_sender, spawn_events) = data;

        for event in spawn_events.read(self.reader.as_mut().unwrap()) {
            // Broadcast entity to players who can see it.
            let position = match positions.get(event.entity) {
                Some(position) => position,
                None => continue,
            };
            let chunk = position.current.chunk_pos();

            if let Some(holders) = chunk_holders.holders_for(chunk) {
                for holder in holders {
                    if networks.get(*holder).is_none() {
                        // Not a player.
                        continue;
                    }

                    // Don't send player to themself.
                    if *holder == event.entity {
                        continue;
                    }

                    entity_sender.send_entity_to_player(*holder, event.entity);
                }
            }
        }
    }

    setup_impl!(reader);
}

/// Returns the packet needed to spawn an entity
/// with given type, position, metadata, optional velocity,
/// and optional name.
fn packet_to_spawn_entity(
    entity: Entity,
    ty: EntityType,
    position: &PositionComponent,
    metadata: &mut Metadata,
    velocity: Option<&VelocityComponent>,
    named: Option<&NamedComponent>,
) -> Box<dyn Packet> {
    let velocity = velocity.cloned().unwrap_or_default(); // Use default velocity of (0, 0, 0)
    let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);

    // Different entity types require different
    // packets to send.
    match ty {
        EntityType::Player => {
            let named = named.unwrap();
            let packet = SpawnPlayer {
                entity_id: entity.id() as i32,
                player_uuid: named.uuid,
                x: position.current.x,
                y: position.current.y,
                z: position.current.z,
                yaw: degrees_to_stops(position.current.yaw),
                pitch: degrees_to_stops(position.current.pitch),
                metadata: metadata.to_raw_metadata(),
            };

            Box::new(packet)
        }
        EntityType::Item => {
            let packet = SpawnObject {
                entity_id: entity.id() as i32,
                object_uuid: Uuid::new_v4(),
                ty: 2, // Type 2 for item stack
                x: position.current.x,
                y: position.current.y,
                z: position.current.z,
                pitch: degrees_to_stops(position.current.pitch),
                yaw: degrees_to_stops(position.current.yaw),
                data: 1, // Has velocity
                velocity_x,
                velocity_y,
                velocity_z,
            };

            Box::new(packet)
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::ChunkCrossSystem;
    use crate::testframework as t;
    use feather_core::network::cast_packet;
    use feather_core::network::packet::PacketType;
    use specs::WorldExt;

    #[test]
    fn test_spawn_player() {
        let (mut w, mut d) = t::init_world();

        let player1 = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        let event = EntitySpawnEvent {
            entity: player1.entity,
            ty: EntityType::Player,
        };

        w.fetch_mut::<EventChannel<_>>().single_write(event);

        d.dispatch(&w);
        w.maintain();

        t::assert_packet_not_received(&player1, PacketType::SpawnPlayer); // Player shouldn't have received packet for themselves

        let packet = t::assert_packet_received(&player2, PacketType::SpawnPlayer);
        let packet = cast_packet::<SpawnPlayer>(&*packet);

        assert_eq!(packet.entity_id, player1.entity.id() as i32);
    }

    #[test]
    fn test_spawn_item() {
        let (mut w, mut d) = t::builder()
            .with(EntityBroadcastSystem::default(), "broadcast")
            .with(ChunkCrossSystem::default(), "chunk_cross")
            .with_dep(EntitySendSystem, "", &["broadcast", "chunk_cross"])
            .build();

        let player = t::add_player(&mut w);

        let item = t::add_entity(&mut w, EntityType::Item, true);

        d.dispatch(&w);
        w.maintain();

        let spawn_entity = t::assert_packet_received(&player, PacketType::SpawnObject);
        let spawn_entity = cast_packet::<SpawnObject>(&*spawn_entity);

        assert_eq!(spawn_entity.entity_id, item.id() as i32);
        assert_eq!(spawn_entity.velocity_x, 0);
    }
}
