//! Feather's multiworld support.

use std::{fmt::Display, time::Duration};

use flume::Sender;
use libcraft::{dimension::DimensionInfo, BlockPosition, BlockState, ChunkPosition, WorldHeight};
use uuid::Uuid;

use crate::{saveload::WorldSource, ChunkHandle};

/// Unique, persistent ID of a world.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorldId(Uuid);

impl Display for WorldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hyphenated())
    }
}

impl WorldId {
    pub fn new_random() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn to_uuid(self) -> Uuid {
        self.0
    }
}

/// Parameters to initialize a world.
pub struct WorldDescriptor {
    /// ID of the world.
    ///
    /// If the world already exists, then this ID should
    /// match the original ID. If it is a new world, you
    /// can use `WorldId::new_random`.
    pub id: WorldId,
    /// Where chunks will be loaded and saved.
    pub source: Box<dyn WorldSource>,
    /// An optional name.
    pub name: Option<String>,
    /// Dimension info.
    pub dimension_info: DimensionInfo,
    /// Whether this is a superflat world with a different
    /// horizon.
    pub flat: bool,
    /// Settings on how the server will load and unload chunks.
    pub settings: WorldSettings,
}

/// Settings on how the server will load and unload chunks.
#[derive(Debug)]
pub struct WorldSettings {
    /// When a chunk loses all its tickets, the server puts
    /// it into a queue to be unloaded. It will wait at least
    /// this duration before unloading the chunk.
    pub unload_delay: Duration,
    /// How the server will save chunks.
    ///
    /// Note that if this is set to `SaveIncrementally` and the configured
    /// `WorldSource` does not support saving, then the server
    /// will panic when creating the world.
    pub save_strategy: WorldSaveStrategy,
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            unload_delay: Duration::from_secs(30),
            save_strategy: WorldSaveStrategy::SaveIncrementally {
                save_interval: Duration::from_secs(60 * 5),
            },
        }
    }
}

/// How the server will save chunks.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WorldSaveStrategy {
    /// Chunks will be saved incrementally at the given interval,
    /// as well as when they are unloaded or the server shuts down.
    ///
    /// The server will save chunks by calling `WorldSource::save_chunk`.
    SaveIncrementally { save_interval: Duration },
    /// The server will not save chunks, instead choosing to drop any changes made
    /// to chunks when they are unloaded. Changes will not persist after
    /// a chunk is unloaded.
    ///
    /// This option is only recommended if you have an immutable world.
    DropChanges,
    /// The server will not save chunks, instead choosing to keep all loaded
    /// chunks in memory. Changes will not persist after a server restart.
    ///
    /// This option is useful for minigames where the map is reset each new game,
    /// and the world is small enough that all chunks can stay loaded.
    KeepLoaded,
}

/// Stores all blocks and chunks in a world.
///
/// A world does not store entities; it only contains blocks.
///
/// Like `Game`, this is a trait. You can pass around `World`s
/// using `&dyn World`.
///
/// # Identifying worlds
/// Every world is associated with a UUID and optionally a world name.
///
/// # Chunk loading
/// Each world manages its own chunk loading and saving.
///
/// A world is associated with a [`WorldSource`](crate::saveload::WorldSource)
/// to which chunks are loaded and saved.
///
/// Chunk loading and unloading works off of a _ticket_ system.
/// Any chunks for which one or more tickets exists are kept
/// loaded. If all tickets for a chunk are removed, then the chunk
/// may be unloaded.
pub trait World: 'static {
    /// Gets the ID of this world.
    ///
    /// The ID is persistent even after a server restart.
    fn id(&self) -> WorldId;

    /// Gets the optionally-set name of this world.
    ///
    /// Note that a world's name can change, although its UUID is stable.
    fn name(&self) -> Option<&str>;

    /// Gets a string to be displayed to the console when describing
    /// this world.
    fn display_name(&self) -> String {
        self.name()
            .map(str::to_owned)
            .unwrap_or_else(|| self.id().to_string())
    }

    /// Gets the block at the given position,
    /// or returns an error if the chunk containing
    /// the block is not loaded.
    fn block_at(&self, pos: BlockPosition) -> Result<BlockState, ChunkNotLoaded>;

    /// Sets the block at the given position,
    /// or returns an error if the chunk containing
    /// the block is not loaded.
    fn set_block_at(&self, pos: BlockPosition, block: BlockState) -> Result<(), ChunkNotLoaded>;

    /// Gets the chunk at the given chunk position.
    fn chunk_handle_at(&self, pos: ChunkPosition) -> Result<ChunkHandle, ChunkNotLoaded>;

    /// Returns whether the given chunk is loaded.
    fn is_chunk_loaded(&self, pos: ChunkPosition) -> bool;

    /// Gets the dimension info of this world.
    fn dimension_info(&self) -> &DimensionInfo;

    /// Gets the minimum Y coordinate of blocks in this world.
    fn min_y(&self) -> i32 {
        self.dimension_info().info.min_y
    }

    /// Gets the height of the world, in blocks.
    ///
    /// Guaranteed to be a multiple of 16 (the height of a chunk section).
    fn height(&self) -> WorldHeight {
        WorldHeight(
            self.dimension_info()
                .info
                .height
                .try_into()
                .unwrap_or_default(),
        )
    }

    /// Creates a ticket for the given chunk.
    ///
    /// If the chunk is not already loaded, then it is queued
    /// for loading. On a future tick, it will be available.
    #[must_use]
    fn create_chunk_ticket(&mut self, chunk: ChunkPosition) -> ChunkTicket;

    /// Returns whether the `WorldSource` backing this world
    /// will save changes.
    fn is_persistent(&self) -> bool;

    /// Returns whether this world has a superflat world's horizon.
    fn is_flat(&self) -> bool;
}

/// Indicates that the chunk needed to perform
/// an operation is not loaded.
#[derive(Debug, thiserror::Error)]
#[error("chunk {0:?} is not loaded")]
pub struct ChunkNotLoaded(pub ChunkPosition);

/// A ticket to keep a chunk loaded.
///
/// When this value is dropped, the ticket is removed automatically.
#[derive(Debug)]
pub struct ChunkTicket {
    id: u64,
    dropped_channel: Sender<u64>,
}

impl ChunkTicket {
    #[doc(hidden)]
    pub fn new(id: u64, dropped_channel: Sender<u64>) -> Self {
        Self {
            id,
            dropped_channel,
        }
    }
}

impl Drop for ChunkTicket {
    fn drop(&mut self) {
        self.dropped_channel.send(self.id).ok();
    }
}
