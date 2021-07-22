use std::sync::Arc;

use base::ChunkPosition;
use flume::{unbounded, Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};
use worldgen::{ComposableGenerator, WorldGenerator};

use super::{ChunkLoadResult, LoadedChunk, WorldSource};

pub struct GeneratingWorldSource {
    _thread_pool: ThreadPool,
    send: Sender<ChunkPosition>,
    recv: Receiver<LoadedChunk>,
}

impl Default for GeneratingWorldSource {
    fn default() -> Self {
        Self::new(Arc::new(ComposableGenerator::default_with_seed(42)))
    }
}

impl GeneratingWorldSource {
    pub fn new<G>(generator: Arc<G>) -> Self
    where
        G: WorldGenerator + 'static,
    {
        let (send_pos, recv_pos) = unbounded::<ChunkPosition>();
        let (send_gen, recv_gen) = unbounded::<LoadedChunk>();
        let thread_pool = ThreadPoolBuilder::default()
            .thread_name(|i| format!("World gen #{}", i))
            .build()
            .unwrap();
        for _ in 0..thread_pool.current_num_threads() {
            let send = send_gen.clone();
            let recv = recv_pos.clone();
            let gen = generator.clone();
            thread_pool.spawn(move || {
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
            _thread_pool: thread_pool,
            send: send_pos,
            recv: recv_gen,
        }
    }

    pub fn default_with_seed(seed: u64) -> Self {
        Self::new(Arc::new(ComposableGenerator::default_with_seed(seed)))
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
