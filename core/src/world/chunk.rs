use super::block::Block;
use super::ChunkPosition;
use hashbrown::HashMap;

/// The number of bits used for each block
/// in the global palette.
const GLOBAL_BITS_PER_BLOCK: u8 = 14;

/// The minimum bits per block allowed when
/// using a section palette.
/// Bits per block values lower than this
/// value will be offsetted to this value.
const MIN_BITS_PER_BLOCK: u8 = 4;

/// The maximum number of bits per block
/// allowed when using a section palette.
/// Values above this will use the global palette
/// instead.
const MAX_BITS_PER_BLOCK: u8 = 8;

/// The height in blocks of a chunk column.
const CHUNK_HEIGHT: u16 = 256;
/// The width in blocks of a chunk column.
const CHUNK_WIDTH: u16 = 16;
/// The volume in blocks of a chunk column
const CHUNK_VOLUME: u16 = CHUNK_HEIGHT * CHUNK_WIDTH * CHUNK_WIDTH;

/// The height in blocks of a chunk section.
const SECTION_HEIGHT: u16 = 16;

/// The width in blocks of a chunk section.
const SECTION_WIDTH: u16 = CHUNK_WIDTH;

/// The volume in blocks of a chunk section.
const SECTION_VOLUME: u16 = SECTION_HEIGHT * SECTION_WIDTH * SECTION_WIDTH;

/// The number of chunk sections in a column.
const NUM_SECTIONS: usize = 16;

/// A chunk column consisting
/// of a 16x256x16 section of blocks.
/// A chunk column maintains an array
/// of up to 16 chunk sections, each corresponding
/// to a 16x16x16 section of blocks in the chunk.
#[derive(Clone)]
pub struct Chunk {
    /// The location of this chunk, in chunk
    /// coordinates.
    location: ChunkPosition,
    /// An array of the sections in this chunk.
    /// A section with Y value `y` can be found at
    /// index `y` in this array.
    /// When an entry in this array is set to `None`,
    /// the section at the entry's Y coordinate
    /// is assumed to empty, meaning that it consists
    /// of only air.
    sections: [Option<ChunkSection>; NUM_SECTIONS],
}

impl Default for Chunk {
    fn default() -> Self {
        // Rust apparently forces you to implement
        // `Copy` on types if you want to use the
        // `[ChunkSection::new(); 16]` syntax,
        // so I had to do this.
        let sections = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];

        Self {
            location: ChunkPosition::new(0, 0),
            sections,
        }
    }
}

impl Chunk {
    /// Creates a new empty chunk
    /// with the specified location.
    pub fn new(location: ChunkPosition) -> Self {
        Self {
            location,
            ..Default::default()
        }
    }

    /// Gets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn block_at(&self, x: u16, y: u16, z: u16) -> Block {
        assert!(x < CHUNK_WIDTH);
        assert!(y < CHUNK_HEIGHT);
        assert!(z < CHUNK_WIDTH);
        let chunk_section = &self.sections[(y / 16) as usize];
        chunk_section.block_at(x, y % 16, z)
    }

    /// Sets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn set_block_at(&mut self, x: u16, y: u16, z: u16, block: Block) {
        assert!(x < CHUNK_WIDTH);
        assert!(y < CHUNK_HEIGHT);
        assert!(z < CHUNK_WIDTH);
        let chunk_section = &mut self.sections[(y / 16) as usize];

        let section;
        if let Some(sec) = chunk_section {
            section = sec;
        } else {
            // The section is empty - create it
            if block == Block::Air {
                return; // Nothing to do - section already empty
            }

            let new_section = ChunkSection::new();
            self.set_section_at((y / 16) as usize, new_section);
            section = self.section_mut((y / 16) as usize);
        }

        section.set_block_at(x, y % 16, z, block);
    }

    /// Returns a slice of the 16
    /// chunk sections in the chunk.
    pub fn sections(&self) -> &[Option<ChunkSection>] {
        &self.sections
    }

    /// Returns the position in chunk coordinates
    /// of this chunk.
    pub fn position(&self) -> ChunkPosition {
        self.location
    }

    /// Returns a reference to the chunk section at the given
    /// Y offset. The Y offset must be between 0 and 15, inclusive;
    /// each Y offset value corresponds to 16 blocks vertically.
    ///
    /// If this function returns `None`, the section is assumed
    /// to be empty, meaning it consists only of air.
    pub fn section(&self, index: usize) -> Option<&ChunkSection> {
        assert_eq!(index < NUM_SECTIONS);
        self.sections[index].as_ref()
    }

    /// Returns a mutable reference to the chunk section at the given
    /// Y offset. The Y offset must be between 0 and 15, inclusive;
    /// each Y offset value corresponds to 16 blocks vertically.
    ///
    /// If this function returns `None`, the section is assumed
    /// to be empty, meaning it consists only of air.
    pub fn section_mut(&mut self, index: usize) -> Option<&mut ChunkSection> {
        assert!(index < NUM_SECTIONS);
        self.sections[index].as_mut()
    }

    /// Sets the section at the given section index.
    pub fn set_section_at(&mut self, index: usize, section: Option<ChunkSection>) {
        assert!(index < NUM_SECTIONS);
        self.sections[index] = section;
    }
}

