use std::{
    alloc::Layout,
    cell::{Ref, RefMut},
    marker::PhantomData,
    mem::size_of,
    panic::AssertUnwindSafe,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use feather_common::Game;
use feather_ecs::EntityBuilder;
use quill_common::Component;
use serde::de::DeserializeOwned;
use vec_arena::Arena;
use wasmer::{FromToNativeWasmType, Instance};

use crate::{host_function::WasmHostFunction, thread_pinned::ThreadPinned, PluginId};

mod native;
mod wasm;

/// Wraps a pointer into a plugin's memory space.
#[derive(Copy, Clone, PartialEq, Eq, Zeroable)]
#[repr(transparent)]
pub struct PluginPtr<T> {
    pub ptr: u64,
    pub _marker: PhantomData<*const T>,
}

impl<T> PluginPtr<T> {
    pub fn as_native(&self) -> *const T {
        self.ptr as usize as *const T
    }

    /// # Safety
    /// Adding `n` to this pointer
    /// must produce a pointer within the same allocated
    /// object.
    #[must_use = "PluginPtr::add returns a new pointer"]
    pub unsafe fn add(self, n: usize) -> Self {
        Self {
            ptr: self.ptr + (n * size_of::<T>()) as u64,
            _marker: self._marker,
        }
    }

    /// # Safety
    /// The cast must be valid.
    pub unsafe fn cast<U>(self) -> PluginPtr<U> {
        PluginPtr {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

unsafe impl<T: Copy + 'static> Pod for PluginPtr<T> {}

/// Wraps a pointer into a plugin's memory space.
#[derive(Copy, Clone, PartialEq, Eq, Zeroable)]
#[repr(transparent)]
pub struct PluginPtrMut<T> {
    pub ptr: u64,
    pub _marker: PhantomData<*mut T>,
}

impl<T> PluginPtrMut<T> {
    pub fn as_native(&self) -> *mut T {
        self.ptr as usize as *mut T
    }

    /// # Safety
    /// A null pointer must be valid in the context it is used.
    pub unsafe fn null() -> Self {
        Self {
            ptr: 0,
            _marker: PhantomData,
        }
    }

    /// # Safety
    /// Adding `n` to this pointer
    /// must produce a pointer within the same allocated
    /// object.
    #[must_use = "PluginPtrMut::add returns a new pointer"]
    pub unsafe fn add(self, n: usize) -> Self {
        Self {
            ptr: self.ptr + (n * size_of::<T>()) as u64,
            _marker: self._marker,
        }
    }

    /// # Safety
    /// The cast must be valid.
    pub unsafe fn cast<U>(self) -> PluginPtrMut<U> {
        PluginPtrMut {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

unsafe impl<T: Copy + 'static> Pod for PluginPtrMut<T> {}

unsafe impl<T: Copy> FromToNativeWasmType for PluginPtr<T> {
    type Native = i64;

    fn from_native(native: Self::Native) -> Self {
        Self {
            ptr: native as u64,
            _marker: PhantomData,
        }
    }

    fn to_native(self) -> Self::Native {
        self.ptr as i64
    }
}

unsafe impl<T: Copy> FromToNativeWasmType for PluginPtrMut<T> {
    type Native = i64;

    fn from_native(native: Self::Native) -> Self {
        Self {
            ptr: native as u64,
            _marker: PhantomData,
        }
    }

    fn to_native(self) -> Self::Native {
        self.ptr as i64
    }
}

/// Context of a running plugin.
///
/// Provides methods to access plugin memory,
/// invoke exported functions, and access the `Game`.
///
/// This type abstracts over WASM or native plugins,
/// providing the same interface for both.
///
/// # Safety
/// The `native` version of the plugin context
/// dereferences raw pointers. We assume pointers
/// passed by plugins are valid. Most functions
/// will cause undefined behavior if these constraints
/// are violated.
///
/// We type-encode that a pointer originates from a plugin
/// using the `PluginPtr` structs. Methods that
/// dereference pointers take instances of these
/// structs. Since creating a `PluginPtr` is unsafe,
/// `PluginContext` methods don't have to be marked
/// unsafe.
///
/// On WASM targets, the plugin is never trusted,
/// and pointer accesses are checked. Undefined behavior
/// can never occur as a result of malicious plugin input.
pub struct PluginContext {
    inner: Inner,

    /// Whether the plugin is currently being invoked
    /// on the main thread.
    /// If this is `true`, then plugin functions are on the call stack.
    invoking_on_main_thread: AtomicBool,

    /// The current `Game`.
    ///
    /// Set to `None` if `invoking_on_main_thread` is `false`.
    /// Otherwise, must point to a valid game. The pointer
    /// must be cleared after the plugin finishes executing
    /// or we risk a dangling reference.
    game: ThreadPinned<Option<NonNull<Game>>>,

    /// ID of the plugin.
    id: PluginId,

    /// Active entity builders for the plugin.
    pub entity_builders: ThreadPinned<Arena<EntityBuilder>>,
}

impl PluginContext {
    /// Creates a new WASM plugin context.
    pub fn new_wasm(id: PluginId) -> Self {
        Self {
            inner: Inner::Wasm(ThreadPinned::new(wasm::WasmPluginContext::new())),
            invoking_on_main_thread: AtomicBool::new(false),
            game: ThreadPinned::new(None),
            id,
            entity_builders: ThreadPinned::new(Arena::new()),
        }
    }

    /// Creates a new native plugin context.
    pub fn new_native(id: PluginId) -> Self {
        Self {
            inner: Inner::Native(native::NativePluginContext::new()),
            invoking_on_main_thread: AtomicBool::new(false),
            game: ThreadPinned::new(None),
            id,
            entity_builders: ThreadPinned::new(Arena::new()),
        }
    }

    pub fn init_with_instance(&self, instance: &Instance) -> anyhow::Result<()> {
        match &self.inner {
            Inner::Wasm(w) => w.borrow_mut().init_with_instance(instance),
            Inner::Native(_) => panic!("cannot initialize native plugin context"),
        }
    }

    /// Enters the plugin context, invoking a function inside the plugin.
    ///
    /// # Panics
    /// Panics if we are already inside the plugin context.
    /// Panics if not called on the main thread.
    pub fn enter<R>(&self, game: &mut Game, callback: impl FnOnce() -> R) -> R {
        let was_already_entered = self.invoking_on_main_thread.swap(true, Ordering::SeqCst);
        assert!(!was_already_entered, "cannot recursively invoke a plugin");

        *self.game.borrow_mut() = Some(NonNull::from(game));

        // If a panic occurs, we need to catch it so
        // we clear `self.game`. Otherwise, we get
        // a dangling pointer.
        let result = std::panic::catch_unwind(AssertUnwindSafe(callback));

        self.invoking_on_main_thread.store(false, Ordering::SeqCst);
        *self.game.borrow_mut() = None;

        self.bump_reset();

        result.unwrap()
    }

    /// Gets a mutable reference to the `Game`.
    ///
    /// # Panics
    /// Panics if the plugin is not currently being
    /// invoked on the main thread.
    pub fn game_mut(&self) -> RefMut<Game> {
        let ptr = self.game.borrow_mut();
        RefMut::map(ptr, |ptr| {
            let game_ptr = ptr.expect("plugin is not exeuctugin");

            assert!(self.invoking_on_main_thread.load(Ordering::Relaxed));

            // SAFETY: `game_ptr` points to a valid `Game` whenever
            // the plugin is executing. If the plugin is not
            // executing, then we already panicked when unwrapping `ptr`.
            unsafe { &mut *game_ptr.as_ptr() }
        })
    }

    /// Gets the plugin ID.
    pub fn plugin_id(&self) -> PluginId {
        self.id
    }

    /// Accesses a byte slice in the plugin's memory space.
    ///
    /// # Safety
    /// **WASM**: mutating plugin memory or invoking
    /// plugin functions while this byte slice is
    /// alive is undefined behavior.
    /// **Native**: `ptr` must be valid.
    pub unsafe fn deref_bytes(&self, ptr: PluginPtr<u8>, len: u32) -> anyhow::Result<&[u8]> {
        match &self.inner {
            Inner::Wasm(w) => {
                let w = w.borrow();
                let bytes = w.deref_bytes(ptr, len)?;
                Ok(unsafe { std::slice::from_raw_parts(bytes.as_ptr(), bytes.len()) })
            }
            Inner::Native(n) => n.deref_bytes(ptr, len),
        }
    }

    /// Accesses a byte slice in the plugin's memory space.
    ///
    /// # Safety
    /// **WASM**: accessing plugin memory or invoking
    /// plugin functions while this byte slice is
    /// alive is undefined behavior.
    /// **Native**: `ptr` must be valid and the aliasing
    /// rules must not be violated.
    pub unsafe fn deref_bytes_mut(
        &self,
        ptr: PluginPtrMut<u8>,
        len: u32,
    ) -> anyhow::Result<&mut [u8]> {
        match &self.inner {
            Inner::Wasm(w) => {
                let w = w.borrow();
                let bytes = w.deref_bytes_mut(ptr, len)?;
                Ok(unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr(), bytes.len()) })
            }
            Inner::Native(n) => n.deref_bytes_mut(ptr, len),
        }
    }

    /// Accesses a `Pod` value in the plugin's memory space.
    pub fn read_pod<T: Pod>(&self, ptr: PluginPtr<T>) -> anyhow::Result<T> {
        // SAFETY: we do not return a reference to these
        // bytes.
        unsafe {
            let bytes = self.deref_bytes(ptr.cast(), size_of::<T>() as u32)?;
            bytemuck::try_from_bytes(bytes)
                .map_err(|_| anyhow!("badly aligned data"))
                .map(|val| *val)
        }
    }

    /// Accesses a `bincode`-encoded value in the plugin's memory space.
    pub fn read_bincode<T: DeserializeOwned>(
        &self,
        ptr: PluginPtr<u8>,
        len: u32,
    ) -> anyhow::Result<T> {
        // SAFETY: we do not return a reference to these
        // bytes.
        unsafe {
            let bytes = self.deref_bytes(ptr.cast(), len)?;
            bincode::deserialize(bytes).map_err(From::from)
        }
    }

    /// Accesses a `json`-encoded value in the plugin's memory space.
    pub fn read_json<T: DeserializeOwned>(
        &self,
        ptr: PluginPtr<u8>,
        len: u32,
    ) -> anyhow::Result<T> {
        // SAFETY: we do not return a reference to these
        // bytes.
        unsafe {
            let bytes = self.deref_bytes(ptr.cast(), len)?;
            serde_json::from_slice(bytes).map_err(From::from)
        }
    }

    /// Deserializes a component value in the plugin's memory space.
    pub fn read_component<T: Component>(&self, ptr: PluginPtr<u8>, len: u32) -> anyhow::Result<T> {
        // SAFETY: we do not return a reference to these
        // bytes.
        unsafe {
            let bytes = self.deref_bytes(ptr.cast(), len)?;
            T::from_bytes(bytes)
                .ok_or_else(|| anyhow!("malformed component"))
                .map(|(component, _bytes_read)| component)
        }
    }

    /// Reads a string from the plugin's memory space.
    pub fn read_string(&self, ptr: PluginPtr<u8>, len: u32) -> anyhow::Result<String> {
        // SAFETY: we do not return a reference to these bytes.
        unsafe {
            let bytes = self.deref_bytes(ptr.cast(), len)?;
            let string = std::str::from_utf8(bytes)?.to_owned();
            Ok(string)
        }
    }

    /// Reads a `Vec<u8>` from the plugin's memory space.
    pub fn read_bytes(&self, ptr: PluginPtr<u8>, len: u32) -> anyhow::Result<Vec<u8>> {
        // SAFETY: we do not return a reference to these bytes.
        unsafe {
            let bytes = self.deref_bytes(ptr.cast(), len)?;
            Ok(bytes.to_owned())
        }
    }

    /// Allocates some memory within the plugin's bump
    /// allocator.
    ///
    /// The memory is reset after the plugin finishes
    /// executing the current system.
    pub fn bump_allocate(&self, layout: Layout) -> anyhow::Result<PluginPtrMut<u8>> {
        match &self.inner {
            Inner::Wasm(w) => w.borrow().bump_allocate(layout),
            Inner::Native(n) => n.bump_allocate(layout),
        }
    }

    /// Bump allocates some memory, then copies `data` into it.
    pub fn bump_allocate_and_write_bytes(&self, data: &[u8]) -> anyhow::Result<PluginPtrMut<u8>> {
        let layout = Layout::array::<u8>(data.len())?;
        let ptr = self.bump_allocate(layout)?;

        // SAFETY: our access to these bytes is isolated to the
        // current function. `ptr` is valid as it was just allocated.
        unsafe {
            self.write_bytes(ptr, data)?;
        }

        Ok(ptr)
    }

    /// Writes `data` to `ptr`.
    ///
    /// # Safety
    /// **WASM**: No concerns.
    /// **NATIVE**: `ptr` must point to a slice
    /// of at least `len` valid bytes.
    pub unsafe fn write_bytes(&self, ptr: PluginPtrMut<u8>, data: &[u8]) -> anyhow::Result<()> {
        let bytes = self.deref_bytes_mut(ptr, data.len() as u32)?;
        bytes.copy_from_slice(data);
        Ok(())
    }

    /// Writes a `Pod` type to `ptr`.
    pub fn write_pod<T: Pod>(&self, ptr: PluginPtrMut<T>, value: T) -> anyhow::Result<()> {
        // SAFETY: Unlike `write_bytes`, we know `ptr` is valid for values
        // of type `T` because of its type parameter.
        unsafe { self.write_bytes(ptr.cast(), bytemuck::bytes_of(&value)) }
    }

    /// Deallocates all bump-allocated memory.
    fn bump_reset(&self) {
        match &self.inner {
            Inner::Wasm(w) => w.borrow().bump_reset(),
            Inner::Native(n) => n.bump_reset(),
        }
    }
}

enum Inner {
    Wasm(ThreadPinned<wasm::WasmPluginContext>),
    Native(native::NativePluginContext),
}
