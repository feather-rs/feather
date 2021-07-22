use crate::{FinishingGenerator, TopBlocks};
use base::{chunk::BiomeStore, Biome, BlockId, Chunk};

/// Finisher for generating snow on top of snow biomes.
#[derive(Default)]
pub struct SnowFinisher;

impl FinishingGenerator for SnowFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &BiomeStore,
        top_blocks: &TopBlocks,
        _seed: u64,
    ) {
        for x in 0..16 {
            for z in 0..16 {
                if !is_snowy_biome(biomes.get_at_block(x, 0, z)) {
                    continue;
                }

                chunk
                    .set_block_at(x, top_blocks.top_block_at(x, z) + 1, z, BlockId::snow())
                    .unwrap();
            }
        }
    }
}

fn is_snowy_biome(biome: Biome) -> bool {
    matches!(
        biome,
        Biome::SnowyTundra
            | Biome::IceSpikes
            | Biome::SnowyTaiga
            | Biome::SnowyTaigaMountains
            | Biome::SnowyBeach
    )
}
