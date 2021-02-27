use std::{
    alloc::Layout,
    cell::{RefCell, RefMut},
    mem::size_of,
};

use anyhow::anyhow;
use anyhow::{bail, Context};
use bump::PluginBump;
use bytemuck::Pod;
use feather_common::Game;
use feather_ecs::EntityBuilder;
use quill_common::Component;
use serde::de::DeserializeOwned;
use vec_arena::Arena;
use wasmer::{Instance, LazyInit, Memory, NativeFunc, WasmPtr};

use crate::PluginId;

mod bump;

/// Context passed to host functions.
///
/// Provides access to the [`Game`](feather_common::Game).
#[derive(Default)]
pub struct PluginContext {
    /// Pointer to the `Game`.
    game: Option<RefCell<*mut Game>>,

    /// The plugin's memory.
    memory: LazyInit<Memory>,

    /// Function to allocate memory within the plugin's linear memory.
    allocate: LazyInit<NativeFunc<(u32, u32), u32>>,
    /// Function to deallocate memory within the plugin's linear memory.
    deallocate: LazyInit<NativeFunc<(u32, u32, u32)>>,
    /// Function to invoke a system within the plugin.
    run_system: LazyInit<NativeFunc<u32>>,

    /// Bump allocator to allocate temporary memory.
    bump: LazyInit<RefCell<PluginBump>>,

    /// ID of this plugin.
    plugin_id: LazyInit<PluginId>,

    instance: LazyInit<Instance>,

    pub entity_builders: Arena<EntityBuilder>,
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

        let bump = PluginBump::new(allocate.clone(), deallocate.clone())?;

        self.memory.initialize(memory.clone());
        self.allocate.initialize(allocate);
        self.deallocate.initialize(deallocate);
        self.run_system.initialize(run_system);
        self.bump.initialize(RefCell::new(bump));
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
    ///
    /// This function uses the bump allocator. Memory
    /// is automatically freed when the plugin returns frim
    /// the current system.
    pub fn bump_allocate(&self, layout: Layout) -> anyhow::Result<WasmPtr<u8>> {
        self.bump.get_ref().unwrap().borrow_mut().alloc(layout)
    }

    /// Resets the bump allocator.
    ///
    /// Will trap the plugin if called incorrectly.
    pub fn bump_reset(&self) -> anyhow::Result<()> {
        self.bump.get_ref().unwrap().borrow_mut().reset()
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
    ///
    /// Uses bump-allocated memory that is freed at
    /// the end of the current system.
    pub fn insert_to_memory<T: Copy>(&self, value: T) -> anyhow::Result<WasmPtr<u8>> {
        let ptr = self.bump_allocate(Layout::new::<T>())?;
        self.write_to_memory(value, ptr)?;
        Ok(ptr)
    }

    /// Gets a `bincode`-encoded value from plugin memory.
    pub fn read_bincode<T: DeserializeOwned>(
        &self,
        ptr: WasmPtr<u8>,
        len: u32,
    ) -> anyhow::Result<T> {
        // SAFETY: No other code is currently
        // accessing the memory mutably.
        let bytes = unsafe { self.read_bytes(ptr, len)? };
        bincode::deserialize(bytes).map_err(From::from)
    }

    /// Gets a `Component` from memory.
    pub fn read_component<T: Component>(&self, ptr: WasmPtr<u8>, len: u32) -> anyhow::Result<T> {
        // SAFETY: No other code is currently
        // accessing the memory mutably.
        let bytes = unsafe { self.read_bytes(ptr, len)? };
        T::from_bytes(bytes)
            .context("malformed component")
            .map(|(value, _offset)| value)
    }

    /// Gets a value implementing `bytemuck::Pod` from memory.
    pub fn read_pod<T: Pod>(&self, ptr: WasmPtr<u8>) -> anyhow::Result<T> {
        // SAFETY: No other code is currently
        // accessing the memory mutably.
        let bytes = unsafe { self.read_bytes(ptr, size_of::<T>() as u32)? };
        let value = *bytemuck::try_from_bytes(bytes).map_err(|e| anyhow!("{}", e))?;
        Ok(value)
    }

    /// # Safety
    /// Mutating the plugin's memory while this slice
    /// is alive is undefined behavior.
    pub unsafe fn read_bytes(&self, ptr: WasmPtr<u8>, len: u32) -> anyhow::Result<&[u8]> {
        if ptr.offset() as u64 + len as u64 >= self.memory().data_size() {
            bail!("pointer out of bounds");
        }
        Ok(&self.memory().data_unchecked()[ptr.offset() as usize..(ptr.offset() + len) as usize])
    }
}
