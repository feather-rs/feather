use bitvec::boxed::BitBox;

/// A presence grid for a 3x3 area of chunks.
///
/// "Presence" simply is used to describe whether
/// something exists at the given column; it is a boolean.
/// For example, the tree generator uses this to
/// indicate whether there is a tree stump at a given
/// column.
///
/// Column positions are the same as `NearbyBiomes`.
#[derive(Debug, Clone)]
pub struct PresenceGrid {
    /// Array of booleans with values set to `true` for
    /// each coordinate presence has been set.
    grid: BitBox,
}

impl PresenceGrid {
    /// Creates a new presence grid, with all values
    /// initialized to `false`.
    pub fn new() -> Self {
        Self {
            grid: bitbox!(0; 16 * 256 * 16 * 9),
        }
    }

    /// Retrieves whether there is presence in the given column,
    /// relative to the center chunk.
    pub fn is_presence_at(&self, x: isize, z: isize) -> bool {
        let index = index(x, z);

        self.grid[index]
    }

    /// Sets the presence value in the given column,
    /// relative to the center chunk.
    pub fn set_presence_at(&mut self, x: isize, z: isize, value: bool) {
        let index = index(x, z);

        self.grid.set(index, value);
    }

    /// Returns whether there is a presence value set to
    /// `true` within the given distance. The distance function
    /// is non-standard; it computes a square of length `distance * 2`
    /// and checks for each column within this square.
    pub fn is_presence_within(&self, x: isize, z: isize, distance: u32) -> bool {
        let distance = distance as isize;
        for offset_x in -distance..=distance {
            for offset_z in -distance..=distance {
                if offset_x == 0 && offset_z == 0 {
                    // Don't test the column itself.
                    continue;
                }

                if self.is_presence_at(x + offset_x, z + offset_z) {
                    return true;
                }
            }
        }

        false
    }
}

fn index(mut x: isize, mut z: isize) -> usize {
    // Account for negatives
    x += 16;
    z += 16;

    (z * 16 * 3 + x) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_grid() {
        let mut grid = PresenceGrid::new();

        assert!(!grid.is_presence_at(0, 0));
        grid.set_presence_at(0, 0, true);
        assert!(grid.is_presence_at(0, 0));

        grid.set_presence_at(-16, -16, true);
        assert!(grid.is_presence_at(-16, -16));

        assert!(grid.is_presence_within(-10, -10, 6));
        assert!(!grid.is_presence_within(5, -5, 2));
    }
}
