//! Biome grid creation.

use crate::worldgen::voronoi::VoronoiGrid;
use crate::worldgen::{BiomeGenerator, ChunkBiomes};
use feather_core::{Biome, ChunkPosition};
use num_traits::FromPrimitive;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use strum::EnumCount;

/// Biome grid generator based on a distorted Voronoi
/// noise.
#[derive(Default)]
pub struct DistortedVoronoiBiomeGenerator;

impl BiomeGenerator for DistortedVoronoiBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> ChunkBiomes {
        let mut voronoi = VoronoiGrid::new(384, seed);

        let mut biomes = ChunkBiomes::from_array([Biome::Plains; 16 * 16]); // Will be overriden

        // Noise is used to distort each coordinate.
        /*let x_noise =
            NoiseBuilder::gradient_2d_offset(chunk.x as f32 * 16.0, 16, chunk.z as f32 * 16.0, 16)
                .with_seed(seed as i32 + 1)
                .generate_scaled(-4.0, 4.0);
        let z_noise =
            NoiseBuilder::gradient_2d_offset(chunk.x as f32 * 16.0, 16, chunk.z as f32 * 16.0, 16)
                .with_seed(seed as i32 + 2)
                .generate_scaled(-4.0, 4.0);*/

        for x in 0..16 {
            for z in 0..16 {
                // Apply distortion to coordinate before passing to voronoi
                // generator.
                //let distort_x = x_noise[(z << 4) | x] as i32 * 8;
                //let distort_z = z_noise[(z << 4) | x] as i32 * 8;

                let distort_x = 0;
                let distort_z = 0;

                let (closest_x, closest_y) = voronoi.get(
                    (chunk.x * 16) + x as i32 + distort_x,
                    (chunk.z * 16) + z as i32 + distort_z,
                );

                // Shift around the closest_x and closest_y values
                // and deterministically select a biome based on the
                // computed value. Continue shifting the value until
                // a valid biome is computed.
                let combined = (i64::from(closest_x) << 32) | i64::from(closest_y);
                let mut rng = XorShiftRng::seed_from_u64(combined as u64);

                loop {
                    let shifted: u64 = rng.gen();

                    let biome = Biome::from_u64(shifted % Biome::count() as u64).unwrap();
                    if is_biome_allowed(biome) {
                        biomes.set_biome_at(x, z, biome);
                        break;
                    }
                }
            }
        }

        biomes
    }
}

/// Returns whether the given biome is allowed in the overworld.
fn is_biome_allowed(biome: Biome) -> bool {
    match biome {
        Biome::TheEnd
        | Biome::TheVoid
        | Biome::Nether
        | Biome::SmallEndIslands
        | Biome::EndBarrens
        | Biome::EndHighlands
        | Biome::EndMidlands => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_all_plains() {
        // Check that the `ChunkBiomes` was overridden correctly.
        let gen = DistortedVoronoiBiomeGenerator::default();

        let chunk = ChunkPosition::new(5433, 132);

        let biomes = gen.generate_for_chunk(chunk, 8344);

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
        let gen = DistortedVoronoiBiomeGenerator::default();

        let chunk = ChunkPosition::new(0, 0);

        let seed = 52;
        let first = gen.generate_for_chunk(chunk, seed);

        for _ in 0..5 {
            let next = gen.generate_for_chunk(chunk, seed);

            for x in 0..16 {
                for z in 0..16 {
                    assert_eq!(first.biome_at(x, z), next.biome_at(x, z));
                }
            }
        }
    }
}
