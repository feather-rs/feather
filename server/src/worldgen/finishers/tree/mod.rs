//! Tree generation.

mod schematic;
mod ty;

use crate::worldgen::{FinishingGenerator, NearbyBiomes, TopBlocks};
use feather_core::Chunk;
pub use ty::TreeType;

/// Finisher for generating trees, depending on biome.
#[derive(Default)]
pub struct TreeFinisher;

impl FinishingGenerator for TreeFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &NearbyBiomes,
        top_blocks: &TopBlocks,
        seed: u64,
    ) {
        unimplemented!()
    }
}
