use feather_blocks::Block;
use feather_core::level::SuperflatGeneratorOptions;
use feather_core::{Chunk, ChunkPosition};

pub trait WorldGenerator: Send + Sync {
    /// Generates the chunk at the given position.
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk;
}

pub struct SuperflatWorldGenerator {
    pub options: SuperflatGeneratorOptions,
}

pub struct EmptyWorldGenerator {}

impl WorldGenerator for EmptyWorldGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        Chunk::new(position)
    }
}

impl WorldGenerator for SuperflatWorldGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let mut chunk = Chunk::new(position);
        let mut y_counter = 0;
        for layer in self.options.clone().layers {
            if layer.height == 0 {
                continue;
            }
            let layer_block = Block::from_name_and_default_props(layer.block.as_str());
            if layer_block.is_none() {
                // Skip this layer
                debug!("Failed to generate layer: unknown block {}", layer.block);
                y_counter += layer.height;
                continue;
            }
            let layer_block = layer_block.unwrap();
            for y in y_counter..(y_counter + layer.height) {
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.set_block_at(x as usize, y as usize, z as usize, layer_block);
                    }
                }
            }
            y_counter += layer.height;
        }
        chunk
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_blocks::GrassBlockData;

    #[test]
    pub fn test_worldgen_empty() {
        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = EmptyWorldGenerator {};
        let chunk = generator.generate_chunk(chunk_pos);

        // No sections have been generated
        assert!(chunk.sections().iter().all(|sec| sec.is_none()));
        assert_eq!(chunk_pos, chunk.position());
    }

    #[test]
    pub fn test_worldgen_flat() {
        let options = SuperflatGeneratorOptions::default();
        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = SuperflatWorldGenerator { options };
        let chunk = generator.generate_chunk(chunk_pos);

        assert_eq!(chunk.position(), chunk_pos);
        for x in 0usize..16 {
            for z in 0usize..16 {
                for (y, block) in &[
                    (0usize, Block::Bedrock),
                    (1usize, Block::Dirt),
                    (2usize, Block::Dirt),
                    (3usize, Block::GrassBlock(GrassBlockData { snowy: false })),
                ] {
                    assert_eq!(chunk.block_at(x, *y, z), *block);
                }
                for y in 4..256 {
                    assert_eq!(
                        chunk.block_at(x as usize, y as usize, z as usize),
                        Block::Air
                    );
                }
            }
        }
    }
}
