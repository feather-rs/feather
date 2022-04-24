//! Allows customizing how the server loads world and player data.

use std::{collections::VecDeque, time::Duration};

use libcraft::{dimension::DimensionInfo, ChunkPosition};

use crate::{ChunkHandle, Game, WorldId};

pub mod worldgen;

/// A source with which chunks and entities are loaded and saved.
///
/// World loading is asynchronous. The server requests the world source
/// to load a chunk by calling `queue_load_chunk`. Then, each tick,
/// it polls for completed chunks by calling `poll_loaded_chunk`.
///
/// Likewise, saving is asynchronous and durable. The server calls
/// `queue_save_chunk` to save a chunk but will not assume the chunk
/// has been properly saved until `poll_saved_chunk` returns an Ok
/// status with the corresponding position.
///
/// An implementation of this trait should take advantage of the asynchronous
/// nature of world loading. It should never do IO or intensive
/// computations (e.g. world generation) on the main thread. Instead,
/// it should run IO on the Tokio runtime or world generation on the compute pool.
pub trait WorldSource: 'static {
    /// Returns whether the world source supports saving.
    fn supports_saving(&self) -> bool;

    /// Queues a chunk to be loaded.
    ///
    /// A future call to `poll_loaded_chunk` should return the result
    /// of loading this chunk.
    fn queue_load_chunk(&mut self, pos: ChunkPosition);

    /// Polls for the next chunk that has finished loading.
    ///
    /// Should return `None` if no chunks have completed loading.
    fn poll_loaded_chunk(&mut self) -> Option<ChunkLoadResult>;

    /// Queues a chunk to be saved into the world source.
    ///
    /// A future call to `poll_saved_chunk` should return the result
    /// of saving this chunk.
    ///
    /// If `supports_saving` returns false, then this method may panic.
    fn queue_save_chunk(&mut self, chunk: StoredChunk);

    /// Polls for the next chunk that has finshed saving.
    ///
    /// Should return `None` if no chunks have completed saving.
    ///
    /// If `supports_saving` returns false, then this method may panic.
    fn poll_saved_chunk(&mut self) -> Option<ChunkSaveResult>;
}

/// A loaded chunk.
///
/// Eventually, this structure will also contain the entities
/// and block entities loaded in the chunk.
#[derive(Debug)]
pub struct StoredChunk {
    /// Position of the chunk.
    pub pos: ChunkPosition,
    /// The chunk data.
    pub chunk: ChunkHandle,
}

/// Result of loading a chunk.
#[derive(Debug)]
pub struct ChunkLoadResult {
    /// Position of the loaded chunk.
    pub pos: ChunkPosition,
    /// The loaded chunk, or the error that occurred.
    pub result: Result<StoredChunk, ChunkLoadError>,
}

/// An error while loading a chunk.
#[derive(Debug, thiserror::Error)]
pub enum ChunkLoadError {
    /// The chunk is not contained in the world source.
    ///
    /// The server will fall back to using an empty chunk.
    #[error("world source does not contain the given chunk")]
    Missing,
    /// An error occurred loading the chunk, e.g., an IO
    /// error or a malformed data error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// The outcome of saving a chunk.
#[derive(Debug)]
pub struct ChunkSaveResult {
    /// The position of the chunk that was (attempted to be) saved.
    pub pos: ChunkPosition,
    /// Whether saving the chunk was successful.
    pub result: Result<ChunkSaved, ChunkSaveError>,
}

/// An error while saving a chunk.
#[derive(Debug)]
pub struct ChunkSaveError {
    /// The error that occurred.
    pub error: anyhow::Error,
    /// How long the server should wait until retrying.
    ///
    /// If saving failed because of a temporary error (e.g., a database
    /// restart), then this functionality may help retain world durability.
    ///
    /// The server will retry the same chunk a maximum number of times,
    /// configured in the world's `WorldSettings`. After reaching the retry limit,
    /// the server drops the chunk, and all changes are lost.
    pub retry_in: Duration,
}

/// Unit struct indicating a chunk was saved and is stored
/// in a persistent, durable location.
#[derive(Debug)]
pub struct ChunkSaved;

/// A world source that never yields any chunks.
#[derive(Default)]
pub struct EmptyWorldSource {
    queued: VecDeque<ChunkPosition>,
}

impl WorldSource for EmptyWorldSource {
    fn supports_saving(&self) -> bool {
        false
    }

    fn queue_load_chunk(&mut self, pos: ChunkPosition) {
        self.queued.push_back(pos);
    }

    fn poll_loaded_chunk(&mut self) -> Option<ChunkLoadResult> {
        self.queued.pop_front().map(|pos| ChunkLoadResult {
            pos,
            result: Err(ChunkLoadError::Missing),
        })
    }

    fn queue_save_chunk(&mut self, _chunk: StoredChunk) {
        unimplemented!()
    }

    fn poll_saved_chunk(&mut self) -> Option<ChunkSaveResult> {
        unimplemented!()
    }
}

/// A factory for a [`WorldSource`].
///
/// Registering `WorldSourceFactory`s allows the `config.toml`
/// file to create worlds using plugins' `WorldSource`s.
pub trait WorldSourceFactory: 'static {
    fn create_world_source(
        &self,
        game: &dyn Game,
        params: &toml::Value,
        dimension_info: &DimensionInfo,
        world_id: WorldId,
    ) -> anyhow::Result<Box<dyn WorldSource>>;
}
