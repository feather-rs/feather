//! Implements a density map generator using 3D Perlin noise.
//!
//! Over the 2D height map generator, this has the advantage that terrain
//! is more interesting; overhangs and the like will be able to generate.

use crate::worldgen::{
    block_index, noise, ChunkBiomes, DensityMapGenerator, Wrapped3DPerlinNoise, SEA_LEVEL,
};
use bitvec::vec::BitVec;
use feather_core::ChunkPosition;

/// A density map generator using 3D Perlin noise.
///
/// This generator should be used over the height map generator
/// when seeking correct-looking worlds;
///
/// # Implementation
/// Density calculation works as follows:
/// * Generate a base 3D Perlin nosie with settings depending
/// on the biome. Use linear interpolation on noise (this
/// is handled by `Wrapped3DPerlinNoise`)`.
/// * Depending on the density value from the noise, decide
/// whether the position is solid or air.
#[derive(Debug, Default)]
pub struct DensityMapGeneratorImpl;

impl DensityMapGenerator for DensityMapGeneratorImpl {
    fn generate_for_chunk(&self, chunk: ChunkPosition, _biomes: &ChunkBiomes, seed: u64) -> BitVec {
        let mut density = bitvec![0; 16 * 256 * 16];

        let noise = Wrapped3DPerlinNoise::new(seed)
            .with_offset(chunk.x, chunk.z)
            .with_amplitude(400.0)
            .with_frequency(0.1)
            .generate();

        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    let mut value = noise[noise::index(x, y, z)];

                    //value += (SEA_LEVEL as i32 - y as i32) as f32;

                    let is_solid = value > 0.0;
                    let index = block_index(x, y, z);
                    density.set(index, is_solid);
                }
            }
        }

        density
    }
}
