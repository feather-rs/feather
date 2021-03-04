use std::sync::Arc;

use anyhow::bail;
use feather_common::Game;
use quill_plugin_format::{PluginFile, PluginMetadata, PluginTarget, Triple};

use crate::{
    context::{PluginContext, PluginPtrMut},
    PluginId, PluginManager,
};

mod native;
mod wasm;

pub struct Plugin {
    inner: Inner,
    context: Arc<PluginContext>,
    metadata: PluginMetadata,
}

impl Plugin {
    /// Loads a plugin from the given plugin file.
    ///
    /// Does not enable the plugin.
    pub fn load(manager: &PluginManager, file: &PluginFile, id: PluginId) -> anyhow::Result<Self> {
        let plugin_type = match &file.metadata().target {
            PluginTarget::Wasm => "WebAssembly",
            PluginTarget::Native { .. } => "native",
        };
        log::info!(
            "Loading {} plugin {} version {}",
            plugin_type,
            file.metadata().name,
            file.metadata().version
        );

        let (inner, context) = match &file.metadata().target {
            PluginTarget::Wasm => {
                let context = Arc::new(PluginContext::new_wasm(id));
                let plugin =
                    wasm::WasmPlugin::load(manager, &context, file.module(), file.metadata())?;
                (Inner::Wasm(plugin), context)
            }
            PluginTarget::Native { target_triple } => {
                if target_triple != &Triple::host() {
                    bail!(
                        "native plguin was built for {}, but this system has target {}",
                        target_triple,
                        Triple::host()
                    );
                }
                let plugin = native::NativePlugin::load(file.module())?;
                let context = PluginContext::new_native(id);
                (Inner::Native(plugin), Arc::new(context))
            }
        };

        Ok(Self {
            inner,
            context,
            metadata: file.metadata().clone(),
        })
    }

    /// Enables the plugin.
    ///
    /// # Panics
    /// Panics if called more than once.
    pub fn enable(&mut self, game: &mut Game) -> anyhow::Result<()> {
        let context = Arc::clone(&self.context);

        self.context.enter(game, || match &self.inner {
            Inner::Wasm(w) => w.enable(),
            Inner::Native(n) => {
                n.enable(context);
                Ok(())
            }
        })?;

        log::info!("Enabled plugin {} ", self.metadata.name);
        Ok(())
    }

    /// Runs a plugin system.
    ///
    /// `data` must be the data pointer passed
    /// to the `register_system` host call.
    pub fn run_system(&self, game: &mut Game, data: PluginPtrMut<u8>) -> anyhow::Result<()> {
        self.context.enter(game, || match &self.inner {
            Inner::Wasm(w) => w.run_system(data),
            Inner::Native(n) => {
                n.run_system(data);
                Ok(())
            }
        })
    }
}

enum Inner {
    Wasm(wasm::WasmPlugin),
    Native(native::NativePlugin),
}
