//! Implements all host calls defined in the `quill-sys` crate.

use std::collections::HashMap;
use std::sync::Arc;

use paste::paste;

use crate::env::PluginEnv;
use crate::host_function::{NativeHostFunction, WasmHostFunction};

mod block;
mod component;
mod entity;
mod entity_builder;
mod event;
mod plugin_message;
mod query;
mod system;

macro_rules! host_calls {
    (
        $($name:literal => $function:ident),* $(,)?
    ) => {
        pub fn generate_vtable() -> HashMap<&'static str, usize> {
            let mut vtable = HashMap::new();
            $(
                paste! {
                    vtable.insert($name, [< $function _struct >].to_function_pointer());
                }
            )*
            vtable
        }

        pub fn generate_import_object(store: &wasmer::Store, env: &PluginEnv) -> wasmer::ImportObject {
            $(
                paste! {
                    let $function = [< $function _struct >].to_wasm_function(store, env.clone());
                }
            )*
            wasmer::imports! {
                "quill_01" => {$(
                    $name => $function,
                )*}
            }
        }
    }
}

use block::*;
use component::*;
use entity::*;
use entity_builder::*;
use event::*;
use plugin_message::*;
use query::*;
use system::*;

host_calls! {
    "register_system" => register_system,
    "entity_get_component" => entity_get_component,
    "entity_set_component" => entity_set_component,
    "entity_add_event" => entity_add_event,
    "add_event" => add_event,
    "entity_builder_new_empty" => entity_builder_new_empty,
    "entity_builder_new" => entity_builder_new,
    "entity_builder_add_component" => entity_builder_add_component,
    "entity_builder_finish" => entity_builder_finish,
    "entity_query" => entity_query,
    "entity_exists" => entity_exists,
    "entity_send_message" => entity_send_message,
    "entity_send_title" => entity_send_title,
    "block_get" => block_get,
    "block_set" => block_set,
    "block_fill_chunk_section" => block_fill_chunk_section,
    "plugin_message_send" => plugin_message_send,
}
