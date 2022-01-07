use blocks::BlockId;

use crate::ChunkSection;

use super::{
    PackedArray, Palette, GLOBAL_BITS_PER_BLOCK, MAX_BITS_PER_BLOCK, MIN_BITS_PER_BLOCK,
    SECTION_VOLUME,
};

/// Stores the blocks of a chunk section.
#[derive(Debug, Clone)]
pub struct BlockStore {
    /// `None` if using the global palette
    palette: Option<Palette>,

    /// Stores indices into `palette`, or just block IDs
    /// if using the global palette
    blocks: PackedArray,

    air_block_count: u32,
}

impl Default for BlockStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockStore {
    /// Creates a new `BlockStore` containing air.
    pub fn new() -> Self {
        Self {
            palette: Some(Palette::new()),
            blocks: PackedArray::new(SECTION_VOLUME, MIN_BITS_PER_BLOCK as usize),
            air_block_count: SECTION_VOLUME as u32,
        }
    }

    /// Creates a new `BlockStore` from the palette
    /// and data array.
    pub fn from_raw_parts(palette: Option<Palette>, blocks: PackedArray) -> Self {
        let air_block_count = Self::count_air_blocks(&blocks, &palette);
        Self {
            palette,
            blocks,
            air_block_count,
        }
    }

    pub fn data(&self) -> &PackedArray {
        &self.blocks
    }

    pub fn data_mut(&mut self) -> &mut PackedArray {
        &mut self.blocks
    }

    pub fn palette(&self) -> Option<&Palette> {
        self.palette.as_ref()
    }

    pub fn palette_mut(&mut self) -> Option<&mut Palette> {
        self.palette.as_mut()
    }

    fn count_air_blocks(blocks: &PackedArray, palette: &Option<Palette>) -> u32 {
        let mut count = 0;
        blocks.iter().for_each(|x| {
            let block = match palette {
                Some(p) => p.get(x as usize),
                None => BlockId::from_vanilla_id(x as u16),
            };
            if block.is_air() {
                count += 1;
            }
        });
        count
    }

    pub fn air_blocks(&self) -> u32 {
        self.air_block_count
    }

    pub fn set_air_blocks(&mut self, new_value: u32) {
        self.air_block_count = new_value;
    }

    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Option<BlockId> {
        let index = ChunkSection::block_index(x, y, z)?;
        let block_index = self.blocks.get(index).expect("block_index out of bounds?");

        Some(match &self.palette {
            Some(palette) => palette.get(block_index as usize),
            None => BlockId::from_vanilla_id(block_index as u16),
        })
    }

    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockId) -> Option<()> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.update_air_block_count(x, y, z, block);

        let block_index = self.get_block_palette_index(block);
        self.blocks.set(index, block_index as u64);

        Some(())
    }

    pub fn fill(&mut self, block: BlockId) {
        let index = if let Some(ref mut palette) = self.palette {
            palette.clear();
            palette.index_or_insert(block)
        } else {
            self.palette = Some(Palette::new());
            self.palette.as_mut().unwrap().index_or_insert(block)
        };

        self.blocks.fill(index as u64);

        if block.is_air() {
            self.air_block_count = SECTION_VOLUME as u32;
        } else {
            self.air_block_count = 0;
        }
    }

    fn get_block_palette_index(&mut self, block: BlockId) -> usize {
        match &mut self.palette {
            Some(p) => {
                let index = p.index_or_insert(block);
                self.resize_if_needed();
                index
            }
            None => block.vanilla_id() as usize,
        }
    }

    fn resize_if_needed(&mut self) {
        let palette = self.palette.as_ref().unwrap();

        if palette.len() - 1 > self.blocks.max_value() as usize {
            // Resize to either the global palette or a new section palette size.
            let new_size = self.blocks.bits_per_value() + 1;
            if new_size > MAX_BITS_PER_BLOCK as usize {
                self.use_global_palette();
            } else {
                self.blocks = self.blocks.resized(new_size);
            }
        }
    }

    fn use_global_palette(&mut self) {
        self.blocks = self.blocks.resized(GLOBAL_BITS_PER_BLOCK as usize);
        let palette = self.palette.as_ref().unwrap();

        // Update blocks to use vanilla IDs instead of palette indices
        for i in 0..SECTION_VOLUME {
            let block = palette.get(self.blocks.get(i).unwrap() as usize);
            self.blocks.set(i, block.vanilla_id() as u64);
        }

        self.palette = None;
    }

    fn update_air_block_count(&mut self, x: usize, y: usize, z: usize, new: BlockId) {
        let old = self.block_at(x, y, z).unwrap();
        if old.is_air() && !new.is_air() {
            self.air_block_count -= 1;
        } else if !old.is_air() && new.is_air() {
            self.air_block_count += 1;
        }
    }
}
