//! Attempts to make passing allocations between host and plugin safe.
//!
//! If a memory block is deallocated with a different allocator than
//! that it was allocated with, there will be undefined behavior.
//! By ensuring that all plugin allocations delegate to the host's
//! allocator, we ensure that the above does not occur.
//!
//! The global allocator is set to `Host` by the `plugin` macro.

use crate::vtable::vtable;
use std::alloc::{GlobalAlloc, Layout};

/// An allocator which delegates to the host's
/// global allocator.
pub struct Host;

unsafe impl GlobalAlloc for Host {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        vtable().alloc(layout.into())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        vtable().dealloc(ptr, layout.into())
    }
}
