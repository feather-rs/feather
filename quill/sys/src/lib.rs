//! Raw FFI functions for host calls.
//!
//! # WASM vs Native
//! `quill-sys` exposes the same API on both WASM and native
//! targets, but there are internal differences in how
//! host functions are called.
//!
//! On WASM, host calls are `extern "C"` functions
//! that the linker adds as an import for the WASM module.
//!
//! On native, host calls are defined in a vtable struct
//! containing a function pointer for each call. The exported
//! functions in this crate defer to the vtable to make their host calls.
//!
//! Additionally, on native, `quill-sys` exports a `HOST_CONTEXT` constant
//! which is passed to every host call. The host expects this to be the
//! value passed to the `quill_setup` method. Failing to set this
//! constant correctly before making host calls
//!  will result in undefined behavior.

use std::mem::MaybeUninit;

use quill_common::{
    block::BlockGetResult, entity::QueryData, EntityId, HostComponent, Pointer, PointerMut,
};

// The attribute macro transforms the block into either:
// 1. On WASM, an extern "C" block defining functions imported from the host.
// 2. On native targets, the necessary glue code to use the HOST_VTABLE
// to call host functions.
// The resulting public API is the same for both targets.
#[quill_sys_macros::host_functions]
#[link(wasm_import_module = "quill_01")]
extern "C" {
    /// Registers a system.
    ///
    /// Each tick, the system is invoked
    /// by calling the plugin's exported `quill_run_system` method.
    /// `quill_run_system` is given the `system_data` pointer passed
    /// to this host call.
    pub fn register_system(system_data: PointerMut<u8>, name_ptr: Pointer<u8>, name_len: u32);

    /// Initiates a query. Returns the query data.
    ///
    /// The returned query buffers are allocated within
    /// the plugin's bump allocator. They will be
    /// freed automatically after the plugin finishes
    /// executing the current system.
    pub fn entity_query(
        components_ptr: Pointer<HostComponent>,
        components_len: u32,
        query_data: PointerMut<MaybeUninit<QueryData>>,
    );

    /// Determines whether the given entity exists.
    pub fn entity_exists(entity: EntityId) -> bool;

    /// Gets a component for an entity.
    ///
    /// Sets `bytes_ptr` to a pointer to the serialized
    /// component bytes and `bytes_len` to the number of bytes.
    ///
    /// If the entity does not have the component,
    /// then `bytes_ptr` is set to null, and `bytes_len`
    /// is left untouched.
    pub fn entity_get_component(
        entity: EntityId,
        component: HostComponent,
        bytes_ptr: PointerMut<Pointer<u8>>,
        bytes_len: PointerMut<u32>,
    );

    /// Sets or replaces a component for an entity.
    ///
    /// `bytes_ptr` is a pointer to the serialized
    /// component.
    ///
    /// This will overwrite any existing component of the same type.
    /// Does nothing if `entity` does not exist.
    pub fn entity_set_component(
        entity: EntityId,
        component: HostComponent,
        bytes_ptr: Pointer<u8>,
        bytes_len: u32,
    );

    /// Adds an event for an entity.
    ///
    /// `bytes_ptr` is a pointer to the serialized
    /// event.
    ///
    /// This will overwrite any existing event of the same type.
    /// Does nothing if `entity` does not exist.
    pub fn entity_add_event(
        entity: EntityId,
        event: HostComponent,
        bytes_ptr: Pointer<u8>,
        bytes_len: u32,
    );

    /// Adds a global event.
    ///
    /// `bytes_ptr` is a pointer to the serialized
    /// component.
    pub fn add_event(event: HostComponent, bytes_ptr: Pointer<u8>, bytes_len: u32);

    /// Sends a message to an entity.
    ///
    /// The given message should be in the JSON format.
    ///
    /// Does nothing if the entity does not exist or it does not have the `Chat` component.
    pub fn entity_send_message(entity: EntityId, message_ptr: Pointer<u8>, message_len: u32);

    /// Sends a title to an entity.
    ///
    /// The given `Title` should contain at least a `title` or a `sub_title`
    ///
    /// Does nothing if the entity does not exist or if it does not have the `Chat` component.
    pub fn entity_send_title(entity: EntityId, title_json_ptr: Pointer<u8>, title_len: u32);

    /// Creates an empty entity builder.
    ///
    /// This builder is used for creating an ecs-entity
    ///
    /// **This is NOT specifically for a minecraft entity!**
    ///
    pub fn entity_builder_new_empty() -> u32;

    /// Creates an entity builder.
    ///
    /// The builder is initialized with the default
    /// components for the given `EntityInit`.
    ///
    /// `entity_init` is a `bincode`-serialized `EntityInit`.
    pub fn entity_builder_new(
        position: Pointer<u8>,
        entity_init_ptr: Pointer<u8>,
        entity_init_len: u32,
    ) -> u32;

    /// Adds a component to an entity builder.
    ///
    /// `bytes` is the serialized component.
    pub fn entity_builder_add_component(
        builder: u32,
        component: HostComponent,
        bytes_ptr: Pointer<u8>,
        bytes_len: u32,
    );

    /// Creates an entity from an entity builder.
    ///
    /// Returns the new entity.
    ///
    /// `builder` is consumed after this call.
    /// Reusing it is undefined behavior.
    pub fn entity_builder_finish(builder: u32) -> EntityId;

    /// Gets the block at the given position.
    ///
    /// Returns `None` if the block's chunk is unloaded
    /// or if the Y coordinate is out of bounds.
    pub fn block_get(x: i32, y: i32, z: i32) -> BlockGetResult;

    /// Sets the block at the given position.
    ///
    /// Returns `true` if successful and `false`
    /// if the block's chunk is not loaded or
    /// the Y coordinate is out of bounds.
    ///
    /// `block` is the vanilla ID of the block.
    pub fn block_set(x: i32, y: i32, z: i32, block: u16) -> bool;

    /// Fills the given chunk section with `block`.
    ///
    /// Replaces all existing blocks in the section.
    ///
    /// This is an optimized bulk operation that will be significantly
    /// faster than calling [`block_set`] on each block in the chunk section.
    ///
    /// Returns `true` if successful and `false` if the
    /// block's chunk is not loaded or the section index is out of bounds.
    pub fn block_fill_chunk_section(chunk_x: i32, section_y: u32, chunk_z: i32, block: u16)
        -> bool;

    /// Sends a custom packet to an entity.
    ///
    /// Does nothing if the entity does not have the `ClientId` component.
    pub fn plugin_message_send(
        entity: EntityId,
        channel_ptr: Pointer<u8>,
        channel_len: u32,
        data_ptr: Pointer<u8>,
        data_len: u32,
    );
}
