//! Composition generator, used to populate chunks with blocks
//! based on the density and biome values.

use crate::worldgen::{block_index, util, ChunkBiomes, CompositionGenerator, SEA_LEVEL};
use bitvec::slice::BitSlice;
use feather_blocks::{GrassBlockData, MyceliumData, SnowData};
use feather_core::{Biome, Block, Chunk, ChunkPosition};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// A composition generator which generates basic
/// terrain based on biome values.
#[derive(Debug, Default)]
pub struct BasicCompositionGenerator;

impl CompositionGenerator for BasicCompositionGenerator {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        _pos: ChunkPosition,
        biomes: &ChunkBiomes,
        density: &BitSlice,
        seed: u64,
    ) {
        // For each column in the chunk, go from top to
        // bottom. The first time a block density value is set to `true`,
        // set it and the next three blocks to dirt. After that, use
        // stone.
        for x in 0..16 {
            for z in 0..16 {
                basic_composition_for_column(x, z, chunk, density, seed, biomes.biome_at(x, z));
            }
        }
    }
}

fn basic_composition_for_column(
    x: usize,
    z: usize,
    chunk: &mut Chunk,
    density: &BitSlice,
    seed: u64,
    biome: Biome,
) {
    let mut rng =
        XorShiftRng::seed_from_u64(util::shuffle_seed_for_column(seed, chunk.position(), x, z));
    let bedrock_height = rng.gen_range(0, 4);

    let mut topsoil_remaining = -1;
    for y in (0..256).rev() {
        if y <= bedrock_height {
            chunk.set_block_at(x, y, z, Block::Bedrock);
        } else {
            let is_solid = density[block_index(x, y, z)];
            let block = if is_solid {
                if topsoil_remaining == -1 {
                    topsoil_remaining = 3;
                    if y >= SEA_LEVEL - 2 {
                        top_soil_block(biome)
                    } else {
                        underneath_top_soil_block(biome, topsoil_remaining)
                    }
                } else if topsoil_remaining > 0 {
                    let block = underneath_top_soil_block(biome, topsoil_remaining);
                    topsoil_remaining -= 1;
                    block
                } else {
                    Block::Stone
                }
            } else {
                topsoil_remaining = -1;
                Block::Air
            };

            if block != Block::Air {
                chunk.set_block_at(x, y, z, block);
            }
        }
    }
}

/// Returns the top soil block for the given biome.
fn top_soil_block(biome: Biome) -> Block {
    match biome {
        Biome::SnowyTundra
        | Biome::IceSpikes
        | Biome::SnowyTaiga
        | Biome::SnowyTaigaMountains
        | Biome::WoodedMountains => Block::Snow(SnowData { layers: 1 }),
        Biome::SnowyBeach => Block::SnowBlock,
        Biome::GravellyMountains | Biome::ModifiedGravellyMountains => Block::Gravel,
        Biome::StoneShore => Block::Stone,
        Biome::Beach | Biome::Desert | Biome::DesertHills | Biome::DesertLakes => Block::Sand,
        Biome::MushroomFields | Biome::MushroomFieldShore => {
            Block::Mycelium(MyceliumData { snowy: false })
        }
        Biome::Badlands
        | Biome::ErodedBadlands
        | Biome::WoodedBadlandsPlateau
        | Biome::BadlandsPlateau
        | Biome::ModifiedBadlandsPlateau
        | Biome::ModifiedWoodedBadlandsPlateau => Block::RedSand,
        _ => Block::GrassBlock(GrassBlockData::default()),
    }
}

/// Returns the block under the top soil block for the given biome.
fn underneath_top_soil_block(biome: Biome, topsoil_remaining: i32) -> Block {
    match biome {
        Biome::SnowyTundra
        | Biome::IceSpikes
        | Biome::SnowyTaiga
        | Biome::SnowyTaigaMountains
        | Biome::WoodedMountains => {
            if topsoil_remaining == 3 {
                Block::GrassBlock(GrassBlockData { snowy: true })
            } else {
                Block::Dirt
            }
        }
        Biome::SnowyBeach => Block::SnowBlock,
        Biome::GravellyMountains | Biome::ModifiedGravellyMountains => Block::Gravel,
        Biome::StoneShore => Block::Stone,
        Biome::Beach | Biome::Desert | Biome::DesertHills | Biome::DesertLakes => Block::Sandstone,
        Biome::MushroomFields | Biome::MushroomFieldShore => Block::Dirt,
        Biome::Badlands
        | Biome::ErodedBadlands
        | Biome::WoodedBadlandsPlateau
        | Biome::BadlandsPlateau
        | Biome::ModifiedBadlandsPlateau
        | Biome::ModifiedWoodedBadlandsPlateau => Block::RedSandstone,
        _ => Block::Dirt,
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
        basic_composition_for_column(x, z, &mut chunk, &density[..], 435, Biome::Plains);

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
