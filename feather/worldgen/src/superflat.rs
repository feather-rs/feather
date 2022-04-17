use std::sync::Arc;

use libcraft::anvil::level::SuperflatGeneratorOptions;
use libcraft::chunk::Chunk;
use libcraft::Sections;
use libcraft::{BlockKind, BlockState, ChunkPosition, CHUNK_WIDTH};
use quill::saveload::worldgen::WorldGenerator;

use crate::BiomeList;

pub struct SuperflatWorldGenerator {
    biomes: Arc<BiomeList>,
    options: SuperflatGeneratorOptions,
    sections: Sections,
    min_y: i32,
}

impl SuperflatWorldGenerator {
    pub fn new(
        options: SuperflatGeneratorOptions,
        biomes: Arc<BiomeList>,
        sections: Sections,
        min_y: i32,
    ) -> Self {
        Self {
            options,
            biomes,
            sections,
            min_y,
        }
    }
}

impl WorldGenerator for SuperflatWorldGenerator {
    fn generate_chunk(&self, position: ChunkPosition) -> Chunk {
        let biome = self
            .biomes
            .get_id(&self.options.biome)
            .unwrap_or_else(|| panic!("Biome does not exist: {}", self.options.biome));
        let mut chunk = Chunk::new(position, self.sections, self.min_y / 16);
        chunk
            .sections_mut()
            .iter_mut()
            .for_each(|s| s.biomes_mut().fill(biome));

        let mut y_counter = self.min_y;
        for layer in self.options.clone().layers {
            if layer.height == 0 {
                continue;
            }
            // FIXME: get rid of this hack by having a consistent naming convention - Item::name() returns `stone` but BlockState::from_namespaced_id requires `minecraft:stone`
            let layer_block = BlockKind::from_namespaced_id(&format!("minecraft:{}", layer.block))
                .map(BlockState::new);
            if let Some(layer_block) = layer_block {
                for y in y_counter..(y_counter + layer.height as i32) {
                    for x in 0..CHUNK_WIDTH {
                        for z in 0..CHUNK_WIDTH {
                            chunk
                                .set_block_at(
                                    x as usize,
                                    (y - self.min_y) as usize,
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
