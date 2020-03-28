use crate::biome::grow::{Grid, GridLayer};
use simdnoise::NoiseBuilder;

pub struct ContinentsLayer;

pub const OCEAN: u16 = 0;
pub const DRY: u16 = 1;
pub const TEMPERATE: u16 = 2;
pub const MOUNTAIN: u16 = 3;
pub const ICE: u16 = 4;

impl GridLayer for ContinentsLayer {
    fn generate(&self, seed: u64, x: i32, z: i32, size_x: i32, size_z: i32) -> Grid {
        let mut grid = Grid::new(size_x, size_z);

        let (continents_noise, _min, _max) =
            NoiseBuilder::fbm_2d_offset(x as f32, size_x as usize, z as f32, size_x as usize)
                .with_seed(seed as i32)
                .with_freq(0.5)
                .generate();

        let biome_group_noise =
            NoiseBuilder::fbm_2d_offset(x as f32, size_x as usize, z as f32, size_z as usize)
                .with_seed(seed as i32 + 1000)
                .with_freq(1.5)
                .generate()
                .0;

        for i in 0..size_x {
            for j in 0..size_z {
                let noise_val = continents_noise[(i + j * size_x) as usize] * 2.0;

                let val = if noise_val > 0.045 {
                    let biome_group = biome_group_noise[(i + j * size_x) as usize] * 2.0;
                    if biome_group > 0.15 {
                        ICE
                    } else if biome_group > 0.0 {
                        TEMPERATE
                    } else if biome_group > -0.05 {
                        MOUNTAIN
                    } else {
                        DRY
                    }
                } else {
                    OCEAN
                };
                grid.set_at(i, j, val);
            }
        }

        grid
    }
}
