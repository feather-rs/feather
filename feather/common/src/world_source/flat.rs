use base::{BlockId, Chunk, ChunkPosition, CHUNK_WIDTH};

use super::{ChunkLoadResult, LoadedChunk, WorldSource};

/// A world source used for debugging that emits
/// only flat chunks and does no IO.
pub struct FlatWorldSource {
    archetype: Chunk,
    loaded: Vec<LoadedChunk>,
}

impl Default for FlatWorldSource {
    fn default() -> Self {
        Self::new()
    }
}

impl FlatWorldSource {
    pub fn new() -> Self {
        let archetype = Self::generate();

        Self {
            archetype,
            loaded: Vec::new(),
        }
    }

    fn generate() -> Chunk {
        let mut archetype = Chunk::new(ChunkPosition::new(0, 0));
        for y in 0..64 {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_WIDTH {
                    archetype.set_block_at(x, y, z, Self::select_block(y));
                }
            }
        }
        archetype
    }

    fn select_block(height: usize) -> BlockId {
        match height {
            0 => BlockId::bedrock(),
            1..=57 => BlockId::stone(),
            58..=62 => BlockId::dirt(),
            63 => BlockId::grass_block(),
            _ => BlockId::air(),
        }
    }
}

impl WorldSource for FlatWorldSource {
    fn queue_load(&mut self, pos: base::ChunkPosition) {
        let mut chunk = self.archetype.clone();
        chunk.set_position(pos);
        self.loaded.push(LoadedChunk {
            pos,
            result: ChunkLoadResult::Loaded { chunk },
        });
    }

    fn poll_loaded_chunk(&mut self) -> Option<super::LoadedChunk> {
        self.loaded.pop()
    }
}
