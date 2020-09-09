//! Read and write from a world save on disk.
//!
//! # Architecture
//! When a chunk needs to be loaded or saved, the server sends a request
//! over a channel to the `ChunkWorker` which runs on a separate
//! thread. The chunk worker then attempts
//! to load that chunk from disk. If the chunk loads successfully, then
//! it returns the chunk to the server. Otherwise, it generates the chunk
//! using `feather-worldgen`, then proceeds to send the chunk to the server.

use ahash::AHashMap;
use base::{
    anvil::block_entity::BlockEntityData,
    anvil::entity::EntityData,
    anvil::region,
    anvil::region::{RegionHandle, RegionPosition},
    Chunk, ChunkPosition,
};
use ecs::EntityBuilder;
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::{path::Path, path::PathBuf, sync::Arc, time::SystemTime, time::UNIX_EPOCH};
use utils::ComputePool;
use worldgen::WorldGenerator;

/// Stores a chunk and associated data (entities, block entities, tile ticks, ...)
/// for saving to the world save.
#[derive(Clone)]
pub struct ChunkSave {
    pub chunk: Arc<RwLock<Chunk>>,
    /// Entities within this chunk.
    pub entities: SmallVec<[EntityData; 4]>,
    /// Block entities within this chunk.
    pub block_entities: SmallVec<[BlockEntityData; 4]>,
}

/// Stores a chunk and associated data loaded
/// from the world save.
pub struct ChunkLoad {
    pub chunk: Chunk,
    /// Entities within this chunk, pre-built with their components.
    /// Includes block entities as well.
    pub entities: SmallVec<[EntityBuilder; 4]>,
}

#[allow(clippy::large_enum_variant)]
pub enum Reply {
    LoadedChunk(ChunkPosition, anyhow::Result<ChunkLoad>),
    SavedChunk(ChunkPosition, anyhow::Result<()>),
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Request {
    LoadChunk(ChunkPosition),
    SaveChunk(ChunkSave),
    ShutDown,
}

/// An open region file
struct RegionFile {
    /// The handle for the file
    handle: RegionHandle,
    /// The timestamp of the last time
    /// this region file was used. This
    /// value is used to close
    /// the file after it isn't used for
    /// some period of time.
    ///
    /// TODO actually implement this
    _last_used: u64,
}

struct ChunkWorker {
    /// The directory in which the world
    /// resides
    dir: PathBuf,

    /// Channel used to send chunks and errors
    /// back to the server thread
    sender: Sender<Reply>,
    /// Channel used to receive chunk load requests
    /// from the server thread
    receiver: Receiver<Request>,

    /// A map of currently open region files
    open_regions: AHashMap<RegionPosition, RegionFile>,

    /// World generator for new chunks.
    world_generator: Arc<dyn WorldGenerator>,

