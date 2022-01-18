#![forbid(unsafe_code)]

//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

mod biomes;
mod composition;
mod density_map;
mod finishers;
pub mod noise;
mod superflat;
mod util;
pub mod voronoi;

use base::chunk::BiomeStore;
use base::{Biome, BlockId, Chunk, ChunkPosition};
pub use biomes::{DistortedVoronoiBiomeGenerator, TwoLevelBiomeGenerator};
use bitvec::vec::BitVec;
use bitvec::{order::LocalBits, slice::BitSlice};
pub use composition::BasicCompositionGenerator;
pub use density_map::{DensityMapGeneratorImpl, HeightMapGenerator};
use finishers::{ClumpedFoliageFinisher, SingleFoliageFinisher, SnowFinisher};
pub use noise::NoiseLerper;
use num_traits::ToPrimitive;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use smallvec::SmallVec;
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

pub struct VoidWorldGenerator;

impl WorldGenerator for VoidWorldGenerator {
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
/// * Finishing generators - generates final elements, such as grass, snow, and trees.
///
/// This generator is based on [this document](http://cuberite.xoft.cz/docs/Generator.html).
pub struct ComposableGenerator {
    /// The biome generator.
    biome: Box<dyn BiomeGenerator>,
    /// The height map generator.
    density_map: Box<dyn DensityMapGenerator>,
    /// The composition generator.
    composition: Box<dyn CompositionGenerator>,
    /// A vector of finishing generators used
    /// by this composable generator.
    finishers: SmallVec<[Box<dyn FinishingGenerator>; 8]>,
    /// The world seed.
    seed: u64,
}

impl ComposableGenerator {
    /// Creates a new `ComposableGenerator` with the given stages.
    pub fn new<B, D, C, F>(
        biome: B,
        density_map: D,
        composition: C,
        finishers: F,
        seed: u64,
    ) -> Self
    where
        B: BiomeGenerator + 'static,
        D: DensityMapGenerator + 'static,
        C: CompositionGenerator + 'static,
        F: IntoIterator<Item = Box<dyn FinishingGenerator>>,
    {
        Self {
            biome: Box::new(biome),
            density_map: Box::new(density_map),
            composition: Box::new(composition),
            finishers: finishers.into_iter().collect(),
            seed,
        }
    }

    /// A default composable generator, used
    /// for worlds with "default" world type.
    pub fn default_with_seed(seed: u64) -> Self {
        let finishers: Vec<Box<dyn FinishingGenerator>> = vec![
            Box::new(SnowFinisher::default()),
            Box::new(SingleFoliageFinisher::default()),
            Box::new(ClumpedFoliageFinisher::default()),
        ];
        Self::new(
            TwoLevelBiomeGenerator::default(),
            DensityMapGeneratorImpl::default(),
            BasicCompositionGenerator::default(),
            finishers,
            seed,
        )
    }
}

impl WorldGenerator for ComposableGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let mut seed_shuffler = XorShiftRng::seed_from_u64(self.seed);

        // Generate biomes for 3x3 grid of chunks around current chunk.
        let biome_seed = seed_shuffler.gen();

        let mut biomes = vec![];

        for z in -1..=1 {
            for x in -1..=1 {
                let pos = ChunkPosition::new(position.x + x, position.z + z);
                biomes.push(self.biome.generate_for_chunk(pos, biome_seed));
            }
        }
        let biomes = NearbyBiomes::from_slice(&biomes[..]).unwrap();

        let density_map =
            self.density_map
                .generate_for_chunk(position, &biomes, seed_shuffler.gen());

        let mut chunk = Chunk::new(position);
        *chunk.biomes_mut() = *biomes.center();

        self.composition.generate_for_chunk(
            &mut chunk,
            position,
            &biomes.biome_stores[4], // Center chunk
            density_map.as_bitslice(),
            seed_shuffler.gen(),
        );

        // Calculate top blocks in chunk.
        // TODO: perhaps this should be moved to `Chunk`?
        let mut top_blocks = TopBlocks::new();
        for x in 0..16 {
            for z in 0..16 {
                for y in (0..256).rev() {
                    if chunk.block_at(x, y, z).unwrap() != BlockId::air() {
                        top_blocks.set_top_block_at(x, z, y);
                        break;
                    }
                }
            }
        }

        chunk.recalculate_heightmaps();

        // Finishers.
        for finisher in &self.finishers {
            finisher.generate_for_chunk(
                &mut chunk,
                &biomes.biome_stores[4],
                &top_blocks,
                seed_shuffler.gen(),
            );
        }

