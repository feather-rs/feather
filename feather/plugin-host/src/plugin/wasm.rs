use std::sync::Arc;

use anyhow::bail;
use quill_plugin_format::PluginMetadata;
use wasmer::{
    ChainableNamedResolver, Features, Function, ImportObject, Instance, Module, NativeFunc, Store,
};
use wasmer_wasi::{WasiEnv, WasiState, WasiVersion};

use crate::{
    context::{PluginContext, PluginPtr, PluginPtrMut},
    env::PluginEnv,
    PluginManager,
};

pub struct WasmPlugin {
    /// The WebAssembly instancing containing
    /// the plugin.
    instance: Instance,

    /// Exported function to enable the plugin.
    enable: Function,

    /// Exported function to call a command
    call_command: NativeFunc<(u32, u32, u32), u32>,

    /// Exported function to run a system given its data pointer.
    run_system: NativeFunc<u32>,
}

impl WasmPlugin {
    pub fn load(
        manager: &PluginManager,
        cx: &Arc<PluginContext>,
        module: &[u8],
        metadata: &PluginMetadata,
    ) -> anyhow::Result<Self> {
        let env = PluginEnv {
            context: Arc::clone(cx),
        };
        let quill_imports = crate::host_calls::generate_import_object(&manager.store, &env);
        let wasi_imports = generate_wasi_import_object(&manager.store, &metadata.identifier)?;
        let imports = quill_imports.chain_back(wasi_imports);

        let module = Module::new(&manager.store, module)?;
        let instance = Instance::new(&module, &imports)?;

        let run_system = instance
            .exports
            .get_function("quill_run_system")?
            .native()?
            .clone();
        let enable = instance.exports.get_function("quill_setup")?.clone();

        let call_command = instance
            .exports
            .get_function("quill_call_command")?
            .native()?
            .clone();

        Ok(Self {
            instance,
            run_system,
            call_command,
            enable,
        })
    }

    pub fn enable(&self) -> anyhow::Result<()> {
        self.enable.call(&[])?;
        Ok(())
    }

    pub fn run_system(&self, data_ptr: PluginPtrMut<u8>) -> anyhow::Result<()> {
        self.run_system.call(data_ptr.ptr as u32)?;
        Ok(())
    }

    pub fn call_command(
        &self,
        function: PluginPtrMut<u8>,
        input: PluginPtrMut<u8>,
        input_len: u32,
    ) -> anyhow::Result<i64> {
        ///  pub unsafe extern "C" fn quill_call_command(ptr: *mut u8, input_ptr: *mut u8, input_len: u32) -> u32 {
        let sucsess = self
            .call_command
            .call(function.ptr as u32, input.ptr as u32, input_len);
            
        
        if sucsess? == (false as u32) {
            bail!("Command parsing failed")
        } else {
            //TODO make return values work.
            Ok(0)
        }
    }
}

fn generate_wasi_import_object(store: &Store, plugin_name: &str) -> anyhow::Result<ImportObject> {
    let state = WasiState::new(plugin_name).build()?;
    let env = WasiEnv::new(state);
    Ok(wasmer_wasi::generate_import_object_from_env(
        store,
        env,
        WasiVersion::Latest,
    ))
}
