//! Composition generator, used to populate chunks with blocks
//! based on the density and biome values.

use crate::worldgen::{block_index, ChunkBiomes, CompositionGenerator, SEA_LEVEL};
use bitvec::slice::BitSlice;
use feather_blocks::{BirchLogData, GrassBlockData};
use feather_core::{Biome, Block, BlockExt, Chunk, ChunkPosition};

/// A composition generator which ignores the biome values
/// and assumes a plains-like terrain.
#[derive(Debug, Default)]
pub struct BasicCompositionGenerator;

impl CompositionGenerator for BasicCompositionGenerator {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        _pos: ChunkPosition,
        biomes: &ChunkBiomes,
        density: &BitSlice,
    ) {
        // For each column in the chunk, go from top to
        // bottom. The first time a block density value is set to `true`,
        // set it and the next three blocks to dirt. After that, use
        // stone.

        for x in 0..16 {
            for z in 0..16 {
                basic_composition_for_column(x, z, chunk, density);

                let block = match biomes.biome_at(x, z) {
                    Biome::Plains => Block::Dirt,
                    Biome::Badlands => Block::Terracotta,
                    Biome::BirchForest => Block::BirchLog(BirchLogData::default()),
                    Biome::IceSpikes => Block::AcaciaPlanks,
                    biome => Block::from_native_state_id(biome.protocol_id() as u16).unwrap(),
                };
                chunk.set_block_at(x, 100, z, block);
            }
        }
    }
}

fn basic_composition_for_column(x: usize, z: usize, chunk: &mut Chunk, density: &BitSlice) {
    let mut dirt_remaining = -1;
    for y in (0..256).rev() {
        // TODO: randomize bedrock height
        if y <= 2 {
            chunk.set_block_at(x, y, z, Block::Bedrock);
        } else {
            let is_solid = density[block_index(x, y, z)];
            let block = if is_solid {
                if dirt_remaining == -1 {
                    dirt_remaining = 3;
                    if y >= SEA_LEVEL - 2 {
                        Block::GrassBlock(GrassBlockData::default())
                    } else {
                        Block::Dirt
                    }
                } else if dirt_remaining > 0 {
                    dirt_remaining -= 1;
                    Block::Dirt
                } else {
                    Block::Stone
                }
            } else {
                dirt_remaining = -1;
                Block::Air
            };

            if block != Block::Air {
                chunk.set_block_at(x, y, z, block);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_composition_for_column() {
        let mut density = bitvec![0; 16 * 256 * 16];

        let x = 0;
        let z = 0;

        for y in 0..=32 {
            density.set(block_index(x, y, z), true);
        }

        for y in 40..=64 {
            density.set(block_index(x, y, z), true);
        }

        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        basic_composition_for_column(x, z, &mut chunk, &density[..]);

        for y in 4..=28 {
            assert_eq!(chunk.block_at(x, y, z), Block::Stone);
        }

        for y in 29..=32 {
            assert_eq!(chunk.block_at(x, y, z), Block::Dirt);
        }

        for y in 33..40 {
            assert_eq!(chunk.block_at(x, y, z), Block::Air);
        }

        for y in 40..=60 {
            assert_eq!(chunk.block_at(x, y, z), Block::Stone);
        }

        assert_eq!(
            chunk.block_at(x, 64, z),
            Block::GrassBlock(GrassBlockData::default())
        );
    }
}
