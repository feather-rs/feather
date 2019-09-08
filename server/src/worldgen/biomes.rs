//! Biome grid creation.

use crate::worldgen::voronoi::VoronoiGrid;
use crate::worldgen::{BiomeGenerator, ChunkBiomes};
use feather_core::{Biome, ChunkPosition};
use num_traits::FromPrimitive;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use simdnoise::NoiseBuilder;
use strum::EnumCount;

/// Biome grid generator based on a two-level
/// distorted Voronoi map.
///
/// TODO: this is not two-level, only one-level
#[derive(Default)]
pub struct TwoLevelBiomeGenerator;

impl BiomeGenerator for TwoLevelBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition) -> ChunkBiomes {
        // TODO: load seed
        let seed = 8344;

        let mut voronoi = VoronoiGrid::new(512, seed);

        let mut biomes = ChunkBiomes::from_array([Biome::Plains; 16 * 16]); // Will be overriden

        // Noise is used to distort each coordinate.
        let x_noise =
            NoiseBuilder::gradient_2d_offset(chunk.x as f32 * 16.0, 16, chunk.z as f32 * 16.0, 16)
                .with_seed(seed as i32 + 1)
                .generate_scaled(-4.0, 4.0);
        let z_noise =
            NoiseBuilder::gradient_2d_offset(chunk.x as f32 * 16.0, 16, chunk.z as f32 * 16.0, 16)
                .with_seed(seed as i32 + 2)
                .generate_scaled(-4.0, 4.0);

        for x in 0..16 {
            for z in 0..16 {
                // Apply distortion to coordinate before passing to voronoi
                // generator.
                let distort_x = x_noise[(z << 4) | x] as i32 * 8;
                let distort_z = z_noise[(z << 4) | x] as i32 * 8;

                let (closest_x, closest_y) = voronoi.get(
                    (chunk.x * 16) + x as i32 + distort_x,
                    (chunk.z * 16) + z as i32 + distort_z,
                );

                // Shift around the closest_x and closest_y values
                // and deterministically select a biome based on the
                // computed value.
                let shifted = {
                    let combined = (i64::from(closest_x) << 32) | i64::from(closest_y);

                    let mut rng = XorShiftRng::seed_from_u64(combined as u64);
                    rng.gen::<u64>()
                };

                let biome = Biome::from_u64(shifted % Biome::count() as u64).unwrap();
                biomes.set_biome_at(x, z, biome);
            }
        }

        biomes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_all_plains() {
        // Check that the `ChunkBiomes` was overridden correctly.
        let gen = TwoLevelBiomeGenerator::default();

        let chunk = ChunkPosition::new(5433, 132);

        let biomes = gen.generate_for_chunk(chunk);

        println!("{:?}", biomes);

        let mut num_plains = 0;
        for x in 0..16 {
            for z in 0..16 {
                if biomes.biome_at(x, z) == Biome::Plains {
                    num_plains += 1;
                }
            }
        }

        assert_ne!(num_plains, 16 * 16);
    }

    #[test]
    fn test_deterministic() {
        // Check that the result is always deterministic.
        let gen = TwoLevelBiomeGenerator::default();

        let chunk = ChunkPosition::new(0, 0);

        let first = gen.generate_for_chunk(chunk);

        for _ in 0..5 {
            let next = gen.generate_for_chunk(chunk);

            for x in 0..16 {
                for z in 0..16 {
                    assert_eq!(first.biome_at(x, z), next.biome_at(x, z));
                }
            }
        }
    }
}
