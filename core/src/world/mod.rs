use crate::world::block::BlockType;
use crate::world::chunk::Chunk;
use std::cell::RefCell;
use std::collections::HashMap;

pub mod block;
#[allow(clippy::cast_lossless)]
pub mod chunk;

#[derive(Clone, Copy, Debug, new)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

impl Position {
    pub fn distance(&self, other: Position) -> f64 {
        (square(self.x - other.x) - square(self.y - other.y) - square(self.z - other.z)).sqrt()
    }
}

fn square(x: f64) -> f64 {
    x * x
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, new)]
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
        ChunkPosition::new(self.x % 16, self.z & 16)
    }
}

pub struct World {
    generator: RefCell<Box<ChunkGenerator>>,
    chunk_map: RefCell<HashMap<ChunkPosition, RefCell<Chunk>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            generator: RefCell::new(Box::new(FlatChunkGenerator {})),
            chunk_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn chunk_at(&self, pos: ChunkPosition) -> RefCell<Chunk> {
        if let Some(chunk) = self.chunk_map.borrow().get(&pos) {
            return chunk.clone();
        }
        {
            self.load_chunk(pos);
            self.chunk_map.borrow().get(&pos).unwrap().clone()
        }
    }

    pub fn set_block_at(&self, pos: BlockPosition, block: BlockType) {
        let chunk = self.chunk_at(pos.chunk_pos());
        chunk.borrow_mut().set_block_at((pos.x % 16) as u16, pos.y as u16, (pos.z % 16) as u16, block);
    }

    fn load_chunk(&self, pos: ChunkPosition) {
        let mut chunk = Chunk::new(pos);
        self.generator.borrow_mut().generate(&mut chunk);
        self.chunk_map.borrow_mut().insert(pos, RefCell::new(chunk));
    }
}

trait ChunkGenerator {
    fn generate(&self, chunk: &mut Chunk);
}

pub struct FlatChunkGenerator {}

impl ChunkGenerator for FlatChunkGenerator {
    fn generate(&self, chunk: &mut Chunk) {
        for x in 0..16 {
            for y in 0..64 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, BlockType::Stone);
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
        let world = World::new();

        let _chunk = world.chunk_at(ChunkPosition::new(0, 0));
        let chunk = _chunk.borrow();

        for x in 0..16 {
            for y in 0..64 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z), BlockType::Stone);
                }
            }
        }

        assert_eq!(chunk.block_at(8, 64, 8), BlockType::Air);
    }
}

/// Calculates the relative move fields
/// as used in the EntityRelativeMove packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (u16, u16, u16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as u16;
    let y = ((current.y * 32.0 - old.x * 32.0) * 128.0) as u16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as u16;
    (x, y, z)
}
