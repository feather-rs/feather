use std::{alloc::Layout, marker::PhantomData};

use crate::thread_pinned::ThreadPinned;

use super::{PluginPtr, PluginPtrMut};

pub struct NativePluginContext {
    bump: ThreadPinned<Vec<(*mut u8, Layout)>>,
}

impl NativePluginContext {
    pub fn new() -> Self {
        Self {
            bump: ThreadPinned::new(Vec::new()),
        }
    }

    pub unsafe fn deref_bytes(&self, ptr: PluginPtr<u8>, len: u32) -> anyhow::Result<&[u8]> {
        Ok(std::slice::from_raw_parts(ptr.as_native(), len as usize))
    }

    pub unsafe fn deref_bytes_mut(
        &self,
        ptr: PluginPtrMut<u8>,
        len: u32,
    ) -> anyhow::Result<&mut [u8]> {
        Ok(std::slice::from_raw_parts_mut(
            ptr.as_native(),
            len as usize,
        ))
    }

    pub fn bump_allocate(&self, layout: Layout) -> anyhow::Result<PluginPtrMut<u8>> {
        let ptr = unsafe { std::alloc::alloc(layout) };
        self.bump.borrow_mut().push((ptr, layout));
        Ok(PluginPtrMut {
            ptr: ptr as usize as u64,
            _marker: PhantomData,
        })
    }

    pub fn bump_reset(&self) {
        for (ptr, layout) in self.bump.borrow_mut().drain(..) {
            unsafe {
                std::alloc::dealloc(ptr, layout);
            }
        }
    }
}
