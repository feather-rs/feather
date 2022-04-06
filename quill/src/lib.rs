//! Quill, Feather's plugin API.

pub mod components;
/// Marker components for each specific entity.
pub mod entities;
pub mod events;
mod game;
mod plugin;

#[doc(inline)]
pub use vane::{Entities, Entity, EntityBuilder, EntityRef, Resources, SysResult};

pub use game::Game;
pub use plugin::{Plugin, Setup};

#[doc(inline)]
pub use libcraft_blocks::{block_data, BlockData, BlockKind, BlockState};
#[doc(inline)]
pub use libcraft_core::{BlockPosition, ChunkPosition, Position};
