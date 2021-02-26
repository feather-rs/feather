use std::sync::Arc;

use wasmer::{ExportError, HostEnvInitError, Instance, WasmerEnv};

use crate::context::PluginContext;

/// The [`WasmerEnv`] passed to host calls.
#[derive(Clone)]
pub struct PluginEnv {
    pub context: Arc<PluginContext>,
}

impl WasmerEnv for PluginEnv {
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        self.context
            .init_with_instance(instance)
            .map_err(|e| wasmer::HostEnvInitError::Export(ExportError::Missing(e.to_string())))?;
        Ok(())
    }
}
