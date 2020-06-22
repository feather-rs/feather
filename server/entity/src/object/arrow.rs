use feather_core::anvil::entity::{ArrowEntityData, BaseEntityData, EntityData};
use feather_core::network::packets::SpawnObject;
use feather_core::network::Packet;
use feather_core::util::{Position, Vec3d};
use feather_server_types::{
    ComponentSerializer, Game, NetworkId, PhysicsBuilder, SpawnPacketCreator, Uuid, Velocity,
};
use feather_server_util::{degrees_to_stops, protocol_velocity};
use fecs::{EntityBuilder, EntityRef};

pub fn create() -> EntityBuilder {
    crate::base()
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
    let entity_id = accessor.get::<NetworkId>().0;

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
    let vel = accessor.get::<Velocity>().0;

    EntityData::Arrow(ArrowEntityData {
        entity: BaseEntityData::new(*accessor.get::<Position>(), Vec3d::new(vel.x, vel.y, vel.z)),
        critical: 0, // TODO
    })
}
