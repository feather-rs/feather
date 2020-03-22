use crate::entity;
use crate::entity::{ComponentSerializer, EntityId, SpawnPacketCreator, Velocity};
use crate::game::Game;
use crate::physics::PhysicsBuilder;
use crate::util::{degrees_to_stops, protocol_velocity};
use feather_core::entity::{ArrowEntityData, BaseEntityData, EntityData};
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::{Packet, Position, Vec3d};
use fecs::{EntityRef, World, EntityBuilder};
use uuid::Uuid;

pub fn create(position: Position, velocity: glm::DVec3) -> EntityBuilder {
    entity::base(position)
        .with(Velocity(velocity))
        .with(SpawnPacketCreator(&create_spawn_packet))
        .with(ComponentSerializer(&serialize))
        .with(
            PhysicsBuilder::new()
                .bbox(0.5, 0.5, 0.5)
                .gravity(-0.05)
                .slip_multiplier(0.0)
                .drag(0.99)
                .build(),
        )
}

fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let position = *accessor.get::<Position>();
    let velocity = *accessor.get::<Velocity>();
    let entity_id = accessor.get::<EntityId>().0;

    let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);

    let packet = SpawnObject {
        entity_id,
        object_uuid: Uuid::new_v4(),
        ty: 60, // Type 60 for arrow projectile
        x: position.x,
        y: position.y,
        z: position.z,
        pitch: degrees_to_stops(position.pitch),
        yaw: degrees_to_stops(position.yaw),
        data: entity_id + 1,
        velocity_x,
        velocity_y,
        velocity_z,
    };

    Box::new(packet)
}

fn serialize(_game: &Game, accessor: &EntityRef) -> EntityData {
    let vel = accessor.get::<Velocity>();

    EntityData::Arrow(ArrowEntityData {
        entity: BaseEntityData::new(*accessor.get::<Position>(), Vec3d::new(vel.x, vel.y, vel.z)),
        critical: 0, // TODO
    })
}