        chunk
    }
}

/// A generator which generates the biome grid for a `ComposableGenerator`.
pub trait BiomeGenerator: Send + Sync {
    /// Generates the biomes for a given chunk.
    /// This function should be deterministic.
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> BiomeStore;
}

/// A generator which generates the density map for a chunk.
/// Used in the `ComposableGenerator` pipeline.
pub trait DensityMapGenerator: Send + Sync {
    /// Generates the density map for a given chunk.
    /// A compact array of booleans is returned, indexable
    /// by (y << 8) | (x << 4) | z. Those set to `true` will
    /// contain solid blacks; those set to `false` will be air.
    fn generate_for_chunk(
        &self,
        chunk: ChunkPosition,
        biomes: &NearbyBiomes,
        seed: u64,
    ) -> BitVec<LocalBits, u8>;
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
        biomes: &BiomeStore,
        density: &BitSlice<LocalBits, u8>,
        seed: u64,
    );
}

/// A generator, run after composition,
/// which can add finishing elements to chunks,
/// such as grass, trees, and snow.
pub trait FinishingGenerator: Send + Sync {
    /// Populates the given chunk with any
    /// finishing blocks.
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &BiomeStore,
        top_blocks: &TopBlocks,
        seed: u64,
    );
}

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: usize, z: usize) -> usize {
    assert!(x < 16 && y < 256 && z < 16);
    (y << 8) | (x << 4) | z
}

/// Represents the highest solid blocks in a chunk.
#[derive(Default)]
pub struct TopBlocks {
    top_blocks: Vec<u8>,
}

impl TopBlocks {
    pub fn new() -> Self {
        Self {
            top_blocks: vec![0; 16 * 16],
        }
    }

    /// Fetches the highest solid blocks for the
    /// given column coordinates (chunk-local).
    pub fn top_block_at(&self, x: usize, z: usize) -> usize {
        self.top_blocks[x + (z << 4)] as usize
    }

    pub fn set_top_block_at(&mut self, x: usize, z: usize, top: usize) {
        self.top_blocks[x + (z << 4)] = top as u8;
    }
}
/// Represents the biomes in a 3x3 grid of chunks,
/// centered on the chunk currently being generated.
pub struct NearbyBiomes {
    /// 2D array of chunk biomes. The chunk biomes
    /// for a given chunk position relative to the center
    /// chunk can be obtained using (x + 1) + (z + 1) * 3.
    pub biome_stores: [BiomeStore; 3 * 3],
}

impl NearbyBiomes {
    pub fn from_slice(biome_store_slice: &[BiomeStore]) -> Option<Self> {
        let mut biome_stores = [BiomeStore::new(Biome::Badlands); 9];
        if biome_store_slice.len() != biome_stores.len() {
            return None;
        }

        biome_stores.clone_from_slice(biome_store_slice);
        Some(Self { biome_stores })
    }

    /// Gets the biome at the given coordinates.
    ///
    /// # Panics
    /// Panics if `x >= 16`, `z >= 16`, or `y >= 256`.
    pub fn get_at_block<N: ToPrimitive>(&self, x: N, y: N, z: N) -> Biome {
        let (index, local_x, local_y, local_z) = self.index(x, y, z);

        self.biome_stores[index].get_at_block(local_x, local_y, local_z)
    }

    /// Gets the biome at the given coordinates, in multiples
    /// of 4 blocks.
    ///
    /// # Panics
    /// Panics if `x >= 4`, `z >= 4`, or `y >= 64`.
    pub fn get<N: ToPrimitive>(&self, x: N, y: N, z: N) -> Biome {
        let (index, local_x, local_y, local_z) = self.index(
            x.to_isize().unwrap() * 4,
            y.to_isize().unwrap() * 4,
            z.to_isize().unwrap() * 4,
        );

        self.biome_stores[index].get(local_x / 4, local_y / 4, local_z / 4)
    }

