//! Low-level plumbing used for calls between the host and the plugin.
//!
//! # Model
//! Function calls are split into two categories: _host_ and _plugin_ calls.
//! A host call consists of a function implemented on the host and called by
//! the plugin, while a plugin call is the other way around.
//!
//! Plugin calls are made by looking up the name of the function in the plugin's
//! dynamic library. This means functions plugins need to expose directly
//! to the host need to be marked as `no_mangle` and `extern "C"`.
//!
//! Host calls work differently. On initialization, the host passes in a `HostVTable`
//! which includes the server version, plus _function pointers_ which point to the
//! host's implementation of the API. To ensure soundness and backwards compatibility,
//! this vtable is passed between host and plugin using JSON.

use crate::log::LogLevel;
use crate::states::{AsyncState, GameState, StartupState};
use crate::system::{HandlerSpec, SystemSpec};
use crate::util::{FLayout, FStr};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, mem};

#[macro_use]
mod r#macro;

vtable! {
    // STARTUP
    /// Indicates to the host that the plugin and host versions
    /// are incompatible.
    ///
    /// `plugin_version`: JSON-serialized semver `Version`
    abort_incompatible_versions: fn(state: *mut StartupState, plugin_version: FStr),

    /// Aborts plugin initialization with a custom error.
    abort_init: fn(state: *mut StartupState, error: FStr),

    /// Registers a system. The system will be run every tick
    /// during the game.
    register_system: fn(state: *mut StartupState, spec: SystemSpec),

    /// Registers an event handler. The handle will be invoked
    /// when events of its type are triggered.
    register_handler: fn(state: *mut StartupState, spec: HandlerSpec),

    // IN GAME
    /// Creates a new `AsyncState`, given some `GameState`.
    ///
    /// The returned `AsyncState` should be dropped using `drop_async_state`
    /// when it exits the plugin's scope.
    create_async_state: fn(state: *mut GameState) -> *mut AsyncState,

    /// Drops an `AsyncState` handle.
    ///
    /// This will invalidate the passed pointer.
    drop_async_state: fn(state: *mut AsyncState),

    // MISC
    /// Prints a log message with the given level.
    log: fn(level: LogLevel, msg: FStr, module_path: FStr<'static>),

    // ALLOCATOR FUNCTIONS
    // These have the same constrants as the
    // methods for `std::alloc::GlobalAlloc`.
    /// Allocates a memory region
    alloc: fn(layout: FLayout) -> *mut u8,
    /// Deallocates a memory region
    dealloc: fn(ptr: *mut u8, layout: FLayout),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_vtable_roundtrip() {
        let vtable = HostVTable::dummy();

        let serialized = serde_json::to_string(&vtable).unwrap();
        let deserialized: HostVTable = serde_json::from_str(&serialized).unwrap();

        assert_eq!(vtable.version, deserialized.version);
        assert_eq!(
            vtable.log as *const () as usize,
            deserialized.log as *const () as usize
        );
    }
}
