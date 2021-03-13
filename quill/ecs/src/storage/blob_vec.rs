use std::{
    alloc::Layout,
    mem::MaybeUninit,
    ptr::{self, NonNull},
    sync::Arc,
};

use crate::layout_ext::LayoutExt;
use crate::space::MemorySpace;

/// A vector of values of the same layout, stored
/// as raw bytes.
pub struct BlobVec {
    /// Memory where the values are allocated.
    space: Arc<MemorySpace>,
    /// Layout of the values stored in the blob vector.
    item_layout: Layout,

    capacity: usize,
    len: usize,
    ptr: NonNull<u8>,
}

impl BlobVec {
    pub fn new(item_layout: Layout) -> Self {
        Self::with_space(item_layout, Arc::new(MemorySpace::host()))
    }

    pub fn with_space(item_layout: Layout, space: Arc<MemorySpace>) -> Self {
        // Create a dangling pointer.
        let ptr = NonNull::new(item_layout.align() as *mut u8).expect("align > 0");

        Self {
            space,
            item_layout,

            capacity: 0,
            len: 0,
            ptr,
        }
    }

    pub fn push<T>(&mut self, value: T) {
        self.assert_layout_matches::<T>();

        let value = MaybeUninit::new(value);
        unsafe {
            self.push_raw(value.as_ptr().cast());
        }
    }

    pub unsafe fn push_raw(&mut self, value: *const u8) {
        let new_len = self.len.checked_add(1).expect("length overflow");
        if new_len > self.capacity {
            self.grow();
            debug_assert!(new_len <= self.capacity);
        }

        self.len = new_len;

        unsafe {
            ptr::copy_nonoverlapping(value, self.item_ptr(self.len - 1), self.item_layout.size());
        }

        self.check_invariants();
    }

    fn grow(&mut self) {
        let old_capacity = self.capacity;
        let new_capacity = match old_capacity {
            0 => 4,
            x => x.checked_mul(2).expect("capacity overflow"),
        };

        self.capacity = new_capacity;

        unsafe {
            let new_layout = self
                .item_layout
                .repeat(new_capacity)
                .expect("capacity overflow")
                .0;
            let new_ptr = self.space.alloc(new_layout);

            // Copy old data
            ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr, self.byte_len());

            if old_capacity != 0 {
                self.space.dealloc(
                    self.ptr.as_ptr(),
                    self.item_layout.repeat(old_capacity).unwrap().0,
                );
            }

            self.ptr = NonNull::new(new_ptr).unwrap();
        }

        self.check_invariants();
    }

    /// # Safety
    /// The values stored in this vector must be of type `T`.
    pub unsafe fn get<T>(&self, index: usize) -> Option<&T> {
        self.assert_layout_matches::<T>();
        let ptr = self.get_raw(index)?;
        Some(&*ptr.cast().as_ptr())
    }

    /// # Safety
    /// The values stored in this vector must be of type `T`.
    pub unsafe fn get_mut<T>(&mut self, index: usize) -> Option<&mut T> {
        let ptr = self.get_raw(index)?;
        Some(&mut *ptr.cast().as_ptr())
    }

    /// Gets the value at index `index` as a raw pointer.
    pub fn get_raw(&self, index: usize) -> Option<NonNull<u8>> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let ptr = self.item_ptr(index);
            Some(NonNull::new_unchecked(ptr))
        }
    }

    /// # Safety
    /// The values stored in this vector must be of type `T`.
    pub unsafe fn swap_remove<T>(&mut self, at_index: usize) -> T {
        self.assert_layout_matches::<T>();
        assert!(
            at_index <= self.len,
            "swap remove index out of bounds ({} > {})",
            at_index,
            self.len
        );

        let ptr = self.item_ptr(at_index);
        let end_ptr = self.item_ptr(self.len - 1);

        let value = ptr::read(ptr.cast::<T>());

        std::ptr::swap(ptr, end_ptr);

        self.len -= 1;

        value
    }

    pub fn swap_remove_raw(&mut self, index: usize) -> Option<NonNull<u8>> {
        
    }

    fn item_ptr(&self, offset: usize) -> *mut u8 {
        unsafe { self.ptr.as_ptr().add(offset * self.item_layout.size()) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn byte_len(&self) -> usize {
        self.len * self.item_layout.size()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn check_invariants(&self) {
        debug_assert!(self.capacity() >= self.len());
    }

    fn assert_layout_matches<T>(&self) {
        assert_eq!(
            Layout::new::<T>(),
            self.item_layout,
            "pushed item must have same layout as item_layout ({:?} != {:?})",
            Layout::new::<T>(),
            self.item_layout
        );
    }
}

impl Drop for BlobVec {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return;
        }
        unsafe {
            self.space.dealloc(
                self.ptr.as_ptr(),
                self.item_layout.repeat(self.capacity).unwrap().0,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_get() {
        let mut vec = BlobVec::new(Layout::new::<usize>());
        vec.push(123usize);
        for i in 0usize..1000 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 1001);
        assert!(vec.capacity() >= vec.len());
        unsafe {
            assert_eq!(vec.get(0), Some(&123usize));
            for i in 1..1001 {
                assert_eq!(vec.get(i), Some(&(i - 1)));
            }
            assert_eq!(vec.get::<usize>(1001), None);
        }
    }

    #[test]
    fn push_get_raw_bytes() {
        let mut vec = BlobVec::new(Layout::new::<usize>());
        for i in 0usize..1000 {
            vec.push(i);
        }

        for i in 0usize..1000 {
            let bytes = vec.get_raw(i).unwrap();
            unsafe {
                assert_eq!(*bytes.as_ptr().cast::<usize>(), i);
            }
        }
    }

    #[test]
    fn swap_remove() {
        let mut vec = BlobVec::new(Layout::new::<usize>());
        for i in 0..100 {
            vec.push(i as usize);
        }

        assert_eq!(vec.len(), 100);

        unsafe {
            let value = vec.swap_remove::<usize>(50);
            assert_eq!(vec.len(), 99);

            assert_eq!(value, 50);

            assert_eq!(vec.get(98), Some(&98usize));
            assert_eq!(vec.get(50), Some(&99usize));
        }
    }

    /*  #[test]
    fn iter() {
        let mut vec = BlobVec::new(Layout::new::<usize>());
        for i in 0usize..1000 {
            vec.push(i);
        }

        unsafe {
            for (i, value) in vec.iter::<usize>().enumerate() {
                assert_eq!(i, *value);
            }
        }
    }

    #[test]
    fn iter_mut() {
        let mut vec = BlobVec::new(Layout::new::<usize>());
        for i in 0usize..1000 {
            vec.push(i);
        }

        unsafe {
            for (i, value) in vec.iter_mut::<usize>().enumerate() {
                assert_eq!(i, *value);
                *value += 1;
            }

            for (i, value) in vec.iter::<usize>().enumerate() {
                assert_eq!(i, *value + 1);
            }
        }
    }*/
}
