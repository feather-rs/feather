//! World generation for Feather.

use crate::biome::BiomeGenerator;

pub mod biome;
pub mod noise;
pub mod util;

/// Defines a world generation pipeline.
///
/// World generation is based on a number of steps:
/// * Biome generation, which generates a biome grid
/// defining the biome at each column of blocks in the chunk.
///
/// * Shape generation, which generates the shape of chunks.
/// This step usually uses Perlin noise to generate density values,
/// using those values to decide whether a block is solid or not.
/// The output of this step is a bit array with bits set for
/// blocks which should be solid.
///
/// * Composition, which takes in the generated shape and biome
/// and turns the binary solid/air grid into actual blocks based
/// on the local conditions.
///
/// * Finishers, which add additional structures on top of the
/// generated chunk: e.g. grass, trees, buildings, caves.
pub struct WorldGenerator {
    biome_generator: Box<dyn BiomeGenerator>,
}
