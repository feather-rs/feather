//! Debugging of the biomes grid.

use crate::{VIEWER_HEIGHT, VIEWER_WIDTH};
use feather_core::chunk::CHUNK_WIDTH;
use feather_core::{Biome, ChunkPosition};
use feather_worldgen::biome::grow::GrowBiomeGenerator;
use feather_worldgen::biome::BiomeGenerator;
use imgui::{im_str, Ui};

pub struct State {
    /// Requested length and width (in blocks) to view of the biome grid.
    pub dimensions: i32,
    pub seed: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            dimensions: 4096,
            seed: 0,
        }
    }
}

impl State {
    pub fn render(&mut self, ui: &Ui, buffer: &mut [u32], is_first_run: bool) -> bool {
        ui.text(im_str!("Biome grid"));

        let dimensions_changed = ui
            .input_int(im_str!("Dimensions (blocks)"), &mut self.dimensions)
            .build();
        let seed_changed = ui.input_int(im_str!("Seed"), &mut self.seed).build();

        if dimensions_changed || seed_changed || is_first_run {
            let mut grid = vec![Biome::Plains; self.dimensions as usize * self.dimensions as usize];

            let generator = GrowBiomeGenerator::new();

            for x in 0..self.dimensions / CHUNK_WIDTH as i32 {
                for z in 0..self.dimensions / CHUNK_WIDTH as i32 {
                    let chunk = ChunkPosition::new(x, z);
                    let biomes = generator.generate(self.seed as u64, chunk);

                    if x + (CHUNK_WIDTH as i32) < self.dimensions
                        && z + (CHUNK_WIDTH as i32) < self.dimensions
                    {
                        for local_x in 0..CHUNK_WIDTH {
                            for local_z in 0..CHUNK_WIDTH {
                                let index = (x * CHUNK_WIDTH as i32 + local_x as i32)
                                    * self.dimensions
                                    + (z * CHUNK_WIDTH as i32 + local_z as i32);

                                let index = index as usize;

                                if grid.len() > index {
                                    grid[index] = biomes[local_x][local_z];
                                }
                            }
                        }
                    }
                }
            }

            let scale = self.dimensions as f64 / VIEWER_WIDTH as f64;

            for screen_x in 0..VIEWER_WIDTH {
                for screen_z in 0..VIEWER_HEIGHT {
                    let x = (screen_x as f64 * scale).round() as usize;
                    let z = (screen_z as f64 * scale).round() as usize;

                    let index = x * self.dimensions as usize + z;

                    let color = if index >= grid.len() {
                        0x00 // black
                    } else {
                        let biome = grid[index];

                        if biome == Biome::Ocean {
                            0xFF // blue
                        } else {
                            0xFF << 8 // green
                        }
                    };

                    buffer[screen_x * VIEWER_WIDTH + screen_z] = color;
                }
            }

            true
        } else {
            false
        }
    }
}
