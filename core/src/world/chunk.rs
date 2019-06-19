use super::ChunkPosition;

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

/// A chunk column consisting
/// of a 16x256x16 section of blocks.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
struct ChunkSection {
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
}

/// A section palette as used by
/// `ChunkSection`.
#[derive(Clone, Debug)]
struct Palette {
    /// If set to `true`, the global
    /// palette is used rather than this
    /// palette.
    global: bool,
    palette: Vec<u32>,
}
