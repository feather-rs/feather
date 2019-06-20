use super::block::BlockType;
use super::ChunkPosition;
use std::collections::HashMap;

/// The number of bits used for each block
/// in the global palette.
const GLOBAL_BITS_PER_BLOCK: usize = 14;

/// The minimum bits per block allowed when
/// using a section palette.
/// Bits per block values lower than this
/// value will be offsetted to this value.
const MIN_BITS_PER_BLOCK: usize = 4;

/// The maximum number of bits per block
/// allowed when using a section pallette.
/// Values above this will use the global pallette
/// instead.
const MAX_BITS_PER_BLOCK: usize = 8;

/// The number of bits in a `u64`
const LONG_BITS: u16 = 64;

/// A chunk column consisting
/// of a 16x256x16 section of blocks.
#[derive(Clone)]
struct Chunk {
    location: ChunkPosition,
    sections: [ChunkSection; 16],
    // TODO block entities
}

/// A chunk section consisting of
/// 16x16x16 blocks. A chunk section
/// maintains an array of `u64` which
/// are used with the global palette
/// or a section palette to store
/// block state information.
#[derive(Clone)]
struct ChunkSection {
    /// If true, this chunk section
    /// consists only of air and will
    /// not be sent in the Chunk Data packet.
    empty: bool,
    /// The number of bits used
    /// for each block in the
    /// data vector.
    bits_per_block: u8,
    /// The palette used for this
    /// chunk section.
    palette: Palette,
    /// Array of longs representing
    /// block state data, with each
    /// length of bits `bits_per_block`
    /// in length representing a single block
    /// state.
    data: Vec<u64>,
    /// The number of time each block type
    /// appears in this chunk section. This
    /// is used to determine when to resize
    /// the palette.
    occurrence_map: HashMap<BlockType, u16>,
    /// The number of distinct block types
    /// needed in the chunk section before
    /// the bits per block value must be increased.
    upper_threshold: u16,
    /// The number of distinct block types needed in
    /// the chunk section before the bits per
    /// block value should be decreased.
    lower_threshold: u16,
    /// Block light at each position in the chunk
    /// section, where 0 is darkest and 15 is brightest.
    /// Each block takes up half a byte in this array.
    block_light: [u8; 16 * 16 * 16 / 2],
    /// Sky light at each position in the chunk section,
    /// where 0 is darkest and 15 is brightest.
    /// Each block takes up half a byte in this array.
    sky_light: [u8; 16 * 16 * 16 / 2],
}

impl ChunkSection {
    /// Recalculates the data and bits per block field
    /// based on the current palette.
    fn resize(&mut self) {
        // TODO
    }

    /// Returns the block at the given
    /// position, local to this chunk section.
    fn get_block_at(&self, x: u16, y: u16, z: u16) -> BlockType {
        let bit_index = get_block_index_from_coords(x, y, z) * (self.bits_per_block as u16);
        let long_index = bit_index / 64;

        let index_in_long = bit_index % 64;
        let mut result = 0;

        // TODO
        BlockType::Air
    }
}

/// Returns the index into the data and light
/// arrays for the given coordinates (local to
/// a chunk section).
fn get_block_index_from_coords(x: u16, y: u16, z: u16) -> u16 {
    (x + (z * 16)) + (y * (16 * 16))
}

/// The inverse of `get_block_index_from_coords`.
/// The returned tuple is in the order (x, y, z).
fn get_coords_from_block_index(index: u16) -> (u16, u16, u16) {
    let x;
    let y;
    let z;

    y = index / 256;
    x = (index % 256) % 16;
    z = (index % 256) / 16;

    (x, y, z)
}

/// A section palette as used by
/// `ChunkSection`.
#[derive(Clone)]
struct Palette {
    /// If set to `true`, the global
    /// palette is used rather than this
    /// palette.
    global: bool,
    /// The palette, mapping indices
    /// of this array to block state IDs
    /// in the global palette.
    palette: Vec<u16>,
    /// Mapping of global state IDs
    /// to indices in this palette. This
    /// value is used to accelerate performance.
    mappings: HashMap<u16, u16>,
}

impl Palette {
    /// Updates the palette to allow
    /// for a new block type to be added
    /// in.
    fn add_block_mapping(&mut self, block_type: BlockType) {
        if !self.global {
            let global_id = block_type.block_state_id();
            self.palette.push(global_id);
            self.mappings
                .insert(global_id, (self.palette.len() - 1) as u16);
        }
    }

    fn remove_block_mapping(&mut self, block_type: BlockType) {
        if !self.global {
            let global_id = block_type.block_state_id();
            self.palette.retain(|val| val.clone() != global_id);
            self.mappings.remove(&global_id).unwrap();
        }
    }

    /// Returns the block type corresponding to
    /// the index into this palette or from the global
    /// palette.
    fn get_type_from_index(&self, index: u16) -> BlockType {
        if self.global {
            BlockType::from_block_state_id(index)
        } else {
            BlockType::from_block_state_id(self.palette[index as usize])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_block_index_from_coords() {
        assert_eq!(get_block_index_from_coords(0, 1, 0), 256);
        assert_eq!(get_block_index_from_coords(1, 1, 1), 256 + 16 + 1);
    }

    #[test]
    fn test_get_coords_from_block_index() {
        assert_eq!(get_coords_from_block_index(256), (0, 1, 0));
        assert_eq!(get_coords_from_block_index(256 + 16 + 1), (1, 1, 1));
    }
}
