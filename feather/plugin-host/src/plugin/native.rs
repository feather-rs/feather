use std::{io::Write, marker::PhantomData, sync::Arc};

use anyhow::{bail, Context};
use libloading::Library;
use tempfile::{NamedTempFile, TempPath};

use crate::context::{PluginContext, PluginPtr, PluginPtrMut};

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

    call_command: unsafe extern "C" fn(
        cmd_ptr: *mut u8,
        input_ptr: *mut u8, // Input  is the string that the user wrote, like "/msg ..."".
        input_len: u32,
        caller_ptr: *mut u8, // Pointer to entity_id that might be null. bincode encoded.
        caller_len: u32,
        result: *mut i64,
    ) -> u32,
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

        let call_command = unsafe {
            *library
                .get("quill_call_command".as_bytes())
                .context("plugin is missing quill_call_command export")?
        };

        Ok(Self {
            tempfile: path,
            library,
            enable,
            run_system,
            call_command,
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
        // SAFETY: we assume the plugin is sound.
        unsafe { (self.run_system)(data.as_native()) }
    }

    pub fn call_command(
        &self,
        function: PluginPtrMut<u8>,
        input_ptr: PluginPtrMut<u8>,
        input_len: u32,
        caller_ptr: PluginPtrMut<u8>,
        caller_len: u32, // Maybe null pointer to i64 (EntityId), null means terminal.
        result_ptr: PluginPtrMut<i64>, // Pointer to were we expect resulting i64 to be stores if return value is true.
    ) -> anyhow::Result<PluginPtr<i64>> {
        let res = unsafe {
            (self.call_command)(
                function.as_native(),
                input_ptr.as_native(),
                input_len,
                caller_ptr.as_native(),
                caller_len,
                result_ptr.as_native(),
            )
        };

        if res == (true as u32) {
            // Turn PluginPtrMut into PluginPtr
            let result_ptr = PluginPtr {
                ptr: result_ptr.ptr,
                _marker: PhantomData,
            };
            Ok(result_ptr)
        } else {
            bail!("Command failed, probably due to not beeing able to parse")
        }
    }
}
