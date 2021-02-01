use std::cell::Cell;

/// An entity's ID used by the protocol
/// in `entity_id` fields.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkId(pub i32);

/// Allocator for `NetworkId`s that ensures entity IDs are unique.
#[derive(Debug, Default)]
pub struct NetworkIdAllocator {
    next: Cell<i32>,
}

impl NetworkIdAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate_id(&mut self) -> NetworkId {
        let id = self.next.get();
        self.next.set(id.wrapping_add(1));
        NetworkId(id)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn ensure_uniqueness() {
        let mut used = HashSet::new();
        let mut allocator = NetworkIdAllocator::new();
        for _ in 0..100 {
            assert!(used.insert(allocator.allocate_id()));
        }
    }
}
