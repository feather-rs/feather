use feather_blocks::Block;
use feather_core::level::SuperflatGeneratorOptions;
use feather_core::{Chunk, ChunkPosition};
use std::collections::HashMap;

pub trait WorldGenerator: Send + Sync {
    /// Generates the chunk at the given position.
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk;
}

pub struct SuperflatWorldGenerator {
    options: SuperflatGeneratorOptions,
}

impl WorldGenerator for SuperflatWorldGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let mut chunk = Chunk::new(position);
        let mut y_counter = 0;
        for layer in self.options.clone().layers {
            if layer.height <= 0 {
                continue;
            }
            let layer_block = Block::from_name_and_props(layer.block.as_str(), &HashMap::new());
            if layer_block.is_none() {
                // Skip this layer
                debug!("Failed to generate layer: unknown block {}", layer.block);
                y_counter += layer.height;
                continue;
            }
            let layer_block = layer_block.unwrap();
            for y in y_counter..=(y_counter + layer.height) {
                for x in 0..16 {
                    for z in 0..16 {
                        chunk.set_block_at(x as usize, y as usize, z as usize, layer_block)
                    }
                }
            }
            y_counter += layer.height;
        }
        chunk
    }
}
