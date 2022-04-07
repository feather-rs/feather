use libcraft::anvil::level::SuperflatGeneratorOptions;
use libcraft::chunk::Chunk;
use libcraft::Sections;
use libcraft::{BlockKind, BlockState, ChunkPosition, CHUNK_WIDTH};

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
    use libcraft::biome::{
        BiomeCategory, BiomeColor, BiomeEffects, BiomeGeneratorInfo, BiomeInfo, BiomeSpawners,
    };
    use libcraft::chunk::SECTION_HEIGHT;

    use super::*;

    #[test]
    pub fn test_worldgen_flat() {
        let options = SuperflatGeneratorOptions {
            biome: "minecraft:mountains".to_string(),
            ..Default::default()
        };

        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = SuperflatWorldGenerator { options };
        let mut biomes = BiomeList::default();
        biomes.insert(
            "minecraft:mountains".to_string(),
            BiomeGeneratorInfo {
                carvers: Default::default(),
                features: vec![],
                spawners: BiomeSpawners {
                    monster: vec![],
                    creature: vec![],
                    ambient: vec![],
                    axolotls: vec![],
                    underground_water_creature: vec![],
                    water_creature: vec![],
                    water_ambient: vec![],
                    misc: vec![],
                },
                spawn_costs: Default::default(),
                info: BiomeInfo {
                    effects: BiomeEffects {
                        mood_sound: None,
                        music: None,
                        ambient_sound: None,
                        additions_sound: None,
                        grass_color_modifier: None,
                        sky_color: BiomeColor { r: 0, g: 0, b: 0 },
                        foliage_color: None,
                        grass_color: None,
                        fog_color: BiomeColor { r: 0, g: 0, b: 0 },
                        water_color: BiomeColor { r: 0, g: 0, b: 0 },
                        water_fog_color: BiomeColor { r: 0, g: 0, b: 0 },
                    },
                    precipitation: "".to_string(),
                    temperature: 0.0,
                    downfall: 0.0,
                    temperature_modifier: None,
                    category: BiomeCategory::Ocean,
                    particle: None,
                },
            },
        );
        let chunk = generator.generate_chunk(chunk_pos, Sections(16), 0, &biomes);

        assert_eq!(chunk.position(), chunk_pos);
        for x in 0usize..16 {
            for z in 0usize..16 {
                for (y, block) in &[
                    (0usize, BlockState::bedrock()),
                    (1usize, BlockState::dirt()),
                    (2usize, BlockState::dirt()),
                    (3usize, BlockState::grass_block()),
                ] {
                    assert_eq!(chunk.block_at(x, *y, z).unwrap(), *block);
                }
                for y in 4..16 * SECTION_HEIGHT {
                    assert_eq!(
                        chunk.block_at(x as usize, y as usize, z as usize).unwrap(),
                        BlockState::air()
                    );
                }
                assert_eq!(
                    chunk.biome_at(x, 0, z).unwrap(),
                    biomes.get_id("minecraft:mountains").unwrap()
                );
            }
        }
    }
}
