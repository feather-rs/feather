//! Plugin providing a `WorldSource` that uses the Feather world format.

use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc, time::Duration};

use anyhow::Context;
use flume::{Receiver, Sender};
use quill::{
    libcraft::{biome::BiomeList, dimension::DimensionInfo, RegionPosition, WorldHeight},
    saveload::{
        ChunkLoadError, ChunkLoadResult, ChunkSaveError, ChunkSaveResult, ChunkSaved, StoredChunk,
        WorldSource, WorldSourceFactory,
    },
    ChunkLock, ChunkPosition, Game, Plugin, PluginInfo, Setup, WorldId,
};
use threadpool::{RegionKey, RegionThreadPool, Shared, WorldInfo};

mod threadpool;

pub struct FeatherWorldFormat;

impl Plugin for FeatherWorldFormat {
    type State = ();

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "FregWorldFormat",
            id: "freg_world_format",
        }
    }

    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> anyhow::Result<Self::State> {
        let thread_pool = RegionThreadPool::new(
            Shared::new(setup.game().resources().get::<Arc<BiomeList>>()?.clone()),
            num_cpus::get(),
        );
        setup
            .game_mut()
            .resources_mut()
            .insert(Rc::new(RefCell::new(thread_pool)));

        setup
            .game_mut()
            .register_world_source_factory("freg", Box::new(FregWorldSourceFactory));

        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
struct Params {
    directory: String,
}

pub struct FregWorldSourceFactory;

impl WorldSourceFactory for FregWorldSourceFactory {
    fn create_world_source(
        &self,
        game: &dyn Game,
        params: &toml::Value,
        dimension_info: &DimensionInfo,
        world_id: WorldId,
    ) -> anyhow::Result<Box<dyn WorldSource>> {
        let params: Params = params.clone().try_into()?;

        Ok(Box::new(FregWorldSource::new(
            params.directory.into(),
            game,
            world_id,
            dimension_info,
        )))
    }
}

const RETRY_INTERVAL: Duration = Duration::from_secs(10);

/// A `WorldSource` that loads and saves chunks from the Feather world format,
/// dubbed `freg`.
///
/// The world source uses a shared thread pool to parallelize chunk loading and saving.
/// Regions are distributed across threads in the pool.

struct FregWorldSource {
    world: WorldId,
    thread_pool: Rc<RefCell<RegionThreadPool>>,

    loaded_receiver: Receiver<ChunkLoadResult>,
    loaded_sender: Sender<ChunkLoadResult>,

    saved_receiver: Receiver<ChunkSaveResult>,
    saved_sender: Sender<ChunkSaveResult>,
}

impl FregWorldSource {
    pub fn new(dir: PathBuf, game: &dyn Game, world: WorldId, info: &DimensionInfo) -> Self {
        let sections = WorldHeight(info.info.height as usize).into();
        let min_y = info.info.min_y;

        let thread_pool = Rc::clone(
            &game
                .resources()
                .get::<Rc<RefCell<RegionThreadPool>>>()
                .unwrap(),
        );
        thread_pool.borrow_mut().register_world(
            world,
            WorldInfo {
                sections,
                min_y,
                dir,
            },
        );

        let (loaded_sender, loaded_receiver) = flume::unbounded();
        let (saved_sender, saved_receiver) = flume::unbounded();
        Self {
            world,
            thread_pool,
            loaded_receiver,
            loaded_sender,
            saved_receiver,
            saved_sender,
        }
    }
}

impl WorldSource for FregWorldSource {
    fn supports_saving(&self) -> bool {
        true
    }

    fn queue_load_chunk(&mut self, pos: ChunkPosition) {
        let sender = self.loaded_sender.clone();
        let region = RegionPosition::from_chunk(pos);
        let key = RegionKey {
            region,
            world: self.world,
        };

        let (chunk_x, chunk_z) = region.chunk_offset(pos).unwrap();

        self.thread_pool.borrow_mut().spawn(key, move |region| {
            let chunk = region
                .context("failed to access region file")
                .and_then(|region| region.load(chunk_x, chunk_z));

            let result = match chunk {
                Ok(Some(chunk)) => Ok(StoredChunk {
                    pos,
                    chunk: Arc::new(ChunkLock::new(chunk)),
                }),
                Ok(None) => Err(ChunkLoadError::Missing),
                Err(e) => Err(ChunkLoadError::Other(e)),
            };

            let result = ChunkLoadResult { pos, result };
            sender.send(result).ok();
        });
    }

    fn poll_loaded_chunk(&mut self) -> Option<ChunkLoadResult> {
        self.loaded_receiver.try_recv().ok()
    }

    fn queue_save_chunk(&mut self, chunk: StoredChunk) {
        let pos = chunk.chunk.read().position();
        let sender = self.saved_sender.clone();
        let region = RegionPosition::from_chunk(pos);
        let key = RegionKey {
            region,
            world: self.world,
        };

        self.thread_pool.borrow_mut().spawn(key, move |region| {
            let result = region
                .and_then(|region| region.save(&chunk.chunk.read()))
                .map(|_| ChunkSaved)
                .map_err(|e| ChunkSaveError {
                    error: e,
                    retry_in: RETRY_INTERVAL,
                });
            sender.send(ChunkSaveResult { pos, result }).ok();
        });
    }

    fn poll_saved_chunk(&mut self) -> Option<ChunkSaveResult> {
        self.saved_receiver.try_recv().ok()
    }
}
