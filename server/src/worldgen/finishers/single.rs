use crate::worldgen::util::shuffle_seed_for_chunk;
use crate::worldgen::{ChunkBiomes, FinishingGenerator, TopBlocks};
use feather_blocks::BlockId;
use feather_core::{Biome, Chunk};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Foliage including shrubs and lilypads.
#[derive(Default)]
pub struct SingleFoliageFinisher;

impl FinishingGenerator for SingleFoliageFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &ChunkBiomes,
        top_blocks: &TopBlocks,
        seed: u64,
    ) {
        let mut rng = XorShiftRng::seed_from_u64(shuffle_seed_for_chunk(seed, chunk.position()));
        for x in 0..16 {
            for z in 0..16 {
                let biome = biomes.biome_at(x, z);

                if let Some(foliage) = biome_foliage(biome) {
                    if chunk.block_at(x, top_blocks.top_block_at(x, z), z) == foliage.required
                        && rng.gen_range(0, 192) == 0
                    {
                        chunk.set_block_at(x, top_blocks.top_block_at(x, z) + 1, z, foliage.block);
                    }
                }
            }
        }
    }
}

struct Foliage {
    /// The block required at the top of the column
    /// for the foliage to generate.
    required: BlockId,
    /// The foliage block.
    block: BlockId,
}

impl Foliage {
    fn new(required: BlockId, block: BlockId) -> Self {
        Self { required, block }
    }
}

fn biome_foliage(biome: Biome) -> Option<Foliage> {
    match biome {
        Biome::Desert => Some(Foliage::new(BlockId::sand(), BlockId::dead_bush())),
        Biome::Swamp => Some(Foliage::new(BlockId::water(), BlockId::lily_pad())),
        _ => None,
    }
}
