//! This module handles the asynchronous loading and saving
//! of chunks. It receives load and save requests from the server
//! (over a channel) and executes them.
//!
//! If a chunk cannot be loaded, it is generated on the Rayon thread pool
//! instead.
use ahash::AHashMap;
use crossbeam::channel::{Receiver, Sender};
use feather_core::anvil::entity::EntityData;
use feather_core::anvil::region;
use feather_core::anvil::{
    block_entity::BlockEntityData,
    region::{RegionHandle, RegionPosition},
};
use feather_core::chunk::Chunk;
use feather_core::util::ChunkPosition;
use feather_server_util::EntityLoader;
use feather_server_worldgen::WorldGenerator;
use fecs::EntityBuilder;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

