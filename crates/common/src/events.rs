use std::sync::Arc;

use base::{Chunk, ChunkPosition};
use parking_lot::RwLock;

use crate::view::View;

/// Triggered when a player joins the `Game`.
#[derive(Debug)]
pub struct PlayerJoinEvent;

/// Event triggered when a player changes their `View`,
/// meaning they crossed into a new chunk.
#[derive(Debug)]
pub struct ViewUpdateEvent {
    pub old_view: View,
    pub new_view: View,
}

/// Triggered when a chunk is loaded.
#[derive(Debug)]
pub struct ChunkLoadEvent {
    pub position: ChunkPosition,
    pub chunk: Arc<RwLock<Chunk>>,
}

/// Triggered when an error occurs while loading a chunk.
#[derive(Debug)]
pub struct ChunkLoadFailEvent {
    pub position: ChunkPosition,
}
