//! Module for broadcasting when an entity comes within
//! range of a player. Also handles sending the correct
//! packet to spawn entities on the client.
//!
//! Sending entities to a client is handled lazily
//! through `LazyUpdate`, because arbitrary components
//! may need to be accessed.

use crate::chunk_logic::ChunkHolders;
use crate::entity::movement::degrees_to_stops;
use crate::entity::{
    EntityType, FallingBlockComponent, LastKnownPositionComponent, PacketCreatorComponent,
    VelocityComponent,
};
use crate::entity::{Metadata, NamedComponent, PositionComponent};
use crate::lazy::LazyUpdateExt;
use crate::network::{send_packet_boxed_to_player, send_packet_to_player, NetworkComponent};
use crate::util::protocol_velocity;
use crossbeam::queue::SegQueue;
use feather_blocks::BlockExt;
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::network::packet::implementation::{PacketEntityMetadata, SpawnPlayer};
use feather_core::Packet;
use shrev::EventChannel;
use specs::{
    Entities, Entity, LazyUpdate, Read, ReadStorage, ReaderId, System, WorldExt, Write,
    WriteStorage,
};
use uuid::Uuid;

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

/// Event triggered when an entity of any
/// type is spawned.
#[derive(Debug, Clone)]
pub struct EntitySpawnEvent {
    /// The spawned entity.
    pub entity: Entity,
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
        Read<'a, EventChannel<EntitySpawnEvent>>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, networks, chunk_holders, spawn_events, lazy) = data;

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

                    lazy.send_entity_to_player(*holder, event.entity);
                }
            }
        }
    }

    setup_impl!(reader);
}

/// Lazily sends an entity to a player.
pub fn send_entity_to_player(lazy: &LazyUpdate, player: Entity, entity: Entity) {
    lazy.exec(move |world| {
        // Attempt to get the `PacketCreator` for the entity.
        // If it doesn't exist, skip sending.
        let packet_creator = match world.read_component::<PacketCreatorComponent>().get(entity) {
            Some(packet_creator) => packet_creator,
            None => return,
        };

        let create_packet = packet_creator.0;
        let packet = create_packet(world, entity);

        if let Some(network) = world.read_component::<NetworkComponent>().get(player) {
            send_packet_boxed_to_player(network, packet);
        }

        // Trigger event
        let event = EntitySendEvent { entity, player };
        world.fetch_mut::<EventChannel<_>>().single_write(event);
    });
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
