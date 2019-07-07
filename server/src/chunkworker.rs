//! This module handles the asynchronous loading of chunks from
//! region files. It runs on a separate thread and receives
//! load requests over a channel. Upon receiving a request,
//! it attempts to load the chunk from the world directory.
//! If the chunk was loaded successfully, it will send
//! the chunk back over a channel. If the chunk did not exist,
//! it will notify the server thread of the incident.
//! In response, the server thread should generate the chunk.
use crossbeam::channel::{Receiver, Sender};
use feather_core::region::{RegionHandle, RegionPosition};
use feather_core::world::chunk::Chunk;
use feather_core::world::ChunkPosition;
use hashbrown::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Reply = (ChunkPosition, Result<Chunk, Error>);
pub type Request = ChunkPosition;

#[derive(Debug)]
pub enum Error {
    ChunkNotExists,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::ChunkNotExists => {
                f.write_str("The specified chunk does not exist in the world save")?
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
    dir: Path,

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
pub fn start(world_dir: Path) -> (Sender<Request>, Receiver<Reply>) {}
