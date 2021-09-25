use std::{alloc::Layout, marker::PhantomData};

use anyhow::bail;
use bump::WasmBump;
use wasmer::{Instance, LazyInit, Memory};

use crate::thread_pinned::ThreadPinned;

use super::{PluginPtr, PluginPtrMut};

mod bump;

#[derive(Default)]
pub struct WasmPluginContext {
    bump: LazyInit<ThreadPinned<WasmBump>>,

    memory: LazyInit<Memory>,
}

impl WasmPluginContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_with_instance(&mut self, instance: &Instance) -> anyhow::Result<()> {
        let allocate = instance.exports.get_function("quill_allocate")?;
        let deallocate = instance.exports.get_function("quill_deallocate")?;

        let bump = WasmBump::new(allocate.native()?, deallocate.native()?)?;
        self.bump.initialize(ThreadPinned::new(bump));

        self.memory
            .initialize(instance.exports.get_memory("memory")?.clone());

        Ok(())
    }

    pub unsafe fn deref_bytes(&self, ptr: PluginPtr<u8>, len: u32) -> anyhow::Result<&[u8]> {
        let data = self.memory.get_ref().unwrap().data_unchecked();
        let offset = ptr.ptr as usize;

        if data.len() <= offset + len as usize {
            bail!("pointer out of bounds");
        }

        Ok(&data[offset..(offset + len as usize)])
    }

    pub unsafe fn deref_bytes_mut(
        &self,
        ptr: PluginPtrMut<u8>,
        len: u32,
    ) -> anyhow::Result<&mut [u8]> {
        let data = self.memory.get_ref().unwrap().data_unchecked_mut();
        let offset = ptr.ptr as usize;

        if data.len() <= offset + len as usize {
            bail!("pointer out of bounds");
        }

        Ok(&mut data[offset..(offset + len as usize)])
    }

    pub fn bump_allocate(&self, layout: Layout) -> anyhow::Result<PluginPtrMut<u8>> {
        self.bump
            .get_ref()
            .unwrap()
            .borrow_mut()
            .alloc(layout)
            .map(|wasm_ptr| PluginPtrMut {
                ptr: wasm_ptr.offset() as u64,
                _marker: PhantomData,
            })
    }

    pub fn bump_reset(&self) {
        let _ = self.bump.get_ref().unwrap().borrow_mut().reset();
    }
}
