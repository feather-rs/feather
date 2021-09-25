use std::{any::Any, marker::PhantomData, sync::Arc};

use wasmer::{FromToNativeWasmType, Store, WasmTypeList, WasmerEnv};

use crate::context::{PluginContext, PluginPtr, PluginPtrMut};
use crate::env::PluginEnv;

/// Signature of a host function.
pub trait WasmHostFunction {
    /// Creates a WASM function given the plugin environment.
    fn to_wasm_function(self, store: &Store, env: PluginEnv) -> wasmer::Function;
}

/// Signature of a host function for native.
/// All HostFunction types also implement NativeHostFunction.
///
/// This trait is implemented by the `#[host_function]`
/// macro attribute.
pub trait NativeHostFunction {
    /// Creates a raw function pointer to be included
    /// in the plugin's vtable.
    fn to_function_pointer(self) -> usize;
}
