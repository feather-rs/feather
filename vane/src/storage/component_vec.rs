use std::{alloc::Layout, ptr::NonNull, sync::Arc};

use arrayvec::ArrayVec;

use crate::component::ComponentMeta;

use super::blob_array::BlobArray;

/// A vector of component values, stored as untyped, raw
/// bytes.
///
/// Components have a stable location in memory unless `swap_remove`
/// is called.
///
/// This implementation uses a series of `BlobArray`s with exponentially
/// increasing capacities. The first 512 components are stored in the first
/// array; the next 1024 in the second; and so on. Therefore, arrays are never resized,
/// which gives two key properties:
/// 1. Values have a stable location in memory: regrowth does not move data.
/// 2. Pushing is `O(1)` in the worst case. On the other hand,
/// if we allowed for regrowth, the worst case would be `O(n)`.
pub struct ComponentVec {
    arrays: ArrayVec<BlobArray, MAX_NUM_ARRAYS>,
    component_meta: ComponentMeta,
    len: u32,
}

impl ComponentVec {
    /// Creates a new `ComponentVec`. Does not allocate.
    pub fn new(component_meta: ComponentMeta) -> Self {
        Self {
            arrays: ArrayVec::new(),
            component_meta,
            len: 0,
        }
    }

    /// Pushes the given value into the component vector.
    ///
    /// After this call, the value pointed by `ptr` is uninitialized.
    ///
    /// # Safety
    /// `ptr` must be a valid pointer to an instance of
    /// the component type stored in this vector.
    pub unsafe fn push(&mut self, value: *const u8) -> *mut u8 {
        let new_len = self
            .len
            .checked_add(1)
            .expect("component vector length exceed u32::MAX");

        self.len = new_len;

        let array = self.array_for_item_or_grow(new_len - 1);

        let ptr = array.push_raw(value).expect("array cannot be full");

        self.check_invariants();

        ptr
    }

    /// Gets the value at the given index.
    ///
    /// # Safety
    /// `index` must be in bounds.
    pub unsafe fn get_unchecked(&self, index: u32) -> NonNull<u8> {
        self.array_for_item(index)
            .get_raw_unchecked(item_index_within_array(index))
    }

    /// Swap-removes the item at the given index.
    ///
    /// This call moves the last item in the array to `index`,
    /// deleting the previous value at `index`.
    ///
    /// # Warning
    /// This function breaks pointer stability. Any pointers into
    /// values in this component vector should be considered invalid
    /// after this call.
    pub fn swap_remove(&mut self, index: u32) {
        assert!(index < self.len, "index out of bounds");

        let item_size = self.component_meta.layout.size();

        unsafe {
            let last_array = self.array_for_item_mut(self.len - 1);
            let last_item = last_array.get_raw_unchecked(last_array.len() - 1);
            last_array.set_len(last_array.len() - 1);

            let item_array = self.array_for_item_mut(index);
            let removed_item = item_array.get_raw_unchecked(item_index_within_array(index));

            (self.component_meta.drop_fn)(removed_item.as_ptr());

            std::ptr::copy(last_item.as_ptr(), removed_item.as_ptr(), item_size);
        }

        self.len -= 1;

        self.check_invariants();
    }

    /// Returns the number of components in the vector.
    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn component_meta(&self) -> &ComponentMeta {
        &self.component_meta
    }

    fn check_invariants(&self) {
        debug_assert_eq!(
            self.arrays.iter().map(|array| array.len()).sum::<usize>(),
            self.len as usize
        );
    }

    /// # Safety
    /// `item_index` must be in bounds.
    unsafe fn array_for_item(&self, item_index: u32) -> &BlobArray {
        // SAFETY: `array_index` always returns indices in bounds.
        unsafe { self.arrays.get_unchecked(array_index(item_index)) }
    }

    /// # Safety
    /// `item_index` must be in bounds.
    unsafe fn array_for_item_mut(&mut self, item_index: u32) -> &mut BlobArray {
        // SAFETY: `array_index` always returns indices in bounds.
        unsafe { self.arrays.get_unchecked_mut(array_index(item_index)) }
    }

    fn array_for_item_or_grow(&mut self, item_index: u32) -> &mut BlobArray {
        let array_index = array_index(item_index);

        if array_index >= self.arrays.len() {
            self.allocate_new_array();
        }

        self.arrays.get_mut(array_index).unwrap()
    }

    fn allocate_new_array(&mut self) {
        let previous_capacity = self
            .arrays
            .last()
            .map(|array| array.capacity())
            .unwrap_or_else(|| 2usize.pow(START_CAP_LOG2 as u32));
        let next_capacity = previous_capacity.checked_mul(2).expect("capacity overflow");
        let array = BlobArray::new(
            self.component_meta.layout,
            self.component_meta.drop_fn,
            next_capacity,
        );
        self.arrays.push(array);
    }
}

const START_CAP_LOG2: usize = 9; // log2(512)
const MAX_NUM_ARRAYS: usize = 32 - START_CAP_LOG2 + 1;

/// Returns the index into `ComponentVec::arrays` of
/// the item at the given index.
fn array_index(item_index: u32) -> usize {
    let log2_index = 32u32 - item_index.leading_zeros();
    let array_index = (log2_index.saturating_sub(START_CAP_LOG2 as u32)) as usize;
    debug_assert!(array_index < MAX_NUM_ARRAYS);
    array_index
}

/// Returns the index inside an array
/// of the given item index.
fn item_index_within_array(item_index: u32) -> usize {
    let next_power_of_two = item_index.next_power_of_two();
    let prev_power_of_two = next_power_of_two / 2;

    (item_index & (prev_power_of_two.max(2u32.pow(START_CAP_LOG2 as u32)) - 1)) as usize
}

#[cfg(test)]
mod tests {
    use crate::Component;

    use super::*;

    #[repr(transparent)]
    struct Comp<T>(T);

    impl<T: 'static> Component for Comp<T> {}

    #[test]
    fn array_index_zero() {
        assert_eq!(array_index(0), 0);
    }

    #[test]
    fn array_index_max_equals_last_array() {
        assert_eq!(array_index(u32::MAX), MAX_NUM_ARRAYS - 1);
    }

    #[test]
    fn array_index_between() {
        assert_eq!(array_index(2u32.pow(START_CAP_LOG2 as u32)), 1);
        assert_eq!(array_index(2u32.pow(START_CAP_LOG2 as u32 + 1)), 2);
        assert_eq!(array_index(1), 0);
    }

    #[test]
    fn item_index_within_array_test() {
        assert_eq!(item_index_within_array(0), 0);
        assert_eq!(item_index_within_array(511), 511);
        assert_eq!(item_index_within_array(512), 0);
        assert_eq!(item_index_within_array(256), 256);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn push_and_get() {
        let meta = ComponentMeta::of::<Comp<i32>>();
        let mut vec = ComponentVec::new(meta);

        unsafe {
            for i in 0..10_000i32 {
                vec.push(&i as *const i32 as *const u8);
            }
            for i in 0..10_000i32 {
                assert_eq!(*vec.get_unchecked(i as u32).cast::<i32>().as_ref(), i);
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn swap_remove() {
        let meta = ComponentMeta::of::<Comp<i32>>();
        let mut vec = ComponentVec::new(meta);

        unsafe {
            for i in 0..1000i32 {
                vec.push(&i as *const i32 as *const u8);
            }
            assert_eq!(vec.len(), 1000);

            vec.swap_remove(50);
            assert_eq!(*vec.get_unchecked(50).cast::<i32>().as_ref(), 999);
            assert_eq!(vec.len(), 999);
        }
    }
}
