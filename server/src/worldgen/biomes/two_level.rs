use crate::worldgen::voronoi::VoronoiGrid;
use crate::worldgen::{voronoi, BiomeGenerator, ChunkBiomes};
use feather_core::{Biome, ChunkPosition};

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
