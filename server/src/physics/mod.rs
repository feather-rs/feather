//! Module for calculating physics interactions.

mod block_bboxes;
mod component;
mod entity;
mod math;

use crate::systems::ENTITY_PHYSICS;
pub use component::{AABBExt, PhysicsBuilder, PhysicsComponent};
pub use entity::{EntityPhysicsLandEvent, EntityPhysicsSystem};
pub use math::*;
use specs::DispatcherBuilder;

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(EntityPhysicsSystem, ENTITY_PHYSICS, &[]);
}

pub fn init_handlers(_dispatcher: &mut DispatcherBuilder) {
    // nothing
}
