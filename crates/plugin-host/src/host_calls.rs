//! Implements all host calls defined in the `quill-sys` crate.

use std::{cell::RefCell, rc::Rc};

use crate::{
    context::PluginContext, env::PluginEnv, wasm_ptr_ext::WasmPtrIntoArray, PluginManager,
};
use anyhow::Context;
use feather_common::{ChatBox, Game};
use feather_ecs::Entity;
use wasmer::{Function, ImportObject, Store, WasmPtr};

mod component;
mod entity_builder;
mod query;

macro_rules! quill_imports {
    ($store:expr, $env:expr, $(
        $name:literal => $function:ident($($param:ident),*),
    )*) => {
        wasmer::imports! {
            "quill_01" => {
                $(
                    $name => Function::new_native_with_env($store, $env.clone(), |env: &PluginEnv, $($param,)*| {
                        let mut cx = env.context();
                        let result = $function(&mut *cx, $($param),*);
                        match result {
                            Ok(val) => val,
                            Err(e) => unsafe { wasmer::raise_user_trap(e.into()) },
                        }
                    }),
                )*
            }
        }
    }
}

pub fn create_import_object(store: &Store, env: PluginEnv) -> ImportObject {
    use self::component::{entity_get_component, entity_set_component};
    use self::entity_builder::{
        entity_builder_add_component, entity_builder_finish, entity_builder_new,
    };
    use self::query::entity_query;
    quill_imports! {
        store, env,
        "register_system" => register_system(system_data_ptr, name_ptr, name_len),
        "entity_query" => entity_query(components_ptr, components_len),
        "entity_exists" => entity_exists(entity),
        "entity_get_component" => entity_get_component(entity, component),
        "entity_set_component" => entity_set_component(entity, component, bytes_ptr, bytes_len),
        "entity_send_message" => entity_send_message(entity, message_ptr, message_len),
        "entity_builder_new" => entity_builder_new(position, init_ptr, init_len),
        "entity_builder_add_component" => entity_builder_add_component(builder, component, bytes_ptr, bytes_len),
        "entity_builder_finish" => entity_builder_finish(builder),
    }
}

fn register_system(
    cx: &mut PluginContext,
    system_data_ptr: WasmPtr<u8>,
    name_ptr: WasmPtr<u8>,
    name_len: u32,
) -> anyhow::Result<()> {
    let id = cx.plugin_id();
    let function = cx.run_system_fn().clone();

    let system = move |game: &mut Game| {
        let plugins = Rc::clone(&*game.resources.get::<Rc<RefCell<PluginManager>>>()?);
        let mut plugins = plugins.borrow_mut();
        plugins.invoke_plugin(game, id, || {
            function
                .call(system_data_ptr.offset())
                .map_err(anyhow::Error::from)
        })
    };

    let name = name_ptr
        .array()
        .get_utf8_string(cx.memory(), name_len)
        .context("invalid system name pointer")?;

    cx.game_mut()
        .system_executor
        .try_borrow_mut()?
        .add_system_with_name(system, &name);
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn entity_exists(cx: &mut PluginContext, entity: u64) -> anyhow::Result<u32> {
    let entity = Entity::from_bits(entity);
    let result = cx.game_mut().ecs.entity(entity).is_ok();
    Ok(result as u32)
}

fn entity_send_message(
    cx: &mut PluginContext,
    entity: u64,
    message_ptr: WasmPtr<u8>,
    message_len: u32,
) -> anyhow::Result<()> {
    let message = message_ptr
        .array()
        .get_utf8_string(cx.memory(), message_len)
        .context("invalid message")?;
    let game = cx.game_mut();
    if let Ok(mut mailbox) = game.ecs.get_mut::<ChatBox>(Entity::from_bits(entity)) {
        mailbox.send_chat(message);
    }
    Ok(())
}
