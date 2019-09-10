//! Implements a density map generator using 3D Perlin noise.
//!
//! Over the 2D height map generator, this has the advantage that terrain
//! is more interesting; overhangs and the like will be able to generate.

use crate::worldgen::{ChunkBiomes, DensityMapGenerator};
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
/// on the biome.
/// *
#[derive(Debug, Default)]
pub struct DensityMapGeneratorImpl;

impl DensityMapGenerator for DensityMapGeneratorImpl {
    fn generate_for_chunk(&self, chunk: ChunkPosition, biomes: &ChunkBiomes, seed: u64) -> BitVec {
        let mut density = bitvec![0; 16 * 256 * 16];
    }
}
