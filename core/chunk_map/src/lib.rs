use feather_blocks::BlockId;
use feather_chunk::{Chunk, CHUNK_HEIGHT};
use feather_util::{BlockPosition, ChunkPosition};
use hashbrown::HashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

pub type ChunkMapInner = HashMap<ChunkPosition, Arc<RwLock<Chunk>>>;

/// The chunk map.
///
/// This struct stores all the chunks on the server,
/// so it allows access to blocks and lighting data.
///
/// Chunks are internally wrapped in `Arc<RwLock>`,
/// allowing multiple systems to access different parts
/// of the world in parallel. Mutable access to this
/// type is only required for inserting and removing
/// chunks.
#[derive(Default)]
pub struct ChunkMap(pub ChunkMapInner);

impl ChunkMap {
    /// Creates a new chunk map with no chunks.
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

    /// Retrieves the block at the specified
    /// location. If the chunk in which the block
    /// exists is not laoded, `None` is returned.
    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockId> {
        check_coords(pos)?;
        let (x, y, z) = chunk_relative_pos(pos);
        self.chunk_at(pos.into())
            .map(|chunk| chunk.block_at(x, y, z))
    }

    /// Sets the block at the given position.
    ///
    /// Returns `true` if the block was set, or `false`
    /// if its chunk was not loaded and thus no operation
    /// was performed.
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
    pub fn insert(&mut self, chunk: Chunk) {
        self.0
            .insert(chunk.position(), Arc::new(RwLock::new(chunk)));
    }

    /// Removes the chunk at the given position, returning `true` if it existed.
    pub fn remove(&mut self, pos: ChunkPosition) -> bool {
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

pub fn chunk_relative_pos(block_pos: BlockPosition) -> (usize, usize, usize) {
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
    fn chunk_map_out_of_bounds() {
        let mut map = ChunkMap::new();
        map.insert(Chunk::new(ChunkPosition::new(0, 0)));

        assert!(map.block_at(BlockPosition::new(0, -1, 0)).is_none());
        assert!(map.block_at(BlockPosition::new(0, 0, 0)).is_some());
    }
}
