//! Implements a basic height map generator using 2D Perlin noise.
//! A superior generator would use 3D noise to allow for overhangs.

use crate::worldgen::{block_index, ChunkBiomes, DensityMapGenerator, SKY_LIMIT};
use bitvec::vec::BitVec;
use feather_core::ChunkPosition;
use simdnoise::NoiseBuilder;
use std::cmp::min;

/// Density map generator which simply uses a height map
/// using two-dimensional Perlin noise.
#[derive(Debug, Default)]
pub struct HeightMapGenerator;

impl DensityMapGenerator for HeightMapGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, _biomes: &ChunkBiomes, seed: u64) -> BitVec {
        let x_offset = (chunk.x * 16) as f32;
        let y_offset = (chunk.z * 16) as f32;

        let dim = 16;
        let elevation = NoiseBuilder::gradient_2d_offset(x_offset, dim, y_offset, dim)
            .with_seed(seed as i32)
            .generate_scaled(0.0, 12.0);
        let detail = NoiseBuilder::gradient_2d_offset(x_offset, dim, y_offset, dim)
            .with_seed(seed as i32 + 1)
            .generate_scaled(-0.5, 1.5);

        let mut density_map = bitvec![0; 16 * 256 * 16];
        for x in 0..16 {
            for z in 0..16 {
                let index = (z << 4) | x;
                let elevation = elevation[index];
                let detail = detail[index];
                let height = (elevation + detail + 64.0) as usize;

                for y in 0..min(height, SKY_LIMIT) {
                    density_map.set(block_index(x, y, z), true);
                }
            }
        }

        density_map
    }
}
