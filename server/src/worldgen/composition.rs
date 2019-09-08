//! Composition generator, used to populate chunks with blocks
//! based on the density and biome values.

use crate::worldgen::{block_index, ChunkBiomes, CompositionGenerator, SEA_LEVEL};
use bitvec::slice::BitSlice;
use feather_blocks::{GrassBlockData, MyceliumData, SnowData};
use feather_core::{Biome, Block, Chunk, ChunkPosition};

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
    ) {
        // For each column in the chunk, go from top to
        // bottom. The first time a block density value is set to `true`,
        // set it and the next three blocks to dirt. After that, use
        // stone.

        for x in 0..16 {
            for z in 0..16 {
                basic_composition_for_column(x, z, chunk, density, biomes.biome_at(x, z));
            }
        }
    }
}

fn basic_composition_for_column(
    x: usize,
    z: usize,
    chunk: &mut Chunk,
    density: &BitSlice,
    biome: Biome,
) {
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
                        top_soil_block(biome)
                    } else {
                        underneath_top_soil_block(biome)
                    }
                } else if dirt_remaining > 0 {
                    dirt_remaining -= 1;
                    underneath_top_soil_block(biome)
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

/// Returns the top soil block for the given biome.
fn top_soil_block(biome: Biome) -> Block {
    match biome {
        Biome::SnowyTundra
        | Biome::IceSpikes
        | Biome::SnowyTaiga
        | Biome::SnowyTaigaMountains
        | Biome::WoodedMountains => Block::Snow(SnowData { layers: 1 }),
        Biome::SnowyBeach => Block::SnowBlock,
        Biome::GravellyMountains => Block::Gravel,
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
fn underneath_top_soil_block(biome: Biome) -> Block {
    match biome {
        Biome::SnowyTundra
        | Biome::IceSpikes
        | Biome::SnowyTaiga
        | Biome::SnowyTaigaMountains
        | Biome::WoodedMountains => Block::GrassBlock(GrassBlockData { snowy: true }),
        Biome::SnowyBeach => Block::SnowBlock,
        Biome::GravellyMountains => Block::Gravel,
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
        basic_composition_for_column(x, z, &mut chunk, &density[..], Biome::Plains);

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
