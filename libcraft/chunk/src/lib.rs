//! Implements an efficient chunk data structure for storing blocks.

use once_cell::sync::Lazy;
use std::usize;

pub use heightmap::{Heightmap, HeightmapFunction, HeightmapStore};
pub use light::LightStore;
pub use packed_array::PackedArray;

use biome::BiomeId;
use libcraft_blocks::{BlockState, HIGHEST_ID};
use libcraft_core::{ChunkPosition, Sections};
use paletted_container::PalettedContainer;

pub const BIOME_SAMPLE_RATE: usize = 4;
/// The width in blocks of a chunk column.
pub const CHUNK_WIDTH: usize = 16;

/// The height in blocks of a chunk section.
pub const SECTION_HEIGHT: usize = 16;

/// The width in blocks of a chunk section.
pub const SECTION_WIDTH: usize = CHUNK_WIDTH;

/// The volume in blocks of a chunk section.
pub const SECTION_VOLUME: usize = SECTION_HEIGHT * SECTION_WIDTH * SECTION_WIDTH;

pub const BIOMES_PER_CHUNK_SECTION: usize = (CHUNK_WIDTH / BIOME_SAMPLE_RATE)
    * (CHUNK_WIDTH / BIOME_SAMPLE_RATE)
    * (SECTION_HEIGHT / BIOME_SAMPLE_RATE);

pub mod biome;
mod heightmap;
mod light;
mod packed_array;
pub mod paletted_container;

/// A 16 x height x 16 chunk of blocks plus associated
/// light, biome, and heightmap data.
#[derive(Debug, Clone)]
pub struct Chunk {
    sections: Vec<ChunkSection>,
    heightmaps: HeightmapStore,
    position: ChunkPosition,
    min_y_section: i32,
}

impl Chunk {
    /// Creates a new empty chunk with the specified
    /// position.
    pub fn new(position: ChunkPosition, sections: Sections, min_y: i32) -> Self {
        Self {
            sections: vec![ChunkSection::default(); *sections + 2],
            position,
            heightmaps: HeightmapStore::new(sections.into()),
            min_y_section: min_y / 16,
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
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Option<BlockState> {
        self.section_for_y(y)?.block_at(x, y % SECTION_HEIGHT, z)
    }

    /// Sets the block at the given position within this chunk.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn set_block_at(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        block: BlockState,
        update_heightmaps: bool,
    ) -> Option<()> {
        if update_heightmaps {
            let old_block = self.block_at(x, y, z)?;
            let result = self
                .section_for_y_mut(y)?
                .set_block_at(x, y % SECTION_HEIGHT, z, block);
            self.heightmaps
                .update(x, y, z, old_block, block, Self::block_at_fn(&self.sections));
            result
        } else {
            self.section_for_y_mut(y)?
                .set_block_at(x, y % SECTION_HEIGHT, z, block)
        }
    }

    /// Gets the biome at the given position within this chunk.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn biome_at(&self, x: usize, y: usize, z: usize) -> Option<BiomeId> {
        self.section_for_y(y)?.biome_at(
            x / BIOME_SAMPLE_RATE,
            (y / BIOME_SAMPLE_RATE) % SECTION_HEIGHT,
            z / BIOME_SAMPLE_RATE,
        )
    }

