use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationalIndex {
    index: usize,
    generation: u32,
}

impl GenerationalIndex {
    pub fn index(&self) -> usize {
        self.index
    }
}

struct AllocatorEntry {
    is_live: bool,
    generation: u32,
}

pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            free: vec![],
        }
    }
    pub fn allocate(&mut self) -> GenerationalIndex {
        if self.free.is_empty() {
            self.entries.push(AllocatorEntry {
                is_live: true,
                generation: 0,
            });

            GenerationalIndex {
                index: self.entries.len() - 1,
                generation: 0,
            }
        } else {
            let index = self.free.pop().unwrap();
            let entry = &mut self.entries[index];

            entry.is_live = true;
            entry.generation += 1;

            GenerationalIndex {
                index,
                generation: entry.generation,
            }
        }
    }

    pub fn deallocate(&mut self, genindex: GenerationalIndex) {
        let index = genindex.index;

        assert_eq!(self.entries[index].generation, genindex.generation);

        self.free.push(index);
        self.entries[index].is_live = false;
    }
}

struct ArrayEntry<T> {
    value: T,
    generation: u32,
}

pub struct GenerationalArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalArray<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        if self.0.len() >= index.index() {
            self.0.push(None);
        }

        self.0[index.index()] = Some(ArrayEntry {
            value,
            generation: index.generation,
        });
    }

    pub fn remove(&mut self, index: GenerationalIndex) -> Option<T> {
        let entry = &mut self.0[index.index()];
        if let Some(entry) = entry {
            if entry.generation == index.generation {
                let entry = self.0.get_mut(index.index()).unwrap().take().unwrap();
                return Some(entry.value)
            }
        }

        None
    }

    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        let entry = self.0.get(index.index());

        if entry.is_none() {
            return None;
        }

        let entry = entry.unwrap();
        if let Some(entry) = entry {
            if entry.generation != index.generation {
                return None;
            }

            Some(&entry.value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        let entry = self.0.get_mut(index.index());

        if entry.is_none() {
            return None;
        }

        let entry = entry.unwrap();
        if let Some(entry) = entry {
            if entry.generation != index.generation {
                return None;
            }

            Some(&mut entry.value)
        } else {
            None
        }
    }
}

impl<T> Index<GenerationalIndex> for GenerationalArray<T> {
    type Output = T;

    fn index(&self, index: GenerationalIndex) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<GenerationalIndex> for GenerationalArray<T> {
    fn index_mut(&mut self, index: GenerationalIndex) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator() {
        let mut allocator = GenerationalIndexAllocator::new();

        let index = allocator.allocate();
        assert_eq!(index.generation, 0);
        assert_eq!(index.index(), 0);

        let index2 = allocator.allocate();
        assert_eq!(index2.generation, 0);
        assert_eq!(index2.index(), 1);

        allocator.deallocate(index);
        let index3 = allocator.allocate();
        assert_eq!(index.index(), index3.index());
        assert_eq!(index3.generation, 1);
    }

    #[test]
    fn test_array() {
        let mut arr = GenerationalArray::new();
        let mut alloc = GenerationalIndexAllocator::new();

        let index1 = alloc.allocate();
        arr.set(index1, "test");
        assert_eq!(arr.get(index1), Some(&"test"));

        alloc.deallocate(index1);

        let index2 = alloc.allocate();
        arr.set(index2, "test2");
        assert_eq!(arr.get(index2), Some(&"test2"));
        assert_eq!(arr.get(index1), None);
    }
}
