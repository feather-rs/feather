use crate::view::View;

use quill::{ChunkPosition, WorldId};

pub use block_change::BlockChangeEvent;
pub use plugin_message::PluginMessageEvent;
use vane::Component;

mod block_change;
mod plugin_message;

pub use quill::events::{EntityCreateEvent, EntityRemoveEvent};

/// Event triggered when a player changes their `View`,
/// meaning they crossed into a new chunk.
#[derive(Debug)]
pub struct ViewUpdateEvent {
    pub old_view: View,
    pub new_view: View,

    /// Chunks that are in `new_view` but not `old_view`
    pub new_chunks: Vec<ChunkPosition>,
    /// Chunks that are in `old_view` but not in `new_view`
    pub old_chunks: Vec<ChunkPosition>,

    pub new_world: WorldId,
    pub old_world: WorldId,
}

impl Component for ViewUpdateEvent {}

impl ViewUpdateEvent {
    pub fn new(old_view: &View, new_view: &View) -> Self {
        let mut this = Self {
            old_view: old_view.clone(),
            new_view: new_view.clone(),
            new_chunks: new_view.difference(old_view),
            old_chunks: old_view.difference(new_view),
            new_world: new_view.world(),
            old_world: old_view.world(),
        };
        this.new_chunks
            .sort_unstable_by_key(|chunk| chunk.distance_squared_to(new_view.center()));
        this.old_chunks
            .sort_unstable_by_key(|chunk| chunk.distance_squared_to(old_view.center()));
        this
    }
}

/// Event triggered when an entity crosses into a new chunk.
///
/// Unlike [`ViewUpdateEvent`], this event triggers for all entities,
/// not just players.
pub struct ChunkCrossEvent {
    pub old_chunk: ChunkPosition,
    pub new_chunk: ChunkPosition,
}

impl Component for ChunkCrossEvent {}

/// Triggered when an error occurs while loading a chunk.
#[derive(Debug)]
pub struct ChunkLoadFailEvent {
    pub position: ChunkPosition,
}

impl Component for ChunkLoadFailEvent {}

/// Triggered when a player joins, changes dimension and respawns after death
#[derive(Debug)]
pub struct PlayerRespawnEvent;

impl Component for PlayerRespawnEvent {}
