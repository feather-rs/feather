//! Feather's implementation of the [Quill API](https://github.com/feather-rs/quill).
//!
//! Uses [`wasmer`](https://docs.rs/wasmer) to run WebAssembly plugins
//! in a sandbox.

use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use ahash::AHashMap;
use anyhow::Context;
use env::PluginEnv;
use feather_common::Game;
use quill_plugin_format::{PluginFile, PluginMetadata};
use wasmer::{
    Cranelift, CraneliftOptLevel, ExportError, Features, Function, Instance, Module, Store, JIT,
};

mod context;
mod env;
mod host_calls;
mod thread_pinned;
mod wasm_ptr_ext;

/// Features enabled for WASM plugins
const WASM_FEATURES: Features = Features {
    threads: true,
    reference_types: false,
    simd: true,
    bulk_memory: true,
    multi_value: false,
    tail_call: false,
    module_linking: false,
    multi_memory: false,
    memory64: false,
};

/// Unique ID of a plugin.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PluginId(usize);

impl PluginId {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let x = NEXT.fetch_add(1, Ordering::SeqCst);
        Self(x)
    }
}

/// Resource storing all enabled plugins plus the WebAssembly VM.
pub struct PluginManager {
    _engine: wasmer::JITEngine,
    store: wasmer::Store,

    plugins: AHashMap<PluginId, Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut compiler_config = Cranelift::new();
        compiler_config
            .opt_level(CraneliftOptLevel::Speed)
            .enable_simd(true);
        let engine_config = JIT::new(compiler_config).features(WASM_FEATURES);
        let engine = engine_config.engine();
        let store = Store::new(&engine);

        Self {
            _engine: engine,
            store,
            plugins: AHashMap::new(),
        }
    }

    /// Loads all plugins found within the given directory.
    pub fn load_plugin_directory(
        &mut self,
        game: &mut Game,
        dir: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let dir = dir.as_ref();
        if !dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            if entry.path().extension() != Some("plugin".as_ref()) {
                continue;
            }

            log::info!("Loading plugin from {}", entry.path().display());
            let plugin = fs::read(entry.path())?;
            self.load_plugin(game, &plugin)?;
        }

        Ok(())
    }

    /// Loads a plugin from its plugin file data.
    pub fn load_plugin(&mut self, game: &mut Game, plugin: &[u8]) -> anyhow::Result<PluginId> {
        let plugin = Plugin::new(&self.store, plugin)?;
        let id = PluginId::new();
        plugin.env.context().set_plugin_id(id);

        let setup_function = plugin.setup_function()?.clone();

        self.plugins.insert(id, plugin);

        // Call the setup function
        self.invoke_plugin(game, id, || {
            setup_function.call(&[]).map_err(From::from).map(|_| ())
        })?;

        let plugin = self.plugin(id).unwrap();
        log::info!(
            "Enabled plugin {} version {}",
            plugin.metadata().name,
            plugin.metadata().version
        );

        Ok(id)
    }

    pub fn plugin(&self, id: PluginId) -> Option<&Plugin> {
        self.plugins.get(&id)
    }

    pub fn plugin_mut(&mut self, id: PluginId) -> Option<&mut Plugin> {
        self.plugins.get_mut(&id)
    }

    /// Invokes a plugin method using the given closure.
    ///
    /// This method ensures that the plugin's context is initialized
    /// with the `Game`.
    pub fn invoke_plugin(
        &mut self,
        game: &mut Game,
        plugin_id: PluginId,
        invoke: impl FnOnce() -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let plugin = self.plugin_mut(plugin_id).context("missing plugin")?;
        plugin.env.context().set_game(game);
        invoke()
    }
}

/// A loaded plugin.
pub struct Plugin {
    _module: Module,
    instance: Instance,
    env: PluginEnv,
    metadata: PluginMetadata,
}

impl Plugin {
    /// Loads a plugin from its binary WASM module.
    pub fn new(store: &Store, plugin: &[u8]) -> anyhow::Result<Self> {
        let plugin_file = PluginFile::decode(plugin)?;

        let module = Module::new(store, plugin_file.wasm_bytecode())?;
        let env = PluginEnv::new();

        let imports = host_calls::create_import_object(store, env.clone());

        let instance = Instance::new(&module, &imports)?;

        Ok(Self {
            _module: module,
            instance,
            env,
            metadata: plugin_file.metadata().clone(),
        })
    }

    pub fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    pub fn setup_function(&self) -> Result<&Function, ExportError> {
        self.instance.exports.get_function("quill_setup")
    }
}
