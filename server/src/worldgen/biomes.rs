//! Biome grid creation.

use crate::worldgen::voronoi::VoronoiGrid;
use crate::worldgen::{voronoi, BiomeGenerator, ChunkBiomes};
use feather_core::{Biome, ChunkPosition};
use num_traits::FromPrimitive;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use strum::EnumCount;

lazy_static! {
    /// Array of biome groups, each containing biomes
    /// which may appear next to each other. This is used in the
    /// two-level biome generator.
    static ref BIOME_GROUPS: Vec<Vec<Biome>> = {
        vec![
            vec![Biome::SnowyTundra, Biome::IceSpikes, Biome::SnowyTaiga, Biome::SnowyTaigaMountains, Biome::FrozenRiver, Biome::SnowyBeach],
            vec![Biome::Mountains, Biome::GravellyMountains, Biome::WoodedMountains, Biome::Taiga, Biome::TaigaMountains],
            vec![Biome::Plains, Biome::SunflowerPlains, Biome::Forest, Biome::FlowerForest, Biome::BirchForest, Biome::Swamp, Biome::Jungle, Biome::Beach],
            vec![Biome::Desert, Biome::Savanna, Biome::Badlands, Biome::SavannaPlateau],
            vec![Biome::Ocean],
        ]
    };
}

/// Biome grid generator which works using two layers
/// of Voronoi. The first layer defines the biome group,
/// and the second determines which biome inside that group
/// to use. This technique allows similar biomes to be grouped
/// together and prevents unrelated biomes from being neighbors.
#[derive(Default)]
pub struct TwoLevelBiomeGenerator;

impl BiomeGenerator for TwoLevelBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> ChunkBiomes {
        // Voronoi used to determine biome group
        let mut group_voronoi = VoronoiGrid::new(1024, seed);
        // Voronoi used to determine biome within group
        let mut local_voronoi = VoronoiGrid::new(384, seed + 1);

        let mut biomes = ChunkBiomes::from_array([Biome::Plains; 16 * 16]); // Will be overridden

        let num_groups = BIOME_GROUPS.len();

        // TODO: distort voronoi

        for x in 0..16 {
            for z in 0..16 {
                // Compute biome group
                let possible_biomes = {
                    let (closest_x, closest_z) =
                        group_voronoi.get(chunk.x * 16 + x, chunk.z * 16 + z);

                    let group_index = voronoi::shuffle(closest_x, closest_z, 0, num_groups);

                    &BIOME_GROUPS[group_index]
                };

                // Compute biome within group
                let biome = {
                    let (closest_x, closest_z) =
                        local_voronoi.get(chunk.x * 16 + x, chunk.z * 16 + z);

                    let biome_index =
                        voronoi::shuffle(closest_x, closest_z, 0, possible_biomes.len());

                    possible_biomes[biome_index]
                };

                biomes.set_biome_at(x as usize, z as usize, biome);
            }
        }

        biomes
    }
}

/// Biome grid generator based on a distorted Voronoi
/// noise.
#[derive(Default)]
pub struct DistortedVoronoiBiomeGenerator;

impl BiomeGenerator for DistortedVoronoiBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> ChunkBiomes {
        let mut voronoi = VoronoiGrid::new(384, seed);

        let mut biomes = ChunkBiomes::from_array([Biome::Plains; 16 * 16]); // Will be overridden

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
