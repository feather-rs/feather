//! Core functionality for Feather. This crate primarily
//! implements or reexports essential data structures, such as:
//! * Inventories
//! * The block ID system
//! * The chunk data structure

use std::time::Duration;

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

pub mod anvil;
mod block;
pub mod chunk;
pub mod chunk_lock;
pub mod inventory;
pub mod metadata;

pub use block::{BlockPositionValidationError, ValidBlockPosition};
pub use blocks::*;
pub use chunk::{Chunk, ChunkSection, CHUNK_HEIGHT, CHUNK_WIDTH};
pub use chunk_lock::*;

pub use libcraft_blocks::{BlockKind, BlockState};
pub use libcraft_core::{
    position, vec3, Biome, BlockPosition, ChunkPosition, EntityKind, Gamemode, Position, Vec3d,
};
pub use libcraft_inventory::{Area, Inventory};
pub use libcraft_items::{Item, ItemStack, ItemStackBuilder, ItemStackError};
pub use libcraft_particles::{Particle, ParticleKind};
pub use libcraft_text::{deserialize_text, Text, Title};
#[doc(inline)]
pub use metadata::EntityMetadata;

/// Number of updates (ticks) to do per second.
pub const TPS: u32 = 20;
/// The number of milliseconds per tick.
pub const TICK_MILLIS: u32 = 1000 / TPS;
/// The duration of a tick.
pub const TICK_DURATION: Duration = Duration::from_millis(TICK_MILLIS as u64);

/// Default port for Minecraft servers.
pub const DEFAULT_PORT: u16 = 25565;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A profile property, which stores metadata
/// for some player's account. This is usually
/// used to store skin data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}
