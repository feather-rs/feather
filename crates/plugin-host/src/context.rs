use std::{
    alloc::Layout,
    cell::{RefCell, RefMut},
    mem::{self, size_of},
};

use anyhow::bail;
use feather_common::Game;
use wasmer::{Instance, LazyInit, Memory, NativeFunc, WasmPtr};

use crate::PluginId;

/// Context passed to host functions.
///
/// Provides access to the [`Game`](feather_common::Game).
#[derive(Default)]
pub struct PluginContext {
    /// Pointer to the [`Game`].
    game: Option<RefCell<*mut Game>>,

    /// The plugin's memory.
    memory: LazyInit<Memory>,

    /// Function to allocate memory within the plugin's linear memory.
    allocate: LazyInit<NativeFunc<(u32, u32), u32>>,
    /// Function to deallocate memory within the plugin's linear memory.
    deallocate: LazyInit<NativeFunc<(u32, u32, u32)>>,
    /// Function to invoke a system within the plugin.
    run_system: LazyInit<NativeFunc<u32>>,

    /// ID of this plugin.
    plugin_id: LazyInit<PluginId>,

    instance: LazyInit<Instance>,

    /// Memory blocks on the plugin that should be freed on a call to `query_end`.
    query_garbage: Vec<(WasmPtr<u8>, Layout)>,
}

impl PluginContext {
    pub fn init_with_instance(&mut self, plugin_instance: &Instance) -> anyhow::Result<()> {
        let memory = plugin_instance.exports.get_memory("memory")?;

        let allocate = plugin_instance
            .exports
            .get_function("quill_allocate")?
            .native()?;
        let deallocate = plugin_instance
            .exports
            .get_function("quill_deallocate")?
            .native()?;
        let run_system = plugin_instance
            .exports
            .get_function("quill_run_system")?
            .native()?;

        self.memory.initialize(memory.clone());
        self.allocate.initialize(allocate);
        self.deallocate.initialize(deallocate);
        self.run_system.initialize(run_system);
        self.instance.initialize(plugin_instance.clone());

        Ok(())
    }

    pub fn set_plugin_id(&mut self, id: PluginId) {
        self.plugin_id.initialize(id);
    }

    pub fn plugin_id(&self) -> PluginId {
        *self.plugin_id.get_ref().unwrap()
    }

    /// Gets a mutable reference to the `Game`.
    ///
    /// # Safety
    /// This function is not marked `unsafe` as it is only
    /// accessible internally. However, it may only be called
    /// within the context of a host call.
    pub fn game_mut(&self) -> RefMut<Game> {
        let ptr = self
            .game
            .as_ref()
            .expect("not inside host call")
            .borrow_mut();
        RefMut::map(ptr, |ptr| unsafe { &mut **ptr })
    }

    /// Sets the `Game` used for host calls.
    pub fn set_game(&mut self, game: &mut Game) {
        self.game = Some(RefCell::new(game as *mut Game));
    }

    /// Gets the plugin memory.
    pub fn memory(&self) -> &Memory {
        self.memory.get_ref().unwrap()
    }

    /// Gets the function to invoke a system on the plugin
    /// given a pointer to its data.
    pub fn run_system_fn(&self) -> &NativeFunc<u32> {
        self.run_system.get_ref().unwrap()
    }

    /// Allocates a pointer with the given layout inside the plugin's linear memory.
    pub fn allocate(&self, layout: Layout) -> anyhow::Result<WasmPtr<u8>> {
        let offset = self
            .allocate
            .get_ref()
            .unwrap()
            .call(layout.size() as u32, layout.align() as u32)?;
        Ok(WasmPtr::new(offset))
    }

    /// Deallocates a pointer with the given layout inside the plugin's linear memory.
    pub fn deallocate(&self, ptr: WasmPtr<u8>, layout: Layout) -> anyhow::Result<()> {
        self.deallocate.get_ref().unwrap().call(
            ptr.offset(),
            layout.size() as u32,
            layout.align() as u32,
        )?;
        Ok(())
    }

    /// Copies data into WASM memory, safely.
    pub fn write_to_memory<T: Copy>(&self, value: T, dst: WasmPtr<u8>) -> anyhow::Result<()> {
        self.write_slice_to_memory(&[value], dst)
    }

    /// Copies a slice of data into WASM memory, safely.
    pub fn write_slice_to_memory<T: Copy>(
        &self,
        values: &[T],
        dst: WasmPtr<u8>,
    ) -> anyhow::Result<()> {
        let src = values.as_ptr().cast::<u8>();
        unsafe { self.write_raw_to_memory(src, values.len() * size_of::<T>(), dst) }
    }

    /// Copies raw data into memory.
    ///
    /// # Safety
    /// `ptr` must be a valid pointer into a slice
    /// of `len` bytes.
    pub unsafe fn write_raw_to_memory(
        &self,
        src: *const u8,
        len: usize,
        dst: WasmPtr<u8>,
    ) -> anyhow::Result<()> {
        if dst.offset() as usize + len > self.memory().data_size() as usize {
            bail!("ptr out of bounds");
        }

        // SAFETY: we do not access the WASM module while
        // we write to its memory, so a data race is not possible.
        let dst = self.memory().data_ptr().add(dst.offset() as usize);
        std::ptr::copy_nonoverlapping(src, dst, len);

        Ok(())
    }

    /// Allocates memory for and writes `value` into the plugin's memory.
    /// Returns the allocated pointer.
    pub fn insert_to_memory<T: Copy>(&self, value: T) -> anyhow::Result<WasmPtr<u8>> {
        let ptr = self.allocate(Layout::new::<T>())?;
        self.write_to_memory(value, ptr)?;
        Ok(ptr)
    }

    pub fn push_query_garbage(&mut self, ptr: WasmPtr<u8>, layout: Layout) {
        self.query_garbage.push((ptr, layout));
    }

    pub fn deallocate_query_garbage(&mut self) -> anyhow::Result<()> {
        let mut query_garbage = mem::take(&mut self.query_garbage);
        for (ptr, layout) in query_garbage.drain(..) {
            self.deallocate(ptr, layout)?;
        }
        self.query_garbage = query_garbage;
        Ok(())
    }
}
