use std::convert::TryInto;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};

use libcraft_blocks::{BlockState, HIGHEST_ID};

use crate::biome::BiomeId;
use crate::{ChunkSection, PackedArray, BIOMES_PER_CHUNK_SECTION, SECTION_VOLUME};

/// Stores blocks or biomes of a chunk section.
/// N = 4 for blocks, 2 for biomes
#[derive(Debug, Clone, PartialEq)]
pub enum PalettedContainer<T>
where
    T: Paletteable,
{
    SingleValue(T),
    MultipleValues { data: PackedArray, palette: Vec<T> },
    GlobalPalette { data: PackedArray },
}

impl<T> PalettedContainer<T>
where
    T: Paletteable,
{
    /// Creates a new empty `BlockStore`.
    pub fn new() -> Self {
        Self::SingleValue(T::default())
    }

    /// Gets the palette. None if using global or single-value palette
    pub fn palette(&self) -> Option<&Vec<T>> {
        match self {
            PalettedContainer::MultipleValues { palette, .. } => Some(palette),
            _ => None,
        }
    }

    /// Gets a mutable reference to the palette. None if using global or single-value palette
    pub fn palette_mut(&mut self) -> Option<&mut Vec<T>> {
        match self {
            PalettedContainer::MultipleValues { palette, .. } => Some(palette),
            _ => None,
        }
    }

    /// Gets the inner representation of this container's data.
    /// None if using single-value palette
    pub fn data(&self) -> Option<&PackedArray> {
        match self {
            PalettedContainer::MultipleValues { data, .. }
            | PalettedContainer::GlobalPalette { data } => Some(data),
            _ => None,
        }
    }

    /// Gets a mutable reference to the inner representation of this container's data.
    /// None if using single-value palette
    pub fn data_mut(&mut self) -> Option<&mut PackedArray> {
        match self {
            PalettedContainer::MultipleValues { data, .. }
            | PalettedContainer::GlobalPalette { data } => Some(data),
            _ => None,
        }
    }

    /// Sets the inner representation of this container's data.
    /// Returns false if the container is single-value, true otherwise
    pub fn set_data(&mut self, new_data: PackedArray) -> bool {
        match self {
            PalettedContainer::MultipleValues { data, .. }
            | PalettedContainer::GlobalPalette { data } => {
                *data = new_data;
                true
            }
            _ => false,
        }
    }

    /// Gets the entry at this index
    pub fn get(&self, index: usize) -> Option<T> {
        match self {
            PalettedContainer::SingleValue(item) => Some(*item),
            PalettedContainer::MultipleValues { data, palette } => {
                let palette_index = data.get(index)?;
                Some(
                    palette
                        .get(palette_index as usize)
                        .copied()
                        .unwrap_or_else(|| {
                            panic!(
                                "palette does not contain entry {} (see: {:?})",
                                palette_index, palette
                            )
                        }),
                )
            }
            PalettedContainer::GlobalPalette { data } => {
                let palette_index = data.get(index)? as u32;
                T::from_default_palette(palette_index)
            }
        }
    }

    /// Sets the entry at this index
    pub fn set(&mut self, index: usize, item: T) {
        let palette_index = self.index_or_insert(item);
        match self {
            PalettedContainer::SingleValue(value) if *value == item => {}
            PalettedContainer::SingleValue(_) => unreachable!(),
            PalettedContainer::MultipleValues { data, .. } => data.set(index, palette_index as u64),
            PalettedContainer::GlobalPalette { data } => {
                data.set(index, item.default_palette_index() as u64)
            }
        }
    }

    /// Inserts a value into this container's palette.
    pub fn index_or_insert(&mut self, item: T) -> u32 {
        if let Some(index) = self.get_palette_index(item) {
            index
        } else {
            self.resize_if_needed();
            match self {
                PalettedContainer::SingleValue(_) => unreachable!(),
                PalettedContainer::MultipleValues { palette, .. } => {
                    assert!(!palette.contains(&item));
                    palette.push(item);
                    palette.len() as u32 - 1
                }
                PalettedContainer::GlobalPalette { .. } => item.default_palette_index(),
            }
        }
    }

    /// Fills this storage with the item
    pub fn fill(&mut self, item: T) {
        *self = PalettedContainer::SingleValue(item)
    }

    /// How many values does this container store
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        T::ENTRIES_PER_CHUNK_SECTION
    }

    pub fn global_palette_bits_per_value() -> NonZeroUsize {
        Self::palette_bits_per_value(T::length())
    }

    pub fn palette_bits_per_value(palette_len: usize) -> NonZeroUsize {
        ((palette_len as f64).log2().ceil() as usize)
            .max(T::MIN_BITS_PER_ENTRY.get())
            .try_into()
            .unwrap()
    }

    fn get_palette_index(&mut self, item: T) -> Option<u32> {
        match self {
            PalettedContainer::SingleValue(value) if *value == item => Some(0),
            PalettedContainer::SingleValue(_) => None,
            PalettedContainer::MultipleValues { palette, .. } => palette
                .iter()
                .position(|value| *value == item)
                .map(|i| i as u32),
            PalettedContainer::GlobalPalette { .. } => Some(item.default_palette_index()),
        }
    }

    fn resize_if_needed(&mut self) {
        let len = self.len();
        match self {
            PalettedContainer::SingleValue(value) => {
                *self = PalettedContainer::MultipleValues {
                    data: PackedArray::new(len, T::MIN_BITS_PER_ENTRY),
                    palette: vec![*value],
                }
            }
            PalettedContainer::MultipleValues { data, palette } => {
                if palette.len() >= data.max_value() as usize {
                    // Resize to either the global palette or a new section palette size.
                    let new_size = (data.bits_per_value().get() + 1).try_into().unwrap();
                    if new_size <= T::MAX_BITS_PER_ENTRY {
                        *data = data.resized(new_size);
                    } else {
                        *self = Self::GlobalPalette {
                            data: {
                                let mut data = data.resized(Self::global_palette_bits_per_value());
                                // Update items to use global IDs instead of palette indices
                                Self::map_to_global_palette(len, palette, &mut data);
                                data
                            },
                        }
                    }
                }
            }
            PalettedContainer::GlobalPalette { .. } => {}
        }
    }

    pub fn map_to_global_palette(len: usize, palette: &[T], data: &mut PackedArray) {
        for i in 0..len {
            let palette_index = data.get(i).unwrap() as usize;
            let item = palette.get(palette_index).copied().unwrap();
            data.set(i, item.default_palette_index() as u64);
        }
    }

    pub fn map_from_global_palette(len: usize, palette: &[T], data: &mut PackedArray) {
        for i in 0..len {
            let palette_index = data.get(i).unwrap();
            let item = T::from_default_palette(palette_index as u32).unwrap();
            data.set(i, palette.iter().position(|it| *it == item).unwrap() as u64);
        }
    }
}

