use std::usize;

use ::blocks::BlockId;
use libcraft_core::Biome;

use crate::ChunkPosition;

/// The number of bits used for each block
/// in the global palette.
pub const GLOBAL_BITS_PER_BLOCK: u8 = 15;

/// The minimum bits per block allowed when
/// using a section palette.
/// Bits per block values lower than this
/// value will be offsetted to this value.
pub const MIN_BITS_PER_BLOCK: u8 = 4;

/// The maximum number of bits per block
/// allowed when using a section palette.
/// Values above this will use the global palette
/// instead.
pub const MAX_BITS_PER_BLOCK: u8 = 8;

/// The height in blocks of a chunk column.
pub const CHUNK_HEIGHT: usize = 256;
/// The width in blocks of a chunk column.
pub const CHUNK_WIDTH: usize = 16;

/// The height in blocks of a chunk section.
pub const SECTION_HEIGHT: usize = 16;

/// The width in blocks of a chunk section.
pub const SECTION_WIDTH: usize = CHUNK_WIDTH;

/// The volume in blocks of a chunk section.
pub const SECTION_VOLUME: usize = (SECTION_HEIGHT * SECTION_WIDTH * SECTION_WIDTH) as usize;

/// The number of chunk sections in a column.
pub const NUM_SECTIONS: usize = 16;

mod biome_store;
mod blocks;
mod heightmap;
mod light;
mod packed_array;
mod palette;

pub use self::blocks::BlockStore;
pub use biome_store::BiomeStore;
pub use heightmap::{Heightmap, HeightmapFunction, HeightmapStore};
pub use light::LightStore;
pub use packed_array::PackedArray;
pub use palette::Palette;

/// A 16x256x16 chunk of blocks plus associated
/// light, biome, and heightmap data.
/// Consists of 16 `ChunkSection`s.
#[derive(Debug, Clone)]
pub struct Chunk {
    sections: [Option<ChunkSection>; NUM_SECTIONS + 2],

    biomes: BiomeStore,

    heightmaps: HeightmapStore,

    position: ChunkPosition,
}

impl Default for Chunk {
    fn default() -> Self {
        let sections = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        ];
        Self {
            sections,
            biomes: BiomeStore::default(),
            position: ChunkPosition::new(0, 0),
            heightmaps: HeightmapStore::new(),
        }
    }
}

impl Chunk {
    /// Creates a new empty chunk with the
    /// specified position.
    ///
    /// Biomes are initialized to plains.
    pub fn new(position: ChunkPosition) -> Self {
        Self::new_with_default_biome(position, Biome::Plains)
    }

    /// Creates a new empty chunk with the specified
    /// position.
    ///
    /// Biomes are initialized to `biome`.
    pub fn new_with_default_biome(position: ChunkPosition, default_biome: Biome) -> Self {
        Self {
            position,
            biomes: BiomeStore::new(default_biome),
            ..Default::default()
        }
    }

    /// Gets the position of this chunk.
    pub fn position(&self) -> ChunkPosition {
        self.position
    }

    /// Sets the position of this chunk.
    pub fn set_position(&mut self, pos: ChunkPosition) {
        self.position = pos;
    }

