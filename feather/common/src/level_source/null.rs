use base::ChunkPosition;

use super::{ChunkLoadResult, LevelSource, LoadedChunk};

/// A world source that always yields `ChunkLoadResult::Missing`.
#[derive(Default)]
pub struct NullLevelSource {
    loaded: Vec<ChunkPosition>,
}

impl LevelSource for NullLevelSource {
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
