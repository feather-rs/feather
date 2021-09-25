use std::{convert::TryInto, iter};

use base::{
    chunk::{SECTION_HEIGHT, SECTION_VOLUME},
    BlockPosition, ChunkPosition, ValidBlockPosition,
};
use itertools::Either;

/// Event triggered when one or more blocks are changed.
///
/// This event can efficiently store bulk block updates
/// using a variety of different representations. Cloning
/// is cheap as it is, at worst, cloning an `Arc`.
#[derive(Debug, Clone)]
pub struct BlockChangeEvent {
    changes: BlockChanges,
}

impl BlockChangeEvent {
    /// Creates an event affecting a single block.
    pub fn single(pos: ValidBlockPosition) -> Self {
        Self {
            changes: BlockChanges::Single { pos },
        }
    }

    /// Creates an event corresponding to a block update
    /// that fills an entire chunk section with the same block.
    pub fn fill_chunk_section(chunk: ChunkPosition, section: u32) -> Self {
        Self {
            changes: BlockChanges::FillChunkSection { chunk, section },
        }
    }

    /// Determines the number of blocks that were
    /// changed in this block change event.
    pub fn count(&self) -> usize {
        match &self.changes {
            BlockChanges::Single { .. } => 1,
            BlockChanges::FillChunkSection { .. } => SECTION_VOLUME,
        }
    }

    /// Returns an iterator over block positions affected by this block change.
    pub fn iter_changed_blocks(&self) -> impl Iterator<Item = ValidBlockPosition> + '_ {
        match &self.changes {
            BlockChanges::Single { pos } => Either::Left(iter::once(*pos)),
            BlockChanges::FillChunkSection { chunk, section } => {
                Either::Right(iter_section_blocks(*chunk, *section))
            }
        }
    }

    /// Returns an iterator over chunk section positions affected by this block change.
    ///
    /// The yielded tuple consists of `(chunk, section_y, num_changed_blocks)`,
    /// where `num_changed_blocks` is the number of blocks changed within that chunk.
    pub fn iter_affected_chunk_sections(
        &self,
    ) -> impl Iterator<Item = (ChunkPosition, usize, usize)> + '_ {
        match &self.changes {
            BlockChanges::Single { pos } => {
                iter::once((pos.chunk(), pos.y() as usize / SECTION_HEIGHT, 1))
            }
            BlockChanges::FillChunkSection { chunk, section } => {
                iter::once((*chunk, *section as usize, SECTION_VOLUME))
            }
        }
    }
}

fn iter_section_blocks(
    chunk: ChunkPosition,
    section: u32,
) -> impl Iterator<Item = ValidBlockPosition> {
    (0..16)
        .flat_map(|x| (0..16).map(move |y| (x, y)))
        .flat_map(|(x, y)| (0..16).map(move |z| (x, y, z)))
        .map(move |(dx, dy, dz)| {
            let x = dx + chunk.x * 16;
            let y = dy + section as i32 * 16;
            let z = dz + chunk.z * 16;

            // It's safe to unwrap because we are working from a valid source of block positions
            BlockPosition::new(x, y, z).try_into().unwrap()
        })
}

#[derive(Debug, Clone)]
enum BlockChanges {
    /// A single block change.
    Single { pos: ValidBlockPosition },
    /// A whole chunk section was filled with the same block.
    FillChunkSection { chunk: ChunkPosition, section: u32 },
}

#[cfg(test)]
mod tests {
    use ahash::AHashSet;
    use base::chunk::SECTION_VOLUME;

    use super::*;

    #[test]
    fn create_single() {
        let pos = BlockPosition::new(5, 64, 9).try_into().unwrap();
        let event = BlockChangeEvent::single(pos);
        assert_eq!(event.count(), 1);
        assert_eq!(event.iter_changed_blocks().collect::<Vec<_>>(), vec![pos]);
        assert_eq!(
            event.iter_affected_chunk_sections().collect::<Vec<_>>(),
            vec![(pos.chunk(), 4, 1)]
        );
    }

    #[test]
    fn create_chunk_section_fill() {
        let chunk = ChunkPosition::new(10, 15);
        let section_y = 5;
        let event = BlockChangeEvent::fill_chunk_section(chunk, section_y);
        assert_eq!(event.count(), SECTION_VOLUME);
        assert_eq!(event.iter_changed_blocks().count(), SECTION_VOLUME);
        assert_eq!(
            event.iter_affected_chunk_sections().collect::<Vec<_>>(),
            vec![(chunk, section_y as usize, SECTION_VOLUME)]
        );
    }

    #[test]
    fn test_iter_section_blocks() {
        let blocks: Vec<ValidBlockPosition> =
            iter_section_blocks(ChunkPosition::new(-1, -2), 5).collect();
        let unique_blocks: AHashSet<ValidBlockPosition> = blocks.iter().copied().collect();

        assert_eq!(blocks.len(), unique_blocks.len());
        assert_eq!(blocks.len(), SECTION_VOLUME);

        for x in -16..0 {
            for y in 80..96 {
                for z in -32..-16 {
                    assert!(
                        unique_blocks.contains(&BlockPosition::new(x, y, z).try_into().unwrap()),
                        "{}, {}, {}",
                        x,
                        y,
                        z
                    );
                }
            }
        }
    }
}
