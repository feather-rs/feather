use crate::ChunkSection;

use super::{PackedArray, SECTION_VOLUME};

/// Contains light data for a chunk section.
#[derive(Debug, Clone)]
pub struct LightStore {
    block_light: PackedArray,
    sky_light: PackedArray,
}

impl LightStore {
    /// Creates a `LightStore` with all light set to 15.
    pub fn new() -> Self {
        let mut this = LightStore {
            block_light: PackedArray::new(SECTION_VOLUME, 4),
            sky_light: PackedArray::new(SECTION_VOLUME, 4),
        };
        fill_with_default_light(&mut this.block_light);
        fill_with_default_light(&mut this.sky_light);
        this
    }

    /// Creates a `LightStore` from packed arrays.
    pub fn from_packed_arrays(block_light: PackedArray, sky_light: PackedArray) -> Option<Self> {
        if block_light.len() != SECTION_VOLUME
            || sky_light.len() != SECTION_VOLUME
            || block_light.bits_per_value() != 4
            || sky_light.bits_per_value() != 4
        {
            None
        } else {
            Some(Self {
                block_light,
                sky_light,
            })
        }
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.block_light.get(index).map(|x| x as u8)
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.sky_light.get(index).map(|x| x as u8)
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.block_light.set(index, light.min(15) as u64);
        Some(())
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.sky_light.set(index, light.min(15) as u64);
        Some(())
    }

    pub fn block_light(&self) -> &PackedArray {
        &self.block_light
    }

    pub fn sky_light(&self) -> &PackedArray {
        &self.sky_light
    }
}

fn fill_with_default_light(arr: &mut PackedArray) {
    for i in 0..arr.len() {
        arr.set(i, 15);
    }
}
