use std::{iter, mem::MaybeUninit, ptr::NonNull};

use component::ComponentTypeId;

use crate::component::{self, ComponentMeta};

use super::blob_array::BlobArray;

/// Stores components in a sparse set.
pub struct SparseSetStorage {
    sparse: Vec<u32>,
    dense: Vec<u32>,
    components: BlobArray,
    component_meta: ComponentMeta,
}

impl SparseSetStorage {
    pub fn new(component_meta: ComponentMeta) -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            components: BlobArray::new(component_meta.layout, 1),
            component_meta,
        }
    }

    pub fn insert<T: 'static>(&mut self, index: u32, value: T) {
        self.assert_type_matches::<T>();
        let value = MaybeUninit::new(value);
        unsafe {
            self.insert_raw(index, value.as_ptr().cast());
        }
    }

    pub unsafe fn insert_raw(&mut self, index: u32, component: *const u8) {
        self.grow_sparse_for(index);

        self.sparse[index as usize] = self.dense.len() as u32;
        self.components.push_raw(component);
        self.dense.push(index);
    }

    pub fn get<T: 'static>(&self, index: u32) -> Option<&T> {
        self.assert_type_matches::<T>();
        unsafe {
            let ptr = self.get_raw(index)?;
            Some(&*ptr.as_ptr().cast())
        }
    }

    pub fn get_mut<T: 'static>(&mut self, index: u32) -> Option<&mut T> {
        self.assert_type_matches::<T>();
        unsafe {
            let ptr = self.get_raw(index)?;
            Some(&mut *ptr.as_ptr().cast())
        }
    }

    pub fn get_raw(&self, index: u32) -> Option<NonNull<u8>> {
        let sparse = *self.sparse.get(index as usize)?;
        let dense = *self.dense.get(sparse as usize)?;

        if dense == index {
            self.components.get_raw(sparse as usize)
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: u32) -> Option<()> {
        let sparse = *self.sparse.get(index as usize)?;
        let dense = self.dense.get_mut(sparse as usize)?;

        if *dense == index {
            *dense = u32::MAX;
            Some(())
        } else {
            None
        }
    }

    fn grow_sparse_for(&mut self, index: u32) {
        if index >= self.sparse.len() as u32 {
            let needed_padding = index as usize - self.sparse.len() + 1;
            self.sparse
                .extend(iter::repeat(u32::MAX).take(needed_padding));
        }
    }

    fn assert_type_matches<T: 'static>(&self) {
        assert_eq!(ComponentTypeId::of::<T>(), self.component_meta.type_id);
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::Layout;

    use super::*;

    #[test]
    fn insert_and_get() {
        let mut storage = SparseSetStorage::new(ComponentMeta::of::<&'static str>());

        let entity_a = 10;
        let entity_b = 15;

        storage.insert(entity_b, "entity b");
        storage.insert(entity_a, "entity a");

        assert_eq!(storage.get::<&'static str>(entity_b).unwrap(), &"entity b");
        assert_eq!(storage.get::<&'static str>(entity_a).unwrap(), &"entity a");

        storage.remove(entity_a);
        assert_eq!(storage.get::<&'static str>(entity_b).unwrap(), &"entity b");
        assert_eq!(storage.get::<&'static str>(entity_a), None);
    }
}
