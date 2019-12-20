use crate::config::Config;
use crate::lazy::{EntityBuilder, Lazy, LazyFn};
use feather_blocks::Block;
use feather_core::world::ChunkMap;
use feather_core::{BlockPosition, Chunk, ChunkPosition};
use legion::world::World;
use parking_lot::RwLockReadGuard;
use std::sync::Arc;
use tonks::Scheduler;

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
#[derive(Resource)]
pub struct State {
    pub config: Arc<Config>,
    pub chunk_map: ChunkMap,

    lazy: Lazy,
}

impl State {
    pub fn new(config: Arc<Config>, chunk_map: ChunkMap) -> Self {
        Self {
            config,
            chunk_map,
            lazy: Lazy::default(),
        }
    }

    /// See `Lazy::exec()`.
    pub fn exec(&self, f: impl FnOnce(&mut World) + Send + 'static) {
        self.lazy.exec(f)
    }

    /// See `Lazy::create_entity()`.
    pub fn create_entity(&self) -> EntityBuilder {
        self.lazy.create_entity()
    }

    /// See `Lazy::flush()`.
    pub fn flush(&self, world: &mut World, scheduler: &mut Scheduler) {
        self.lazy.flush(world, scheduler);
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
        self.chunk_map.set_block_at(pos, block)
    }

    /// Retrieves a reference to the chunk at the given position,
    /// or `None` if it not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.chunk_map.chunk_at(pos)
    }

    /// Lazily inserts the given chunk into the chunk map.
    pub fn lazy_insert_chunk(&self, chunk: Chunk) {
        self.lazy.exec_with_scheduler(move |_, scheduler| {
            scheduler
                .resources()
                .get_mut::<State>()
                .chunk_map
                .insert(chunk);
        });
    }

    /// Lazily removes the given chunk from the chunk map.
    pub fn lazy_remove_chunk(&self, pos: ChunkPosition) {
        self.lazy
            .exec_with_scheduler(move |_: &mut World, scheduler: &mut Scheduler| {
                scheduler
                    .resources()
                    .get_mut::<State>()
                    .chunk_map
                    .remove(pos);
            });
    }
}