    /// Thread pool for world generation.
    compute_pool: ComputePool,
    /*/// State for loading entities.
    entity_loader: EntityLoader,*/
}

/// Starts a chunk worker on a new thread.
/// The returned channels can be used
/// to communicate with the worker.
pub fn start(
    world_dir: &Path,
    world_gen: Arc<dyn WorldGenerator>,
    compute_pool: &ComputePool,
) -> (Sender<Request>, Receiver<Reply>) {
    let (request_tx, request_rx) = flume::unbounded();
    let (reply_tx, reply_rx) = flume::unbounded();

    let worker = ChunkWorker {
        dir: world_dir.to_path_buf(),
        sender: reply_tx,
        receiver: request_rx,
        open_regions: AHashMap::new(),
        world_generator: world_gen,
        compute_pool: compute_pool.clone(),
        // entity_loader: EntityLoader::new(),
    };

    log::info!("Starting chunk worker");

    std::thread::Builder::new()
        .name("Chunk Worker Thread".to_owned())
        .spawn(move || run(worker))
        .expect("unable to start chunk worker thread");

    (request_tx, reply_rx)
}

/// Runs the chunk worker on the current thread,
/// blocking indefinitely.
fn run(mut worker: ChunkWorker) {
    while let Ok(request) = worker.receiver.recv() {
        match request {
            Request::ShutDown => break,
            Request::SaveChunk(save) => {
                save_chunk(&mut worker, save);
            }
            Request::LoadChunk(pos) => {
                if let Some(reply) = load_chunk(&mut worker, pos) {
                    let _ = worker.sender.send(reply);
                }
            }
        }
    }

    log::info!("Chunk worker terminating");
}

/// Attempts to load the chunk at the specified position.
fn load_chunk(worker: &mut ChunkWorker, pos: ChunkPosition) -> Option<Reply> {
    let rpos = RegionPosition::from_chunk(pos);

    let file = worker_region(&mut worker.open_regions, &worker.dir, rpos);
    // Load from region file
    load_chunk_from_handle(
        pos,
        &mut file.handle,
        &Arc::from(worker.sender.clone()),
        &worker.world_generator,
        &worker.compute_pool,
        //&worker.entity_loader,
    )
}

fn load_chunk_from_handle(
    pos: ChunkPosition,
    handle: &mut RegionHandle,
    sender: &Arc<Sender<Reply>>,
    generator: &Arc<dyn WorldGenerator>,
    compute_pool: &ComputePool,
    //entity_loader: &EntityLoader,
) -> Option<Reply> {
    let result = handle.load_chunk(pos);

    match result {
        Ok((chunk, _entities, _block_entities)) => {
            /*let entities = entities
            .into_iter()
            .filter_map(|entity| entity_loader.load(entity))
            .chain(
                block_entities
                    .into_iter()
                    .filter_map(|block_entity| entity_loader.load_block(block_entity)),
            )
            .collect::<Result<SmallVec<_>, anyhow::Error>>();*/

            Some(Reply::LoadedChunk(
                pos,
                /*match entities {
                    Ok(entities) => Ok(ChunkLoad { chunk, entities }),
                    Err(e) => Err(e),
                },*/
                Ok(ChunkLoad {
                    chunk,
                    entities: Default::default(),
                }),
            ))
        }
        Err(e) => match e {
            region::Error::ChunkNotExist => {
                schedule_generate_new_chunk(sender, pos, generator, compute_pool);
                None
            }
            err => Some(Reply::LoadedChunk(pos, Err(err.into()))),
        },
    }
}

/// Generates a new chunk asynchronously,
/// sending the result to the provided Sender.
fn schedule_generate_new_chunk(
    sender: &Arc<Sender<Reply>>,
    pos: ChunkPosition,
    generator: &Arc<dyn WorldGenerator>,
    compute_pool: &ComputePool,
) {
    let sender = sender.clone();
    let generator = Arc::clone(generator);
    compute_pool
        .spawn(async move {
            let _ = sender.send(generate_new_chunk(pos, &generator));
        })
        .detach();
}

/// Generates a new chunk synchronously,
/// returning a Reply to send to a Sender.
fn generate_new_chunk(pos: ChunkPosition, generator: &Arc<dyn WorldGenerator>) -> Reply {
    Reply::LoadedChunk(
        pos,
        Ok(ChunkLoad {
            chunk: generator.generate_chunk(pos),
            entities: SmallVec::new(),
        }),
    )
}

/// Saves the chunk at the specified position.
fn save_chunk(worker: &mut ChunkWorker, save: ChunkSave) {
    let chunk = save.chunk.read();
    let rpos = RegionPosition::from_chunk(chunk.position());

    let file = worker_region(&mut worker.open_regions, &worker.dir, rpos);

    let result = file
        .handle
        .save_chunk(&*chunk, &save.entities, &save.block_entities);

    let _ = worker.sender.send(Reply::SavedChunk(
        chunk.position(),
        result.map_err(|err| err.into()),
    ));
}

/// Returns whether the given chunk's region
/// is already loaded.
fn is_region_loaded(
    open_regions: &AHashMap<RegionPosition, RegionFile>,
    rpos: RegionPosition,
) -> bool {
    open_regions.contains_key(&rpos)
}

fn worker_region<'a>(
    open_regions: &'a mut AHashMap<RegionPosition, RegionFile>,
    dir: &PathBuf,
    rpos: RegionPosition,
) -> &'a mut RegionFile {
    if !is_region_loaded(open_regions, rpos) {
        // Need to load region into memory
        let mut handle = region::load_region(&dir, rpos);
        if handle.is_err() {
            // Create a new region file
            handle = region::create_region(&dir, rpos);
        }

        let handle = handle.unwrap();

        let last_used = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let file = RegionFile {
            handle,
            _last_used: last_used,
        };

        open_regions.insert(rpos, file);
    }
    open_regions.get_mut(&rpos).unwrap()
}

