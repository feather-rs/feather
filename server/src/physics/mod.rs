//! Module for calculating physics interactions.

mod block_bboxes;
mod component;
mod entity;
mod math;

pub use component::{AABBExt, Physics, PhysicsBuilder};
pub use entity::EntityPhysicsLandEvent;
pub use math::*;
