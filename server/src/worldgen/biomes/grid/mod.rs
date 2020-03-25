//! Biome generation based on a growing 2D array of integers.
//! Each step in the pipeline interprets the integers in a different
//! way. At the core of this design is the "zoom" operation, which
//! randomly increases detail in such an array and expands its size.
//!
//! See http://cuberite.xoft.cz/docs/Generator.html#biomegen (section "Grown biomes")
//! for an expanded explanation.

use crate::worldgen::noise::normalized_fbm;
use crate::worldgen::util::map;
use crate::worldgen::{BiomeGenerator, ChunkBiomes};
use feather_core::{Biome, ChunkPosition};
use zoom::zoom;

mod zoom;

/// Wrapper over `Vec`.
#[derive(Debug, Clone)]
pub struct Grid {
    vec: Vec<u16>,
    size_x: usize,
    /// Global offsets of the origin of this grid in blocks
    pub offset_x: i32,
    pub offset_z: i32,
    /// Scale along each axis of this grid. For example,
    /// if set to 4, then moving 1 to the right in this
    /// grid corresponds to moving 4 blocks.
    pub scale: usize,
    // size_z can be inferred using vec.len() and size_x
}

impl Grid {
    /// Creates a new `Grid` with the provided dimensions.
    /// Values are initialized with 0.
    pub fn new(size_x: usize, size_z: usize, offset_x: i32, offset_z: i32, scale: usize) -> Self {
        Self {
            vec: vec![0; size_x * size_z],
            size_x,
            offset_x,
            offset_z,
            scale,
        }
    }

    /// Creates a new `Grid` with the same offset
    /// as another `Grid` but with new dimensions.
    /// The scale is adjusted to account for the new dimensions.
    pub fn from_input(other: &Grid, new_size_x: usize, new_size_z: usize) -> Self {
        let scale = (other.size_x as f64 / new_size_x as f64 * other.scale as f64).round() as usize;

        Self::new(
            new_size_x,
            new_size_z,
            other.offset_x,
            other.offset_z,
            scale,
        )
    }

    /// Retrieves the value at (x, z).
    pub fn at(&self, x: usize, z: usize) -> u16 {
        self.vec[x + (z * self.size_x)]
    }

    /// Sets the value at (x, z).
    pub fn set_at(&mut self, x: usize, z: usize, val: u16) {
        self.vec[x + (z * self.size_x)] = val;
    }

    /// Returns the size of this grid along the X axis.
    pub fn size_x(&self) -> usize {
        self.size_x
    }

    /// Returns the size of this grid along the Z axis.
    pub fn size_z(&self) -> usize {
        self.vec.len() / self.size_x
    }

    /// Samples the value at the given position, in absolute blocks
    /// from the world origin. Nearest interpolation is used.
    pub fn sample(&self, abs_x: i32, abs_z: i32) -> u16 {
        let mut x = (abs_x - self.offset_x) as usize;
        let mut z = (abs_z - self.offset_z) as usize;

        x /= self.scale;
        z /= self.scale;

        self.at(x, z)
    }
}

#[derive(Debug, Default)]
pub struct GridBiomeGenerator;

// starting grid is 3x3 with scale 512: i.e. each
// tile in the grid represents 512x512 blocks
const STARTING_GRID_SIZE: usize = 3;
const STARTING_GRID_SCALE: usize = 512;
const STARTING_GRID_OFFSET_RELATIVE_TO_CHUNK: i32 = -(STARTING_GRID_SCALE as i32) - 8;

impl BiomeGenerator for GridBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> ChunkBiomes {
        let mut grid = Grid::new(
            STARTING_GRID_SIZE,
            STARTING_GRID_SIZE,
            STARTING_GRID_OFFSET_RELATIVE_TO_CHUNK + chunk.x * 16,
            STARTING_GRID_OFFSET_RELATIVE_TO_CHUNK + chunk.z * 16,
            STARTING_GRID_SCALE,
        );
        fill_with_ocean_land(&mut grid, seed);
        for _ in 0..8 {
            grid = zoom(grid, seed + 1);
        }

        let mut biomes = ChunkBiomes::default();

        for x in 0..16 {
            for z in 0..16 {
                let biome = if grid.sample(x + chunk.x * 16, z + chunk.z * 16) > 0 {
                    Biome::Ocean
                } else {
                    Biome::Plains
                };
                biomes.set_biome_at(x as usize, z as usize, biome);
            }
        }

        biomes
    }
}

/// Fills in a grid with ocean-land values. 0=land; >0=ocean.
fn fill_with_ocean_land(grid: &mut Grid, seed: u64) {
    for x in 0..grid.size_x() {
        for z in 0..grid.size_z() {
            let abs_x = x as i32 * grid.scale as i32 + grid.offset_x;
            let abs_z = z as i32 * grid.scale as i32 + grid.offset_z;

            let noise = normalized_fbm(abs_x as f32, abs_z as f32, 0.02, 0.5, 2.0, 3, seed);

            grid.set_at(x, z, map(noise, 0.0, 1.0, 0.0, 2.0).floor() as u16);
        }
    }
}
