//! Implements a basic height map generator using 2D Perlin noise.
//! A superior generator would use 3D noise to allow for overhangs.

use crate::{block_index, DensityMapGenerator, NearbyBiomes, OCEAN_DEPTH, SKY_LIMIT};
use base::{Biome, ChunkPosition};
use bitvec::order::LocalBits;
use bitvec::vec::BitVec;
use simdnoise::NoiseBuilder;
use std::cmp::min;

/// Density map generator which simply uses a height map
/// using two-dimensional Perlin noise.
#[derive(Debug, Default)]
pub struct HeightMapGenerator;

impl DensityMapGenerator for HeightMapGenerator {
    fn generate_for_chunk(
        &self,
        chunk: ChunkPosition,
        biomes: &NearbyBiomes,
        seed: u64,
    ) -> BitVec<LocalBits, u8> {
        let x_offset = (chunk.x * 16) as f32;
        let y_offset = (chunk.z * 16) as f32;

        let dim = 16;
        let (elevation, _, _) = NoiseBuilder::gradient_2d_offset(x_offset, dim, y_offset, dim)
            .with_seed(seed as i32)
            .with_freq(0.01)
            .generate();
        let (detail, _, _) = NoiseBuilder::gradient_2d_offset(x_offset, dim, y_offset, dim)
            .with_seed(seed as i32 + 1)
            .generate();

        let mut density_map = BitVec::from_vec(vec![0u8; 16 * 256 * 16 / 8]);
        for x in 0..16 {
            for z in 0..16 {
                let biome = biomes.get_at_block(x, 0, z);
                let index = (z << 4) | x;
                let mut elevation = elevation[index].abs() * 400.0;
                let detail = detail[index] * 50.0;

                if biome == Biome::Ocean {
                    elevation -= OCEAN_DEPTH as f32;
                }

                let height = (elevation + detail + 64.0) as usize;

                for y in 0..min(height, SKY_LIMIT) {
                    density_map.set(block_index(x, y, z), true);
                }
            }
        }

        density_map
    }
}
