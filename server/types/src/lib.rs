//! Defines components and resources so that subcrates can interact
//! in some ways without depending on each other.

extern crate nalgebra_glm as glm;

mod components;
mod events;
mod game;
mod misc;
mod resources;
pub mod task;

pub use components::*;
pub use events::*;
pub use misc::*;
pub use resources::*;

// Constants
/// The number of ticks executed per second.
pub const TPS: u64 = 20;
/// The number of milliseconds per tick.
pub const TICK_LENGTH: u64 = 1000 / TPS;

/// Height from a player's position where the camera lies.
pub const PLAYER_EYE_HEIGHT: f64 = 1.62;
