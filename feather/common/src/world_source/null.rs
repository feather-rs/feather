use base::ChunkPosition;

use super::{ChunkLoadResult, LoadedChunk, WorldSource};

/// A world source that always yields `ChunkLoadResult::Missing`.
#[derive(Default)]
pub struct NullWorldSource {
    loaded: Vec<ChunkPosition>,
}

impl WorldSource for NullWorldSource {
    fn queue_load(&mut self, pos: ChunkPosition) {
        self.loaded.push(pos);
    }

    fn poll_loaded_chunk(&mut self) -> Option<super::LoadedChunk> {
        self.loaded.pop().map(|pos| LoadedChunk {
            pos,
            result: ChunkLoadResult::Missing,
        })
    }
}
