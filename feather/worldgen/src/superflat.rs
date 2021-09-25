use base::{anvil::level::SuperflatGeneratorOptions, Biome, BlockId, Chunk, ChunkPosition};

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
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let biome = Biome::from_name(self.options.biome.as_str()).unwrap_or(Biome::Plains);
        let mut chunk = Chunk::new_with_default_biome(position, biome);

        let mut y_counter = 0;
        for layer in self.options.clone().layers {
            if layer.height == 0 {
                continue;
            }
            // FIXME: get rid of this hack by having a consistent naming convention - Item::name() returns `stone` but BlockId::from_identifier requires `minecraft:stone`
            let layer_block = BlockId::from_identifier(&format!("minecraft:{}", layer.block));
            if let Some(layer_block) = layer_block {
                for y in y_counter..(y_counter + layer.height) {
                    for x in 0..16 {
                        for z in 0..16 {
                            chunk.set_block_at(x as usize, y as usize, z as usize, layer_block);
                        }
                    }
                }
            } else {
                // Skip this layer
                log::warn!("Failed to generate layer: unknown block {}", layer.block);
            }

            y_counter += layer.height;
        }

        chunk.recalculate_heightmaps();

        chunk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_worldgen_flat() {
        let options = SuperflatGeneratorOptions {
            biome: Biome::Mountains.name().to_owned(),
            ..Default::default()
        };

        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = SuperflatWorldGenerator { options };
        let chunk = generator.generate_chunk(chunk_pos);

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
                for y in 4..256 {
                    assert_eq!(
                        chunk.block_at(x as usize, y as usize, z as usize).unwrap(),
                        BlockId::air()
                    );
                }
                assert_eq!(chunk.biomes().get_at_block(x, 0, z), Biome::Mountains);
            }
        }
    }
}