    /// Sets the biome at the given coordinates, in multiples
    /// of 4 blocks.
    ///
    /// # Panics
    /// Panics if `x >= 4`, `z >= 4`, or `y >= 64`.
    pub fn set<N: ToPrimitive>(&mut self, x: N, y: N, z: N, biome: Biome) {
        let (index, local_x, local_y, local_z) = self.index(
            x.to_isize().unwrap() * 4,
            y.to_isize().unwrap() * 4,
            z.to_isize().unwrap() * 4,
        );

        self.biome_stores[index].set(local_x / 4, local_y / 4, local_z / 4, biome);
    }
    pub fn center(&self) -> &BiomeStore {
        &self.biome_stores[4]
    }
    pub fn center_mut(&mut self) -> &mut BiomeStore {
        &mut self.biome_stores[4]
    }
    /// Returns a tuple of (chunk_index, local_x, local_y, local_z)
    fn index<N: ToPrimitive>(&self, ox: N, oy: N, oz: N) -> (usize, usize, usize, usize) {
        // FIXME: Does this function need to so complicated?
        let ox = ox.to_isize().unwrap();
        let oy = oy.to_isize().unwrap();
        let oz = oz.to_isize().unwrap();

        let x = ox + 16;
        let z = oz + 16;

        let chunk_x = (x / 16) as usize;
        let chunk_z = (z / 16) as usize;

        let mut local_x = (ox % 16).abs() as usize;
        let local_y = (oy % 16).abs() as usize;
        let mut local_z = (oz % 16).abs() as usize;

        if ox < 0 {
            local_x = 16 - local_x;
        }
        if oz < 0 {
            local_z = 16 - local_z;
        }

        (chunk_x + chunk_z * 3, local_x, local_y, local_z)
    }
}
/// A biome generator which always generates plains.
#[derive(Debug, Default)]
pub struct StaticBiomeGenerator;

impl BiomeGenerator for StaticBiomeGenerator {
    fn generate_for_chunk(&self, _chunk: ChunkPosition, _seed: u64) -> BiomeStore {
        BiomeStore::new(Biome::Plains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reproducability() {
        let seeds: [u64; 4] = [std::u64::MAX, 3243, 0, 100];

        let chunks = [
            ChunkPosition::new(0, 0),
            ChunkPosition::new(-1, -1),
            ChunkPosition::new(1, 1),
        ];

        for seed in seeds.iter() {
            let gen = ComposableGenerator::default_with_seed(*seed);
            for chunk in chunks.iter() {
                let first = gen.generate_chunk(*chunk);

                let second = gen.generate_chunk(*chunk);

                test_chunks_eq(&first, &second);
            }
        }
    }

    fn test_chunks_eq(a: &Chunk, b: &Chunk) {
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..256 {
                    assert_eq!(a.block_at(x, y, z), b.block_at(x, y, z));
                }
            }
        }
        for x in 0..4 {
            for z in 0..4 {
                for y in 0..64 {
                    assert_eq!(a.biomes().get(x, y, z), b.biomes().get(x, y, z));
                }
            }
        }
    }

    #[test]
    pub fn test_worldgen_void() {
        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = VoidWorldGenerator;
        let chunk = generator.generate_chunk(chunk_pos);

        // No sections have been generated
        assert!(chunk.sections().iter().all(|sec| sec.is_none()));
        assert_eq!(chunk_pos, chunk.position());
    }

    #[test]
    fn test_chunk_biomes() {
        let mut biomes = BiomeStore::new(Biome::Plains);

        for x in 0..4 {
            for z in 0..4 {
                for y in 0..64 {
                    assert_eq!(biomes.get(x, y, z), Biome::Plains);
                    biomes.set(x, y, z, Biome::Ocean);
                    assert_eq!(biomes.get(x, y, z), Biome::Ocean);
                }
            }
        }
    }

    #[test]
    fn test_static_biome_generator() {
        let gen = StaticBiomeGenerator::default();

        let biomes = gen.generate_for_chunk(ChunkPosition::new(0, 0), 0);

        for x in 0..4 {
            for z in 0..4 {
                for y in 0..64 {
                    assert_eq!(biomes.get(x, y, z), Biome::Plains);
                }
            }
        }
    }

    #[test]
    fn test_nearby_biomes() {
        let biomes = vec![
            BiomeStore::new(Biome::Plains),
            BiomeStore::new(Biome::Swamp),
            BiomeStore::new(Biome::Savanna),
            BiomeStore::new(Biome::BirchForest),
            BiomeStore::new(Biome::DarkForest),
            BiomeStore::new(Biome::Mountains),
            BiomeStore::new(Biome::Ocean),
            BiomeStore::new(Biome::Desert),
            BiomeStore::new(Biome::Taiga),
        ];
        let biomes = NearbyBiomes::from_slice(&biomes[..]).unwrap();

        assert_eq!(biomes.get_at_block(0, 0, 0), Biome::DarkForest);
        assert_eq!(biomes.get_at_block(16, 0, 16), Biome::Taiga);
        assert_eq!(biomes.get_at_block(-1, 0, -1), Biome::Plains);
        assert_eq!(biomes.get_at_block(-1, 0, 0), Biome::BirchForest);
    }
}
