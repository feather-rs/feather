//! Core functionality for Feather. This crate primarily
//! implements or reexports essential data structures, such as:
//! * Inventories
//! * The block ID system
//! * The chunk data structure

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

pub use block::{BlockPositionValidationError, ValidBlockPosition};
pub use chunk::{Chunk, ChunkSection, CHUNK_WIDTH};
pub use chunk_lock::*;
pub use libcraft_blocks::{BlockKind, BlockState};
pub use libcraft_core::{
    consts::*, position, vec3, BlockPosition, ChunkPosition, EntityKind, Gamemode, Position, Vec3d,
};
pub use libcraft_inventory::{Area, Inventory};
pub use libcraft_items::{Item, ItemStack, ItemStackBuilder, ItemStackError};
pub use libcraft_particles::{Particle, ParticleKind};
pub use libcraft_text::{deserialize_text, Text, Title};
#[doc(inline)]
pub use metadata::EntityMetadata;

pub mod anvil;
pub mod biome;
mod block;
pub mod chunk;
pub mod chunk_lock;
pub mod inventory;
pub mod metadata;
pub mod world;

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
