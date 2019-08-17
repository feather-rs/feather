//! Module for broadcasting when an entity comes within
//! range of a player.

use crate::entity::movement::degrees_to_stops;
use crate::entity::{EntityType, VelocityComponent};
use crate::entity::{Metadata, NamedComponent, PositionComponent};
use crate::network::{send_packet_to_all_players, NetworkComponent};
use crate::util::protocol_velocity;
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::network::packet::implementation::{PacketEntityMetadata, SpawnPlayer};
use shrev::EventChannel;
use specs::{
    Entities, Entity, Read, ReadStorage, ReaderId, System, SystemData, World, WriteStorage,
};
use uuid::Uuid;

//const ITEM_OBJECT_ID: i8 = 2;

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
/// Different entity types require different packets
/// to send.
///
/// This system listens to `EntitySpawnEvent`s.
#[derive(Default)]
pub struct EntityBroadcastSystem {
    reader: Option<ReaderId<EntitySpawnEvent>>,
}

impl<'a> System<'a> for EntityBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, NamedComponent>,
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, VelocityComponent>,
        WriteStorage<'a, Metadata>,
        Read<'a, EventChannel<EntitySpawnEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, nameds, networks, velocities, mut metadatas, events, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let position = positions.get(event.entity).unwrap();
            let metadata = metadatas.get_mut(event.entity).unwrap();
            let velocity = velocities.get(event.entity).cloned().unwrap_or_default();
            let (velocity_x, velocity_y, velocity_z) = protocol_velocity(*velocity);

            // Send spawn packet to clients.
            // The packet type depends on the type
            // of entity.

            // The Player Info packet was already sent by `JoinBroadcastSystem`.
            match event.ty {
                EntityType::Player => {
                    let named = nameds.get(event.entity).unwrap();
                    let packet = SpawnPlayer {
                        entity_id: event.entity.id() as i32,
                        player_uuid: named.uuid,
                        x: position.current.x,
                        y: position.current.y,
                        z: position.current.z,
                        yaw: degrees_to_stops(position.current.yaw),
                        pitch: degrees_to_stops(position.current.pitch),
                        metadata: metadata.to_raw_metadata(),
                    };

                    send_packet_to_all_players(&networks, &entities, packet, Some(event.entity));
                }
                EntityType::Item => {
                    let packet = SpawnObject {
                        entity_id: event.entity.id() as i32,
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

                    send_packet_to_all_players(&networks, &entities, packet, Some(event.entity));
                }
                EntityType::ExperienceOrb => unimplemented!(),
                EntityType::Thunderbolt => unimplemented!(),
            }

            // Send metadata.
            let entity_metadata = PacketEntityMetadata {
                entity_id: event.entity.id() as i32,
                metadata: metadata.to_raw_metadata(),
            };
            send_packet_to_all_players(&networks, &entities, entity_metadata, None);
            // Players should know their own metadata
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let (mut w, mut d) = t::init_world();

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
