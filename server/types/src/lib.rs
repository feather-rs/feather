#![forbid(unsafe_code)]

//! Defines components and resources so that subcrates can interact
//! in some ways without depending on each other.

extern crate nalgebra_glm as glm;

// CONSTANTS
/// The number of ticks per second.
pub const TPS: u64 = 20;
/// The number of milliseconds per tick.
pub const TICK_LENGTH: u64 = 1000 / TPS;

// COMPONENTS

mod physics;

pub use feather_inventory::Inventory;
pub use physics::{AABBExt, Physics, PhysicsBuilder};
pub use uuid::Uuid;

use feather_inventory::SlotIndex;
use feather_util::Position;

/// The item an entity is currently holding.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HeldItem(pub SlotIndex);

/// An entity's name.
#[derive(Debug, Clone, Default)]
pub struct Name(pub String);

/// Position of an entity on the previous tick.
#[derive(Copy, Clone, Debug)]
pub struct PreviousPosition(pub Position);

/// An entity's velocity.
#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub glm::DVec3);

/// Velocity of an entity on the previous tick.
#[derive(Copy, Clone, Debug)]
pub struct PreviousVelocity(pub glm::DVec3);

// RESOURCES

mod game;
pub use game::Game;
