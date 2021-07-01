use blocks::BlockId;

/// Stores the set of distinct `BlockId`s in a chunk section.
///
/// Empty entries in the palette default to air.
///
/// The entry with index 0 is always air.
#[derive(Debug, Clone)]
pub struct Palette {
    blocks: Vec<BlockId>,
    free_indices: Vec<usize>,
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::len_without_is_empty)] // palette is never empty
impl Palette {
    /// Creates an empty palette.
    pub fn new() -> Self {
        Self {
            blocks: vec![BlockId::air()],
            free_indices: Vec::new(),
        }
    }

    /// Gets the blocks in this palette as a slice.
    pub fn as_slice(&self) -> &[BlockId] {
        &self.blocks
    }

    /// Gets the index in the palette of `block`.
    /// Inserts the block into the palette if it
    /// does not already exist.
    pub fn index_or_insert(&mut self, block: BlockId) -> usize {
        match self.index_of(block) {
            Some(i) => i,
            None => self.insert(block),
        }
    }

    fn insert(&mut self, block: BlockId) -> usize {
        match self.free_indices.pop() {
            Some(i) => {
                self.blocks[i] = block;
                i
            }
            None => {
                let i = self.blocks.len();
                self.blocks.push(block);
                i
            }
        }
    }

    /// Gets the block at index `i`, or air if
    /// the palette does not contain `i`.
    pub fn get(&self, index: usize) -> BlockId {
        self.blocks.get(index).copied().unwrap_or_else(BlockId::air)
    }

    /// Gets the number of blocks in the palette.
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Removes the given block from this palette.
    pub fn remove(&mut self, block: BlockId) {
        if let Some(index) = self.index_of(block) {
            self.blocks[index] = BlockId::air();
            self.free_indices.push(index);
        }
    }

    /// Clears the palette, leaving only the air entry at index 0.
    pub fn clear(&mut self) {
        self.blocks.clear();
        self.blocks.push(BlockId::air());
    }

    fn index_of(&self, block: BlockId) -> Option<usize> {
        self.blocks.iter().position(|b| *b == block)
    }
}

#[cfg(test)]
mod tests {
    use ahash::AHashMap;
    use blocks::BlockId;

    use super::*;

    #[test]
    fn add_blocks() {
        let mut palette = Palette::new();

        for i in 0..100 {
            let index = palette.index_or_insert(BlockId::from_vanilla_id(i));
            assert_eq!(index, i as usize);
            assert_eq!(palette.get(index), BlockId::from_vanilla_id(i));
        }

        assert_eq!(palette.len(), 100);
    }

    #[test]
    fn empty_entries_are_air() {
        let palette = Palette::new();
        assert_eq!(palette.get(0), BlockId::air());
    }

    #[test]
    fn remove_blocks() {
        let mut palette = Palette::new();

        let mut mapping: AHashMap<BlockId, usize> = AHashMap::new();

        for i in 0..100 {
            let block = BlockId::from_vanilla_id(i);
            let index = palette.index_or_insert(BlockId::from_vanilla_id(i));
            if i % 2 == 0 {
                palette.remove(block);
            } else {
                mapping.insert(block, index);
            }
        }

        assert_eq!(palette.len(), 50);

        for i in 0..100 {
            if i % 2 == 0 {
                continue;
            }
            let block = BlockId::from_vanilla_id(i);
            assert_eq!(palette.index_or_insert(block), mapping[&block]);
        }
    }

    #[test]
    fn clear() {
        let mut palette = Palette::new();
        for id in 100..200 {
            palette.index_or_insert(BlockId::from_vanilla_id(id));
        }
        palette.clear();
        assert_eq!(palette.len(), 1);
        assert_eq!(palette.index_of(BlockId::air()), Some(0));
    }
}
