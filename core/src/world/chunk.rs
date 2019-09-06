use super::block::*;
use super::ChunkPosition;

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
const CHUNK_HEIGHT: usize = 256;
/// The width in blocks of a chunk column.
const CHUNK_WIDTH: usize = 16;

/// The height in blocks of a chunk section.
const SECTION_HEIGHT: usize = 16;

/// The width in blocks of a chunk section.
const SECTION_WIDTH: usize = CHUNK_WIDTH;

/// The volume in blocks of a chunk section.
const SECTION_VOLUME: usize = (SECTION_HEIGHT * SECTION_WIDTH * SECTION_WIDTH) as usize;

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
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Block {
        assert!(x < CHUNK_WIDTH);
        assert!(y < CHUNK_HEIGHT);
        assert!(z < CHUNK_WIDTH);
        let chunk_section = &self.sections[(y / 16) as usize];
        match chunk_section {
            Some(section) => section.block_at(x, y % 16, z),
            None => Block::Air,
        }
    }

    /// Sets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: Block) {
        assert!(x < CHUNK_WIDTH);
        assert!(y < CHUNK_HEIGHT);
        assert!(z < CHUNK_WIDTH);
        let chunk_section = &mut self.sections[y / 16];

        let section;
        if let Some(sec) = chunk_section {
            section = sec;
        } else {
            // The section is empty - create it
            if block == Block::Air {
                return; // Nothing to do - section already empty
            }

            let new_section = ChunkSection::new();
            self.set_section_at(y / 16, Some(new_section));
            section = self.section_mut(y / 16).unwrap();
        }

        section.set_block_at(x, y % 16, z, block);
    }

    /// Returns a slice of the 16
    /// chunk sections in the chunk.
    pub fn sections(&self) -> Vec<Option<&ChunkSection>> {
        self.sections.iter().map(|sec| sec.as_ref()).collect()
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
        assert!(index < NUM_SECTIONS);
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

    /// Optimizes each section in this chunk.
    ///
    /// Returns the number of sections which were actually
    /// optimized - sections which have not been
    /// modified since the last time they were optimized
    /// are not optimized.
    pub fn optimize(&mut self) -> u32 {
        let mut count = 0;
        let mut to_remove = vec![];
        for (i, s) in self.sections.iter_mut().enumerate() {
            if let Some(section) = s {
                if section.optimize() {
                    // Section was optimized - increment count
                    count += 1;
                }

                if section.empty() {
                    to_remove.push(i);
                }
            }
        }

        for i in to_remove {
            self.set_section_at(i, None);
        }

        count
    }
}

/// A chunk section consisting of a 16x16x16
/// cube of blocks.
#[derive(Clone, Debug)]
pub struct ChunkSection {
    /// The block state data for this chunk section.
    data: BitArray,
    /// This section's palette. `None` if using the global palette.
    /// The palette should always remain sorted so that a binary
    /// search can be performed on it.
    palette: Option<Vec<u16>>,
    /// The number of solid blocks in this chunk, i.e. those
    /// that are not air. This value is used to figure out when
    /// the section becomes empty.
    solid_block_count: u16,

    block_light: BitArray,
    sky_light: BitArray,

    /// A section is considered dirty when it has been
    /// modified since the last time it was optimized.
    dirty: bool,
}

impl ChunkSection {
    /// Creates a new, empty `ChunkSection`.
    pub fn new() -> Self {
        let air_id = Block::Air.native_state_id();
        Self {
            data: BitArray::new(4, SECTION_VOLUME),
            palette: Some(vec![air_id]),
            solid_block_count: 0,
            dirty: false,
            block_light: BitArray::new(4, SECTION_VOLUME),
            sky_light: BitArray::new(4, SECTION_VOLUME),
        }
    }

