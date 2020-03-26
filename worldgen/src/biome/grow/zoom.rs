use crate::biome::grow::{Grid, GridLayer};
use crate::util::scramble2;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

pub struct ZoomLayer {
    pub below: Box<dyn GridLayer>,
}

impl GridLayer for ZoomLayer {
    fn generate(&self, seed: u64, x: i32, z: i32, size_x: i32, size_z: i32) -> Grid {
        let below_x = x / 2;
        let below_z = z / 2;
        let below_size_x = (size_x / 2) + 2;
        let below_size_z = (size_z / 2) + 2;
        let input = self
            .below
            .generate(seed + 1, below_x, below_z, below_size_x, below_size_z);

        let zoom_size_x = (below_size_x - 1) * 2;
        let zoom_size_z = (below_size_z - 1) * 2;

        let mut temp = Grid::new(zoom_size_x, zoom_size_z);

        for i in 0..below_size_z - 1 {
            let mut upper_left = input.at(0, i);
            let mut lower_left = input.at(0, i + 1);
            for j in 0..below_size_z - 1 {
                let mut rng = XorShiftRng::seed_from_u64(scramble2(seed, i, j));

                temp.set_at(j * 2, i * 2, upper_left);
                temp.set_at(
                    j * 2,
                    i * 2 + 1,
                    select(&mut rng, &[upper_left, lower_left]),
                );

                let upper_right = input.at(j + 1, i);
                let lower_right = input.at(j + 1, i + 1);

                temp.set_at(
                    j * 2 + 1,
                    i * 2,
                    select(&mut rng, &[upper_left, upper_right]),
                );
                temp.set_at(
                    j * 2 + 1,
                    i * 2 + 1,
                    select(
                        &mut rng,
                        &[upper_left, upper_right, lower_left, lower_right],
                    ),
                );

                upper_left = upper_right;
                lower_left = lower_right;
            }
        }

        let mut result = Grid::new(size_x, size_z);

        for i in 0..size_z {
            for j in 0..size_x {
                result.set_at(j, i, temp.at((x % 2) + j, (z % 2) + i));
            }
        }

        result
    }
}

fn select<T>(rng: &mut impl Rng, values: &[T]) -> T
where
    T: Copy,
{
    let index = rng.gen_range(0, values.len());
    values[index]
}
