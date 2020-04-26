use crate::{ChunkBiomes, FinishingGenerator, TopBlocks};
use feather_core::biomes::Biome;
use feather_core::blocks::BlockId;
use feather_core::chunk::Chunk;

/// Finisher for generating snow on top of snow biomes.
#[derive(Default)]
pub struct SnowFinisher;

impl FinishingGenerator for SnowFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &ChunkBiomes,
        top_blocks: &TopBlocks,
        _seed: u64,
    ) {
        for x in 0..16 {
            for z in 0..16 {
                if !is_snowy_biome(biomes.biome_at(x, z)) {
                    continue;
                }

                chunk.set_block_at(x, top_blocks.top_block_at(x, z) + 1, z, BlockId::snow())
            }
        }
    }
}

fn is_snowy_biome(biome: Biome) -> bool {
    match biome {
        Biome::SnowyTundra
        | Biome::IceSpikes
        | Biome::SnowyTaiga
        | Biome::SnowyTaigaMountains
        | Biome::SnowyBeach => true,
        _ => false,
    }
}
