use std::convert::TryInto;
use std::marker::PhantomData;

use libcraft_blocks::{BlockState, SimplifiedBlockKind};
use libcraft_core::{WorldHeight, CHUNK_WIDTH};

use super::PackedArray;

/// Stores heightmaps for a chunk.
#[derive(Debug, Clone)]
pub struct HeightmapStore {
    pub motion_blocking: Heightmap<MotionBlocking>,
    pub motion_blocking_no_leaves: Heightmap<MotionBlockingNoLeaves>,
    pub ocean_floor: Heightmap<OceanFloor>,
    pub world_surface: Heightmap<WorldSurface>,
}

impl HeightmapStore {
    pub fn new(height: WorldHeight) -> Self {
        Self {
            motion_blocking: Heightmap::new(height),
            motion_blocking_no_leaves: Heightmap::new(height),
            ocean_floor: Heightmap::new(height),
            world_surface: Heightmap::new(height),
        }
    }

    pub fn update(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        old_block: BlockState,
        new_block: BlockState,
        get_block: impl Fn(usize, usize, usize) -> BlockState,
    ) {
        self.motion_blocking
            .update(x, y, z, old_block, new_block, &get_block);
        self.motion_blocking_no_leaves
            .update(x, y, z, old_block, new_block, &get_block);
        self.ocean_floor
            .update(x, y, z, old_block, new_block, &get_block);
        self.world_surface
            .update(x, y, z, old_block, new_block, &get_block);
    }

    pub fn recalculate(&mut self, get_block: impl Fn(usize, usize, usize) -> BlockState) {
        self.motion_blocking.recalculate(&get_block);
        self.motion_blocking_no_leaves.recalculate(&get_block);
        self.ocean_floor.recalculate(&get_block);
        self.world_surface.recalculate(&get_block);
    }
}

/// A function used to compute heightmaps.
pub trait HeightmapFunction {
    /// Returns whether a block should be considered
    /// "solid" during the heightmap computation.
    fn is_solid(block: BlockState) -> bool;
}

#[derive(Debug, Clone)]
pub struct LightBlocking;
impl HeightmapFunction for LightBlocking {
    fn is_solid(block: BlockState) -> bool {
        block.kind().opaque()
    }
}

#[derive(Debug, Clone)]
pub struct MotionBlocking;
impl HeightmapFunction for MotionBlocking {
    fn is_solid(block: BlockState) -> bool {
        block.kind().solid() || block.kind().fluid()
    }
}

#[derive(Debug, Clone)]
pub struct MotionBlockingNoLeaves;
impl HeightmapFunction for MotionBlockingNoLeaves {
    fn is_solid(block: BlockState) -> bool {
        (block.kind().solid() || block.kind().fluid())
            && block.simplified_kind() != SimplifiedBlockKind::Leaves
    }
}

#[derive(Debug, Clone)]
pub struct OceanFloor;
impl HeightmapFunction for OceanFloor {
    fn is_solid(block: BlockState) -> bool {
        block.kind().solid()
    }
}

#[derive(Debug, Clone)]
pub struct WorldSurface;
impl HeightmapFunction for WorldSurface {
    fn is_solid(block: BlockState) -> bool {
        !block.kind().is_air()
    }
}

#[derive(Debug, Clone)]
pub struct Heightmap<F> {
    heights: PackedArray,
    height: WorldHeight,
    _marker: PhantomData<F>,
}

impl<F> Heightmap<F>
where
    F: HeightmapFunction,
{
    pub fn new(height: WorldHeight) -> Self {
        Self {
            heights: PackedArray::new(
                (CHUNK_WIDTH * CHUNK_WIDTH) as usize,
                ((*height as f64 + 1.0).log2().ceil() as usize)
                    .try_into()
                    .unwrap(),
            ),
            height,
            _marker: PhantomData,
        }
    }

    pub fn set_height(&mut self, x: usize, z: usize, height: usize) {
        let index = self.index(x, z);
        self.heights.set(index, height as u64);
    }

    pub fn set_height_index(&mut self, index: usize, height: i64) {
        self.heights.as_u64_mut_vec()[index] = height as u64;
    }

    pub fn height(&self, x: usize, z: usize) -> Option<usize> {
        let index = self.index(x, z);
        self.heights.get(index).map(|x| x as usize)
    }

    pub fn as_u64_slice(&self) -> &[u64] {
        self.heights.as_u64_slice()
    }

    fn index(&self, x: usize, z: usize) -> usize {
        (z << 4) | x
    }

    /// Updates this height map after a block has been updated.
    pub fn update(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        old_block: BlockState,
        new_block: BlockState,
        get_block: impl Fn(usize, usize, usize) -> BlockState,
    ) {
        if F::is_solid(old_block) && self.height(x, z) == Some(y) {
            // This was old the highest block
            for i in (0..y).rev() {
                let block = get_block(x, i, z);

                if F::is_solid(block) {
                    self.set_height(x, z, i + 1);
                    break;
                }
            }
        }
        if F::is_solid(new_block) && self.height(x, z).unwrap() < y {
            // This is the new highest block
            self.set_height(x, z, y);
        }
    }

    /// Recalculates this entire heightmap.
    pub fn recalculate(&mut self, get_block: impl Fn(usize, usize, usize) -> BlockState) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in (0..*self.height).rev() {
                    if F::is_solid(get_block(x, y, z)) {
                        self.set_height(x, z, y + 1);
                        break;
                    }
                }
            }
        }
    }

    pub fn from_u64_vec(vec: Vec<u64>, height: WorldHeight) -> Heightmap<F> {
        Heightmap {
            heights: PackedArray::from_u64_vec(vec, CHUNK_WIDTH * CHUNK_WIDTH),
            height,
            _marker: PhantomData,
        }
    }
}
