use std::sync::Arc;

use ahash::AHashMap;
use libcraft::{BlockPosition, BlockState, Chunk, ChunkPosition, WorldHeight};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};

use quill::ChunkHandle;

pub type ChunkMapInner = AHashMap<ChunkPosition, ChunkHandle>;

/// This struct stores all the chunks on the server,
/// so it allows access to blocks and lighting data.
///
/// Chunks are internally wrapped in `Arc<RwLock>`,
/// allowing multiple systems to access different parts
/// of the world in parallel. Mutable access to this
/// type is only required for inserting and removing
/// chunks.
pub struct ChunkMap {
    inner: ChunkMapInner,
    height: WorldHeight,
    min_y: i32,
}

impl ChunkMap {
    /// Creates a new, empty world.
    pub fn new(world_height: WorldHeight, min_y: i32) -> Self {
        ChunkMap {
            inner: ChunkMapInner::default(),
            height: world_height,
            min_y,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.inner.get(&pos).map(|lock| lock.read())
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at_mut(&self, pos: ChunkPosition) -> Option<RwLockWriteGuard<Chunk>> {
        self.inner.get(&pos).map(|lock| lock.write()).flatten()
    }

    /// Returns an `Arc<RwLock<Chunk>>` at the given position.
    pub fn chunk_handle_at(&self, pos: ChunkPosition) -> Option<ChunkHandle> {
        self.inner.get(&pos).map(Arc::clone)
    }

    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockState> {
        check_coords(pos, self.height, self.min_y)?;

        let (x, y, z) = chunk_relative_pos(pos);
        self.chunk_at(pos.chunk())
            .map(|chunk| chunk.block_at(x, (y - self.min_y as isize) as usize, z))
            .flatten()
    }

    pub fn set_block_at(&self, pos: BlockPosition, block: BlockState) -> bool {
        if check_coords(pos, self.height, self.min_y).is_none() {
            return false;
        }

        let (x, y, z) = chunk_relative_pos(pos.into());

        self.chunk_at_mut(pos.chunk())
            .map(|mut chunk| {
                chunk.set_block_at(x, (y - self.min_y as isize) as usize, z, block, true)
            })
            .is_some()
    }

    /// Inserts a new chunk into the chunk map.
    pub fn insert_chunk(&mut self, chunk: ChunkHandle) {
        let pos = chunk.read().position();
        self.inner.insert(pos, chunk);
    }

    /// Removes the chunk at the given position.
    pub fn remove_chunk(&mut self, pos: ChunkPosition) -> Option<ChunkHandle> {
        self.inner.remove(&pos)
    }

    pub fn contains(&self, pos: ChunkPosition) -> bool {
        self.inner.contains_key(&pos)
    }

    pub fn iter(&self) -> impl Iterator<Item = ChunkPosition> + '_ {
        self.inner.keys().copied()
    }
}

fn check_coords(pos: BlockPosition, world_height: WorldHeight, min_y: i32) -> Option<()> {
    if pos.y >= min_y && pos.y < *world_height as i32 + min_y {
        Some(())
    } else {
        None
    }
}

fn chunk_relative_pos(block_pos: BlockPosition) -> (usize, isize, usize) {
    (
        block_pos.x as usize & 0xf,
        block_pos.y as isize,
        block_pos.z as usize & 0xf,
    )
}
