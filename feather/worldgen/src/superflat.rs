use base::anvil::level::SuperflatGeneratorOptions;
use base::chunk::Chunk;
use base::world::Sections;
use base::{BlockId, ChunkPosition, CHUNK_WIDTH};

use crate::BiomeList;
use crate::WorldGenerator;

pub struct SuperflatWorldGenerator {
    pub options: SuperflatGeneratorOptions,
}

impl SuperflatWorldGenerator {
    pub fn new(options: SuperflatGeneratorOptions) -> Self {
        Self { options }
    }
}

impl WorldGenerator for SuperflatWorldGenerator {
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        biomes: &BiomeList,
    ) -> Chunk {
        let biome = biomes
            .get_id(&self.options.biome)
            .unwrap_or_else(|| panic!("Biome does not exist: {}", self.options.biome));
        let mut chunk = Chunk::new(position, sections, min_y / 16);
        chunk
            .sections_mut()
            .iter_mut()
            .for_each(|s| s.biomes_mut().fill(biome));

        let mut y_counter = min_y;
        for layer in self.options.clone().layers {
            if layer.height == 0 {
                continue;
            }
            // FIXME: get rid of this hack by having a consistent naming convention - Item::name() returns `stone` but BlockId::from_identifier requires `minecraft:stone`
            let layer_block = BlockId::from_identifier(&format!("minecraft:{}", layer.block));
            if let Some(layer_block) = layer_block {
                for y in y_counter..(y_counter + layer.height as i32) {
                    for x in 0..CHUNK_WIDTH {
                        for z in 0..CHUNK_WIDTH {
                            chunk
                                .set_block_at(
                                    x as usize,
                                    (y - min_y) as usize,
                                    z as usize,
                                    layer_block,
                                    false,
                                )
                                .unwrap();
                        }
                    }
                }
            } else {
                // Skip this layer
                log::warn!("Failed to generate layer: unknown block {}", layer.block);
            }

            y_counter += layer.height as i32;
        }

        chunk.recalculate_heightmaps();

        chunk
    }
}

#[cfg(test)]
mod tests {
    use base::biome::BiomeId;
    use base::chunk::SECTION_HEIGHT;
    use libcraft_blocks::BlockId;

    use crate::base::chunk::SECTION_HEIGHT;

    use super::*;

    #[test]
    pub fn test_worldgen_flat() {
        let options = SuperflatGeneratorOptions {
            biome: BiomeId::Mountains.name().to_owned(),
            ..Default::default()
        };

        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = SuperflatWorldGenerator { options };
        let chunk = generator.generate_chunk(chunk_pos, Sections(16));

        assert_eq!(chunk.position(), chunk_pos);
        for x in 0usize..16 {
            for z in 0usize..16 {
                for (y, block) in &[
                    (0usize, BlockId::bedrock()),
                    (1usize, BlockId::dirt()),
                    (2usize, BlockId::dirt()),
                    (3usize, BlockId::grass_block()),
                ] {
                    assert_eq!(chunk.block_at(x, *y, z).unwrap(), *block);
                }
                for y in 4..16 * SECTION_HEIGHT {
                    assert_eq!(
                        chunk.block_at(x as usize, y as usize, z as usize).unwrap(),
                        BlockId::air()
                    );
                }
                assert_eq!(chunk.biomes().get_at_block(x, 0, z), BiomeId::Mountains);
            }
        }
    }
}
