use std::sync::Arc;

use base::ChunkPosition;
use flume::{unbounded, Receiver, Sender};
use worldgen::{ComposableGenerator, WorldGenerator};

use super::{ChunkLoadResult, LoadedChunk, WorldSource};

pub struct GeneratingWorldSource {
    send: Sender<ChunkPosition>,
    recv: Receiver<LoadedChunk>,
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
        let (send_pos, recv_pos) = unbounded::<ChunkPosition>();
        let (send_gen, recv_gen) = unbounded::<LoadedChunk>();
        for _ in 0..8 { // FIXME: do not hardcode '8' as a magic number
            let send = send_gen.clone();
            let recv = recv_pos.clone();
            let gen = generator.clone();
            rayon::spawn(move || {
                for pos in recv.iter() {
                    let chunk = gen.generate_chunk(pos);
                    let loaded = LoadedChunk {
                        pos,
                        result: ChunkLoadResult::Loaded { chunk },
                    };
                    send.send(loaded).unwrap();
                }
            });
        }
        Self {
            send: send_pos,
            recv: recv_gen,
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
        self.send.send(pos).unwrap()
    }

    fn poll_loaded_chunk(&mut self) -> Option<LoadedChunk> {
        self.recv.try_recv().ok()
    }
}
