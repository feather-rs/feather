use crate::config::Config;
use crate::lazy::{EntityBuilder, Lazy};
use feather_blocks::Block;
use feather_core::world::ChunkMap;
use feather_core::{BlockPosition, Chunk, ChunkPosition};
use legion::world::World;
use parking_lot::RwLockReadGuard;
use std::sync::Arc;

/// The state of the server.
///
/// This state wraps numerous commonly-used resources,
/// including the chunk map (block access), config, and
/// various cached data structures, among others.
///
/// Systems should never require mutable access to the
/// state; it is designed for read-only use. (The chunk
/// map uses `RwLock` internally, so write access isn't
/// needed to update blocks.)
pub struct State {
    pub config: Arc<Config>,

    chunk_map: ChunkMap,
    lazy: Lazy,
}

impl State {
    /// See `Lazy::exec()`.
    pub fn exec(&self, f: impl FnOnce(&mut World)) {
        self.lazy.exec(f)
    }

    /// See `Lazy::create_entity()`.
    pub fn create_entity(&self) -> EntityBuilder {
        self.lazy.create_entity()
    }

    /// See `Lazy::flush()`.
    pub fn flush(&self, world: &mut World) {
        self.lazy.flush(world);
    }

    /// Retrieves the block at the given position,
    /// or `None` if the block's chunk is not loaded.
    pub fn block_at(&self, pos: BlockPosition) -> Option<Block> {
        self.chunk_map.block_at(pos)
    }

    /// Sets the block at the given position.
    ///
    /// If the block's chunk's is not loaded, returns `false`;
    /// otherwise, returns `true`.
    pub fn set_block_at(&self, pos: BlockPosition, block: Block) -> bool {
        self.chunk_map.set_block_at(pos, block).is_ok()
    }

    /// Retrieves a reference to the chunk at the given position,
    /// or `None` if it not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.chunk_map.chunk_at(pos)
    }
}
