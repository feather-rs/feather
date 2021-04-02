use std::{
    collections::hash_map::Entry,
    path::PathBuf,
    time::{Duration, Instant},
};

use ahash::AHashMap;
use base::{
    anvil::region::{RegionHandle, RegionPosition},
    ChunkPosition,
};
use flume::{Receiver, Sender};

use super::{ChunkLoadResult, LevelSource, LoadedChunk};

/// World source loading from a vanilla (Anvil) world.
pub struct RegionLevelSource {
    request_sender: Sender<ChunkPosition>,
    result_receiver: Receiver<LoadedChunk>,
}

impl RegionLevelSource {
    pub fn new(world_dir: impl Into<PathBuf>) -> Self {
        let (request_sender, request_receiver) = flume::unbounded();
        let (worker, result_receiver) = Worker::new(world_dir.into(), request_receiver);

        worker.start();

        Self {
            request_sender,
            result_receiver,
        }
    }
}

impl LevelSource for RegionLevelSource {
    fn queue_load(&mut self, pos: ChunkPosition) {
        self.request_sender
            .send(pos)
            .expect("chunk worker panicked");
    }

    fn poll_loaded_chunk(&mut self) -> Option<super::LoadedChunk> {
        self.result_receiver.try_recv().ok()
    }
}

/// Duration to keep a region file open when not in use.
const CACHE_TIME: Duration = Duration::from_secs(60);

struct OpenRegionFile {
    handle: RegionHandle,
    last_used: Instant,
}

impl OpenRegionFile {
    pub fn new(handle: RegionHandle) -> Self {
        Self {
            handle,
            last_used: Instant::now(),
        }
    }

    pub fn should_close(&self) -> bool {
        self.last_used.elapsed() >= CACHE_TIME
    }
}

struct Worker {
    request_receiver: Receiver<ChunkPosition>,
    result_sender: Sender<LoadedChunk>,
    world_dir: PathBuf,
    region_files: AHashMap<RegionPosition, OpenRegionFile>,
    last_cache_update: Instant,
}

impl Worker {
    pub fn new(
        world_dir: PathBuf,
        request_receiver: Receiver<ChunkPosition>,
    ) -> (Self, Receiver<LoadedChunk>) {
        let (result_sender, result_receiver) = flume::bounded(256);
        (
            Self {
                request_receiver,
                result_sender,
                world_dir,
                region_files: AHashMap::new(),
                last_cache_update: Instant::now(),
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
            self.update_cache();
        }
    }

    fn load_chunk(&mut self, pos: ChunkPosition) {
        let result = self.get_chunk_load_result(pos);
        let _ = self.result_sender.send(LoadedChunk { pos, result });
    }

    fn get_chunk_load_result(&mut self, pos: ChunkPosition) -> ChunkLoadResult {
        let region = RegionPosition::from_chunk(pos);
        let file = match self.region_file_handle(region) {
            Some(file) => file,
            None => return ChunkLoadResult::Missing,
        };

        let chunk = match file.handle.load_chunk(pos) {
            Ok((chunk, _, _)) => chunk,
            Err(e) => return ChunkLoadResult::Error(e.into()),
        };

        file.last_used = Instant::now();

        ChunkLoadResult::Loaded { chunk }
    }

    fn region_file_handle(&mut self, region: RegionPosition) -> Option<&mut OpenRegionFile> {
        match self.region_files.entry(region) {
            Entry::Occupied(e) => Some(e.into_mut()),
            Entry::Vacant(e) => {
                let handle = base::anvil::region::load_region(&self.world_dir, region);
                if let Ok(handle) = handle {
                    Some(e.insert(OpenRegionFile::new(handle)))
                } else {
                    None
                }
            }
        }
    }

    fn update_cache(&mut self) {
        if self.last_cache_update.elapsed() >= CACHE_TIME {
            let initial_len = self.region_files.len();

            self.region_files.retain(|_, file| !file.should_close());
            self.last_cache_update = Instant::now();

            let num_closed = initial_len - self.region_files.len();
            if num_closed != 0 {
                log::debug!(
                    "Closed {} region files ({} still open)",
                    num_closed,
                    self.region_files.len()
                );
            }
        }
    }
}
