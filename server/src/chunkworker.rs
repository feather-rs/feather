//! This module handles the asynchronous loading and saving
//! of chunks. It receives load and save requests from the server
//! (over a channel) and executes them.
//!
//! If a chunk cannot be loaded, it is generated on the Rayon thread pool
//! instead.
use crate::worldgen::WorldGenerator;
use crossbeam::channel::{Receiver, Sender};
use feather_core::entity::EntityData;
use feather_core::region;
use feather_core::region::{RegionHandle, RegionPosition};
use feather_core::world::chunk::Chunk;
use feather_core::world::ChunkPosition;
use hashbrown::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(clippy::large_enum_variant)]
pub enum Reply {
    LoadedChunk(ChunkPosition, Result<(Chunk, Vec<EntityData>), Error>),
    SavedChunk(ChunkPosition),
}

#[derive(Clone)]
pub enum Request {
    LoadChunk(ChunkPosition),
    SaveChunk(Arc<Chunk>, Vec<EntityData>),
    ShutDown,
}

#[derive(Debug)]
pub enum Error {
    ChunkNotExist,
    LoadError(region::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::ChunkNotExist => {
                f.write_str("The specified chunk does not exist in the world save")?
            }
            Error::LoadError(e) => {
                f.write_str("Error loading chunk: ")?;
                e.fmt(f)?;
            }
        }

        Ok(())
    }
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
    open_regions: HashMap<RegionPosition, RegionFile>,

    /// World generator for new chunks.
    world_generator: Arc<dyn WorldGenerator>,
}

/// Starts a chunk worker on a new thread.
/// The returned channels can be used
/// to communicate with the worker.
pub fn start(
    world_dir: &Path,
    world_gen: Arc<dyn WorldGenerator>,
) -> (Sender<Request>, Receiver<Reply>) {
    let (request_tx, request_rx) = crossbeam::channel::unbounded();
    let (reply_tx, reply_rx) = crossbeam::channel::unbounded();

    let worker = ChunkWorker {
        dir: world_dir.to_path_buf(),
        sender: reply_tx,
        receiver: request_rx,
        open_regions: HashMap::new(),
        world_generator: world_gen,
    };

    // Without changing the stack size,
    // a stack overflow occurs here.
    // This seeks to solve that.
    std::thread::Builder::new()
        .stack_size(1024 * 1024 * 5)
        .name("Chunk Worker Thread".to_string())
        .spawn(move || run(worker))
        .expect("Unable to start chunk worker thread");

    (request_tx, reply_rx)
}

/// Runs the chunk worker on the current thread,
/// blocking indefinitely.
fn run(mut worker: ChunkWorker) {
    while let Ok(request) = worker.receiver.recv() {
        match request {
            Request::ShutDown => break,
            Request::SaveChunk(chunk, entities) => {
                save_chunk(&mut worker, &chunk, entities);
            }
            Request::LoadChunk(pos) => {
                if let Some(reply) = load_chunk(&mut worker, pos) {
                    worker.sender.send(reply).unwrap();
                }
            }
        }
    }

    info!("Chunk worker terminating");
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
    )
}

fn load_chunk_from_handle(
    pos: ChunkPosition,
    handle: &mut RegionHandle,
    sender: &Arc<Sender<Reply>>,
    generator: &Arc<dyn WorldGenerator>,
) -> Option<Reply> {
    let result = handle.load_chunk(pos);

    match result {
        Ok(chunk) => Some(Reply::LoadedChunk(pos, Ok(chunk))),
        Err(e) => match e {
            region::Error::ChunkNotExist => {
                schedule_generate_new_chunk(sender, pos, generator);
                None
            }
            err => Some(Reply::LoadedChunk(pos, Err(Error::LoadError(err)))),
        },
    }
}

/// Generates a new chunk asynchronously,
/// sending the result to the provided Sender.
fn schedule_generate_new_chunk(
    sender: &Arc<Sender<Reply>>,
    pos: ChunkPosition,
    generator: &Arc<dyn WorldGenerator>,
) {
    let sender = sender.clone();
    let generator = Arc::clone(generator);
    rayon::spawn(move || {
        sender.send(generate_new_chunk(pos, &generator)).unwrap();
    });
}

/// Generates a new chunk synchronously,
/// returning a Reply to send to a Sender.
fn generate_new_chunk(pos: ChunkPosition, generator: &Arc<dyn WorldGenerator>) -> Reply {
    Reply::LoadedChunk(pos, Ok((generator.generate_chunk(pos), vec![])))
}

/// Saves the chunk at the specified position.
fn save_chunk(worker: &mut ChunkWorker, chunk: &Chunk, entities: Vec<EntityData>) {
    let rpos = RegionPosition::from_chunk(chunk.position());

    let file = worker_region(&mut worker.open_regions, &worker.dir, rpos);

    file.handle.save_chunk(chunk, entities).unwrap();
    worker
        .sender
        .send(Reply::SavedChunk(chunk.position()))
        .unwrap();
}

/// Returns whether the given chunk's region
/// is already loaded.
fn is_region_loaded(
    open_regions: &HashMap<RegionPosition, RegionFile>,
    rpos: RegionPosition,
) -> bool {
    open_regions.contains_key(&rpos)
}

fn worker_region<'a>(
    open_regions: &'a mut HashMap<RegionPosition, RegionFile>,
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
