#[macro_use]
mod utils;
#[macro_use]
pub mod component;
pub mod block;
pub mod components;
pub mod entities;
pub mod entity;
pub mod entity_init;
pub mod events;

use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};

pub use component::{Component, HostComponent};
pub use entity::EntityId;

/// Wrapper type that enforces 64-bit pointers
/// for all targets. Needed for ABI compatibility
/// between WASM-compiled and native-compiled plugins.
#[derive(Debug, PartialEq, Eq, Zeroable)]
#[repr(transparent)]
pub struct Pointer<T> {
    ptr: u64,
    _marker: PhantomData<*const T>,
}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            _marker: self._marker,
        }
    }
}

impl<T> Copy for Pointer<T> {}

impl<T> Pointer<T> {
    pub fn new(ptr: *const T) -> Self {
        Self {
            ptr: ptr as usize as u64,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(self) -> *const T {
        self.ptr as usize as *const T
    }
}

impl<T> From<*const T> for Pointer<T> {
    fn from(ptr: *const T) -> Self {
        Self::new(ptr)
    }
}

// SAFETY: Pointer<T> contains a u64 regardless
// of T. bytemuck won't derive Pod for generic
// types because it cannot guarantee this.
unsafe impl<T: 'static> Pod for Pointer<T> {}

/// Wrapper type that enforces 64-bit pointers
/// for all targets. Needed for ABI compatibility
/// between WASM-compiled and native-compiled plugins.
#[derive(Debug, PartialEq, Eq, Zeroable)]
#[repr(transparent)]
pub struct PointerMut<T> {
    ptr: u64,
    _marker: PhantomData<*mut T>,
}

impl<T> Clone for PointerMut<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            _marker: self._marker,
        }
    }
}

impl<T> Copy for PointerMut<T> {}

impl<T> PointerMut<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self {
            ptr: ptr as usize as u64,
            _marker: PhantomData,
        }
    }

    pub fn as_mut_ptr(self) -> *mut T {
        self.ptr as usize as *mut T
    }
}

impl<T> From<*mut T> for PointerMut<T> {
    fn from(ptr: *mut T) -> Self {
        Self::new(ptr)
    }
}

// SAFETY: see impl Pod for Pointer.
unsafe impl<T: 'static> Pod for PointerMut<T> {}