    /// Gets the block at the given position within this chunk.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Option<BlockId> {
        let section = self.section_for_y(y)?;
        match section {
            Some(section) => section.block_at(x, y % SECTION_HEIGHT, z),
            None => Some(BlockId::air()),
        }
    }

    /// Sets the block at the given position within this chunk.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    /// FIXME: Do not update heightmap when it is not neccessary
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockId) -> Option<()> {
        let old_block = self.block_at(x, y, z)?;
        let section = self.section_for_y_mut(y)?;
        let result = match section {
            Some(section) => {
                let result = section.set_block_at(x, y % SECTION_HEIGHT, z, block);
                // If the block update caused the section to contain only
                // air, free it to conserve memory.
                if section.is_empty() {
                    self.clear_section(y);
                }
                result
            }
            None => {
                if !block.is_air() {
                    let mut section = ChunkSection::default();
                    let result = section.set_block_at(x, y % SECTION_HEIGHT, z, block);
                    self.set_section_at((y / SECTION_HEIGHT) as isize, Some(section));
                    result
                } else {
                    Some(())
                }
            }
        };
        self.heightmaps
            .update(x, y, z, old_block, block, Self::block_at_fn(&self.sections));
        result
    }

    /// Fills the given chunk section with `block`.
    pub fn fill_section(&mut self, section: usize, block: BlockId) -> bool {
        let section = match self.sections.get_mut(section) {
            Some(section) => section,
            None => return false,
        };

        if block == BlockId::air() {
            *section = None;
        } else {
            let section = section.get_or_insert_with(Default::default);
            section.fill(block);
        }

        true
    }

    /// Recalculates heightmaps for this chunk.
    pub fn recalculate_heightmaps(&mut self) {
        self.heightmaps
            .recalculate(Self::block_at_fn(&self.sections))
    }

    fn block_at_fn(
        sections: &[Option<ChunkSection>],
    ) -> impl Fn(usize, usize, usize) -> BlockId + '_ {
        move |x, y, z| {
            let section = &sections[(y / SECTION_HEIGHT) + 1];
            match section {
                Some(section) => section.block_at(x, y % SECTION_HEIGHT, z).unwrap(),
                None => BlockId::air(),
            }
        }
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        match self.section_for_y(y)? {
            Some(s) => s.block_light_at(x, y % SECTION_HEIGHT, z),
            None => Some(15),
        }
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        match self.section_for_y(y)? {
            Some(s) => s.sky_light_at(x, y % SECTION_HEIGHT, z),
            None => Some(15),
        }
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        if let Some(section) = self.section_for_y_mut(y)? {
            section.set_block_light_at(x, y, z, light)
        } else {
            Some(())
        }
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        if let Some(section) = self.section_for_y_mut(y)? {
            section.set_sky_light_at(x, y, z, light)
        } else {
            Some(())
        }
    }

    fn section_for_y(&self, y: usize) -> Option<&Option<ChunkSection>> {
        self.sections.get((y / SECTION_HEIGHT) + 1)
    }

    fn section_for_y_mut(&mut self, y: usize) -> Option<&mut Option<ChunkSection>> {
        self.sections.get_mut((y / SECTION_HEIGHT) + 1)
    }

    fn clear_section(&mut self, y: usize) {
        self.sections[(y / SECTION_HEIGHT) + 1] = None;
    }

    /// Gets the [`BiomeStore`] for this chunk.
    pub fn biomes(&self) -> &BiomeStore {
        &self.biomes
    }

    /// Mutably gets the [`BiomeStore`] for this chunk.
    pub fn biomes_mut(&mut self) -> &mut BiomeStore {
        &mut self.biomes
    }

    /// Gets the [`HeightmapStore`] for this chunk.
    pub fn heightmaps(&self) -> &HeightmapStore {
        &self.heightmaps
    }

    /// Mutably gets the [`HeightmapStore`] for this chunk.
    pub fn heightmaps_mut(&mut self) -> &mut HeightmapStore {
        &mut self.heightmaps
    }

    /// Gets the chunk section at index `y`.
    pub fn section(&self, y: isize) -> Option<&ChunkSection> {
        self.sections.get((y + 1) as usize)?.as_ref()
    }

    /// Mutably gets the chunk section at index `y`.
    pub fn section_mut(&mut self, y: isize) -> Option<&mut ChunkSection> {
        self.sections.get_mut((y + 1) as usize)?.as_mut()
    }

    /// Sets the section at index `y`.
    pub fn set_section_at(&mut self, y: isize, section: Option<ChunkSection>) {
        self.sections[(y + 1) as usize] = section;
    }

    /// Gets the sections of this chunk.
    pub fn sections(&self) -> &[Option<ChunkSection>] {
        &self.sections
    }
}

/// A 16x16x16 chunk of blocks.
#[derive(Debug, Clone)]
pub struct ChunkSection {
    blocks: BlockStore,

    light: LightStore,
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new(BlockStore::new(), LightStore::new())
    }
}

impl ChunkSection {
    /// Creates new `ChunkSection` from its
    /// raw parts.
    pub fn new(blocks: BlockStore, light: LightStore) -> Self {
        Self { blocks, light }
    }

    /// Determines whether this chunk is empty (contains only air).
    pub fn is_empty(&self) -> bool {
        self.non_air_blocks() == 0
    }

    /// Returns the number of air blocks in this chunk section.
    pub fn air_blocks(&self) -> u32 {
        self.blocks.air_blocks()
    }

    /// Returns the number of non-air blocks in this chunk section.
    pub fn non_air_blocks(&self) -> u32 {
        SECTION_VOLUME as u32 - self.air_blocks()
    }

    /// Gets the block at the given coordinates within this
    /// chunk section.
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Option<BlockId> {
        self.blocks.block_at(x, y, z)
    }

