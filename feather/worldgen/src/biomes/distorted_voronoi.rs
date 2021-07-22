use crate::voronoi::VoronoiGrid;
use crate::BiomeGenerator;
use base::chunk::BiomeStore;
use base::{Biome, ChunkPosition};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Biome grid generator based on a distorted Voronoi
/// noise.
#[derive(Default)]
pub struct DistortedVoronoiBiomeGenerator;

impl BiomeGenerator for DistortedVoronoiBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> BiomeStore {
        let mut voronoi = VoronoiGrid::new(384, seed);

        let mut biomes = BiomeStore::default(); // Will be overridden

        // Noise is used to distort each coordinate.
        /*let x_noise =
            NoiseBuilder::gradient_2d_offset(chunk.x as f32 * 16.0, 16, chunk.z as f32 * 16.0, 16)
                .with_seed(seed as i32 + 1)
                .generate_scaled(-4.0, 4.0);
        let z_noise =
            NoiseBuilder::gradient_2d_offset(chunk.x as f32 * 16.0, 16, chunk.z as f32 * 16.0, 16)
                .with_seed(seed as i32 + 2)
                .generate_scaled(-4.0, 4.0);*/

        for x in 0..4 {
            for z in 0..4 {
                // Apply distortion to coordinate before passing to voronoi
                // generator.
                //let distort_x = x_noise[(z << 4) | x] as i32 * 8;
                //let distort_z = z_noise[(z << 4) | x] as i32 * 8;

                let distort_x = 0;
                let distort_z = 0;

                let (closest_x, closest_y) = voronoi.get(
                    (chunk.x * 16) + x as i32 * 4 + distort_x,
                    (chunk.z * 16) + z as i32 * 4 + distort_z,
                );

                // Shift around the closest_x and closest_y values
                // and deterministically select a biome based on the
                // computed value. Continue shifting the value until
                // a valid biome is computed.
                let combined = (i64::from(closest_x) << 32) | i64::from(closest_y);
                let mut rng = XorShiftRng::seed_from_u64(combined as u64);

                loop {
                    let shifted: u32 = rng.gen();

                    let biome = Biome::from_id(shifted % 60).unwrap();
                    if is_biome_allowed(biome) {
                        for y in 0..64 {
                            biomes.set(x, y, z, biome);
                        }
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
    !matches!(
        biome,
        Biome::TheEnd
            | Biome::TheVoid
            | Biome::NetherWastes
            | Biome::CrimsonForest
            | Biome::WarpedForest
            | Biome::BasaltDeltas
            | Biome::SmallEndIslands
            | Biome::EndBarrens
            | Biome::EndHighlands
            | Biome::EndMidlands
    )
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
        for x in 0..4 {
            for z in 0..4 {
                for y in 0..64 {
                    if biomes.get(x, y, z) == Biome::Plains {
                        num_plains += 1;
                    }
                }
            }
        }

        assert_ne!(num_plains, 4 * 64 * 4);
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

            for x in 0..4 {
                for z in 0..4 {
                    for y in 0..64 {
                        assert_eq!(first.get(x, y, z), next.get(x, y, z));
                    }
                }
            }
        }
    }
}
