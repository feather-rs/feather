//! A fake entity implementation for unit tests.

use crate::entity::{EntitySpawnEvent, PositionComponent, VelocityComponent};
use feather_core::Position;
use shrev::EventChannel;
use specs::{Builder, EntityBuilder, World, WorldExt};

pub fn create(world: &mut World, pos: Position) -> EntityBuilder {
    let builder = world
        .create_entity()
        .with(PositionComponent {
            current: pos,
            previous: pos,
        })
        .with(VelocityComponent(glm::vec3(0.0, 0.0, 0.0)));
    builder
        .world
        .fetch_mut::<EventChannel<EntitySpawnEvent>>()
        .single_write(EntitySpawnEvent {
            entity: builder.entity,
        });
    builder
}
