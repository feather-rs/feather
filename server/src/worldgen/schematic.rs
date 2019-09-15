use feather_blocks::Block;
use feather_core::{Chunk, ChunkPosition};

/// A 3D schematic of some structure, such as a tree or building.
///
/// Schematics are not resizable and have static dimensions.
///
/// # Notes
/// * Positions in any parameter passed to a method here are
/// local to the schematic. (0, 0, 0) corresponds to the center
/// of the schematic, which is set in `new`. Negative indices
/// are allowed.
#[derive(Debug, Clone)]
pub struct Schematic {
    /// 3D grid of blocks. Length of this vector
    /// is always length * width * height.
    blocks: Vec<Block>,
    /// Length of this schematic (X axis).
    length: usize,
    /// Width of this schematic (Z axis).
    width: usize,
    /// Height of this schematic (Y axis).
    height: usize,
    /// Position of the center of this schematic.
    center_x: usize,
    center_y: usize,
    center_z: usize,
}

impl Schematic {
    /// Creates a new, empty (all air) schematic with the given dimensions
    /// and center offset.
    pub fn new(
        length: usize,
        height: usize,
        width: usize,
        center_x: usize,
        center_y: usize,
        center_z: usize,
    ) -> Self {
        Self {
            blocks: vec![Block::Air; length * width * height],
            length,
            width,
            height,
            center_x,
            center_y,
            center_z,
        }
    }

    /// Sets the block at the given location local to this schematic.
    ///
    /// Coordinates are relative to the center set in `new`.
    pub fn set_block_at(&mut self, x: isize, y: isize, z: isize, block: Block) {
        let index = self.index(x, y, z);
        self.blocks[index] = block;
    }

    /// Retrieves the block at the given location local to this schematic.
    ///
    /// Coordinates are relative to the center set in `new`.
    pub fn block_at(&self, x: isize, y: isize, z: isize) -> Block {
        let index = self.index(x, y, z);
        self.blocks[index]
    }

    /// Writes at least part of this schematic to the given chunk.
    ///
    /// The `x`, `y`, and `z` parameters are the absolute coordinates
    /// of the center of this schematic. The schematic will then be projected
    /// onto the chunk based on the chunk's position, writing the part
    /// of the schematic that is inside the chunk.
    ///
    /// # Panics
    /// Panics of either dimension of this schematic is greater
    /// than the dimension of a chunk. (This may be implemented in the
    /// future, but there is no need right now.)
    pub fn write_to_chunk(&self, chunk: &mut Chunk, abs_x: isize, abs_y: isize, abs_z: isize) {
        assert!(self.length <= 16);
        assert!(self.width <= 16);
        assert!(self.height <= 256);

        // This is a rather inefficient way of handling this.
        // TODO: consider a faster algorithm by not checking
        // for blocks we know are not within the chunk

        let length = self.length as isize;
        let height = self.height as isize;
        let width = self.width as isize;

        let center_x = self.center_x as isize;
        let center_y = self.center_y as isize;
        let center_z = self.center_z as isize;

        let min_x = -center_x;
        let min_y = -center_y;
        let min_z = -center_z;

        let max_x = length - center_x;
        let max_y = height - center_y;
        let max_z = width - center_z;

        // Go through each block in the schematic and find
        // its coordinates local to the chunk.
        for x in min_x..max_x {
            for y in min_y..max_y {
                for z in min_z..max_z {
                    // Compute absolute position of this block.
                    let ax = abs_x + x;
                    let ay = abs_y + y;
                    let az = abs_z + z;

                    let block = self.block_at(x, y, z);
                    if block == Block::Air {
                        continue; // Don't set air blocks
                    }

                    if ay < 0 || ay >= 256 {
                        continue;
                    }

                    if let Some((chunk_x, chunk_z)) = coords_within_chunk(ax, az, chunk.position())
                    {
                        chunk.set_block_at(chunk_x, ay as usize, chunk_z, block);
                    }
                }
            }
        }
    }

    fn index(&self, x: isize, y: isize, z: isize) -> usize {
        let x = (x + self.center_x as isize) as usize;
        let y = (y + self.center_y as isize) as usize;
        let z = (z + self.center_z as isize) as usize;

        self.unoffsetted_index(x, y, z)
    }

    fn unoffsetted_index(&self, x: usize, y: usize, z: usize) -> usize {
        y * self.length * self.width + z * self.length + x
    }
}

fn coords_within_chunk(ax: isize, az: isize, chunk: ChunkPosition) -> Option<(usize, usize)> {
    let ax = ax as i32;
    let az = az as i32;

    if ax / 16 != chunk.x || az / 16 != chunk.z {
        return None; // Not within chunk
    }

    let mut offset_x = (ax - chunk.x * 16).abs();
    let mut offset_z = (az - chunk.z * 16).abs();

    if ax < 0 {
        offset_x = 15 - offset_x;
    }
    if az < 0 {
        offset_z = 15 - offset_z;
    }

    Some((offset_x as usize, offset_z as usize))
}

/// Builder construct for `Schematic`.
#[derive(Default)]
pub struct SchematicBuilder {
    length: usize,
    height: usize,
    width: usize,
    center_x: usize,
    center_y: usize,
    center_z: usize,
}

impl SchematicBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dimensions(mut self, length: usize, height: usize, width: usize) -> Self {
        self.length = length;
        self.height = height;
        self.width = width;
        self
    }

    pub fn with_center(mut self, x: usize, y: usize, z: usize) -> Self {
        self.center_x = x;
        self.center_y = y;
        self.center_z = z;
        self
    }

    pub fn build(self) -> Schematic {
        Schematic::new(
            self.length,
            self.height,
            self.width,
            self.center_x,
            self.center_y,
            self.center_z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut schem = Schematic::new(8, 8, 8, 4, 4, 4);

        schem.set_block_at(0, 0, 0, Block::Stone);
        assert_eq!(schem.block_at(0, 0, 0), Block::Stone);

        schem.set_block_at(-4, -4, -4, Block::Sandstone);
        assert_eq!(schem.block_at(-4, -4, -4), Block::Sandstone);
    }

    #[test]
    fn test_write_to_chunk() {
        let mut schem = Schematic::new(8, 8, 8, 4, 4, 4);

        for x in -2..=2 {
            for y in -2..=2 {
                for z in -2..=2 {
                    schem.set_block_at(x, y, z, Block::Stone);
                }
            }
        }

        let mut chunk = Chunk::new(ChunkPosition::new(1, 1));
        schem.write_to_chunk(&mut chunk, 0, 0, 0);

        // Nothing should have been written.
        chunk
            .sections()
            .iter()
            .for_each(|section| assert!(section.is_none()));

        // Now write...
        schem.write_to_chunk(&mut chunk, 16, 16, 16);

        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    let block = if x < 3 && y >= 14 && y <= 18 && z < 3 {
                        Block::Stone
                    } else {
                        Block::Air
                    };

                    assert_eq!(
                        chunk.block_at(x, y, z),
                        block,
                        "failed at {} {} {}",
                        x,
                        y,
                        z
                    );
                }
            }
        }
    }
}