/// A chunk section consisting of a 16x16x16
/// cube of blocks.
pub struct ChunkSection {}

/// A "bit array." This struct manages
/// an internal array of `u64` to which
/// values of arbitrary bit length can be written.
pub struct BitArray {
    /// The internal data array containing all values
    data: Vec<u64>,
    /// The capacity, in values, of this array
    capacity: usize,
    /// The number of bits used to represent each value
    bits_per_value: u8,
    /// The maximum value represented by an entry in this array
    value_mask: u64,
}

impl BitArray {
    /// Creates a new `BitArray` with the given
    /// bits per value and capacity. The array
    /// will be initialized with zeroes.
    pub fn new(bits_per_value: u8, capacity: usize) -> Self {
        assert!(
            bits_per_value <= 64,
            "Bits per value cannot be more than 64"
        );
        assert!(bits_per_value > 0, "Bits per value must be positive");
        let data = {
            let len = (((capacity * (bits_per_value as usize)) as f32) / 64.0).ceil() as usize;
            vec![0u64; len]
        };

        let value_mask = (1 << (bits_per_value as u64)) - 1;

        Self {
            data,
            capacity,
            bits_per_value,
            value_mask,
        }
    }

    /// Returns the value at the given location in this `BitArray`.
    pub fn get(&self, index: usize) -> u64 {
        assert!(index < self.capacity, "Index out of bounds");

        let bit_index = index * (self.bits_per_value as usize);

        let start_long_index = bit_index / 64;
        let end_long_index = (bit_index + (self.bits_per_value as usize)) / 64;

        let start_long = self.data[start_long_index];
        let end_long = self.data[end_long_index];

        let index_in_start_long = (bit_index % 64) as u64;

        let mut result = start_long >> index_in_start_long;

        if start_long_index != end_long_index {
            // Value stretches across multiple longs
            result |= (end_long << (64 - index_in_start_long));
        }

        result
    }

    /// Sets the value at the given index into this `BitArray`
    pub fn set(&mut self, index: usize, mut val: u64) {
        assert!(index < self.capacity, "Index out of bounds");
        assert!(
            val <= self.value_mask,
            "Value does not fit into bits_per_value"
        );

        let bit_index = index * (self.bits_per_value as usize);

        let start_long_index = bit_index / 64;
        let end_long_index = (bit_index + (self.bits_per_value as usize)) / 64;

        let index_in_start_long = (bit_index % 64) as u64;

        // Clear bits of this value first
        self.data[start_long_index] &=
            !((((1 << self.bits_per_value) - 1) as u64) << index_in_start_long as u64);

        self.data[start_long_index] |= val << index_in_start_long;

        if start_long_index != end_long_index {
            // Value stretches across multiple longs
            self.data[end_long_index] &=
                !((((1 << self.bits_per_value) - 1) as u64) << (64 - index_in_start_long) as u64);
            self.data[end_long_index] |= val << (64 - index_in_start_long);
        }
    }

