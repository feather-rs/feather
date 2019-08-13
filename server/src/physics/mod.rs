//! Module for calculating physics interactions.

mod component;
mod entity;

pub use component::{BoundingBoxComponent, PhysicsInitSystem};
pub use entity::EntityPhysicsSystem;
