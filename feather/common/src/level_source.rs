use base::{Chunk, ChunkPosition};

pub mod flat;
pub mod null;
pub mod region;

/// A chunk loaded from a [`WorldSource`].
pub struct LoadedChunk {
    pub pos: ChunkPosition,
    pub result: ChunkLoadResult,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ChunkLoadResult {
    /// The chunk does not exist in this source.
    Missing,
    /// An error occurred while loading the chunk.
    Error(anyhow::Error),
    /// Successfully loaded the chunk.
    Loaded { chunk: Chunk },
}

/// Provides methods to load chunks, entities, and global world data.
pub trait LevelSource: 'static {
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

    /// Creates a `WorldSource` that falls back to `fallback`
    /// if chunks in `self` are missing or corrupt.
    fn with_fallback(self, fallback: impl LevelSource) -> FallbackWorldSource
    where
        Self: Sized,
    {
        FallbackWorldSource {
            first: Box::new(self),
            fallback: Box::new(fallback),
        }
    }
}

/// `WorldSource` wrapping two world sources. Falls back
/// to the second source if the first one is missing a chunk.
pub struct FallbackWorldSource {
    first: Box<dyn LevelSource>,
    fallback: Box<dyn LevelSource>,
}

impl LevelSource for FallbackWorldSource {
    fn queue_load(&mut self, pos: ChunkPosition) {
        self.first.queue_load(pos);
    }

    fn poll_loaded_chunk(&mut self) -> Option<LoadedChunk> {
        self.first
            .poll_loaded_chunk()
            .map(|chunk| {
                if matches!(
                    &chunk.result,
                    ChunkLoadResult::Error(_) | ChunkLoadResult::Missing
                ) {
                    self.fallback.queue_load(chunk.pos);
                    log::trace!(
                        "Chunk load falling back (failure cause: {:?})",
                        chunk.result
                    );
                    None
                } else {
                    Some(chunk)
                }
            })
            .flatten()
            .or_else(|| self.fallback.poll_loaded_chunk())
    }
}
