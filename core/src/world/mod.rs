use crate::world::block::*;
use crate::world::chunk::Chunk;
use glm::Vec3;
use hashbrown::HashMap;
use std::ops::{Add, Sub};

pub mod block;
#[allow(clippy::cast_lossless)]
pub mod chunk;

#[derive(Clone, Copy, Debug, Default, PartialEq, new)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

impl Position {
    pub fn distance(&self, other: Position) -> f64 {
        (square(self.x - other.x) + square(self.y - other.y) + square(self.z - other.z)).sqrt()
    }

    /// Returns the position of the chunk
    /// this position is in.
    pub fn chunk_pos(&self) -> ChunkPosition {
        ChunkPosition::new(self.x as i32 / 16, self.z as i32 / 16)
    }

    /// Retrieves the position of the block
    /// this position is in.
    pub fn block_pos(&self) -> BlockPosition {
        BlockPosition::new(self.x as i32, self.y as i32, self.z as i32)
    }
}

impl Add<Vec3> for Position {
    type Output = Position;

    fn add(mut self, vec: Vec3) -> Self::Output {
        self.x += f64::from(vec.x);
        self.y += f64::from(vec.y);
        self.z += f64::from(vec.z);
        self
    }
}

impl Sub<Vec3> for Position {
    type Output = Position;

    fn sub(mut self, vec: Vec3) -> Self::Output {
        self.x -= f64::from(vec.x);
        self.y -= f64::from(vec.y);
        self.z -= f64::from(vec.z);
        self
    }
}

fn square(x: f64) -> f64 {
    x * x
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, new)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, new)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPosition {
    pub fn chunk_pos(&self) -> ChunkPosition {
        ChunkPosition::new(self.x / 16, self.z / 16)
    }
}

pub struct ChunkMap {
    chunk_map: HashMap<ChunkPosition, Chunk>,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self {
            chunk_map: HashMap::new(),
        }
    }

    pub fn inner(&self) -> &HashMap<ChunkPosition, Chunk> {
        &self.chunk_map
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<ChunkPosition, Chunk> {
        &mut self.chunk_map
    }

    /// Retrieves the chunk at the specified location.
    /// If the chunk is not loaded, `None` will be returned.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<&Chunk> {
        if let Some(chunk) = self.chunk_map.get(&pos) {
            return Some(chunk);
        }

        None
    }

    /// Retrieves the block at the specified
    /// location. If the chunk in which the block
    /// exists is not laoded, `None` is returned.
    pub fn block_at(&self, pos: BlockPosition) -> Option<Block> {
        let chunk_pos = pos.chunk_pos();

        if let Some(chunk) = self.chunk_at(chunk_pos) {
            let rpos = chunk_relative_pos(pos);
            Some(chunk.block_at(rpos.0, rpos.1, rpos.2))
        } else {
            None
        }
    }

    /// Sets the block at the given position.
    /// If the chunk in which the position resides
    /// does not exist, `Err` is returned. In all
    /// other cases, `Ok` is returned.
    ///
    /// Note that on the server side, calling this function
    /// does not broadcast the update in any way. As such,
    /// the according function should be called instead.
    pub fn set_block_at(&mut self, pos: BlockPosition, block: Block) -> Result<(), ()> {
        let chunk_pos = pos.chunk_pos();

        if let Some(chunk) = self.chunk_map.get_mut(&chunk_pos) {
            let (x, y, z) = chunk_relative_pos(pos);
            chunk.set_block_at(x, y, z, block);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Sets the chunk at the given location.
    pub fn set_chunk_at(&mut self, pos: ChunkPosition, chunk: Chunk) {
        self.chunk_map.insert(pos, chunk);
    }

    /// Removes the chunk at the given location,
    /// effectively unloading it.
    pub fn unload_chunk_at(&mut self, pos: ChunkPosition) {
        self.chunk_map.remove(&pos);
    }

    /// Returns an immutable reference to the internal map.
    pub fn chunks(&self) -> &HashMap<ChunkPosition, Chunk> {
        &self.chunk_map
    }

    /// Returns a mutable reference to the internal
    /// map.
    pub fn chunks_mut(&mut self) -> &mut HashMap<ChunkPosition, Chunk> {
        &mut self.chunk_map
    }
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self::new()
    }
}

fn chunk_relative_pos(block_pos: BlockPosition) -> (usize, usize, usize) {
    (
        block_pos.x as usize & 0xf,
        block_pos.y as usize,
        block_pos.z as usize & 0xf,
    )
}

pub trait ChunkGenerator {
    fn generate(&self, chunk: &mut Chunk);
}

pub struct FlatChunkGenerator {}

impl ChunkGenerator for FlatChunkGenerator {
    fn generate(&self, chunk: &mut Chunk) {
        for x in 0..16 {
            for y in 0..64 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, Block::Stone);
                }
            }
        }
    }
}

pub struct GridChunkGenerator {}

impl ChunkGenerator for GridChunkGenerator {
    fn generate(&self, chunk: &mut Chunk) {
        for x in 0..15 {
            for y in 0..64 {
                for z in 0..15 {
                    chunk.set_block_at(x, y, z, Block::Stone);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_map() {
        let mut world = ChunkMap::new();

        let chunk = world.chunk_at(ChunkPosition::new(0, 0));
        if chunk.is_some() {
            panic!();
        }

        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        FlatChunkGenerator {}.generate(&mut chunk);
        world.chunk_map.insert(ChunkPosition::new(0, 0), chunk);

        let chunk = world.chunk_at(ChunkPosition::new(0, 0)).unwrap();

        for x in 0..15 {
            for y in 0..64 {
                for z in 0..15 {
                    assert_eq!(chunk.block_at(x, y, z), Block::Stone);
                }
            }
        }

        assert_eq!(chunk.block_at(8, 64, 8), Block::Air);
    }

    #[test]
    fn test_set_block_at() {
        let mut world = ChunkMap::new();

        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        GridChunkGenerator {}.generate(&mut chunk);
        world.chunk_map.insert(ChunkPosition::new(0, 0), chunk);

        println!("-----");
        world
            .set_block_at(BlockPosition::new(1, 63, 1), Block::Air)
            .unwrap();

        println!("-----");
        assert_eq!(
            world.block_at(BlockPosition::new(1, 63, 1)).unwrap(),
            Block::Air
        );
    }
}