    /// Sets the biome at the given position within this chunk.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn set_biome_at(&mut self, x: usize, y: usize, z: usize, biome: BiomeId) -> Option<()> {
        self.section_for_y_mut(y)?.set_biome_at(
            x / BIOME_SAMPLE_RATE,
            (y / BIOME_SAMPLE_RATE) % SECTION_HEIGHT,
            z / BIOME_SAMPLE_RATE,
            biome,
        )
    }

    /// Fills the given chunk section with `block`.
    pub fn fill_section(&mut self, section: usize, block: BlockState) -> bool {
        let section = match self.sections.get_mut(section) {
            Some(section) => section,
            None => return false,
        };

        section.fill(block);

        true
    }

    /// Recalculates heightmaps for this chunk.
    pub fn recalculate_heightmaps(&mut self) {
        self.heightmaps
            .recalculate(Self::block_at_fn(&self.sections))
    }

    fn block_at_fn(sections: &[ChunkSection]) -> impl Fn(usize, usize, usize) -> BlockState + '_ {
        move |x, y, z| {
            sections[(y / SECTION_HEIGHT) + 1]
                .block_at(x, y % SECTION_HEIGHT, z)
                .unwrap()
        }
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.section_for_y(y)?
            .block_light_at(x, y % SECTION_HEIGHT, z)
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.section_for_y(y)?
            .sky_light_at(x, y % SECTION_HEIGHT, z)
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        self.section_for_y_mut(y)?
            .set_block_light_at(x, y, z, light)
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        self.section_for_y_mut(y)?.set_sky_light_at(x, y, z, light)
    }

    fn section_for_y(&self, y: usize) -> Option<&ChunkSection> {
        self.sections.get((y / SECTION_HEIGHT) + 1)
    }

    fn section_for_y_mut(&mut self, y: usize) -> Option<&mut ChunkSection> {
        self.sections.get_mut((y / SECTION_HEIGHT) + 1)
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
        self.sections.get((y + 1) as usize)
    }

    /// Mutably gets the chunk section at index `y`.
    pub fn section_mut(&mut self, y: isize) -> Option<&mut ChunkSection> {
        self.sections.get_mut((y + 1) as usize)
    }

    /// Sets the section at index `y`.
    pub fn set_section_at(&mut self, y: isize, section: ChunkSection) {
        self.sections[(y + 1) as usize] = section;
    }

    /// Gets the sections of this chunk.
    pub fn sections(&self) -> &[ChunkSection] {
        &self.sections
    }

    /// Gets the sections of this chunk.
    pub fn sections_mut(&mut self) -> &mut [ChunkSection] {
        &mut self.sections
    }

    pub fn min_y_section(&self) -> i32 {
        self.min_y_section
    }

    pub fn min_y(&self) -> i32 {
        self.min_y_section * SECTION_HEIGHT as i32
    }
}

/// A 16x16x16 chunk of blocks.
#[derive(Debug, Clone)]
pub struct ChunkSection {
    blocks: PalettedContainer<BlockState>,
    biomes: PalettedContainer<BiomeId>,
    air_block_count: u32,
    light: LightStore,
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new(
            PalettedContainer::new(),
            PalettedContainer::new(),
            SECTION_VOLUME as u32,
            LightStore::new(),
        )
    }
}

impl ChunkSection {
    /// Creates new `ChunkSection` from its
    /// raw parts.
    pub fn new(
        blocks: PalettedContainer<BlockState>,
        biomes: PalettedContainer<BiomeId>,
        air_block_count: u32,
        light: LightStore,
    ) -> Self {
        Self {
            blocks,
            biomes,
            air_block_count,
            light,
        }
    }

    /// Determines whether this chunk is empty (contains only air).
    pub fn is_empty(&self) -> bool {
        self.non_air_blocks() == 0
    }

    /// Returns the number of air blocks in this chunk section.
    pub fn air_blocks(&self) -> u32 {
        self.air_block_count
    }

    /// Returns the number of non-air blocks in this chunk section.
    pub fn non_air_blocks(&self) -> u32 {
        SECTION_VOLUME as u32 - self.air_blocks()
    }

    /// Gets the block at the given coordinates within this
    /// chunk section.
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Option<BlockState> {
        self.blocks.get_block_at(x, y, z)
    }

