//! Integrates world generation into the `WorldSource` design.

use std::sync::Arc;

use flume::{Receiver, Sender};
use libcraft::{Chunk, ChunkPosition, Sections};

use crate::{threadpool::ThreadPool, ChunkHandle, ChunkLock, Game, WorldId};

use super::{ChunkLoadError, ChunkLoadResult, ChunkSaveResult, StoredChunk, WorldSource};

/// A world generator.
///
/// Note that world generation happens in parallel, so generators
/// are required to be `Send + Sync`.
pub trait WorldGenerator: Send + Sync + 'static {
    /// Generates the chunk at the given position.
    fn generate_chunk(&self, pos: ChunkPosition) -> Chunk;
}

/// A factory for a world generator.
///
/// Used to initialize a `WorldGenerator` from a config file.
pub trait WorldGeneratorFactory: 'static {
    fn create_world_generator(
        &self,
        game: &dyn Game,
        params: &toml::Value,
        world_id: WorldId,
        sections: Sections,
        min_y: i32,
    ) -> anyhow::Result<Arc<dyn WorldGenerator>>;
}

/// A `WorldSource` that wraps an inner world source
/// and generates a new chunk when the inner source
/// is missing a chunk.
///
/// World generation is performed on a separate thread
/// (in parallel on the server's compute thread pool.)
pub struct WorldGeneratorWorldSource {
    inner: Box<dyn WorldSource>,
    generator: Arc<dyn WorldGenerator>,
    generated_chunks_sender: Sender<StoredChunk>,
    generated_chunks_receiver: Receiver<StoredChunk>,
    pool: ThreadPool,
}

impl WorldGeneratorWorldSource {
    pub fn new(
        inner: Box<dyn WorldSource>,
        generator: Arc<dyn WorldGenerator>,
        compute_pool: &ThreadPool,
    ) -> Self {
        let (generated_chunks_sender, generated_chunks_receiver) = flume::unbounded();
        Self {
            inner,
            generator,
            generated_chunks_sender,
            generated_chunks_receiver,
            pool: compute_pool.clone(),
        }
    }

    fn queue_generate_chunk(&mut self, pos: ChunkPosition) {
        let generator = Arc::clone(&self.generator);
        let sender = self.generated_chunks_sender.clone();
        self.pool.spawn(move || {
            let chunk = generator.generate_chunk(pos);
            sender
                .send(StoredChunk {
                    pos,
                    chunk: ChunkHandle::new(ChunkLock::new(chunk)),
                })
                .ok();
        });
    }

    fn poll_generated_chunk(&mut self) -> Option<ChunkLoadResult> {
        self.generated_chunks_receiver
            .try_recv()
            .ok()
            .map(|chunk| ChunkLoadResult {
                pos: chunk.pos,
                result: Ok(chunk),
            })
    }
}

impl WorldSource for WorldGeneratorWorldSource {
    fn supports_saving(&self) -> bool {
        self.inner.supports_saving()
    }

    fn queue_load_chunk(&mut self, pos: ChunkPosition) {
        self.inner.queue_load_chunk(pos)
    }

    fn poll_loaded_chunk(&mut self) -> Option<ChunkLoadResult> {
        self.poll_generated_chunk().or_else(|| {
            // Greedily consume all missing chunks.
            loop {
                let result = self.inner.poll_loaded_chunk()?;

                match &result.result {
                    Err(ChunkLoadError::Missing) => {
                        self.queue_generate_chunk(result.pos);
                    }
                    Ok(_) | Err(_) => break Some(result),
                }
            }
        })
    }

    fn queue_save_chunk(&mut self, chunk: StoredChunk) {
        self.inner.queue_save_chunk(chunk)
    }

    fn poll_saved_chunk(&mut self) -> Option<ChunkSaveResult> {
        self.inner.poll_saved_chunk()
    }
}