    /// Creates a new `ChunkSection` based on the given
    /// data, palette and lighting.
    pub fn from_data_palette_and_light(mut data: BitArray,
                                       mut palette: Option<Vec<u16>>,
                                       block_light: BitArray,
                                       sky_light: BitArray) -> Self {
        // Correct palette if not using the global palette
        if let Some(palette) = palette.as_mut() {
            Self::correct_data_and_palette(&mut data, palette);
        }

        // Count solid blocks
        let mut solid_block_count = 0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if data.get(block_index(x, y, z)) != 0 {
                        solid_block_count += 1;
                    }
                }
            }
        }

        Self {
            data,
            palette,
            solid_block_count,
            dirty: false,
            block_light,
            sky_light,
        }
    }

    /// Corrects a given raw palette and data array.
    ///
    /// Since chunk data stored by external sources
    /// (e.g. Vanilla) might not require a sorted palette
    /// like Feather does, we need to sort the palette and
    /// correct data in the array when reading from external
    /// sources.
    ///
    /// The correction is done in-place.
    fn correct_data_and_palette(data: &mut BitArray, palette: &mut Vec<u16>) {
        let original_palette = palette.clone(); // Palette without sorting guarantees

        palette.sort_unstable();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    // Replace index into palette of each block with
                    // new index into the sorted palette.
                    let block_index = block_index(x, y, z);
                    let old_index = data.get(block_index);
                    let new_index = palette
                        .binary_search(&original_palette[old_index as usize])
                        .unwrap();
                    data.set(block_index, new_index as u64);
                }
            }
        }
    }

    /// Returns whether this chunk section is empty.
    pub fn empty(&self) -> bool {
        self.solid_block_count == 0
    }

    /// Retrieves the block at the given position in this chunk section.
    /// The position is local to this section.
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Block {
        let index = block_index(x, y, z);
        let block_id = self.data.get(index);

        let global_id = match &self.palette {
            Some(palette) => palette[block_id as usize] as u16,
            None => block_id as u16,
        };

        Block::from_native_state_id(global_id).unwrap()
    }

    /// Sets the block at the given position in this chunk section.
    /// The position is local to this section.
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.dirty = true;

        let index = block_index(x, y, z);
        let block_id = block.native_state_id();

        // The value that will be put into the
        let mut paletted_index;
        if let Some(palette) = self.palette.as_mut() {
            // Retrieve the block index from the palette.

            // If necessary, add the block to the palette.
            match palette.binary_search(&block_id) {
                Ok(index) => paletted_index = index,
                Err(insertion_index) => {
                    palette.insert(insertion_index, block_id);
                    paletted_index = insertion_index;

                    // Resize if necessary
                    if needed_bits((palette.len() - 1) as u64) > self.data.bits_per_value {
                        let new_bits_per_value = self.data.bits_per_value + 1;
                        if new_bits_per_value <= MAX_BITS_PER_BLOCK {
                            self.data = self.data.resize_to(self.data.bits_per_value + 1).unwrap();
                            paletted_index = insertion_index;
                        } else {
                            // Switch to the global palette
                            let mut new_data = BitArray::new(GLOBAL_BITS_PER_BLOCK, SECTION_VOLUME);
                            for _x in 0..16 {
                                for _y in 0..16 {
                                    for _z in 0..16 {
                                        let block = self.block_at(_x, _y, _z);
                                        new_data.set(
                                            block_index(_x, _y, _z),
                                            block.native_state_id() as u64,
                                        );
                                    }
                                }
                            }

                            self.palette = None;
                            paletted_index = block_id as usize;
                            self.data = new_data;
                        }
                    }

                    // Correct data, since palette entries after
                    // the one which was inserted will be offsetted
                    // by one.
                    for x in 0..16 {
                        for y in 0..16 {
                            for z in 0..16 {
                                let index = block_index(x, y, z);

                                let entry = self.data.get(index);
                                if entry >= insertion_index as u64 {
                                    self.data.set(index, entry + 1);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Use the global palette.
            paletted_index = block_id as usize;
        }

        let old_block = self.block_at(x, y, z);
        if block == Block::Air && old_block != Block::Air {
            self.solid_block_count -= 1;
        } else if block != Block::Air && old_block == Block::Air {
            self.solid_block_count += 1;
        }

        self.data.set(index, paletted_index as u64);
        debug_assert_eq!(self.block_at(x, y, z), block);
    }

    /// Optimizes this chunk section, reducing the bits
    /// per block value as much as possible and removing unused
    /// entries from the palette.
    ///
    /// This function only optimizes the chunk if it is dirt,
    /// i.e. if it has been modified since the last time
    /// it was optimized. The returned value is `true` when
    /// the chunk was optimized and `false` when it wasn't.
    pub fn optimize(&mut self) -> bool {
        // Only optimize the chunk if it has been modified.
        if !self.dirty {
            return false;
        }

        self.dirty = false;

        // Replace palette with new one.
        let mut new_palette = vec![];
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = self.block_at(x, y, z).native_state_id();
                    match new_palette.binary_search(&block) {
                        Ok(_) => (),
                        Err(insert_index) => {
                            new_palette.insert(insert_index, block);
                        }
                    }
                }
            }
        }

        // Recalculate all block IDs to match with the new palette.
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = self.block_at(x, y, z).native_state_id();
                    self.data.set(
                        block_index(x, y, z),
                        new_palette.binary_search(&block).unwrap() as u64,
                    );
                }
            }
        }

        self.palette = Some(new_palette);

        // Recalculate bits per block value.
        let mut new_bits_per_block = needed_bits(self.palette.as_ref().unwrap().len() as u64);
        if new_bits_per_block > MAX_BITS_PER_BLOCK {
            self.palette = None;
        } else {
            if new_bits_per_block < MIN_BITS_PER_BLOCK {
                new_bits_per_block = MIN_BITS_PER_BLOCK;
            }
            self.data = self.data.resize_to(new_bits_per_block).unwrap();
        }

        true // Chunk was optimized
    }

    /// Returns the internal data array for this section.
    pub fn data(&self) -> &BitArray {
        &self.data
    }

    /// Returns the palette for this section.
    pub fn palette(&self) -> Option<&Vec<u16>> {
        self.palette.as_ref()
    }

    /// Returns the number of bits used to store each block.
    pub fn bits_per_block(&self) -> u8 {
        self.data.bits_per_value
    }

    pub fn sky_light(&self) -> &BitArray { &self.sky_light }

    pub fn block_light(&self) -> &BitArray { &self.block_light }
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the index into a block state array
/// for the given block position.
fn block_index(x: usize, y: usize, z: usize) -> usize {
    assert!(x < 16);
    assert!(y < 16);
    assert!(z < 16);
    (y << 8) | (z << 4) | x
}

