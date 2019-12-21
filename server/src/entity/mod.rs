//! Dealing with entities.

use crate::lazy::EntityBuilder;
use crate::state::State;
use feather_core::Position;
use legion::prelude::Entity;
use legion::query::{Read, Write};
use std::ops::{Deref, DerefMut};
use tonks::{PreparedWorld, Query};

/// Event triggered when an entity is removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityDeleteEvent {
    pub(crate) entity: Entity,
}

/// Event triggered when an entity moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityMoveEvent {
    /// Entity which moved.
    pub entity: Entity,
}

/// The velocity of an entity.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Velocity(pub glm::DVec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(glm::vec3(0.0, 0.0, 0.0))
    }
}

impl Deref for Velocity {
    type Target = glm::DVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The display name of the entity.
///
/// Note that unnamed entities do not have this component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameComponent(pub String);

/// Position of an entity on the last tick.
///
/// This is updated by `position_reset` system.
#[derive(Debug, Clone, Copy)]
pub struct PreviousPosition(pub Position);

#[event_handler]
pub fn position_reset(
    events: &[EntityMoveEvent],
    _query: &mut Query<(Read<Position>, Write<PreviousPosition>)>,
    world: &mut PreparedWorld,
) {
    events.iter().for_each(|event| {
        let pos = *world.get_component::<Position>(event.entity).unwrap();
        let mut prev_pos = world
            .get_component_mut::<PreviousPosition>(event.entity)
            .unwrap();

        prev_pos.0 = pos;
    });
}

/// Inserts the base components for an entity into an `EntityBuilder`.
///
/// This currently includes:
/// * Position
/// * Velocity (0)
pub fn base(state: &State, position: Position) -> EntityBuilder {
    state
        .create_entity()
        .with_component(position)
        .with_component(PreviousPosition(position))
        .with_component(Velocity::default())
}
