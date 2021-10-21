use std::mem::ManuallyDrop;
use std::{io::Write, sync::Arc};

use anyhow::{bail, Context};
use commands::dispatcher::{Args, CommandOutput, TabCompletion};
use libloading::Library;
use tempfile::{NamedTempFile, TempPath};

use quill::CommandContext;

use crate::context::{PluginContext, PluginPtrMut};

/// A native plugin loaded from a shared library
pub struct NativePlugin {
    /// The tempfile containing the shared library.
    tempfile: TempPath,

    /// The plugin's shared library.
    library: Library,

    /// The plugin's exported quill_setup function.
    ///
    /// Parameters:
    /// 1. Host context pointer
    /// 2. Pointer to bincode-encoded vtable
    /// 3. Length of bincode-encoded vtable
    enable: unsafe extern "C" fn(*const u8, *const u8, usize),

    /// The plugin's exported quill_run_system function.
    ///
    /// Parameters:
    /// 1. Plugin data pointer for this system
    run_system: unsafe extern "C" fn(*mut u8),

    /// The plugin's exported quill_run_command function.
    ///
    /// Parameters:
    /// 1. Plugin data pointer for the command executor
    /// 2. Arguments buffer pointer
    /// 3. Arguments length
    /// 4. Pointer to the command context
    ///
    /// Returns: command response (first: 0 = Ok(second), 1 = Err(_))
    run_command: unsafe extern "C" fn(*mut u8, *mut u8, u32, *mut u8) -> (u32, u64),

    /// The plugin's exported quill_run_command function.
    ///
    /// Parameters:
    /// 1. Plugin data pointer for the command executor
    /// 2. Arguments buffer pointer
    /// 3. Arguments length
    /// 4. Pointer to the command context
    /// 5. Fork index
    ///
    /// Returns: command response (first: 0 = Ok(second), 1 = Err(_))
    run_command_fork: unsafe extern "C" fn(*mut u8, *mut u8, u32, *mut u8, u32) -> (u32, u64),

    /// The plugin's exported quill_run_command function.
    ///
    /// Parameters:
    /// 1. Plugin data pointer for the command executor
    /// 2. Text buffer pointer
    /// 3. Text length
    /// 4. Pointer to the command context
    ///
    /// Returns: command completions Vec<(String, is_some, optional String)>
    run_command_completer:
        unsafe extern "C" fn(*mut u8, *mut u8, u32, *mut u8) -> (u32, u32, u32, u32, u32),
}

impl NativePlugin {
    pub fn load(module: &[u8]) -> anyhow::Result<Self> {
        // Libraries have to be loaded from files, so
        // we'll create a tempfile containing the module bytes.
        let mut tempfile = NamedTempFile::new()?;
        tempfile.write_all(module)?;
        tempfile.flush()?;
        let path = tempfile.into_temp_path();

        // SAFETY: Library::new() is unsafe because
        // the loaded module can execute arbitrary
        // code. Since native plugins are trusted,
        // this is sound.
        let library = unsafe { Library::new(&path)? };

        // SAFETY: these functions will not be accessed after the plugin is unloaded.
        let enable = unsafe {
            *library
                .get("quill_setup".as_bytes())
                .context("plugin is missing quill_setup export")?
        };
        let run_system = unsafe {
            *library
                .get("quill_run_system".as_bytes())
                .context("plugin is missing quill_run_system export")?
        };
        let run_command = unsafe {
            *library
                .get("quill_run_command".as_bytes())
                .context("plugin is missing quill_run_command export")?
        };
        let run_command_fork = unsafe {
            *library
                .get("quill_run_command_fork".as_bytes())
                .context("plugin is missing quill_run_command_fork export")?
        };
        let run_command_completer = unsafe {
            *library
                .get("quill_run_command_completer".as_bytes())
                .context("plugin is missing quill_run_command export")?
        };

        Ok(Self {
            tempfile: path,
            library,
            enable,
            run_system,
            run_command,
            run_command_fork,
            run_command_completer,
        })
    }

    pub fn enable(&self, context: Arc<PluginContext>) {
        let vtable = self.generate_vtable();
        let context_ptr = Arc::as_ptr(&context);
        // Ensure context stays alive
        std::mem::forget(context);

        // SAFETY: we assume the plugin is sound.
        unsafe {
            (self.enable)(
                context_ptr.cast::<u8>(),
                vtable.as_ptr(),
                vtable.len() as usize,
            )
        }
    }

    fn generate_vtable(&self) -> Vec<u8> {
        let vtable = crate::host_calls::generate_vtable();
        bincode::serialize(&vtable).expect("can't serialize vtable")
    }

    pub fn run_system(&self, data: PluginPtrMut<u8>) {
        unsafe { (self.run_system)(data.as_native()) }
    }

    pub fn run_command(
        &self,
        data_ptr: PluginPtrMut<u8>,
        args: &mut Args,
        ctx: CommandContext<()>,
    ) -> CommandOutput {
        // SAFETY: we assume the plugin is sound.
        unsafe {
            match {
                (self.run_command)(
                    data_ptr.as_native(),
                    args.as_mut_ptr() as *const _ as *mut _,
                    args.len() as u32,
                    &ctx as *const _ as *mut _,
                )
            } {
                (0, result) => Ok(result as i32),
                (1, error_ptr) => {
                    // Reading string from a pointer
                    const USIZE_SIZE: usize = std::mem::size_of::<usize>(); // 4 on wasm32, same as usize on native
                    type USIZE = usize; // u32 on wasm32, usize on native

                    let ptr = error_ptr as *const USIZE;
                    let len = *ptr.add(1);
                    let cap = *ptr.add(2);
                    let s = String::from_raw_parts(USIZE::from(*ptr) as *mut u8, len, cap);

                    bail!("Plugin command returned an error: {}", s)
                }
                _ => bail!("Invalid bool returned from function"),
            }
        }
    }

    pub fn run_command_fork(
        &self,
        data_ptr: PluginPtrMut<u8>,
        args: &mut Args,
        ctx: CommandContext<()>,
        f: u32,
    ) -> CommandOutput {
        // SAFETY: we assume the plugin is sound.
        unsafe {
            match {
                (self.run_command_fork)(
                    data_ptr.as_native(),
                    args.as_ptr() as *const _ as *mut _,
                    args.len() as u32,
                    &ctx as *const _ as *mut _,
                    f,
                )
            } {
                (0, result) => Ok(result as i32),
                (1, error_ptr) => {
                    // Reading string from a pointer
                    const USIZE_SIZE: usize = std::mem::size_of::<usize>(); // 4 on wasm32, same as usize on native
                    type USIZE = usize; // u32 on wasm32, usize on native

                    let ptr = error_ptr as *const USIZE;
                    let len = *ptr.add(1);
                    let cap = *ptr.add(2);
                    let s = String::from_raw_parts(USIZE::from(*ptr) as *mut u8, len, cap);

                    bail!("Plugin command returned an error: {}", s)
                }
                _ => bail!("Invalid bool returned from function"),
            }
        }
    }

    pub fn run_command_completer(
        &self,
        data_ptr: PluginPtrMut<u8>,
        text: &str,
        ctx: CommandContext<()>,
    ) -> anyhow::Result<TabCompletion> {
        // SAFETY: Text should be dropped on plugin side
        let mut text = ManuallyDrop::new(text.to_string());

        // SAFETY: we assume the plugin is sound.
        let (start, end, ptr, len, cap) = unsafe {
            (self.run_command_completer)(
                data_ptr.as_native(),
                text.as_mut_ptr(),
                text.len() as u32,
                &ctx as *const _ as *mut _,
            )
        };

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
