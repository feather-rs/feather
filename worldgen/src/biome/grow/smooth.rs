use crate::biome::grow::{select, Grid, GridLayer};
use crate::util::scramble2;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

pub struct SmoothLayer {
    pub below: Box<dyn GridLayer>,
}

impl GridLayer for SmoothLayer {
    fn generate(&self, seed: u64, x: i32, z: i32, size_x: i32, size_z: i32) -> Grid {
        let lower_x = x - 1;
        let lower_z = z - 1;
        let lower_size_x = size_x + 2;
        let lower_size_z = size_z + 2;
        let input = self
            .below
            .generate(seed + 1, lower_x, lower_z, lower_size_x, lower_size_z);

        let mut result = Grid::new(size_x, size_z);

        for i in 0..size_z {
            for j in 0..size_x {
                let mut rng = XorShiftRng::seed_from_u64(scramble2(seed, z + i, x + j));

                let val = input.at(j + 1, i + 1);
                let above = input.at(j + 1, i);
                let below = input.at(j + 1, i + 2);
                let left = input.at(j, i + 1);
                let right = input.at(j + 2, i + 1);

                let val = if left == right && above == below {
                    select(&mut rng, &[left, above])
                } else if left == right {
                    left
                } else if above == below {
                    above
                } else {
                    val
                };
                result.set_at(j, i, val);
            }
        }

        result
    }
}
