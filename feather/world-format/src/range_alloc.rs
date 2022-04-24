use bitvec::prelude::*;

pub type Range = std::ops::Range<u32>;

/// An allocator over a fixed address space.
///
/// Used to allocate sectors within region files.
pub struct RangeAllocator {
    sectors: BitVec,
}

impl RangeAllocator {
    pub fn new(size: u32) -> Self {
        let sectors = bitvec![usize, Lsb0; 0; size as usize];
        Self { sectors }
    }

    pub fn size(&self) -> u32 {
        self.sectors.len() as u32
    }

    /// Resizes the allocator.
    pub fn grow_to(&mut self, new_size: u32) {
        assert!(new_size >= self.size(), "cannot shrink a range allocator");
        let excess = new_size - self.size();
        let zeros = bitvec![usize, Lsb0; 0; excess as usize];
        self.sectors.extend_from_bitslice(&zeros);
        assert_eq!(self.size(), new_size);
    }

    /// Allocates a new range.
    pub fn allocate(&mut self, size: u32) -> Option<Range> {
        let mut slice = &mut self.sectors[..];
        let mut offset = 0;
        while let Some(index) = slice.first_zero() {
            if index + size as usize > slice.len() {
                return None;
            }

            // Check if there is enough space in this free block
            for i in index..index + size as usize {
                if slice[i] {
                    slice = &mut slice[i..];
                    offset += i;
                    continue;
                }
            }

            // Mark sectors as used, then return the new range.
            for i in index..index + size as usize {
                slice.set(i, true);
            }

            return Some(Range {
                start: offset as u32 + index as u32,
                end: offset as u32 + index as u32 + size,
            });
        }

        None
    }

    /// Deallocates a range.
    pub fn deallocate(&mut self, range: Range) {
        for i in range {
            assert!(self.sectors[i as usize]);
            self.sectors.set(i as usize, false);
        }
    }

    /// Marks the given block as used.
    pub fn mark_used(&mut self, range: Range) {
        for i in range {
            self.sectors.set(i as usize, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_consecutive() {
        let mut alloc = RangeAllocator::new(101);

        for i in 0..10 {
            let block = alloc.allocate(10).unwrap();
            assert_eq!(block.start, i * 10);
            assert_eq!(block.end, i * 10 + 10);
        }

        assert!(alloc.allocate(10).is_none());

        alloc.deallocate(90..100);
        alloc.allocate(10).unwrap();

        assert!(alloc.allocate(10).is_none());

        alloc.allocate(1).unwrap();
    }
}
