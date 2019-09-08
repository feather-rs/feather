//! Implements a basic height map generator using 2D Perlin noise.
//! A superior generator would use 3D noise to allow for overhangs.

use crate::worldgen::{block_index, ChunkBiomes, HeightMapGenerator};
use bitvec::vec::BitVec;
use feather_core::ChunkPosition;
use simdnoise::NoiseBuilder;

/// Height map generator using two-dimensional Perlin noise.
#[derive(Debug, Default)]
pub struct BasicHeightMapGenerator;

impl HeightMapGenerator for BasicHeightMapGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, _biomes: &ChunkBiomes, seed: u64) -> BitVec {
        let x_offset = (chunk.x * 16) as f32;
        let y_offset = (chunk.z * 16) as f32;

        let dim = 16;

        let noise = NoiseBuilder::gradient_2d_offset(x_offset, dim, y_offset, dim)
            //.with_lacunarity(2.0)
            //.with_gain(0.5)
            .with_seed(seed as i32)
            //.with_octaves(2)
            .generate_scaled(0.0, 1.0);

        let mut density_map = bitvec![0; 16 * 256 * 16];
        for x in 0..16 {
            for z in 0..16 {
                let value = noise[(x << 4) | z];
                let height = (value * 16.0 + 64.0) as usize;

                for y in 0..height {
                    density_map.set(block_index(x, y, z), true);
                }
            }
        }

        density_map
    }
}
