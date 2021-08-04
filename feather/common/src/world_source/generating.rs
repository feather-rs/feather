use std::sync::Arc;

use base::ChunkPosition;
use flume::{unbounded, Receiver, Sender};
use worldgen::{ComposableGenerator, WorldGenerator};

use super::{ChunkLoadResult, LoadedChunk, WorldSource};

pub struct GeneratingWorldSource {
    send: Sender<LoadedChunk>,
    recv: Receiver<LoadedChunk>,
    generator: Arc<dyn WorldGenerator>,
}

impl Default for GeneratingWorldSource {
    fn default() -> Self {
        Self::new(ComposableGenerator::default_with_seed(42))
    }
}

impl GeneratingWorldSource {
    pub fn from_arc<G>(generator: Arc<G>) -> Self
    where
        G: WorldGenerator + 'static,
    {
        let (send_gen, recv_gen) = unbounded::<LoadedChunk>();
        Self {
            send: send_gen,
            recv: recv_gen,
            generator,
        }
    }
    pub fn new<G>(generator: G) -> Self
    where
        G: WorldGenerator + 'static,
    {
        Self::from_arc(Arc::new(generator))
    }
    pub fn default_with_seed(seed: u64) -> Self {
        Self::new(ComposableGenerator::default_with_seed(seed))
    }
}

impl WorldSource for GeneratingWorldSource {
    fn queue_load(&mut self, pos: ChunkPosition) {
        let send = self.send.clone();
        let gen = self.generator.clone();
        rayon::spawn(move || {
            let chunk = gen.generate_chunk(pos);
            let loaded = LoadedChunk {
                pos,
                result: ChunkLoadResult::Loaded { chunk },
            };
            send.send(loaded).unwrap();
        });
    }

    fn poll_loaded_chunk(&mut self) -> Option<LoadedChunk> {
        self.recv.try_recv().ok()
    }
}
