use crate::voronoi::VoronoiGrid;
use crate::{voronoi, BiomeGenerator};
use base::chunk::BiomeStore;
use base::{Biome, ChunkPosition};
use once_cell::sync::Lazy;

/// Array of biome groups, each containing biomes
/// which may appear next to each other. This is used in the
/// two-level biome generator.
static BIOME_GROUPS: Lazy<Vec<Vec<Biome>>> = Lazy::new(|| {
    vec![
        vec![Biome::SnowyTundra, Biome::SnowyTaiga],
        vec![
            Biome::Plains,
            Biome::BirchForest,
            Biome::Forest,
            Biome::Taiga,
            Biome::Mountains,
            Biome::Swamp,
            Biome::DarkForest,
        ],
        vec![Biome::Savanna, Biome::Desert],
    ]
});

/// Biome grid generator which works using two layers
/// of Voronoi. The first layer defines the biome group,
/// and the second determines which biome inside that group
/// to use. This technique allows similar biomes to be grouped
/// together and prevents unrelated biomes from being neighbors.
#[derive(Default)]
pub struct TwoLevelBiomeGenerator;

impl BiomeGenerator for TwoLevelBiomeGenerator {
    fn generate_for_chunk(&self, chunk: ChunkPosition, seed: u64) -> BiomeStore {
        // Voronoi used to determine biome group
        let mut group_voronoi = VoronoiGrid::new(1024, seed);
        // Voronoi used to determine biome within group
        let mut local_voronoi = VoronoiGrid::new(256, seed + 1);

        let mut biomes = BiomeStore::default(); // Will be overridden

        let num_groups = BIOME_GROUPS.len();

        // TODO: distort voronoi

        for x in 0..4 {
            for z in 0..4 {
                // Compute biome group
                let possible_biomes = {
                    let (closest_x, closest_z) =
                        group_voronoi.get(chunk.x * 16 + x * 4, chunk.z * 16 + z * 4);

                    let group_index = voronoi::shuffle(closest_x, closest_z, 0, num_groups);

                    &BIOME_GROUPS[group_index]
                };

                // Compute biome within group
                let biome = {
                    let (closest_x, closest_z) =
                        local_voronoi.get(chunk.x * 16 + x * 4, chunk.z * 16 + z * 4);

                    let biome_index =
                        voronoi::shuffle(closest_x, closest_z, 0, possible_biomes.len());

                    possible_biomes[biome_index]
                };
                for y in 0..64 {
                    biomes.set(x as usize, y, z as usize, biome);
                }
            }
        }

        biomes
    }
}
