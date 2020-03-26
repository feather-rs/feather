use crate::biome::grow::{Grid, GridLayer};
use simdnoise::NoiseBuilder;

pub struct ContinentsLayer;

impl GridLayer for ContinentsLayer {
    fn generate(&self, seed: u64, x: i32, z: i32, size_x: i32, size_z: i32) -> Grid {
        let mut grid = Grid::new(size_x, size_z);

        let (noise, _min, _max) =
            NoiseBuilder::fbm_2d_offset(x as f32, size_x as usize, z as f32, size_x as usize)
                .with_seed(seed as i32)
                .with_freq(0.02)
                .generate();

        for i in 0..size_x {
            for j in 0..size_z {
                let noise_val = noise[(i + j * size_x) as usize] * 2.0;

                let val = if noise_val > 0.05 { 1 } else { 0 };
                grid.set_at(i, j, val);
            }
        }

        grid
    }
}
