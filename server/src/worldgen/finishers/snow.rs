use crate::worldgen::{ChunkBiomes, FinishingGenerator, TopBlocks};
use feather_blocks::{Block, SnowData};
use feather_core::{Biome, Chunk, ChunkPosition};

/// Finisher for generating snow on top of snow biomes.
#[derive(Default)]
pub struct SnowFinisher;

impl FinishingGenerator for SnowFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        _pos: ChunkPosition,
        biomes: &ChunkBiomes,
        top_blocks: &TopBlocks,
        _seed: u64,
    ) {
        for x in 0..16 {
            for z in 0..16 {
                if !is_snowy_biome(biomes.biome_at(x, z)) {
                    continue;
                }

                chunk.set_block_at(
                    x,
                    top_blocks.top_block_at(x, z) + 1,
                    z,
                    Block::Snow(SnowData { layers: 1 }),
                )
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
