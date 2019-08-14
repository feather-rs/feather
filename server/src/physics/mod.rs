//! Module for calculating physics interactions.

mod component;
mod entity;
mod math;

pub use component::{BoundingBoxComponent, EntityVelocityUpdateEvent, PhysicsInitSystem};
pub use entity::EntityPhysicsSystem;
pub use math::*;
