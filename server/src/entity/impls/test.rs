//! A fake entity implementation for unit tests.

use crate::entity::PositionComponent;
use feather_core::Position;
use specs::{Builder, EntityBuilder, World, WorldExt};

pub fn create(world: &mut World, pos: Position) -> EntityBuilder {
    world.create_entity().with(PositionComponent {
        current: pos,
        previous: pos,
    })
}
