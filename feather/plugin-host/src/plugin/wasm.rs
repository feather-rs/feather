use std::mem::ManuallyDrop;
use std::sync::Arc;

use anyhow::bail;
use commands::dispatcher::{Args, CommandOutput, TabCompletion};
use wasmer::{
    ChainableNamedResolver, Features, Function, ImportObject, Instance, Module, NativeFunc, Store,
};
use wasmer_wasi::{WasiEnv, WasiState, WasiVersion};

use quill::CommandContext;
use quill_common::{Pointer, PointerMut};
use quill_plugin_format::PluginMetadata;

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

    /// Exported function to run a system given its data pointer.
    run_system: NativeFunc<(u32)>,

    /// Exported function to run a command executor given its data pointer, args, args length,
    /// command context pointer and return command execution result (should be casted to bool).
    run_command: NativeFunc<(u32, u32, u32, u32), (u32, u64)>,

    /// Exported function to run a command fork given its data pointer, args, args length,
    /// command context pointer, fork index and return command execution result
    run_command_fork: NativeFunc<(u32, u32, u32, u32, u32), (u32, u64)>,

    /// Exported function to run a command completer given its data pointer, text, text length,
    /// command context pointer and return command completions: Vec<(String, is_some, optional String)>.
    run_command_completer: NativeFunc<(u32, u32, u32, u32), (u32, u32, u32, u32, u32)>,
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
        let run_command = instance
            .exports
            .get_function("quill_run_command")?
            .native()?
            .clone();
        let run_command_fork = instance
            .exports
            .get_function("quill_run_command_fork")?
            .native()?
            .clone();
        let run_command_completer = instance
            .exports
            .get_function("quill_run_command_completer")?
            .native()?
            .clone();
        let enable = instance.exports.get_function("quill_setup")?.clone();

        Ok(Self {
            instance,
            enable,
            run_system,
            run_command,
            run_command_fork,
            run_command_completer,
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

    pub fn run_command(
        &self,
        data_ptr: PluginPtrMut<u8>,
        args: &mut Args,
        ctx: CommandContext<()>,
    ) -> CommandOutput {
        match self.run_command.call(
            data_ptr.ptr as u32,
            args.as_mut_ptr() as usize as u32,
            args.len() as u32,
            &ctx as *const _ as usize as u32,
        )? {
            (0, result) => Ok(result as i32),
            (1, error_ptr) => {
                // Reading string from a pointer
                const USIZE_SIZE: usize = 4; // 4 on wasm32, same as usize on native
                type USIZE = u32; // u32 on wasm32, usize on native

                unsafe {
                    let ptr = error_ptr as *const USIZE;
                    let len = *ptr.add(1);
                    let cap = *ptr.add(2);
                    let s = String::from_raw_parts(todo!(), len as usize, cap as usize);

                    bail!("Plugin command returned an error: {}", s)
                }
            }
            _ => bail!("Invalid bool returned from function"),
        }
    }

    pub fn run_command_fork(
        &self,
        data_ptr: PluginPtrMut<u8>,
        args: &mut Args,
        ctx: CommandContext<()>,
        f: u32,
    ) -> CommandOutput {
        match self.run_command_fork.call(
            data_ptr.ptr as u32,
            args.as_ptr() as usize as u32,
            args.len() as u32,
            &ctx as *const _ as usize as u32,
            f as u32,
        )? {
            (0, result) => Ok(result as i32),
            (1, error_ptr) => {
                // Reading string from a pointer
                const USIZE_SIZE: usize = 4; // 4 on wasm32, same as usize on native
                type USIZE = u32; // u32 on wasm32, usize on native

                unsafe {
                    let ptr = error_ptr as *const USIZE;
                    let len = *ptr.add(1);
                    let cap = *ptr.add(2);
                    let s = String::from_raw_parts(todo!(), len as usize, cap as usize);

                    bail!("Plugin command returned an error: {}", s)
                }
            }
            _ => bail!("Invalid bool returned from function"),
        }
    }

    pub fn run_command_completer(
        &self,
        data_ptr: PluginPtrMut<u8>,
        text: &str,
        ctx: CommandContext<()>,
    ) -> anyhow::Result<TabCompletion> {
        // SAFETY: Text should be dropped on plugin side
        let text = ManuallyDrop::new(text.to_string());

        let (start, end, ptr, len, cap) = self.run_command_completer.call(
            data_ptr.ptr as u32,
            text.as_ptr() as usize as u32,
            text.len() as u32,
            &ctx as *const _ as usize as u32,
        )?;

        // SAFETY: assuming plugin sent valid data
        Ok((start as usize, end as usize, unsafe {
            Vec::from_raw_parts(
                ptr as *mut ((*mut u8, u32, u32), bool, (*mut u8, u32, u32)),
                len as usize,
                cap as usize,
            )
            .into_iter()
            .map(
                |((comp, comp_len, comp_cap), is_some, (tooltip, tooltip_len, tooltip_cap))| {
                    (
                        String::from_raw_parts(comp, comp_len as usize, comp_cap as usize),
                        if is_some {
                            Some(String::from_raw_parts(
                                tooltip,
                                tooltip_len as usize,
                                tooltip_cap as usize,
                            ))
                        } else {
                            None
                        },
                    )
                },
            )
            .collect()
        }))
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