/*
use std::{path::PathBuf, sync::Arc, thread};

use ahash::AHashMap;
use base::{
    anvil::region::load_region, anvil::region::RegionHandle, anvil::region::RegionPosition, Chunk,
    ChunkPosition,
};
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use utils::ComputePool;
use worldgen::WorldGenerator;

/// A handle to the chunk worker.
#[derive(Clone)]
pub struct ChunkWorkerHandle {
    sender: Sender<ToWorker>,
    receiver: Receiver<FromWorker>,
}

/// Message sent from the server to the chunk worker.
enum ToWorker {
    /// Request that a chunk be loaded.
    LoadChunk(ChunkPosition),
    /// Request that a chunk be saved.
    SaveChunk(Arc<RwLock<Chunk>>),
    /// Request that the worker shuts down
    /// after completing all remaining requests.
    ShutDown,
}

/// Message sent from the chunk worker to the server.
enum FromWorker {
    /// A chunk was loaded or generated successfully.
    ChunkLoaded(Chunk),
    /// A chunk was saved successfully.
    ChunkSave(ChunkPosition),
}

/// A separate thread which loads and saves chunks.
pub struct ChunkWorker {
    sender: Sender<FromWorker>,
    receiver: Receiver<ToWorker>,
    worldgen: Arc<dyn WorldGenerator>,
    compute_pool: ComputePool,
    world_dir: PathBuf,

    /// Region files that are currently open.
    regions: AHashMap<RegionPosition, RegionHandle>,
}

impl ChunkWorker {
    /// Spawns the chunk worker thread, returning a handle to it.
    ///
    /// You must provide a world generator with which to generate nonexistent
    /// chunks and a compute pool to run world generation on.
    pub fn spawn(
        worldgen: Arc<dyn WorldGenerator>,
        compute_pool: ComputePool,
        world_dir: PathBuf,
    ) -> ChunkWorkerHandle {
        let to_worker = flume::unbounded();
        let from_worker = flume::unbounded();

        let mut worker = ChunkWorker {
            sender: from_worker.0,
            receiver: to_worker.1,
            worldgen,
            world_dir,
            compute_pool,
            regions: AHashMap::new(),
        };
        thread::Builder::new()
            .name(String::from("Chunk Worker"))
            .spawn(move || worker.run())
            .expect("failed to spawn chunk worker thread");

        let handle = ChunkWorkerHandle {
            sender: to_worker.0,
            receiver: from_worker.1,
        };

        handle
    }

    fn run(&mut self) {
        log::debug!("Chunk worker started");
        loop {
            let message = match self.receiver.recv() {
                Ok(m) => m,
                Err(e) => {
                    log::warn!("Chunk worker lost channel: {:?}", e);
                    return;
                }
            };

            match message {
                ToWorker::LoadChunk(pos) => {
                    if let Err(e) = self.load_chunk(pos) {
                        log::error!("Failed to load chunk at {:?}: {}", pos, e)
                    }
                }
                ToWorker::SaveChunk(chunk) => {
                    if let Err(e) = self.save_chunk(&*chunk.read()) {
                        log::error!(
                            "Failed to save chunk at {:?}: {}",
                            chunk.read().position(),
                            e
                        );
                    }
                }
                ToWorker::ShutDown => {
                    log::info!("Chunk worker shutting down");
                    break;
                }
            }
        }
    }

    fn load_chunk(&mut self, pos: ChunkPosition) -> anyhow::Result<()> {
        let region = self.region(RegionPosition::from_chunk(pos));
    }

    fn region(&mut self, pos: RegionPosition) -> anyhow::Result<RegionHandle> {
        if let Some(handle) = self.regions.remove(pos) {
            Ok(handle)
        } else {
            load_region(&self.world_dir, pos).map_err(anyhow::Error::from)
        }
    }
}
*/
