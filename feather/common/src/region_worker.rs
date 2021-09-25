use std::{
    collections::hash_map::Entry,
    path::PathBuf,
    time::{Duration, Instant},
};

use ahash::AHashMap;
use base::anvil::{
    self,
    region::{RegionHandle, RegionPosition},
};
use flume::{Receiver, Sender};

use crate::chunk::worker::{ChunkLoadResult, LoadRequest, LoadedChunk, SaveRequest, WorkerRequest};

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

pub struct RegionWorker {
    request_receiver: Receiver<WorkerRequest>,
    result_sender: Sender<ChunkLoadResult>,
    world_dir: PathBuf,
    region_files: AHashMap<RegionPosition, OpenRegionFile>,
    last_cache_update: Instant,
}

impl RegionWorker {
    pub fn new(
        world_dir: PathBuf,
        request_receiver: Receiver<WorkerRequest>,
    ) -> (Self, Receiver<ChunkLoadResult>) {
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
            match self.request_receiver.recv_timeout(CACHE_TIME) {
                Ok(req) => match req {
                    WorkerRequest::Load(load) => self.load_chunk(load),
                    WorkerRequest::Save(save) => self.save_chunk(save).unwrap(),
                },
                Err(flume::RecvTimeoutError::Timeout) => (),
                Err(flume::RecvTimeoutError::Disconnected) => {
                    log::info!("Chunk worker shutting down");
                    return;
                }
            }
            self.update_cache();
        }
    }

    fn save_chunk(&mut self, req: SaveRequest) -> anyhow::Result<()> {
        let reg_pos = RegionPosition::from_chunk(req.pos);
        let handle = &mut match self.region_file_handle(reg_pos) {
            Some(h) => h,
            None => {
                let new_handle = anvil::region::create_region(&self.world_dir, reg_pos)?;
                self.region_files
                    .insert(reg_pos, OpenRegionFile::new(new_handle));
                self.region_file_handle(reg_pos).unwrap()
            }
        }
        .handle;
        handle.save_chunk(
            &req.chunk.read(),
            &req.entities[..],
            &req.block_entities[..],
        )?;
        Ok(())
    }

    fn load_chunk(&mut self, req: LoadRequest) {
        let result = self.get_chunk_load_result(req);
        let _ = self.result_sender.send(result);
    }

    fn get_chunk_load_result(&mut self, req: LoadRequest) -> ChunkLoadResult {
        let pos = req.pos;
        let region = RegionPosition::from_chunk(pos);
        let file = match self.region_file_handle(region) {
            Some(file) => file,
            None => return ChunkLoadResult::Missing(pos),
        };

        let chunk = match file.handle.load_chunk(pos) {
            Ok((chunk, _, _)) => chunk,
            Err(e) => match e {
                anvil::region::Error::ChunkNotExist => return ChunkLoadResult::Missing(pos),
                err => return ChunkLoadResult::Error(err.into()),
            },
        };

        file.last_used = Instant::now();

        ChunkLoadResult::Loaded(LoadedChunk { pos, chunk })
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