    /// Produces a `BitArray` with the same values
    /// as this `BitArray` but with a new bits per value.
    /// If a value in this `BitArray` cannot be represented
    /// by the new bits per value, `Err` is returned.
    pub fn resize_to(&self, new_bits_per_value: u8) -> Result<BitArray, ()> {
        assert!(
            new_bits_per_value <= 64,
            "Bits per value cannot be more than 64"
        );

        let mut new_arr = BitArray::new(new_bits_per_value, self.capacity);

        for i in 0..self.capacity {
            let val = self.get(i);
            if needed_bits(val) > new_bits_per_value {
                return Err(());
            }

            new_arr.set(i, val);
        }

        Ok(new_arr)
    }
}

/// Returns the number of bits
/// needed to represent the given value.
fn needed_bits(mut val: u64) -> u8 {
    let mut result = 0;
    loop {
        val >>= 1;
        result += 1;

        if val == 0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_new() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        // Confirm that chunk is empty
        for x in 0..16 {
            assert_eq!(chunk.section(x), None);
            assert_eq!(chunk.section_mut(x), None);
        }

        assert_eq!(chunk.position(), pos);

        assert_eq!(
            chunk.sections(),
            &[
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None
            ]
        );
    }

    #[test]
    fn set_block_simple() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        chunk.set_block_at(0, 0, 0, Block::Andesite);
        assert_eq!(chunk.block_at(0, 0, 0), Block::Andesite);
        assert!(chunk.section(0).is_some());
    }

    #[test]
    fn fill_chunk() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let block = Block::Stone;

        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, block);
                    assert_eq!(chunk.block_at(x, y, z), block);
                }
            }
        }

        // Check again, just to be sure
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z), block);
                }
            }
        }
    }

    #[test]
    fn spray_chunk() {
        // This test fills each section of the chunk
        // with the blocks with IDs corresponding
        // to 0-4095 in order, testing that
        // resizing, etc. works correctly.

        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        for section in 0..16 {
            let mut counter = 0;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = Block::from_block_state_id(counter);
                        chunk.set_block_at(x, y * section, z, block);
                        assert_eq!(chunk.block_at(x, y * section, z), block);
                        counter += 1;
                    }
                }
            }
        }

        // Go through again to be sure
        for section in 0..16 {
            let mut counter = 0;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = Block::from_block_state_id(counter);
                        assert_eq!(chunk.block_at(x, y * section, z), block);
                        counter += 1;
                    }
                }
            }
        }

        // Now, empty the chunk, call optimize(), and ensure
        // that the sections become empty.
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, Block::Air);
                }
            }
        }

        for section in chunk.sections() {
            assert_eq!(section, None);
        }
    }

    #[test]
    fn section_from_data_and_palette() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let data = [0u64; 4096 / 16];
        let palette = Palette::from_slice(&[1]);
        let section = ChunkSection::from_data_and_palette(data, palette);
        chunk.set_section_at(0, Some(section));

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z), Block::Stone);
                }
            }
        }
    }

    #[test]
    fn bit_array() {
        let mut barr = BitArray::new(5, 4096);
        assert_eq!(barr.highest_possible_value(), 31);

        for i in 0..4096 {
            barr.set(i, 8);
            assert_eq!(barr.get(i), 8);
        }

        for i in 0..4096 {
            assert_eq!(barr.get(i), 8);
        }

        let resized = barr.resize_to(8).unwrap();
        for i in 0..4096 {
            assert_eq!(resized.get(i), 8);
        }

        let resized = barr.resize_to(4).unwrap();
        for i in 0..4096 {
            assert_eq!(resized.get(i), 8);
        }
    }

    #[test]
    fn bit_array_resize_fail() {
        let mut barr = BitArray::new(5, 4096);

        for i in 0..4096 {
            barr.set(i, 31);
        }

        assert(barr.resize_to(4).is_err());
    }

    #[test]
    fn test_needed_bits() {
        assert_eq!(needed_bits(31), 5);
        assert_eq!(needed_bits(255), 8);
        assert_eq!(needed_bits(256), 9);
        assert_eq!(needed_bits(1), 1);
    }
}
