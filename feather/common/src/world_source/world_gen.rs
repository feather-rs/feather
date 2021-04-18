use super::{ChunkLoadResult, LoadedChunk, WorldSource};
use base::ChunkPosition;
use feather_worldgen::{ComposableGenerator, WorldGenerator};
use flume::{Receiver, Sender};
use std::time::Duration;

// World source that uses the world generator crate
pub struct WorldGenWorldSource {
    request_sender: Sender<ChunkPosition>,
    result_receiver: Receiver<LoadedChunk>,
}

impl WorldGenWorldSource {
    pub fn new() -> Self {
        let gen = ComposableGenerator::default_with_seed(0);

        let (request_sender, request_receiver) = flume::unbounded();
        let (worker, result_receiver) = Worker::new(request_receiver, gen);

        worker.start();

        Self {
            request_sender,
            result_receiver,
        }
    }
}

impl WorldSource for WorldGenWorldSource {
    fn queue_load(&mut self, pos: ChunkPosition) {
        self.request_sender
            .send(pos)
            .expect("chunk worker panicked");
    }

    fn poll_loaded_chunk(&mut self) -> Option<super::LoadedChunk> {
        self.result_receiver.try_recv().ok()
    }
}

impl Default for WorldGenWorldSource {
    fn default() -> Self {
        Self::new()
    }
}

struct Worker {
    request_receiver: Receiver<ChunkPosition>,
    result_sender: Sender<LoadedChunk>,
    world_gen: ComposableGenerator,
}

impl Worker {
    pub fn new(
        request_receiver: Receiver<ChunkPosition>,
        world_gen: ComposableGenerator,
    ) -> (Self, Receiver<LoadedChunk>) {
        let (result_sender, result_receiver) = flume::bounded(256);
        (
            Self {
                request_receiver,
                result_sender,
                world_gen,
            },
            result_receiver,
        )
    }

    pub fn start(self) {
        std::thread::Builder::new()
            .name("chunk_worker".to_owned())
            .spawn(move || self.run())
            .expect("failed to create chunk worker thread");
    }

    fn run(mut self) {
        log::info!("Chunk worker started");
        loop {
            match self.request_receiver.recv_timeout(Duration::from_secs(30)) {
                Ok(pos) => self.load_chunk(pos),
                Err(flume::RecvTimeoutError::Timeout) => (),
                Err(flume::RecvTimeoutError::Disconnected) => {
                    log::info!("Chunk worker shutting down");
                    return;
                }
            }
        }
    }

    fn load_chunk(&mut self, pos: ChunkPosition) {
        let chunk = self.world_gen.generate_chunk(pos);
        let result = ChunkLoadResult::Loaded { chunk };
        let _ = self.result_sender.send(LoadedChunk { pos, result });
    }
}