impl PalettedContainer<BlockState> {
    /// Gets the block at this position
    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> Option<BlockState> {
        let index = ChunkSection::block_index(x, y, z)?;

        self.get(index)
    }

    /// Sets the block at this position
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockState) -> Option<()> {
        let index = ChunkSection::block_index(x, y, z)?;
        self.set(index, block);
        Some(())
    }
}

impl PalettedContainer<BiomeId> {
    /// Gets the biome at this position
    pub fn biome_at(&self, x: usize, y: usize, z: usize) -> Option<BiomeId> {
        let index = ChunkSection::biome_index(x, y, z)?;
        self.get(index)
    }

    /// Sets the biome at this position
    pub fn set_biome_at(&mut self, x: usize, y: usize, z: usize, biome: BiomeId) -> Option<()> {
        let index = ChunkSection::biome_index(x, y, z)?;
        self.set(index, biome);
        Some(())
    }
}

impl<T> Default for PalettedContainer<T>
where
    T: Paletteable,
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait Paletteable: Default + Copy + PartialEq + Debug {
    const MIN_BITS_PER_ENTRY: NonZeroUsize;
    // FIXME replace with .unwrap() when it's const stable
    // SAFETY: non-zero * 2 = non-zero
    const MAX_BITS_PER_ENTRY: NonZeroUsize =
        unsafe { NonZeroUsize::new_unchecked(Self::MIN_BITS_PER_ENTRY.get() * 2) };
    const ENTRIES_PER_CHUNK_SECTION: usize;

    fn from_default_palette(index: u32) -> Option<Self>;
    fn default_palette_index(&self) -> u32;
    fn length() -> usize;
}

impl Paletteable for BlockState {
    // SAFETY: 4 is non-zero
    const MIN_BITS_PER_ENTRY: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(4) };
    const ENTRIES_PER_CHUNK_SECTION: usize = SECTION_VOLUME;

    fn from_default_palette(index: u32) -> Option<Self> {
        Self::from_id(index as u16)
    }

    fn default_palette_index(&self) -> u32 {
        self.id() as u32
    }

    fn length() -> usize {
        HIGHEST_ID.into()
    }
}

pub static BIOMES_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Paletteable for BiomeId {
    // SAFETY: 2 is non-zero
    const MIN_BITS_PER_ENTRY: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(2) };
    const ENTRIES_PER_CHUNK_SECTION: usize = BIOMES_PER_CHUNK_SECTION;

    fn from_default_palette(index: u32) -> Option<Self> {
        Some(Self::from(index as usize))
    }

    fn default_palette_index(&self) -> u32 {
        **self as u32
    }

    fn length() -> usize {
        BIOMES_COUNT.load(Ordering::Relaxed)
    }
}

/*
#[cfg(test)]
mod tests {
    use libcraft_blocks::BlockState;

    use crate::chunk::paletted_container::PalettedContainer;

    #[test]
    fn test() {
        let mut container = PalettedContainer::<BlockState>::new();
        assert_eq!(container, PalettedContainer::default());
        container
            .set_block_at(10, 10, 5, BlockState::stone())
            .unwrap();
        assert_eq!(
            container.palette().unwrap(),
            &vec![BlockState::default(), BlockState::stone()]
        );
        assert_eq!(
            container.get_block_at(10, 10, 5).unwrap(),
            BlockState::stone()
        );
        for id in 0..256 {
            container
                .set_block_at(
                    id / 16,
                    0,
                    id % 16,
                    BlockState::from_vanilla_id(id as u16).unwrap(),
                )
                .unwrap();
        }
        assert!(matches!(container, PalettedContainer::GlobalPalette { .. }));
        for id in 0..256 {
            assert_eq!(
                container.get_block_at(id / 16, 0, id % 16).unwrap(),
                BlockState::from_vanilla_id(id as u16).unwrap()
            );
        }
    }
}
*/
