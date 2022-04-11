use std::{
    alloc::Layout,
    mem::MaybeUninit,
    ptr::{self, NonNull},
    sync::Arc,
};

use crate::layout_ext::LayoutExt;

/// Returned from `push` when the blob array is full.
#[derive(Debug)]
pub struct Full;

/// A heap-allocated array of values of the same layout, stored
/// as raw bytes.
///
/// This is an untyped version of a `Box<[T]>`.
///
/// Does not support resizing. Values have a fixed location in memory
/// unless `swap_remove` is called.
pub struct BlobArray {
    /// Layout of the values stored in the blob vector.
    item_layout: Layout,
    /// Function to drop items in the array.
    item_drop_fn: unsafe fn(*mut u8),

    /// The maximum number of items.
    capacity: usize,
    /// The number of items.
    len: usize,
    /// Untyped pointer to the items.
    ptr: NonNull<u8>,
}

impl BlobArray {
    pub fn new(item_layout: Layout, item_drop_fn: unsafe fn(*mut u8), capacity: usize) -> Self {
        assert!(
            capacity < isize::MAX as usize,
            "capacity cannot exceed isize::MAX"
        );
        assert_ne!(capacity, 0, "capacity cannot be zero");

        // Allocate space for the items.
        let ptr = if item_layout.size() == 0 {
            // Use a dangling pointer - we can't make allocations of zero size.
            NonNull::new(item_layout.align() as *mut _).expect("zero align")
        } else {
            let ptr = unsafe { std::alloc::alloc(item_layout.repeat(capacity).unwrap().0) };
            NonNull::new(ptr).expect("allocation failed")
        };

        Self {
            item_layout,
            item_drop_fn,

            capacity,
            len: 0,
            ptr,
        }
    }

    pub fn push<T>(&mut self, value: T) -> Result<*mut u8, Full> {
        self.assert_layout_matches::<T>();

        // Put the value in a MaybeUninit so that
        // it is not dropped after its bytes have been copied.
        // Otherwise, we risk a double free.
        let value = MaybeUninit::new(value);
        unsafe { self.push_raw(value.as_ptr().cast()) }
    }

    pub unsafe fn push_raw(&mut self, value: *const u8) -> Result<*mut u8, Full> {
        let new_len = self.len + 1;
        if new_len > self.capacity {
            return Err(Full);
        }

        self.len = new_len;

        let ptr = self.item_ptr(self.len - 1);
        unsafe {
            ptr::copy_nonoverlapping(value, ptr, self.item_layout.size());
        }

        self.check_invariants();

        Ok(ptr)
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
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

        // SAFETY: `index` is in bounds.
        unsafe { Some(self.get_raw_unchecked(index)) }
    }

    /// # Safety
    /// `index` must be within bounds.
    pub unsafe fn get_raw_unchecked(&self, index: usize) -> NonNull<u8> {
        let ptr = self.item_ptr(index);
        NonNull::new_unchecked(ptr)
    }

    /// Removes the value at `at_index` and moves
    /// the last item to `at_index`.
    ///
    /// # Panics
    /// Panics if `at_index >= self.len()`.
    pub fn swap_remove(&mut self, at_index: usize) {
        assert!(
            at_index < self.len,
            "swap remove index out of bounds ({} > {})",
            at_index,
            self.len
        );

        let ptr = self.item_ptr(at_index);
        let end_ptr = self.item_ptr(self.len - 1);

        unsafe {
            (self.item_drop_fn)(ptr);

            // Move the value at `end_ptr` to
            // `ptr`, overwriting the value at `ptr`.
            std::ptr::copy(end_ptr, ptr, self.item_layout.size());
        }

        self.len -= 1;
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

impl Drop for BlobArray {
    fn drop(&mut self) {
        for i in 0..self.len() {
            let ptr = self.get_raw(i).unwrap().as_ptr();
            unsafe {
                (self.item_drop_fn)(ptr);
            }
        }
        if self.item_layout.size() != 0 {
            unsafe {
                std::alloc::dealloc(
                    self.ptr.as_ptr(),
                    self.item_layout.repeat(self.capacity).unwrap().0,
                );
            }
        }
    }
}

unsafe impl Send for BlobArray {}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::*;

    #[test]
    fn push_get() {
        let mut vec = BlobArray::new(Layout::new::<usize>(), |_| (), 1001);
        vec.push(123usize);
        for i in 0usize..1000 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 1001);
        assert!(vec.capacity() >= vec.len());
        assert!(vec.push(10usize).is_err());
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
        let mut vec = BlobArray::new(Layout::new::<usize>(), |_| (), 1000);
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
        let mut vec = BlobArray::new(Layout::new::<usize>(), |_| (), 100);
        for i in 0..100 {
            vec.push(i as usize);
        }

        assert_eq!(vec.len(), 100);

        unsafe {
            vec.swap_remove(50);
            assert_eq!(vec.len(), 99);

            assert_eq!(vec.get(98), Some(&98usize));
            assert_eq!(vec.get(50), Some(&99usize));
        }
    }

    #[test]
    fn calls_drop_fn() {
        struct NeedsDrop {
            x: i32,
        }

        static WAS_DROPPED: AtomicBool = AtomicBool::new(false);

        impl Drop for NeedsDrop {
            fn drop(&mut self) {
                WAS_DROPPED.store(true, Ordering::SeqCst);
                assert_eq!(self.x, 15);
            }
        }

        let mut vec = BlobArray::new(
            Layout::new::<NeedsDrop>(),
            |ptr| unsafe { ptr::drop_in_place::<NeedsDrop>(ptr.cast()) },
            1,
        );
        vec.push(NeedsDrop { x: 15 });

        vec.swap_remove(0);

        assert!(WAS_DROPPED.load(Ordering::SeqCst));
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