    /// Sets the block at the given coordinates within
    /// this chunk section.
    ///
    /// Returns `None` if the coordinates were out of bounds.
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockState) -> Option<()> {
        self.update_air_block_count(x, y, z, block);
        self.blocks.set_block_at(x, y, z, block)
    }

    /// Gets the biome at the given coordinates within this
    /// chunk section.
    pub fn biome_at(&self, x: usize, y: usize, z: usize) -> Option<BiomeId> {
        self.biomes.biome_at(x, y, z)
    }

    /// Sets the biome at the given coordinates within
    /// this chunk section.
    ///
    /// Returns `None` if the coordinates were out of bounds.
    pub fn set_biome_at(&mut self, x: usize, y: usize, z: usize, biome: BiomeId) -> Option<()> {
        self.biomes.set_biome_at(x, y, z, biome)
    }

    /// Fills this chunk section with the given block.
    ///
    /// Does not currently update heightmaps.
    pub fn fill(&mut self, block: BlockState) {
        self.blocks.fill(block);

        if block.kind().is_air() {
            self.air_block_count = SECTION_VOLUME as u32;
        } else {
            self.air_block_count = 0;
        }
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

    pub fn blocks(&self) -> &PalettedContainer<BlockState> {
        &self.blocks
    }

    pub fn blocks_mut(&mut self) -> &mut PalettedContainer<BlockState> {
        &mut self.blocks
    }

    pub fn biomes(&self) -> &PalettedContainer<BiomeId> {
        &self.biomes
    }

    pub fn biomes_mut(&mut self) -> &mut PalettedContainer<BiomeId> {
        &mut self.biomes
    }

    pub fn count_air_blocks(blocks: &PalettedContainer<BlockState>) -> u32 {
        match blocks {
            PalettedContainer::SingleValue(value) if value.kind().is_air() => blocks.len() as u32,
            PalettedContainer::SingleValue(_) => 0,
            PalettedContainer::MultipleValues { data, palette } => {
                let air_blocks_in_palette = palette
                    .iter()
                    .enumerate()
                    .filter_map(|(i, value)| if value.kind().is_air() { Some(i) } else { None })
                    .map(|i| i as u32)
                    .collect::<Vec<_>>();
                data.iter()
                    .filter(|block| air_blocks_in_palette.contains(&(*block as u32)))
                    .count() as u32
            }
            PalettedContainer::GlobalPalette { data } => {
                static AIR_BLOCKS: Lazy<Vec<u32>> = Lazy::new(|| {
                    (0..HIGHEST_ID)
                        .filter(|index| BlockState::from_id(*index as u16).unwrap().kind().is_air())
                        .map(|i| i as u32)
                        .collect::<Vec<_>>()
                });
                data.iter()
                    .filter(|block| AIR_BLOCKS.contains(&(*block as u32)))
                    .count() as u32
            }
        }
    }

    fn update_air_block_count(&mut self, x: usize, y: usize, z: usize, new: BlockState) {
        let old = self.block_at(x, y, z).unwrap();
        if old.kind().is_air() && !new.kind().is_air() {
            self.air_block_count -= 1;
        } else if !old.kind().is_air() && new.kind().is_air() {
            self.air_block_count += 1;
        }
    }

    fn block_index(x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= SECTION_WIDTH || y >= SECTION_WIDTH || z >= SECTION_WIDTH {
            None
        } else {
            Some((y << 8) | (z << 4) | x)
        }
    }

    fn biome_index(x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= SECTION_WIDTH / BIOME_SAMPLE_RATE
            || y >= SECTION_WIDTH / BIOME_SAMPLE_RATE
            || z >= SECTION_WIDTH / BIOME_SAMPLE_RATE
        {
            None
        } else {
            Some((y << 4) | (z << 2) | x)
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn chunk_new() {
        let pos = ChunkPosition::new(0, 0);
        let chunk = Chunk::new(pos, Sections(16), 0);

        // Confirm that chunk is empty
        for x in 0..16 {
            assert_eq!(
                chunk.section(x).unwrap().blocks,
                PalettedContainer::default()
            );
            assert_eq!(
                chunk.section(x).unwrap().biomes,
                PalettedContainer::default()
            );
        }

        assert_eq!(chunk.position(), pos);
    }

    #[test]
    fn set_block_simple() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos, Sections(16), 0);

        chunk.set_block_at(0, 0, 0, BlockState::andesite(), true);
        assert_eq!(chunk.block_at(0, 0, 0).unwrap(), BlockState::andesite());
        assert!(chunk.section(0).is_some());
    }

    #[test]
    fn fill_chunk() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos, Sections(16), 0);

        let block = BlockState::stone();

        for x in 0..SECTION_WIDTH {
            for y in 0..16 * SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
                    chunk.set_block_at(x, y, z, block, true).unwrap();
                    assert_eq!(chunk.block_at(x, y, z), Some(block));
                }
            }
        }

        // Check again, just to be sure
        for x in 0..SECTION_WIDTH {
            for y in 0..16 * SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
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
        let mut chunk = Chunk::new(pos, Sections(16), 0);

        for section in 0..16 {
            let mut counter = 0;
            for x in 0..SECTION_WIDTH {
                for y in 0..SECTION_HEIGHT {
                    for z in 0..SECTION_WIDTH {
                        let block = BlockState::from_vanilla_id(counter).unwrap();
                        chunk.set_block_at(x, (section * SECTION_HEIGHT) + y, z, block, true);
                        assert_eq!(
                            chunk.block_at(x, (section * SECTION_HEIGHT) + y, z),
                            Some(block)
                        );
                        counter += 1;
                    }
                }
            }
        }

        // Go through again to be sure
        for section in 0..16 {
            assert!(chunk.section(section).is_some());
            let mut counter = 0;
            for x in 0..SECTION_WIDTH {
                for y in 0..SECTION_HEIGHT {
                    for z in 0..SECTION_WIDTH {
                        let block = BlockState::from_vanilla_id(counter).unwrap();
                        assert_eq!(
                            chunk.block_at(x, (section as usize * SECTION_HEIGHT) + y, z),
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
        for x in 0..SECTION_WIDTH {
            for y in 0..16 * SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
                    chunk.set_block_at(x, y, z, BlockState::air(), false);
                }
            }
        }
    }

    #[test]
    fn section_from_data_and_palette() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos, Sections(16), 0);

        let mut palette = PalettedContainer::new();
        let stone_index = palette.index_or_insert(BlockState::stone());

        let mut data = PackedArray::new(16 * SECTION_WIDTH * SECTION_WIDTH, 5.try_into().unwrap());
        for i in 0..4096 {
            data.set(i, stone_index as u64);
        }
        palette.set_data(data);

        let section =
            ChunkSection::new(palette, PalettedContainer::default(), 0, LightStore::new());
        chunk.set_section_at(0, section);

        for x in 0..SECTION_WIDTH {
            for y in 0..16 {
                for z in 0..SECTION_WIDTH {
                    assert_eq!(chunk.block_at(x, y, z).unwrap(), BlockState::stone());
                }
            }
        }
    }

    #[test]
    fn test_palette_insertion_in_middle() {
        let mut chunk = ChunkSection::default();

        chunk
            .set_block_at(0, 0, 0, BlockState::cobblestone())
            .unwrap();
        chunk.set_block_at(0, 1, 0, BlockState::stone()).unwrap();

        assert_eq!(chunk.block_at(0, 0, 0).unwrap(), BlockState::cobblestone());
        assert_eq!(chunk.block_at(0, 1, 0).unwrap(), BlockState::stone());
    }

    #[test]
    fn test_biomes() {
        let mut chunk = Chunk::new(ChunkPosition::default(), Sections(16), 0);

        for x in (0..SECTION_WIDTH).step_by(BIOME_SAMPLE_RATE) {
            for z in (0..SECTION_WIDTH).step_by(BIOME_SAMPLE_RATE) {
                assert_eq!(chunk.biome_at(x, 0, z), Some(0.into()));
                chunk.set_biome_at(x, 0, z, 1.into());
                assert_eq!(chunk.biome_at(x, 0, z), Some(1.into()));
            }
        }
    }

    #[test]
    fn test_light() {
        let mut chunk = Chunk::new(ChunkPosition::default(), Sections(16), 0);

        chunk
            .set_block_at(0, 0, 0, BlockState::stone(), true)
            .unwrap();

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
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0), Sections(16), 0);

        chunk.set_block_at(0, 10, 0, BlockState::stone(), true);
        assert_eq!(chunk.heightmaps.motion_blocking.height(0, 0), Some(10));
    }

    #[test]
    fn fill_chunk_section() {
        let mut section = ChunkSection::default();
        section.set_block_at(0, 0, 0, BlockState::stone());
        section.fill(BlockState::acacia_wood());

        for x in 0..CHUNK_WIDTH {
            for y in 0..SECTION_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    assert_eq!(section.block_at(x, y, z), Some(BlockState::acacia_wood()));
                }
            }
        }
    }
}
 */