    /// Sets the block at the given coordinates within
    /// this chunk section.
    ///
    /// Returns `None` if the coordinates were out of bounds.
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockId) -> Option<()> {
        self.blocks.set_block_at(x, y, z, block)
    }

    /// Fills this chunk section with the given block.
    ///
    /// Does not currently update heightmaps.
    pub fn fill(&mut self, block: BlockId) {
        self.blocks.fill(block);
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.light.block_light_at(x, y, z)
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.light.sky_light_at(x, y, z)
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        self.light.set_block_light_at(x, y, z, light)
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        self.light.set_sky_light_at(x, y, z, light)
    }

    pub fn light(&self) -> &LightStore {
        &self.light
    }

    pub fn light_mut(&mut self) -> &mut LightStore {
        &mut self.light
    }

    pub fn blocks(&self) -> &BlockStore {
        &self.blocks
    }

    pub fn blocks_mut(&mut self) -> &mut BlockStore {
        &mut self.blocks
    }

    fn block_index(x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= SECTION_WIDTH || y >= SECTION_WIDTH || z >= SECTION_WIDTH {
            None
        } else {
            Some((y << 8) | (z << 4) | x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HIGHEST_ID;

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
    fn chunk_new_with_default_biome() {
        let pos = ChunkPosition::new(0, 0);
        let chunk = Chunk::new_with_default_biome(pos, Biome::Mountains);

        // Confirm that chunk is empty
        for x in 0..16 {
            assert!(chunk.section(x).is_none());
            assert!(chunk.section(x).is_none());
        }

        assert_eq!(chunk.position(), pos);

        // Confirm that biomes are set
        for x in 0..4 {
            for z in 0..4 {
                assert_eq!(chunk.biomes.get(x, 0, z), Biome::Mountains);
            }
        }
    }

    #[test]
    fn set_block_simple() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        chunk.set_block_at(0, 0, 0, BlockId::andesite());
        assert_eq!(chunk.block_at(0, 0, 0).unwrap(), BlockId::andesite());
        assert!(chunk.section(0).is_some());
    }

    #[test]
    fn fill_chunk() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let block = BlockId::stone();

        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, block).unwrap();
                    assert_eq!(chunk.block_at(x, y, z), Some(block));
                }
            }
        }

        // Check again, just to be sure
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z), Some(block));
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
                        let block = BlockId::from_vanilla_id(counter);
                        chunk.set_block_at(x, (section * 16) + y, z, block);
                        assert_eq!(chunk.block_at(x, (section * 16) + y, z), Some(block));
                        if counter != 0 {
                            assert!(
                                chunk.section(section as isize).is_some(),
                                "Section {} failed",
                                section
                            );
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
                        let block = BlockId::from_vanilla_id(counter);
                        assert_eq!(
                            chunk.block_at(x, (section as usize * 16) + y, z),
                            Some(block)
                        );
                        assert!(chunk.section(section).is_some());
                        counter += 1;
                    }
                }
            }
        }

        // Now, empty the chunk and ensure
        // that the sections become empty.
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, BlockId::air());
                }
            }
        }

        for section in chunk.sections() {
            assert!(section.is_none());
        }
    }

    #[test]
    fn section_from_data_and_palette() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let mut palette = Palette::new();
        let stone_index = palette.index_or_insert(BlockId::stone());

        let mut data = PackedArray::new(4096, 5);
        for i in 0..4096 {
            data.set(i, stone_index as u64);
        }

        let section = ChunkSection::new(
            BlockStore::from_raw_parts(Some(palette), data),
            LightStore::new(),
        );
        chunk.set_section_at(0, Some(section));

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z).unwrap(), BlockId::stone());
                }
            }
        }
    }

    #[test]
    fn test_palette_insertion_in_middle() {
        let mut chunk = ChunkSection::default();

        chunk.set_block_at(0, 0, 0, BlockId::cobblestone()).unwrap();
        chunk.set_block_at(0, 1, 0, BlockId::stone()).unwrap();

        assert_eq!(chunk.block_at(0, 0, 0).unwrap(), BlockId::cobblestone());
        assert_eq!(chunk.block_at(0, 1, 0).unwrap(), BlockId::stone());
    }

    #[test]
    fn test_biomes() {
        let mut chunk = Chunk::default();

        for x in 0..4 {
            for z in 0..4 {
                assert_eq!(chunk.biomes().get(x, 0, z), Biome::Plains);
                chunk.biomes_mut().set(x, 0, z, Biome::BirchForest);
                assert_eq!(chunk.biomes().get(x, 0, z), Biome::BirchForest);
            }
        }
    }

    #[test]
    fn test_light() {
        let mut chunk = Chunk::default();

        chunk.set_block_at(0, 0, 0, BlockId::stone()).unwrap();

        for x in 0..SECTION_WIDTH {
            for y in 0..SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
                    chunk.set_block_light_at(x, y, z, 10);
                    chunk.set_sky_light_at(x, y, z, 8);
                    assert_eq!(chunk.block_light_at(x, y, z), Some(10));
                    assert_eq!(chunk.sky_light_at(x, y, z), Some(8));
                }
            }
        }
    }

    #[test]
    fn heightmaps() {
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

        chunk.set_block_at(0, 10, 0, BlockId::stone());
        assert_eq!(chunk.heightmaps.motion_blocking.height(0, 0), Some(10));
    }

    #[test]
    fn fill_chunk_section() {
        let mut section = ChunkSection::default();
        section.set_block_at(0, 0, 0, BlockId::stone());
        section.fill(BlockId::acacia_wood());

        for x in 0..CHUNK_WIDTH {
            for y in 0..SECTION_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    assert_eq!(section.block_at(x, y, z), Some(BlockId::acacia_wood()));
                }
            }
        }
    }

    #[test]
    fn global_bits() {
        //The highest block state id must fit into GLOBAL_BITS_PER_BLOCK
        assert_eq!(HIGHEST_ID >> GLOBAL_BITS_PER_BLOCK, 0)
    }
}
