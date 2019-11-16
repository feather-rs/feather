//! Dealing with entities.

use crate::lazy::EntityBuilder;
use crate::state::State;
use feather_core::{ChunkPosition, Position};
use legion::prelude::Entity;
use parking_lot::Mutex;

/// Event triggered when an entity is removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityDeleteEvent {
    pub(crate) entity: Entity,
}

/// The velocity of an entity.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Velocity(pub glm::DVec3);

/// The display name of the entity.
///
/// Note that unnamed entities do not have this component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameComponent(pub String);

/// Inserts the base components for an entity into an `EntityBuilder`.
///
/// This currently includes:
/// * Position
/// * Velocity (0)
pub fn base(state: &State, position: Position) -> EntityBuilder {
    state
        .create_entity()
        .with_component(position)
        .with_component(Velocity::default())
}
