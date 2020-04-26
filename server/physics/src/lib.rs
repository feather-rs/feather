//! Module for calculating physics interactions.

extern crate nalgebra_glm as glm;

mod block_bboxes;
mod entity;
mod math;

pub use entity::entity_physics;
pub use math::*;
