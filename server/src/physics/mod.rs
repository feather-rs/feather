//! Module for calculating physics interactions.

mod component;
mod entity;
mod math;

use crate::systems::{ENTITY_PHYSICS, PHYSICS_INIT};
pub use component::{BoundingBoxComponent, PhysicsInitSystem};
pub use entity::EntityPhysicsSystem;
pub use math::*;
use specs::DispatcherBuilder;

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(EntityPhysicsSystem, ENTITY_PHYSICS, &[]);
}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(PhysicsInitSystem::default(), PHYSICS_INIT, &[]);
}
