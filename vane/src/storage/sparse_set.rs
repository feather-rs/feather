use core::slice;
use std::{
    any::TypeId,
    iter::{self, Enumerate},
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
};

use once_cell::sync::Lazy;
use thread_local::ThreadLocal;

use crate::{
    borrow::BorrowFlag,
    component::{self, ComponentMeta},
    BorrowError, ComponentError, Ref, RefMut,
};

use super::{blob_array::BlobArray, component_vec::ComponentVec};

static EMPTY: Lazy<ThreadLocal<SparseSetStorage>> = Lazy::new(ThreadLocal::new);

/// Stores components in a sparse set.
pub struct SparseSetStorage {
    // Data structure invariant: if `dense[sparse[i]] == i`,
    // then the storage contains a component for the entity with index `i`,
    // and that component is stored at `components[sparse[i]]`.
    //
    // It follows that the component at `components[i]` belongs to
    // the entity index at `dense[i]` for all `i`.
    sparse: Vec<u32>,
    dense: Vec<u32>,
    components: ComponentVec,
    borrow_flags: Vec<BorrowFlag>,
}

impl SparseSetStorage {
    pub fn new(component_meta: ComponentMeta) -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            components: ComponentVec::new(component_meta),
            borrow_flags: Vec::new(),
        }
    }

    /// The empty sparse set.
    /// Attempting to access components from this sparse
    /// set will usually cause a panic.
    pub fn empty() -> &'static Self {
        EMPTY.get_or(|| SparseSetStorage::new(ComponentMeta::of::<()>()))
    }

    pub fn insert<T: 'static>(&mut self, index: u32, value: T) {
        self.assert_type_matches::<T>();
        let value = MaybeUninit::new(value);
        unsafe {
            self.insert_raw(index, value.as_ptr().cast());
        }
    }

    /// # Safety
    /// `component` must point to a valid (but not necessarily aligned) instance of the component
    /// stored in this sparse set.
    pub unsafe fn insert_raw(&mut self, index: u32, component: *const u8) -> *mut u8 {
        self.grow_sparse_for(index);

        if let Some((existing_component, _)) = self.get_raw(index) {
            // Component already exists for this index; overwrite it.
            ptr::drop_in_place(existing_component.as_ptr());
            ptr::copy_nonoverlapping(
                component,
                existing_component.as_ptr(),
                self.components.component_meta().layout.size(),
            );
            existing_component.as_ptr()
        } else {
            self.sparse[index as usize] = self.dense.len() as u32;
            let ptr = self.components.push(component);
            self.dense.push(index);
            self.borrow_flags.push(BorrowFlag::default());

            ptr
        }
    }

    pub fn get<T: 'static>(&self, index: u32) -> Result<Option<Ref<T>>, BorrowError> {
        self.assert_type_matches::<T>();
        unsafe {
            let (ptr, borrow_flag) = match self.get_raw(index) {
                Some(x) => x,
                None => return Ok(None),
            };

            borrow_flag.borrow()?;
            let component = &*ptr.as_ptr().cast();

            Ok(Some(Ref::new(component, borrow_flag)))
        }
    }

    pub fn get_mut<T: 'static>(&self, index: u32) -> Result<Option<RefMut<T>>, BorrowError> {
        self.assert_type_matches::<T>();
        unsafe {
            let (ptr, borrow_flag) = match self.get_raw(index) {
                Some(x) => x,
                None => return Ok(None),
            };

            borrow_flag.borrow_mut()?;
            let component = &mut *ptr.as_ptr().cast();

            Ok(Some(RefMut::new(component, borrow_flag)))
        }
    }

    pub(crate) fn get_raw(&self, index: u32) -> Option<(NonNull<u8>, &BorrowFlag)> {
        let sparse = *self.sparse.get(index as usize)?;
        let dense = *self.dense.get(sparse as usize)?;

        if dense == index {
            // SAFETY: by the data structure invariant,
            // `sparse` exists in `components` if `dense[sparse] == index`.
            let component = unsafe { self.components.get_unchecked(sparse) };
            let borrow_flag = unsafe { self.borrow_flags.get_unchecked(sparse as usize) };

            Some((component, borrow_flag))
        } else {
            None
        }
    }

    /// # Safety
    /// The sparse set must contain a value at dense
    /// index `index`. (Note that dense indices are _not_
    /// the same as entity indices, referred to as sparse indices here.)
    pub unsafe fn get_unchecked_by_dense_index<T: 'static>(
        &self,
        dense_index: u32,
    ) -> (NonNull<T>, &BorrowFlag) {
        let component = self.components.get_unchecked(dense_index).cast();
        let borrow_flag = self.borrow_flags.get_unchecked(dense_index as usize);
        (component, borrow_flag)
    }

    pub fn remove(&mut self, index: u32) -> bool {
        let sparse = match self.sparse.get(index as usize) {
            Some(&s) => s,
            None => return false,
        };
        let dense = match self.dense.get(sparse as usize) {
            Some(&d) => d,
            None => return false,
        };

        if dense == index {
            // Swap-remove the entity.
            self.dense.swap_remove(sparse as usize);
            if let Some(&new_entity_at_index) = self.dense.get(sparse as usize) {
                self.sparse[new_entity_at_index as usize] = sparse;
            }
            self.components.swap_remove(sparse);
            self.borrow_flags.swap_remove(sparse as usize);
            true
        } else {
            false
        }
    }

    pub fn to_ref(&self) -> SparseSetRef {
        SparseSetRef {
            sparse: &self.sparse,
            dense: &self.dense,
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
        assert_eq!(TypeId::of::<T>(), self.components.component_meta().type_id);
    }
}

