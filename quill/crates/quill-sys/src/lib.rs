//! Raw FFI functions for host calls.

use quill_common::{
    entity::{GetComponentResult, QueryData},
    EntityId, HostComponent,
};

#[link(wasm_import_module = "quill_01")]
extern "C" {
    /// Registers a system.
    ///
    /// Each tick, the system is invoked
    /// by calling the plugin's exported `quill_run_system` method.
    /// `quill_run_systsem` is given the `system_data` pointer passed
    /// to this host call.
    pub fn register_system(system_data: *mut u8, name_ptr: *const u8, name_len: u32);

    /// Initiates a query. Returns the query data.
    ///
    /// The returned query buffers are allocated within
    /// the plugin's bump allocator. They will be
    /// freed automatically after the plugin finishes
    /// executing the current system.
    pub fn entity_query(
        components_ptr: *const HostComponent,
        components_len: u32,
    ) -> *const QueryData;

    /// Determines whether the given entity exists.
    pub fn entity_exists(entity: EntityId) -> bool;

    /// Gets a component for an entity.
    ///
    /// Returns a pointer to the component's
    /// serialized bytes as well as the length
    /// of the bytes.
    pub fn entity_get_component(
        entity: EntityId,
        component: HostComponent,
    ) -> Option<GetComponentResult>;

    /// Sets or replaces a component for an entity.
    ///
    /// `bytes_ptr` is a pointer to the serialized
    /// component.
    ///
    /// This will overwrite any existing component of the same type.
    ///
    /// Does nothing if `entity` does not exist.
    pub fn entity_set_component(
        entity: EntityId,
        component: HostComponent,
        bytes_ptr: u32,
        bytes_len: u32,
    );

    /// Sends a message to an entity.
    ///
    /// The given message should be in the JSON format.
    ///
    /// Does nothing if the entity does not exist or it does not have the `Chat` component.
    pub fn entity_send_message(entity: EntityId, message_ptr: *const u8, message_len: u32);

    /// Creates an entity builder.
    ///
    /// The builder is initialized with the default
    /// components for the given `EntityInit`.
    ///
    /// `entity_init` is a `bincode`-serialized `EntityInit`.
    pub fn entity_builder_new(
        position: *const u8,
        entity_init_ptr: *const u8,
        entity_init_len: u32,
    ) -> u32;

    /// Adds a component to an entity builder.
    ///
    /// `bytes` is the serialized component.
    pub fn entity_builder_add_component(
        builder: u32,
        component: HostComponent,
        bytes_ptr: *const u8,
        bytes_len: u32,
    );

    /// Creates an entity from an entity builder.
    ///
    /// Returns the new entity.
    ///
    /// `builder` is consumed after this call.
    /// Reusing it is undefined behavior.
    pub fn entity_builder_finish(builder: u32) -> EntityId;
}
