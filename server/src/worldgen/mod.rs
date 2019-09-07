//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

use feather_core::{Biome, Chunk, ChunkPosition};

mod composition;
mod height_map;
mod superflat;

use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
pub use superflat::SuperflatWorldGenerator;

/// Sea-level height.
pub const SEA_LEVEL: usize = 64;

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
/// * Terrain height - generates the terrain height using Perlin noise.
/// * Terrain composition - sets the correct block types based on the biome and terrain height.
///
/// This generator is based on [this document](http://cuberite.xoft.cz/docs/Generator.html).
pub struct ComposableGenerator {
    /// The biome generator.
    biome: Box<dyn BiomeGenerator>,
    /// The height map generator.
    height_map: Box<dyn HeightMapGenerator>,
    /// The composition generator.
    composition: Box<dyn CompositionGenerator>,
}

impl ComposableGenerator {
    /// Creates a new `ComposableGenerator` with the given stages.
    pub fn new<
        B: BiomeGenerator + 'static,
        H: HeightMapGenerator + 'static,
        C: CompositionGenerator + 'static,
    >(
        biome: B,
        height_map: H,
        composition: C,
    ) -> Self {
        Self {
            biome: Box::new(biome),
            height_map: Box::new(height_map),
            composition: Box::new(composition),
        }
    }
}

impl WorldGenerator for ComposableGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let biomes = self.biome.generate_for_chunk(position);
        let density_map = self.height_map.generate_for_chunk(position, &biomes);

        let mut chunk = Chunk::new(position);
        self.composition.generate_for_chunk(
            &mut chunk,
            position,
            &biomes,
            density_map.as_bitslice(),
        );

        chunk
    }
}

/// A generator which generates the biome grid for a `ComposableGenerator`.
pub trait BiomeGenerator: Send + Sync {
    /// Generates the biomes for a given chunk.
    /// This function should be deterministic.
    fn generate_for_chunk(&self, chunk: ChunkPosition) -> ChunkBiomes;
}

/// A generator which generates the height map for a chunk.
/// Used in the `ComposableGenerator` pipeline.
pub trait HeightMapGenerator: Send + Sync {
    /// Generates the height map for a given chunk.
    /// A compact array of booleans is returned, indexable
    /// by ((x << 8) | (y << 4)) | z. Those set to `true` will
    /// contain solid blacks; those set to `false` will be air.
    fn generate_for_chunk(&self, chunk: ChunkPosition, biomes: &ChunkBiomes) -> BitVec;
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
    );
}

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: usize, z: usize) -> usize {
    ((x << 8) | y << 4) | z
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
    pub fn biome_at(&self, x: usize, y: usize) -> Biome {
        assert!(x < 16 && y < 16);

        let index = Self::index(x, y);
        self.biomes[index]
    }

    /// Sets the biome for the given column index.
    ///
    /// # Panics
    /// Panics if `x >= 16 | z >= 16`.
    pub fn set_biome_at(&mut self, x: usize, y: usize, biome: Biome) {
        assert!(x < 16 && y < 16);

        let index = Self::index(x, y);
        self.biomes[index] = biome;
    }

    fn index(x: usize, y: usize) -> usize {
        (x << 4) | y
    }
}

/// A biome generator which always generates plains.
#[derive(Debug, Default)]
pub struct StaticBiomeGenerator;

impl BiomeGenerator for StaticBiomeGenerator {
    fn generate_for_chunk(&self, _chunk: ChunkPosition) -> ChunkBiomes {
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

        let biomes = gen.generate_for_chunk(ChunkPosition::new(0, 0));

        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(biomes.biome_at(x, z), Biome::Plains);
            }
        }
    }
}
