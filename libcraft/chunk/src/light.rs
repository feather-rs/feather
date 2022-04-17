use crate::ChunkSection;
use std::convert::TryInto;

use super::{PackedArray, SECTION_VOLUME};

/// Contains light data for a chunk section.
#[derive(Debug, Clone)]
pub struct LightStore {
    /// Could be None in some dimensions (see [has_skylight](crate::common::world::DimensionTypeInfo.has_skylight))
    /// or when you get this packet from deserialization of the LightData packet
    sky_light: Option<PackedArray>,
    /// Could be None when you get this packet from deserialization of the LightData packet
    block_light: Option<PackedArray>,
}

impl Default for LightStore {
    fn default() -> Self {
        Self::new()
    }
}

impl LightStore {
    /// Creates a `LightStore` with sky light set to 15.
    pub fn new() -> Self {
        let mut this = LightStore {
            sky_light: Some(PackedArray::new(SECTION_VOLUME, 4.try_into().unwrap())),
            block_light: Some(PackedArray::new(SECTION_VOLUME, 4.try_into().unwrap())),
        };
        fill_with_default_light(this.sky_light.as_mut().unwrap());
        fill_with_default_light(this.block_light.as_mut().unwrap());
        this
    }
    /// Creates a `LightStore` with all light set to 0.
    pub fn empty() -> Self {
        LightStore {
            block_light: Some(PackedArray::new(SECTION_VOLUME, 4.try_into().unwrap())),
            sky_light: Some(PackedArray::new(SECTION_VOLUME, 4.try_into().unwrap())),
        }
    }

    /// Creates a `LightStore` from packed arrays.
    pub fn from_packed_arrays(
        sky_light: Option<PackedArray>,
        block_light: Option<PackedArray>,
    ) -> Option<Self> {
        if (sky_light.is_some()
            && (sky_light.as_ref().unwrap().len() != SECTION_VOLUME
                || sky_light.as_ref().unwrap().bits_per_value() != 4.try_into().unwrap()))
            || (block_light.is_some()
                && (block_light.as_ref().unwrap().len() != SECTION_VOLUME
                    || block_light.as_ref().unwrap().bits_per_value() != 4.try_into().unwrap()))
        {
            None
        } else {
            Some(Self {
                block_light,
                sky_light,
            })
        }
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.sky_light.as_ref()?.get(index).map(|x| x as u8)
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.block_light.as_ref()?.get(index).map(|x| x as u8)
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.sky_light.as_mut()?.set(index, light.min(15) as u64);
        Some(())
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, light: u8) -> Option<()> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.block_light.as_mut()?.set(index, light.min(15) as u64);
        Some(())
    }

    pub fn sky_light(&self) -> Option<&PackedArray> {
        self.sky_light.as_ref()
    }

    pub fn block_light(&self) -> Option<&PackedArray> {
        self.block_light.as_ref()
    }

    pub fn sky_light_mut(&mut self) -> Option<&mut PackedArray> {
        self.sky_light.as_mut()
    }

    pub fn block_light_mut(&mut self) -> Option<&mut PackedArray> {
        self.block_light.as_mut()
    }
}

fn fill_with_default_light(arr: &mut PackedArray) {
    arr.fill(15);
}
