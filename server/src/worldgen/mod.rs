//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

use feather_core::{Biome, Chunk, ChunkPosition};

mod biomes;
mod composition;
mod density_map;
pub mod noise;
mod superflat;
mod util;
pub mod voronoi;

pub use biomes::{DistortedVoronoiBiomeGenerator, TwoLevelBiomeGenerator};
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
pub use composition::BasicCompositionGenerator;
pub use density_map::{DensityMapGeneratorImpl, HeightMapGenerator};
pub use noise::Wrapped3DPerlinNoise;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::fmt;
pub use superflat::SuperflatWorldGenerator;

/// Sea-level height.
pub const SEA_LEVEL: usize = 64;
/// Sky limit.
pub const SKY_LIMIT: usize = 255;
/// Depth of an ocean.
const OCEAN_DEPTH: usize = 30;

pub trait WorldGenerator: Send + Sync {
    /// Generates the chunk at the given position.
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk;
}

pub struct EmptyWorldGenerator {}

impl WorldGenerator for EmptyWorldGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        Chunk::new(position)
    }
}

/// A "composable" world generator.
///
/// This generator will generate the world based
/// on a pipeline, and each step in the pipeline passes
/// data to the next stage.
///
/// The pipeline stages are as follows:
/// * Biomes - generates a biome grid.
/// * Terrain density - generates the terrain density values using Perlin noise.
/// * Terrain composition - sets the correct block types based on the biome and terrain density.
///
/// This generator is based on [this document](http://cuberite.xoft.cz/docs/Generator.html).
pub struct ComposableGenerator {
    /// The biome generator.
    biome: Box<dyn BiomeGenerator>,
    /// The height map generator.
    density_map: Box<dyn DensityMapGenerator>,
    /// The composition generator.
    composition: Box<dyn CompositionGenerator>,
    /// The world seed.
    seed: u64,
}

impl ComposableGenerator {
    /// Creates a new `ComposableGenerator` with the given stages.
    pub fn new<B, D, C>(biome: B, density_map: D, composition: C, seed: u64) -> Self
    where
        B: BiomeGenerator + 'static,
        D: DensityMapGenerator + 'static,
        C: CompositionGenerator + 'static,
    {
        Self {
            biome: Box::new(biome),
            density_map: Box::new(density_map),
            composition: Box::new(composition),
            seed,
        }
    }
}

impl WorldGenerator for ComposableGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let mut seed_shuffler = XorShiftRng::seed_from_u64(self.seed);

        let biomes = self.biome.generate_for_chunk(position, seed_shuffler.gen());

        let density_map =
            self.density_map
                .generate_for_chunk(position, &biomes, seed_shuffler.gen());

        let mut chunk = Chunk::new(position);

        for x in 0..16 {
            for z in 0..16 {
                chunk.set_biome_at(x, z, biomes.biome_at(x, z));
            }
        }

        self.composition.generate_for_chunk(
            &mut chunk,
            position,
            &biomes,
            density_map.as_bitslice(),
            seed_shuffler.gen(),
        );

        // TODO: correct lighting.
        // Fill chunk with 15 light levels.
        chunk
            .sections_mut()
            .into_iter()
            .filter(|section| section.is_some())
            .map(|section| section.unwrap())
            .for_each(|section| {
                let sky_light = section.sky_light_mut();

                (0..4096).for_each(|index| {
                    sky_light.set(index, 15);
                })
            });

        chunk
    }
}

/// A generator which generates the biome grid for a `ComposableGenerator`.
pub trait BiomeGenerator: Send + Sync {
    /// Generates the biomes for a given chunk.
    /// This function should be deterministic.
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> ChunkBiomes;
}

/// A generator which generates the density map for a chunk.
/// Used in the `ComposableGenerator` pipeline.
pub trait DensityMapGenerator: Send + Sync {
    /// Generates the density map for a given chunk.
    /// A compact array of booleans is returned, indexable
    /// by (y << 8) | (x << 4) | z. Those set to `true` will
    /// contain solid blacks; those set to `false` will be air.
    fn generate_for_chunk(&self, chunk: ChunkPosition, biomes: &ChunkBiomes, seed: u64) -> BitVec;
}

/// A generator which populates the given chunk using blocks
/// based on the given density map and biomes.
pub trait CompositionGenerator: Send + Sync {
    /// Populates the given chunk with blocks based on the given
    /// biomes and density map.
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        pos: ChunkPosition,
        biomes: &ChunkBiomes,
        density: &BitSlice,
        seed: u64,
    );
}

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: usize, z: usize) -> usize {
    assert!(x < 16 && y < 256 && z < 16);
    (y << 8) | (x << 4) | z
}

/// Represents the biomes of a chunk.
pub struct ChunkBiomes {
    /// 2D array of biome values. The biome for a given
    /// column local to the chunk can be indexed using
    /// (x << 4) | z.
    biomes: [Biome; 16 * 16],
}

impl ChunkBiomes {
    /// Creates a `ChunkBiomes` wrapping the given array of biomes.
    #[inline]
    pub fn from_array(biomes: [Biome; 16 * 16]) -> Self {
        Self { biomes }
    }

    /// Gets the biome for the given column index.
    ///
    /// # Panics
    /// Panics if `x >= 16 | z >= 16`.
    pub fn biome_at(&self, x: usize, z: usize) -> Biome {
        assert!(x < 16 && z < 16);

        let index = Self::index(x, z);
        self.biomes[index]
    }

    /// Sets the biome for the given column index.
    ///
    /// # Panics
    /// Panics if `x >= 16 | z >= 16`.
    pub fn set_biome_at(&mut self, x: usize, z: usize, biome: Biome) {
        assert!(x < 16 && z < 16);

        let index = Self::index(x, z);
        self.biomes[index] = biome;
    }

    fn index(x: usize, z: usize) -> usize {
        (x << 4) | z
    }
}

impl fmt::Debug for ChunkBiomes {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for i in 0..256 {
            write!(f, "{:?}, ", self.biomes[i])?;
        }

        Ok(())
    }
}

/// A biome generator which always generates plains.
#[derive(Debug, Default)]
pub struct StaticBiomeGenerator;

impl BiomeGenerator for StaticBiomeGenerator {
    fn generate_for_chunk(&self, _chunk: ChunkPosition, _seed: u64) -> ChunkBiomes {
        ChunkBiomes::from_array([Biome::Plains; 16 * 16])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_worldgen_empty() {
        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = EmptyWorldGenerator {};
        let chunk = generator.generate_chunk(chunk_pos);

        // No sections have been generated
        assert!(chunk.sections().iter().all(|sec| sec.is_none()));
        assert_eq!(chunk_pos, chunk.position());
    }

    #[test]
    fn test_chunk_biomes() {
        let biomes = [Biome::Plains; 16 * 16];

        let mut biomes = ChunkBiomes::from_array(biomes);

        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(biomes.biome_at(x, z), Biome::Plains);
                biomes.set_biome_at(x, z, Biome::Ocean);
                assert_eq!(biomes.biome_at(x, z), Biome::Ocean);
            }
        }
    }

    #[test]
    fn test_static_biome_generator() {
        let gen = StaticBiomeGenerator::default();

        let biomes = gen.generate_for_chunk(ChunkPosition::new(0, 0), 0);

        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(biomes.biome_at(x, z), Biome::Plains);
            }
        }
    }
}
