use crate::biome::grow::{Grid, GridLayer};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
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
                .with_freq(0.3)
                .generate();

        let biome_group_noise =
            NoiseBuilder::fbm_2d_offset(x as f32, size_x as usize, z as f32, size_z as usize)
                .with_seed(seed as i32 + 1000)
                .with_freq(1.2)
                .generate()
                .0;

        for i in 0..size_x {
            for j in 0..size_z {
                let mut noise_val = continents_noise[(i + j * size_x) as usize] as f64 * 2.0;

                // Ensure area around origin will be land.
                // This is done by giving a boost to noise_val
                // f(x)=1/x where x is the distance to the origin
                // times a coefficient.
                let abs_x = i + x;
                let abs_z = j + z;
                noise_val += 1.0 / (f64::from(abs_x * abs_x + abs_z * abs_z).sqrt() * 3.0);

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
