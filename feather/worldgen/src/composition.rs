//! Composition generator, used to populate chunks with blocks
//! based on the density and biome values.

use crate::{block_index, util, CompositionGenerator, SEA_LEVEL};
use base::{chunk::BiomeStore, Biome, BlockId, Chunk, ChunkPosition};
use bitvec::order::LocalBits;
use bitvec::slice::BitSlice;
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
        biomes: &BiomeStore,
        density: &BitSlice<LocalBits, u8>,
        seed: u64,
    ) {
        // For each column in the chunk, go from top to
        // bottom. The first time a block density value is set to `true`,
        // set it and the next three blocks to dirt. After that, use
        // stone.
        for x in 0..16 {
            for z in 0..16 {
                basic_composition_for_column(
                    x,
                    z,
                    chunk,
                    density,
                    seed,
                    biomes.get_at_block(x, 0, z),
                );
            }
        }
    }
}

fn basic_composition_for_column(
    x: usize,
    z: usize,
    chunk: &mut Chunk,
    density: &BitSlice<LocalBits, u8>,
    seed: u64,
    biome: Biome,
) {
    basic_composition_for_solid_biome(x, z, chunk, density, seed, biome);
}

fn basic_composition_for_solid_biome(
    x: usize,
    z: usize,
    chunk: &mut Chunk,
    density: &BitSlice<LocalBits, u8>,
    seed: u64,
    biome: Biome,
) {
    let mut rng =
        XorShiftRng::seed_from_u64(util::shuffle_seed_for_column(seed, chunk.position(), x, z));

    let top_soil = top_soil_block(biome);

    let mut topsoil_remaining = -1;
    let mut water_level = 0; // `level` block data starts at 0 and skips to min(8+n, 15) for each level of water downward
    for y in (0..256).rev() {
        let mut block = BlockId::air();

        let is_solid = density[block_index(x, y, z)];

        let mut skip = false;

        if biome == Biome::Ocean {
            if y <= SEA_LEVEL && !is_solid {
                block = BlockId::water().with_water_level(water_level);
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
                block = BlockId::bedrock();
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
                        BlockId::stone()
                    }
                } else {
                    topsoil_remaining = -1;
                    BlockId::air()
                };
            }
        }

        if !block.is_air() {
            chunk.set_block_at(x, y, z, block);
        }
    }
}

/// Returns the top soil block for the given biome.
fn top_soil_block(biome: Biome) -> BlockId {
    match biome {
        Biome::SnowyTundra
        | Biome::IceSpikes
        | Biome::SnowyTaiga
        | Biome::SnowyTaigaMountains
        | Biome::SnowyBeach => BlockId::grass_block().with_snowy(true),
        Biome::GravellyMountains | Biome::ModifiedGravellyMountains => BlockId::gravel(),
        Biome::StoneShore => BlockId::stone(),
        Biome::Beach | Biome::Desert | Biome::DesertHills | Biome::DesertLakes => BlockId::sand(),
        Biome::MushroomFields | Biome::MushroomFieldShore => BlockId::mycelium(),

        Biome::Badlands
        | Biome::ErodedBadlands
        | Biome::WoodedBadlandsPlateau
        | Biome::BadlandsPlateau
        | Biome::ModifiedBadlandsPlateau
        | Biome::ModifiedWoodedBadlandsPlateau => BlockId::red_sand(),
        Biome::Ocean => BlockId::sand(),
        _ => BlockId::grass_block(),
    }
}

/// Returns the block under the top soil block for the given biome.
fn underneath_top_soil_block(biome: Biome) -> BlockId {
    match biome {
        Biome::SnowyBeach => BlockId::snow_block(),
        Biome::GravellyMountains | Biome::ModifiedGravellyMountains => BlockId::gravel(),
        Biome::StoneShore => BlockId::stone(),
        Biome::Beach | Biome::Desert | Biome::DesertHills | Biome::DesertLakes => {
            BlockId::sandstone()
        }
        Biome::MushroomFields | Biome::MushroomFieldShore => BlockId::dirt(),
        Biome::Badlands
        | Biome::ErodedBadlands
        | Biome::WoodedBadlandsPlateau
        | Biome::BadlandsPlateau
        | Biome::ModifiedBadlandsPlateau
        | Biome::ModifiedWoodedBadlandsPlateau => BlockId::red_sandstone(),
        Biome::Ocean => BlockId::sand(),
        _ => BlockId::dirt(),
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
            assert_eq!(chunk.block_at(x, y, z).unwrap(), BlockId::stone());
        }

        for y in 29..=31 {
            assert_eq!(chunk.block_at(x, y, z).unwrap(), BlockId::dirt());
        }

        for y in 33..40 {
            assert_eq!(chunk.block_at(x, y, z).unwrap(), BlockId::air());
        }

        for y in 40..=60 {
            assert_eq!(chunk.block_at(x, y, z).unwrap(), BlockId::stone());
        }

        assert_eq!(chunk.block_at(x, 64, z).unwrap(), BlockId::grass_block());
    }
}
