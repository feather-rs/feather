use base::{Chunk, ChunkPosition};

pub mod flat;
pub mod null;

/// A chunk loaded from a [`WorldSource`].
pub struct LoadedChunk {
    pub pos: ChunkPosition,
    pub result: ChunkLoadResult,
}

pub enum ChunkLoadResult {
    /// The chunk does not exist in this source.
    Missing,
    /// An error occurred while loading the chunk.
    Error(anyhow::Error),
    /// Successfully loaded the chunk.
    Loaded { chunk: Chunk },
}

/// Provides methods to load chunks, entities, and global world data.
pub trait WorldSource {
    /// Enqueues the chunk at `pos` to be loaded.
    /// A future call to `poll_loaded_chunk` should
    /// return this chunk.
    fn queue_load(&mut self, pos: ChunkPosition);

    /// Polls for the next loaded chunk. Should not block
    ///
    /// The order in which chunks are loaded is not defined. In other
    /// words, this method does not need to yield chunks in the
    /// same order they were queued for loading.
    fn poll_loaded_chunk(&mut self) -> Option<LoadedChunk>;
}
