//! Raw FFI functions for host calls.

use quill_common::{EntityId, HostComponent, QueryData};

#[link(wasm_import_module = "quill_01")]
extern "C" {
    pub fn register_system(system_data: *mut u8, name_ptr: *const u8, name_len: u32);

    /// Initiates a query. Returns the query data.
    ///
    /// [`query_end`] should be called to free memory after the query is processed.
    pub fn query_begin(
        components_ptr: *const HostComponent,
        components_len: u32,
    ) -> *const QueryData;

    /// Should be called after any number of calls to`query_begin` to free
    /// query allocations.
    ///
    /// References to data returned by the query are invalidated
    /// after this call.
    pub fn query_end();

    /// Determines whether the given entity exists.
    pub fn entity_exists(entity: EntityId) -> bool;

    /// Sends a message to an entity. The given message should be in the JSON format.
    ///
    /// Does nothing if the entity does not exist or it does not have the `Chat` component.
    pub fn entity_send_message(entity: EntityId, message_ptr: *const u8, message_len: u32);
}
