//! Feather's implementation of the [Quill API](https://github.com/feather-rs/quill).
//!
//! Uses [`wasmer`](https://docs.rs/wasmer) to run WebAssembly plugins
//! in a sandbox.

#![allow(warnings)] // TEMP

use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use ahash::AHashMap;
use anyhow::Context;
use env::PluginEnv;
use feather_common::Game;
use plugin::Plugin;
use quill_plugin_format::{PluginFile, PluginMetadata};
use vec_arena::Arena;
use wasmer::{
    ChainableNamedResolver, CompilerConfig, ExportError, Features, Function, ImportObject,
    Instance, Module, Store, JIT,
};
use wasmer_wasi::{WasiEnv, WasiState, WasiVersion};

mod context;
mod env;
mod host_calls;
mod host_function;
mod plugin;
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
    exceptions: true,
};

/// Unique ID of a plugin.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PluginId(usize);

/// Resource storing all enabled plugins plus the WebAssembly VM.
pub struct PluginManager {
    plugins: Arena<Plugin>,

    store: wasmer::Store,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    /// Creates a plugin manager with no plugins.
    pub fn new() -> Self {
        let compiler_config = compiler_config();
        let engine_config = JIT::new(compiler_config).features(WASM_FEATURES);
        let engine = engine_config.engine();
        let store = Store::new(&engine);

        Self {
            plugins: Arena::new(),
            store,
        }
    }

    /// Loads all plugins in the given directory.
    pub fn load_dir(&mut self, game: &mut Game, dir: impl AsRef<Path>) -> anyhow::Result<()> {
        let dir = dir.as_ref();
        if !dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }

            if entry.path().extension() != Some("plugin".as_ref()) {
                continue;
            }

            let bytes = fs::read(entry.path())?;
            self.load(game, &bytes).with_context(|| {
                format!("failed to load plugin from {}", entry.path().display())
            })?;
        }

        Ok(())
    }

    /// Loads and enables a plugin from the given plugin file bytes.
    ///
    /// Returns the ID of the loaded plugin.
    pub fn load(&mut self, game: &mut Game, file: &[u8]) -> anyhow::Result<PluginId> {
        let file = PluginFile::decode(file).context("malformed plugin file")?;

        let id = PluginId(self.plugins.next_vacant());
        let mut plugin = Plugin::load(self, &file, id)?;

        plugin.enable(game).context("failed to enable plugin")?;

        self.plugins.insert(plugin);

        Ok(id)
    }

    /// Gets the plugin with the given ID,
    /// or `None` if it has been unloaded.
    pub fn plugin(&self, id: PluginId) -> Option<&Plugin> {
        self.plugins.get(id.0)
    }

    /// Mutably gets the plugin with the given ID,
    /// or `None` if it has been unloaded.
    pub fn plugin_mut(&mut self, id: PluginId) -> Option<&mut Plugin> {
        self.plugins.get_mut(id.0)
    }
}

#[cfg(all(feature = "cranelift", not(feature = "llvm")))]
fn compiler_config() -> impl CompilerConfig {
    use wasmer::{Cranelift, CraneliftOptLevel};
    let mut cfg = Cranelift::new();
    cfg.opt_level(CraneliftOptLevel::Speed);
    cfg
}

#[cfg(feature = "llvm")]
fn compiler_config() -> impl CompilerConfig {
    use wasmer::{LLVMOptLevel, LLVM};
    let mut cfg = LLVM::new();
    cfg.opt_level(LLVMOptLevel::Aggressive);
    cfg
}
