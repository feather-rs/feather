use ahash::{AHashMap, AHashSet};
use base::{BlockPosition, Chunk, ChunkPosition, CHUNK_HEIGHT};
use blocks::BlockId;
use ecs::Ecs;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

use crate::{
    events::ChunkLoadEvent,
    world_source::{null::NullWorldSource, ChunkLoadResult, WorldSource},
};

/// Stores all blocks and chunks in a world,
/// along with global world data like weather, time,
/// and the [`WorldSource`](crate::world_source::WorldSource).
///
/// NB: _not_ what most Rust ECSs call "world."
/// This does not store entities; it only contains blocks.
pub struct World {
    chunk_map: ChunkMap,
    world_source: Box<dyn WorldSource>,
    loading_chunks: AHashSet<ChunkPosition>,
    canceled_chunk_loads: AHashSet<ChunkPosition>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            chunk_map: ChunkMap::new(),
            world_source: Box::new(NullWorldSource::default()),
            loading_chunks: AHashSet::new(),
            canceled_chunk_loads: AHashSet::new(),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `World` from a `WorldSource` for loading chunks.
    pub fn with_source(world_source: impl WorldSource + 'static) -> Self {
        Self {
            world_source: Box::new(world_source),
            ..Default::default()
        }
    }

    /// Queues the given chunk to be loaded.
    pub fn queue_chunk_load(&mut self, pos: ChunkPosition) {
        self.loading_chunks.insert(pos);
        self.world_source.queue_load(pos);
    }

    /// Loads any chunks that have been loaded asynchronously
    /// after a call to [`World::queue_chunk_load`].
    pub fn load_chunks(&mut self, ecs: &mut Ecs) {
        while let Some(loaded) = self.world_source.poll_loaded_chunk() {
            self.loading_chunks.remove(&loaded.pos);
            if self.canceled_chunk_loads.remove(&loaded.pos) {
                continue;
            }

            let chunk = match loaded.result {
                ChunkLoadResult::Missing => {
                    log::debug!(
                        "Chunk {:?} is missing; using default empty chunk",
                        loaded.pos
                    );
                    Chunk::new(loaded.pos)
                }
                ChunkLoadResult::Error(e) => {
                    log::error!("Failed to load chunk {:?}: {:?}", loaded.pos, e);
                    continue;
                }
                ChunkLoadResult::Loaded { chunk } => chunk,
            };
            self.chunk_map.insert_chunk(chunk);
            ecs.insert_event(ChunkLoadEvent {
                chunk: Arc::clone(&self.chunk_map.0[&loaded.pos]),
                position: loaded.pos,
            });
            log::trace!("Loaded chunk {:?}", loaded.pos);
        }
    }

    /// Unloads the given chunk.
    pub fn unload_chunk(&mut self, pos: ChunkPosition) {
        self.chunk_map.remove_chunk(pos);
        if self.is_chunk_loading(pos) {
            self.canceled_chunk_loads.insert(pos);
        }

        log::trace!("Unloaded chunk {:?}", pos);
    }

    /// Returns whether the given chunk is loaded.
    pub fn is_chunk_loaded(&self, pos: ChunkPosition) -> bool {
        self.chunk_map.0.contains_key(&pos)
    }

    /// Returns whether the given chunk is queued to be loaded.
    pub fn is_chunk_loading(&self, pos: ChunkPosition) -> bool {
        self.loading_chunks.contains(&pos)
    }

    /// Sets the block at the given position.
    ///
    /// Returns `true` if the block was set, or `false`
    /// if its chunk was not loaded or the coordinates
    /// are out of bounds and thus no operation
    /// was performed.
    pub fn set_block_at(&self, pos: BlockPosition, block: BlockId) -> bool {
        self.chunk_map.set_block_at(pos, block)
    }

    /// Retrieves the block at the specified
    /// location. If the chunk in which the block
    /// exists is not loaded or the coordinates
    /// are out of bounds, `None` is returned.
    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockId> {
        self.chunk_map.block_at(pos)
    }

    /// Returns the chunk map.
    pub fn chunk_map(&self) -> &ChunkMap {
        &self.chunk_map
    }

    /// Mutably gets the chunk map.
    pub fn chunk_map_mut(&mut self) -> &mut ChunkMap {
        &mut self.chunk_map
    }
}

pub type ChunkMapInner = AHashMap<ChunkPosition, Arc<RwLock<Chunk>>>;

/// This struct stores all the chunks on the server,
/// so it allows access to blocks and lighting data.
///
/// Chunks are internally wrapped in `Arc<RwLock>`,
/// allowing multiple systems to access different parts
/// of the world in parallel. Mutable access to this
/// type is only required for inserting and removing
/// chunks.
#[derive(Default)]
pub struct ChunkMap(ChunkMapInner);

impl ChunkMap {
    /// Creates a new, empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.0.get(&pos).map(|lock| lock.read())
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at_mut(&self, pos: ChunkPosition) -> Option<RwLockWriteGuard<Chunk>> {
        self.0.get(&pos).map(|lock| lock.write())
    }

    /// Returns an `Arc<RwLock<Chunk>>` at the given position.
    pub fn chunk_handle_at(&self, pos: ChunkPosition) -> Option<Arc<RwLock<Chunk>>> {
        self.0.get(&pos).map(Arc::clone)
    }

    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockId> {
        check_coords(pos)?;
        let (x, y, z) = chunk_relative_pos(pos);
        self.chunk_at(pos.into())
            .map(|chunk| chunk.block_at(x, y, z))
            .flatten()
    }

    pub fn set_block_at(&self, pos: BlockPosition, block: BlockId) -> bool {
        if check_coords(pos).is_none() {
            return false;
        }
        let (x, y, z) = chunk_relative_pos(pos);

        self.chunk_at_mut(pos.into())
            .map(|mut chunk| chunk.set_block_at(x, y, z, block))
            .is_some()
    }

    /// Returns an iterator over chunks.
    pub fn iter_chunks(&self) -> impl IntoIterator<Item = &Arc<RwLock<Chunk>>> {
        self.0.values()
    }

    /// Inserts a new chunk into the chunk map.
    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.0
            .insert(chunk.position(), Arc::new(RwLock::new(chunk)));
    }

    /// Removes the chunk at the given position, returning `true` if it existed.
    pub fn remove_chunk(&mut self, pos: ChunkPosition) -> bool {
        self.0.remove(&pos).is_some()
    }
}

fn check_coords(pos: BlockPosition) -> Option<()> {
    if pos.y >= 0 && pos.y < CHUNK_HEIGHT as i32 {
        Some(())
    } else {
        None
    }
}

fn chunk_relative_pos(block_pos: BlockPosition) -> (usize, usize, usize) {
    (
        block_pos.x as usize & 0xf,
        block_pos.y as usize,
        block_pos.z as usize & 0xf,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_out_of_bounds() {
        let mut world = World::new();
        world
            .chunk_map_mut()
            .insert_chunk(Chunk::new(ChunkPosition::new(0, 0)));

        assert!(world.block_at(BlockPosition::new(0, -1, 0)).is_none());
        assert!(world.block_at(BlockPosition::new(0, 0, 0)).is_some());
    }
}
