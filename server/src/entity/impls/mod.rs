//! Entity implementations.
//!
//! Every entity implementation is expected to define
//! the following functions:
//!
//! * `create(&LazyUpdate, &EntitiesRes) -> LazyBuilder`. When a system spawns an entity
//! of a known type, it should call this function on the `LazyBuilder`
//! returned by `LazyUpdate::spawn_entity` to apply components, such as markers, metadata,
//! `SerializerComponent`, and `SpawnPacketComponent`. This function may
//! take parameters. This function should not apply generic components,
//! such as position and velocity; the callee is responsible for this.
//! * `create_from_data(&LazyUpdate, &EntitiesRes, &{Entity}Data) -> Option<Entity>`. Spawns an
//! entity loaded from the given entity data. If the entity creation failed, `None` is returned.
//!
//! These functions should be invoked in the form `name::function`, e.g.
//! `arrow::create` or `item::create_from_data`.
//!
//! Entity implementations should also define systems related to the entity: for
//! example, most entities will have an update system which updates an entity
//! on each tick.

pub mod arrow;
pub mod falling_block;
pub mod item;

mod animal;
pub use animal::*;

use crate::entity::{
    degrees_to_stops, metadata::EMPTY_METADATA, Metadata, NamedComponent, PositionComponent,
    VelocityComponent,
};
use crate::util::protocol_velocity;
use feather_core::entity::BaseEntityData;
use feather_core::network::packet::implementation::SpawnMob;
use feather_core::Packet;
use specs::{Entity, World, WorldExt};
use uuid::Uuid;

#[cfg(test)]
pub mod test;

/// Returns a `Spawn Mob` packet with the given entity type ID.
pub fn create_mob_packet(world: &World, entity: Entity, type_id: i32) -> Box<dyn Packet> {
    let entity_id = entity.id() as i32;
    let entity_uuid = world
        .read_component::<NamedComponent>()
        .get(entity)
        .map(|named| named.uuid)
        .unwrap_or_else(Uuid::new_v4);

    let positions = world.read_component::<PositionComponent>();
    let position = positions.get(entity).copied().unwrap_or_default();
    let velocities = world.read_component::<VelocityComponent>();
    let velocity = velocities.get(entity).copied().unwrap_or_default();

    let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);

    let metadatas = world.read_component::<Metadata>();
    let metadata = metadatas.get(entity).unwrap_or(&EMPTY_METADATA);

    let packet = SpawnMob {
        entity_id,
        entity_uuid,
        ty: type_id,
        x: position.current.x,
        y: position.current.y,
        z: position.current.z,
        yaw: degrees_to_stops(position.current.yaw),
        pitch: degrees_to_stops(position.current.pitch),
        head_pitch: degrees_to_stops(position.current.pitch), // FIXME: is this correct?
        velocity_x,
        velocity_y,
        velocity_z,
        meta: metadata.to_full_raw_metadata(),
    };

    Box::new(packet)
}

/// Creates a `BaseEntityData` for the given entity.
pub fn base_data(world: &World, entity: Entity) -> BaseEntityData {
    let position = world
        .read_component::<PositionComponent>()
        .get(entity)
        .copied()
        .unwrap_or_default();
    let velocity = world
        .read_component::<VelocityComponent>()
        .get(entity)
        .copied()
        .unwrap_or_default();

    BaseEntityData::new(position.current, velocity.0)
}