/// A "bit array." This struct manages
/// an internal array of `u64` to which
/// values of arbitrary bit length can be written.
#[derive(Clone, Debug)]
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
            let len = (((capacity * (bits_per_value as usize)) as f64) / 64.0).ceil() as usize;
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

    /// Creates a new `BitArray` based on the given raw parts.
    pub fn from_raw(data: Vec<u64>, bits_per_value: u8, capacity: usize) -> Self {
        assert!(
            bits_per_value <= 64,
            "Bits per value cannot be more than 64"
        );
        assert!(bits_per_value > 0, "Bits per value must be positive");

        let value_mask = (1 << (bits_per_value as u64)) - 1;

        Self {
            data,
            capacity,
            bits_per_value,
            value_mask,
        }
    }

    /// Returns the highest possible value represented
    /// by and entry in this `BitArray`.
    pub fn highest_possible_value(&self) -> u64 {
        self.value_mask
    }

    /// Returns the value at the given location in this `BitArray`.
    pub fn get(&self, index: usize) -> u64 {
        assert!(index < self.capacity, "Index out of bounds");

        let bit_index = index * (self.bits_per_value as usize);

        let start_long_index = bit_index / 64;

        let start_long = self.data[start_long_index];

        let index_in_start_long = (bit_index % 64) as u64;

        let mut result = start_long >> index_in_start_long;

        let end_bit_offset = index_in_start_long + self.bits_per_value as u64;

        if end_bit_offset > 64 {
            // Value stretches across multiple longs
            let end_long = self.data[start_long_index + 1];
            result |= end_long << (64 - index_in_start_long);
        }

        result & self.value_mask
    }

    /// Sets the value at the given index into this `BitArray`
    pub fn set(&mut self, index: usize, val: u64) {
        assert!(index < self.capacity, "Index out of bounds");
        assert!(
            val <= self.value_mask,
            "Value does not fit into bits_per_value"
        );

        let bit_index = index * (self.bits_per_value as usize);

        let start_long_index = bit_index / 64;

        let index_in_start_long = (bit_index % 64) as u64;

        // Clear bits of this value first
        self.data[start_long_index] = (self.data[start_long_index]
            & !(self.value_mask << index_in_start_long))
            | ((val & self.value_mask) << index_in_start_long);

        let end_bit_offset = index_in_start_long + self.bits_per_value as u64;
        if end_bit_offset > 64 {
            // Value stretches across multiple longs
            self.data[start_long_index + 1] = (self.data[start_long_index + 1]
                & !((1 << (end_bit_offset - 64)) - 1))
                | val >> (64 - index_in_start_long);
        }

        debug_assert_eq!(self.get(index), val);
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
            debug_assert_eq!(new_arr.get(i), val);
        }

        Ok(new_arr)
    }

    /// Returns the internal array.
    pub fn inner(&self) -> &Vec<u64> {
        &self.data
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
        let chunk = Chunk::new(pos);

        // Confirm that chunk is empty
        for x in 0..16 {
            assert!(chunk.section(x).is_none());
            assert!(chunk.section(x).is_none());
        }

        assert_eq!(chunk.position(), pos);
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

        for section in chunk.sections() {
            assert!(section.is_none());
        }

        for section in 0..16 {
            let mut counter = 0;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = Block::from_native_state_id(counter).unwrap();
                        chunk.set_block_at(x, (section * 16) + y, z, block);
                        assert_eq!(chunk.block_at(x, (section * 16) + y, z), block);
                        if counter != 0 {
                            assert!(chunk.section(section).is_some(), "Section {} bad", section);
                        }
                        counter += 1;
                    }
                }
            }
        }

        // Go through again to be sure
        for section in 0..16 {
            assert!(chunk.section(section).is_some());
            let mut counter = 0;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = Block::from_native_state_id(counter).unwrap();
                        assert_eq!(chunk.block_at(x, (section * 16) + y, z), block);
                        assert!(chunk.section(section).is_some());
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

        chunk.optimize();

        for section in chunk.sections() {
            assert!(section.is_none());
        }
    }

    #[test]
    fn section_from_data_and_palette() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let mut data = BitArray::new(5, 4096);
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    data.set(block_index(x, y, z), 0);
                }
            }
        }

        let palette = vec![1];
        let section = ChunkSection::from_data_and_palette(data, Some(palette));
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

        assert!(barr.resize_to(4).is_err());
    }

    #[test]
    fn bit_array_big_test() {
        let mut barr = BitArray::new(14, 4096);

        for i in 0..4096 {
            barr.set(i, i as u64);
            assert_eq!(barr.get(i), i as u64);
            if i != 4095 {
                assert_eq!(barr.get(i + 1), 0);
            }
            if i != 0 {
                assert_eq!(barr.get(i - 1), (i - 1) as u64);
            }
        }

        for i in 0..4096 {
            assert_eq!(barr.get(i), i as u64);
        }
    }

    #[test]
    fn bit_array_resize() {
        let mut barr = BitArray::new(12, 4096);
        assert_eq!(barr.bits_per_value, 12);

        for i in 0..4096 {
            barr.set(i, i as u64);
            assert_eq!(barr.get(i), i as u64);
        }

        let mut barr = barr.resize_to(13).unwrap();
        assert_eq!(barr.bits_per_value, 13);

        for i in 0..4096 {
            assert_eq!(barr.get(i), i as u64);
            barr.set(i, (i + 1) as u64);
            assert_eq!(barr.get(i), (i + 1) as u64);
        }

        let mut barr = barr.resize_to(14).unwrap();
        assert_eq!(barr.bits_per_value, 14);

        for i in 0..4096 {
            assert_eq!(barr.get(i), (i + 1) as u64);
            barr.set(i, i as u64);
            assert_eq!(barr.get(i), i as u64);
        }

        for i in 0..4096 {
            assert_eq!(barr.get(i), i as u64);
        }
    }

    #[test]
    fn test_needed_bits() {
        assert_eq!(needed_bits(31), 5);
        assert_eq!(needed_bits(255), 8);
        assert_eq!(needed_bits(256), 9);
        assert_eq!(needed_bits(1), 1);
    }

    #[test]
    fn test_block_index() {
        assert_eq!(block_index(0, 1, 0), 256);
        assert_eq!(block_index(1, 1, 1), 256 + 16 + 1);
    }

    #[test]
    fn test_correct_data_and_palette() {
        let mut data = BitArray::new(4, 4096);
        let mut palette = vec![0, 4, 2, 7, 3];
        ChunkSection::correct_data_and_palette(&mut data, &mut palette);
        assert_eq!(palette.len(), 5);
    }

    #[test]
    fn test_palette_insertion_in_middle() {
        let mut chunk = ChunkSection::new();

        chunk.set_block_at(0, 0, 0, Block::Cobblestone);
        chunk.set_block_at(0, 1, 0, Block::Stone);

        assert_eq!(chunk.block_at(0, 0, 0), Block::Cobblestone);
        assert_eq!(chunk.block_at(0, 1, 0), Block::Stone);
    }
}
