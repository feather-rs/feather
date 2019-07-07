//! This module handles the asynchronous loading of chunks from
//! region files. It runs on a separate thread and receives
//! load requests over a channel. Upon receiving a request,
//! it attempts to load the chunk from the world directory.
//! If the chunk was loaded successfully, it will send
//! the chunk back over a channel. If the chunk did not exist,
//! it will notify the server thread of the incident.
//! In response, the server thread should generate the chunk.
use crossbeam::channel::{Receiver, Sender};
use feather_core::region;
use feather_core::region::{RegionHandle, RegionPosition};
use feather_core::world::chunk::Chunk;
use feather_core::world::ChunkPosition;
use hashbrown::HashMap;
use serde_json::error::Category::Syntax;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Reply = (ChunkPosition, Result<Chunk, Error>);

#[derive(Debug, Clone, Copy)]
pub enum Request {
    LoadChunk(ChunkPosition),
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
    last_used: u64,
}

struct ChunkWorker {
    /// The directory in which the world
    /// resides
    dir: String,

    /// Channel used to send chunks and errors
    /// back to the server thread
    sender: Sender<Reply>,
    /// Channel used to receive chunk load requests
    /// from the server thread
    receiver: Receiver<Request>,

    /// A map of currently open region files
    open_regions: HashMap<RegionPosition, RegionFile>,
}

/// Starts a chunk worker on a new thread.
/// The returned channels can be used
/// to communicate with the worker.
pub fn start(world_dir: &str) -> (Sender<Request>, Receiver<Reply>) {
    let (request_tx, request_rx) = crossbeam::channel::unbounded();
    let (reply_tx, reply_rx) = crossbeam::channel::unbounded();

    let worker = ChunkWorker {
        dir: world_dir.to_string(),
        sender: reply_tx,
        receiver: request_rx,
        open_regions: HashMap::new(),
    };

    std::thread::spawn(move || {
        run(worker);
    });

    (request_tx, reply_rx)
}

/// Runs the chunk worker on the current thread,
/// blocking indefinitely.
fn run(mut worker: ChunkWorker) {
    while let Ok(request) = worker.receiver.recv() {
        match request {
            Request::ShutDown => break,
            Request::LoadChunk(pos) => {
                let reply = load_chunk(&mut worker, pos);
                worker.sender.send(reply).unwrap();
            }
        }
    }

    info!("Chunk worker terminating");
}

/// Attempts to load the chunk at the specified position.
fn load_chunk(worker: &mut ChunkWorker, pos: ChunkPosition) -> Reply {
    let rpos = RegionPosition::from_chunk(pos);
    if !is_region_loaded(worker, pos) {
        // Need to load region into memory
        let handle = region::load_region(&worker.dir, rpos);
        if handle.is_err() {
            return (pos, Err(Error::LoadError(handle.err().unwrap())));
        }

        let handle = handle.unwrap();

        let last_used = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let file = RegionFile { handle, last_used };

        worker.open_regions.insert(rpos, file);
    }

    // Load from region file
    let file = worker.open_regions.get_mut(&rpos).unwrap();
    let handle = &mut file.handle;

    load_chunk_from_handle(pos, handle)
}

fn load_chunk_from_handle(pos: ChunkPosition, handle: &mut RegionHandle) -> Reply {
    let result = handle.load_chunk(pos);

    match result {
        Ok(chunk) => (pos, Ok(chunk)),
        Err(e) => match e {
            region::Error::ChunkNotExist => (pos, Err(Error::ChunkNotExist)),
            err => (pos, Err(Error::LoadError(err))),
        },
    }
}

/// Returns whether the given chunk's region
/// is already loaded.
fn is_region_loaded(worker: &ChunkWorker, chunk_pos: ChunkPosition) -> bool {
    worker
        .open_regions
        .contains_key(&RegionPosition::from_chunk(chunk_pos))
}