/// An immutable reference to a sparse set.
#[derive(Copy, Clone)]
pub struct SparseSetRef<'a> {
    sparse: &'a [u32],
    dense: &'a [u32],
}

impl<'a> SparseSetRef<'a> {
    pub fn empty() -> &'static Self {
        const EMPTY: SparseSetRef<'static> = SparseSetRef {
            sparse: &[],
            dense: &[],
        };
        &EMPTY
    }

    pub(crate) fn dense_index_of(&self, index: u32) -> Option<u32> {
        let sparse = *self.sparse.get(index as usize)?;
        let dense = *self.dense.get(sparse as usize)?;
        if dense == index {
            Some(sparse)
        } else {
            None
        }
    }

    /// Returns whether the sparse set contains the given index.
    pub fn contains(&self, index: u32) -> bool {
        self.dense_index_of(index).is_some()
    }

    /// Returns the number of values stored in the sparse set.
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over (sparse_index, dense_index) within this sparse set.
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            inner: self.dense.iter().enumerate(),
        }
    }
}

pub struct Iter<'a> {
    inner: Enumerate<slice::Iter<'a, u32>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let (dense_index, sparse_index) = self.inner.next()?;
        Some((*sparse_index, dense_index as u32))
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::Layout;

    use crate::Component;

    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Comp<T>(T);

    impl<T: 'static> Component for Comp<T> {}

    #[test]
    fn insert_and_get() {
        let mut storage = SparseSetStorage::new(ComponentMeta::of::<Comp<&'static str>>());

        let entity_a = 10;
        let entity_b = 15;

        storage.insert(entity_b, Comp("entity b"));
        storage.insert(entity_a, Comp("entity a"));

        assert_eq!(
            *storage
                .get::<Comp<&'static str>>(entity_b)
                .unwrap()
                .unwrap(),
            Comp("entity b")
        );
        assert_eq!(
            *storage
                .get::<Comp<&'static str>>(entity_a)
                .unwrap()
                .unwrap(),
            Comp("entity a")
        );

        storage.remove(entity_a);
        assert_eq!(
            *storage
                .get::<Comp<&'static str>>(entity_b)
                .unwrap()
                .unwrap(),
            Comp("entity b")
        );
        assert!(storage
            .get::<Comp<&'static str>>(entity_a)
            .unwrap()
            .is_none());
    }
}
