//! Composition generator, used to populate chunks with blocks
//! based on the density and biome values.

use crate::worldgen::{block_index, util, ChunkBiomes, CompositionGenerator, SEA_LEVEL};
use bitvec::order::Local;
use bitvec::slice::BitSlice;
use feather_blocks::{GrassBlockData, MyceliumData, WaterData};
use feather_core::{Biome, Block, Chunk, ChunkPosition};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::cmp::min;

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
        density: &BitSlice<Local, u8>,
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
    density: &BitSlice<Local, u8>,
    seed: u64,
    biome: Biome,
) {
    basic_composition_for_solid_biome(x, z, chunk, density, seed, biome);
}

fn basic_composition_for_solid_biome(
    x: usize,
    z: usize,
    chunk: &mut Chunk,
    density: &BitSlice<Local, u8>,
    seed: u64,
    biome: Biome,
) {
    let mut rng =
        XorShiftRng::seed_from_u64(util::shuffle_seed_for_column(seed, chunk.position(), x, z));

    let top_soil = top_soil_block(biome);

    let mut topsoil_remaining = -1;
    let mut water_level = 0; // `level` block data starts at 0 and skips to min(8+n, 15) for each level of water downward
    for y in (0..256).rev() {
        let mut block = Block::Air;

        let is_solid = density[block_index(x, y, z)];

        let mut skip = false;

        if biome == Biome::Ocean {
            if y <= SEA_LEVEL && !is_solid {
                block = Block::Water(WaterData { level: water_level });
                if water_level == 0 {
                    water_level = 8;
                } else {
                    water_level = min(water_level + 1, 15);
                }
                skip = true;
            } else if y >= SEA_LEVEL {
                continue; // Leave at air - no blocks above sea level in ocean
            }
        }

        if !skip {
            if y <= rng.gen_range(0, 4) {
                block = Block::Bedrock;
            } else {
                block = if is_solid {
                    if topsoil_remaining == -1 {
                        topsoil_remaining = 3;
                        top_soil
                    } else if topsoil_remaining > 0 {
                        let block = underneath_top_soil_block(biome);
                        topsoil_remaining -= 1;
                        block
                    } else {
                        Block::Stone
                    }
                } else {
                    topsoil_remaining = -1;
                    Block::Air
                };
            }
        }

        if block != Block::Air {
            chunk.set_block_at(x, y, z, block);
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
        | Biome::SnowyBeach => Block::GrassBlock(GrassBlockData { snowy: true }),
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
        Biome::Ocean => Block::Sand,
        _ => Block::GrassBlock(GrassBlockData::default()),
    }
}

/// Returns the block under the top soil block for the given biome.
fn underneath_top_soil_block(biome: Biome) -> Block {
    match biome {
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
        Biome::Ocean => Block::Sand,
        _ => Block::Dirt,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::vec::BitVec;

    #[test]
    fn test_basic_composition_for_column() {
        let mut density = BitVec::from_vec(vec![0u8; 16 * 256 * 16 / 8]);

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

        for y in 29..=31 {
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
