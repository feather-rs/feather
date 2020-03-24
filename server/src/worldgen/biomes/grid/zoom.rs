use crate::worldgen::biomes::grid::{Grid, GridOperator};
use crate::worldgen::util::shuffle_seed_for_block;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Grid operator which increases detail in the input.
///
/// This is implemented as follows: for each 2x2 grid
/// of values in the input array, a 3x3 grid is outputted.
/// The corner values of the output grid are inherited
/// from the original grid, while values in between are
/// selected at random from adjacent corner values.
pub struct ZoomOperator;

impl GridOperator for ZoomOperator {
    fn operate(&self, input: Grid, seed: u64) -> Grid {
        let mut output = Grid::from_input(&input, input.size_x() * 2 - 1, input.size_z() * 2 - 1);

        for x in 0..input.size_x() - 1 {
            for z in 0..input.size_z() - 1 {
                let a = input.at(x, z);
                let b = input.at(x + 1, z);
                let c = input.at(x, z + 1);
                let d = input.at(x + 1, z + 1);

                let output_x = x * 2 + 1;
                let output_z = x * 2 + 1;

                // Inherit corner values
                output.set_at(output_x, output_z, a);
                output.set_at(output_x + 2, output_z, b);
                output.set_at(output_x, output_z + 2, c);
                output.set_at(output_x + 2, output_z + 2, c);

                // Randomly select a corner value for each in-between spot
                let ax = input.offset_x + x as i32;
                let az = input.offset_z + z as i32;
                output.set_at(output_x + 1, output_z, select(ax, az, seed, &[a, b]));
                output.set_at(output_x, output_z + 1, select(ax, az, seed + 1, &[a, c]));
                output.set_at(
                    output_x + 1,
                    output_z + 1,
                    select(ax, az, seed + 2, &[a, b, c, d]),
                );
                output.set_at(
                    output_x + 2,
                    output_z + 1,
                    select(ax, az, seed + 3, &[b, d]),
                );
                output.set_at(
                    output_x + 1,
                    output_z + 3,
                    select(ax, az, seed + 4, &[c, d]),
                );
            }
        }

        output
    }
}

fn select<T>(abs_x: i32, abs_z: i32, seed: u64, values: &[T]) -> T
where
    T: Copy,
{
    let index = XorShiftRng::seed_from_u64(shuffle_seed_for_block(seed, abs_x, abs_z))
        .gen_range(0, values.len());

    values[index]
}
