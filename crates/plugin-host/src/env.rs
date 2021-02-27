use std::{cell::RefMut, sync::Arc};

use wasmer::{ExportError, HostEnvInitError, WasmerEnv};

use crate::{context::PluginContext, thread_pinned::ThreadPinned};

/// The [`WasmerEnv`] passed to host calls.
#[derive(Clone)]
pub struct PluginEnv {
    pub context: Arc<ThreadPinned<PluginContext>>,
}

impl PluginEnv {
    pub fn new() -> Self {
        Self {
            context: Arc::new(ThreadPinned::new(PluginContext::default())),
        }
    }

    pub fn context(&self) -> RefMut<PluginContext> {
        self.context.borrow_mut()
    }
}

impl WasmerEnv for PluginEnv {
    fn init_with_instance(&mut self, instance: &wasmer::Instance) -> Result<(), HostEnvInitError> {
        self.context
            .borrow_mut()
            .init_with_instance(instance)
            .map_err(|e| HostEnvInitError::Export(ExportError::Missing(e.to_string())))
    }
}
