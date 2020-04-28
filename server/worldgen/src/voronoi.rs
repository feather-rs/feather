//! Basic Voronoi implementation.

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Position of a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellPos {
    x: i32,
    y: i32,
}

/// Position of a seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SeedPos {
    x: i32,
    y: i32,
}

/// Representation of a Voronoi grid.
///
/// Seeds around the most recently requested cell are cached.
///
/// # Implementation
/// This Voronoi implementation works using a grid with jitter
/// offsets. A seed is allocated for each cell in the grid with
/// a random offset from the center of the cell based on a hash
/// function of the cell position.
///
/// This allows the grid to be deterministic and efficient compared
/// to when using random cell positions.
pub struct VoronoiGrid {
    /// Length and width of each grid square.
    length: u32,
    /// The seed used for generation.
    seed: u64,
    /// The currently cached cell position.
    cached: CellPos,
    /// The positions of the seeds around the cached cell position.
    cached_seeds: [[SeedPos; 5]; 5],
}

impl VoronoiGrid {
    /// Creates a new voronoi grid with the given
    /// length and seed.
    ///
    /// This function does not
    /// actually compute any values.
    pub fn new(length: u32, seed: u64) -> Self {
        Self {
            length,
            seed,
            cached: CellPos {
                x: 999_999_999,
                y: 999_999_999,
            }, // Use values so that this will be replaced
            cached_seeds: [[SeedPos { x: 0, y: 0 }; 5]; 5],
        }
    }

    /// Returns the position of the seed closest to the given
    /// position.
    pub fn get(&mut self, x: i32, y: i32) -> (i32, i32) {
        let cell_pos = CellPos {
            x: x / self.length as i32,
            y: y / self.length as i32,
        };

        self.update_cache(cell_pos);

        // TODO: this is fairly inefficient. There is
        // probably a way to optimize this.
        let closest_seed = self
            .cached_seeds
            .iter()
            .flatten()
            .min_by_key(|seed| {
                // Distance squared to cell position
                square(seed.x - x) + square(seed.y - y)
            })
            .unwrap(); // Safe - iterator is never empty

        (closest_seed.x, closest_seed.y)
    }

    /// Updates the currently cached seed positions.
    ///
    /// If the given cell position is equal to the cached
    /// cell position, this is a no-op.
    fn update_cache(&mut self, cell: CellPos) {
        if cell == self.cached {
            return;
        }

        self.cached = cell;

        let half_length = (self.length / 2) as i32;

        for x in -2..=2 {
            for y in -2..=2 {
                // Calculate center of grid position and then
                // apply an offset based on a hash of the cell position.

                let cell_x = cell.x + x;
                let cell_y = cell.y + y;

                let pos_x = cell_x * self.length as i32;
                let pos_y = cell_y * self.length as i32;

                let mut rng = XorShiftRng::seed_from_u64(
                    self.seed ^ (((i64::from(cell_x)) << 32) | (i64::from(cell_y))) as u64,
                );
                let offset = rng.gen_range(-half_length, half_length);

                let center_x = pos_x + half_length as i32;
                let center_y = pos_y + half_length as i32;

                let offsetted_pos = SeedPos {
                    x: center_x + offset,
                    y: center_y + offset,
                };
                self.cached_seeds[(x + 2) as usize][(y + 2) as usize] = offsetted_pos;
            }
        }
    }
}

/// Shuffles the given closest_x and closest_y values
/// and returns a deterministic random value in the given range based
/// on those values.
///
/// This can be used to determine a value corresponding to a voronoi seed,
/// for example.
pub fn shuffle(closest_x: i32, closest_y: i32, min: usize, max: usize) -> usize {
    let combined = ((closest_x as u64) << 32) | closest_y as u64;

    let mut rng = XorShiftRng::seed_from_u64(combined);

    rng.gen_range(min, max)
}

fn square(x: i32) -> i32 {
    x * x
}
