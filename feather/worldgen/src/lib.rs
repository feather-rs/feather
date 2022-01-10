#![forbid(unsafe_code)]

//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

use base::biome::BiomeList;
pub use superflat::SuperflatWorldGenerator;

use base::chunk::Chunk;
use base::world::{Sections, WorldHeight};
use base::ChunkPosition;
mod superflat;

pub trait WorldGenerator: Send + Sync {
    /// Generates the chunk at the given position.
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        biomes: &BiomeList,
    ) -> Chunk;
}

pub struct EmptyWorldGenerator {}

impl WorldGenerator for EmptyWorldGenerator {
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        _biomes: &BiomeList,
    ) -> Chunk {
        Chunk::new(position, sections, min_y / 16)
    }
}

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: i32, z: usize, world_height: WorldHeight, min_y: i32) -> usize {
    assert!(x < 16 && y >= min_y && y < min_y + *world_height as i32 && z < 16);
    (((y - min_y) as usize) << 8) | (x << 4) | z
}

#[cfg(test)]
mod tests {
    use crate::base::chunk::{BIOME_SAMPLE_RATE, SECTION_HEIGHT, SECTION_WIDTH};
    use base::chunk::{BIOME_SAMPLE_RATE, SECTION_HEIGHT, SECTION_WIDTH};
    use base::CHUNK_WIDTH;

    use super::*;

    #[test]
    fn test_reproducibility() {
        let seeds: [u64; 4] = [u64::MAX, 3243, 0, 100];

        let chunks = [
            ChunkPosition::new(0, 0),
            ChunkPosition::new(-1, -1),
            ChunkPosition::new(1, 1),
        ];

        for seed in seeds.iter() {
            let gen = ComposableGenerator::default_with_seed(*seed);
            for chunk in chunks.iter() {
                let first = gen.generate_chunk(*chunk, Sections(16));

                let second = gen.generate_chunk(*chunk, Sections(16));

                test_chunks_eq(&first, &second);
            }
        }
    }

    fn test_chunks_eq(a: &Chunk, b: &Chunk) {
        assert_eq!(a.sections().len(), b.sections().len());
        for x in 0..SECTION_WIDTH {
            for z in 0..SECTION_WIDTH {
                for y in 0..a.sections().len() * SECTION_HEIGHT {
                    assert_eq!(a.block_at(x, y, z), b.block_at(x, y, z));
                }
            }
        }
        for x in 0..(CHUNK_WIDTH / BIOME_SAMPLE_RATE) {
            for z in 0..(CHUNK_WIDTH / BIOME_SAMPLE_RATE) {
                for y in 0..(a.sections().len() - 2) * (SECTION_HEIGHT / BIOME_SAMPLE_RATE) {
                    assert_eq!(a.biomes().get(x, y, z), b.biomes().get(x, y, z));
                }
            }
        }
    }

    #[test]
    pub fn test_worldgen_empty() {
        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = EmptyWorldGenerator {};
        let chunk = generator.generate_chunk(chunk_pos, Sections(16));

        // No sections have been generated
        assert!(chunk.sections().iter().all(|sec| sec.is_none()));
        assert_eq!(chunk_pos, chunk.position());
    }

    #[test]
    fn test_chunk_biomes() {
        let mut biomes = BiomeStore::new(BiomeId::Plains, Sections(16));

        for x in 0..BIOME_SAMPLE_RATE {
            for z in 0..BIOME_SAMPLE_RATE {
                for y in 0..16 * BIOME_SAMPLE_RATE {
                    assert_eq!(biomes.get(x, y, z), BiomeId::Plains);
                    biomes.set(x, y, z, BiomeId::TheVoid);
                    assert_eq!(biomes.get(x, y, z), BiomeId::TheVoid);
                }
            }
        }
    }

    #[test]
    fn test_static_biome_generator() {
        let gen = StaticBiomeGenerator::default();

        let biomes = gen.generate_for_chunk(ChunkPosition::new(0, 0), 0, Sections(16));

        for x in 0..BIOME_SAMPLE_RATE {
            for z in 0..BIOME_SAMPLE_RATE {
                for y in 0..16 * BIOME_SAMPLE_RATE {
                    assert_eq!(biomes.get(x, y, z), BiomeId::TheVoid);
                }
            }
        }
    }

    #[test]
    fn test_nearby_biomes() {
        let biomes = vec![
            BiomeStore::new(BiomeId::Plains, Sections(16)),
            BiomeStore::new(BiomeId::Swamp, Sections(16)),
            BiomeStore::new(BiomeId::Savanna, Sections(16)),
            BiomeStore::new(BiomeId::BirchForest, Sections(16)),
            BiomeStore::new(BiomeId::DarkForest, Sections(16)),
            BiomeStore::new(BiomeId::Mountains, Sections(16)),
            BiomeStore::new(BiomeId::TheVoid, Sections(16)),
            BiomeStore::new(BiomeId::Desert, Sections(16)),
            BiomeStore::new(BiomeId::Taiga, Sections(16)),
        ];
        let biomes = NearbyBiomes::from_slice(&biomes[..]).unwrap();

        assert_eq!(biomes.get_at_block(0, 0, 0), BiomeId::DarkForest);
        assert_eq!(biomes.get_at_block(16, 0, 16), BiomeId::Taiga);
        assert_eq!(biomes.get_at_block(-1, 0, -1), BiomeId::Plains);
        assert_eq!(biomes.get_at_block(-1, 0, 0), BiomeId::BirchForest);
    }
}
