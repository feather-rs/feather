use std::{
    alloc::Layout, collections::btree_map::Entry, convert::TryInto, marker::PhantomData, sync::Arc,
};

use anyhow::bail;
use bincode::config::Bounded;
use feather_common::Game;
use feather_ecs::Entity;
use quill_common::EntityId;
use quill_plugin_format::{PluginFile, PluginMetadata, PluginTarget, Triple};

use crate::{
    context::{PluginContext, PluginPtr, PluginPtrMut},
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

    /// 'function' must be pointer passed to the
    /// register command host call.
    ///
    /// caller: Some(entity) => player.
    /// caller: None => Terminal
    pub fn call_command(
        &self,
        game: &mut Game,
        input: &str,
        function: PluginPtrMut<u8>,
        caller: Option<EntityId>,
    ) -> anyhow::Result<i64> {
        self.context.enter(game, || {
            let context = Arc::clone(&self.context);

            // Allocate space inside the plugin memmory and copy over the command input
            // @TODO mention this as a potential error case.
            let input_len: u32 = input.len().try_into()?;
            let input_ptr = context.bump_allocate_and_write_bytes(input.as_bytes())?;

            // Allocate space inside the plugin memmory were we want the resulting i64 to be stor.
            let result_ptr_mut: PluginPtrMut<i64> =
                unsafe { context.bump_allocate(Layout::new::<i64>())?.cast() };

            // Allocate space inside the plugin memmory were we store callers entity_id: u64, but its an optional
            let encoded_caller = bincode::serialize(&caller)?;
            let encoded_caller_len = encoded_caller.len() as u32;
            let encoded_caller_ptr =
                context.bump_allocate_and_write_bytes(encoded_caller.as_slice())?;

            match &self.inner {
                Inner::Wasm(w) => {
                    let res = w.call_command(
                        function,
                        input_ptr,
                        input_len,
                        encoded_caller_ptr,
                        encoded_caller_len,
                        result_ptr_mut,
                    )?;
                    context.read_bincode::<i64>(unsafe { res.cast::<u8>() }, 64 / 8)
                }
                Inner::Native(n) => {
                    // n.call_command(game,input, function)
                    let res = n.call_command(
                        function,
                        input_ptr,
                        input_len,
                        encoded_caller_ptr,
                        encoded_caller_len,
                        result_ptr_mut,
                    )?;

                    context.read_bincode::<i64>(unsafe { res.cast::<u8>() }, 64 / 8)
                }
            }
        })
    }
}

enum Inner {
    Wasm(wasm::WasmPlugin),
    Native(native::NativePlugin),
}
