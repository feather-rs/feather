//! Implements all host calls defined in the `quill-sys` crate.

use std::{alloc::Layout, any::TypeId, cell::RefCell, mem::size_of, rc::Rc};

use crate::{
    context::PluginContext,
    env::PluginEnv,
    wasm_ptr_ext::{WasmPtrExt, WasmPtrIntoArray},
    PluginManager,
};
use anyhow::Context;
use feather_common::{ChatBox, Game};
use feather_ecs::{DynamicQueryTypes, Entity};
use quill_common::{EntityId, HostComponent, QueryData};
use wasmer::{Function, ImportObject, Store, WasmPtr};

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
    quill_imports! {
        store, env,
        "register_system" => register_system(system_data_ptr, name_ptr, name_len),
        "query_begin" => query_begin(components_ptr, components_len),
        "query_end" => query_end(),
        "entity_exists" => entity_exists(entity),
        "entity_send_message" => entity_send_message(entity, message_ptr, message_len),
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

fn query_begin(
    cx: &mut PluginContext,
    components_ptr: WasmPtr<u32>,
    components_len: u32,
) -> anyhow::Result<WasmPtr<u8>> {
    let component_types = read_component_types(cx, components_ptr, components_len)?;
    let query_types = DynamicQueryTypes::new(&component_types, &[]);

    if cx
        .game_mut()
        .ecs
        .query_dynamic(query_types)
        .iter_entities()
        .count()
        == 0
    {
        return Ok(cx.insert_to_memory(QueryData {
            num_entities: 0,
            entities_ptr: 0,
            component_ptrs: 0,
        })?);
    }

    let (num_entities, entites_ptr) = write_query_entities(cx, query_types)?;
    let component_ptrs = write_query_components(cx, query_types)?;

    let data = QueryData {
        num_entities: num_entities as u32,
        entities_ptr: entites_ptr.offset(),
        component_ptrs: component_ptrs.offset(),
    };

    Ok(cx.insert_to_memory(data)?)
}

fn read_component_types(
    cx: &PluginContext,
    components_ptr: WasmPtr<u32>,
    components_len: u32,
) -> anyhow::Result<Vec<TypeId>> {
    let mut components = Vec::new();
    for i in 0..components_len {
        let component: u32 = WasmPtr::<u32>::new(components_ptr.offset() + i)
            .deref(cx.memory())
            .context("invalid components ptr")?
            .get();
        let component = HostComponent::from_u32(component).context("invalid component")?;
        components.push(component.type_id());
    }
    Ok(components)
}

fn write_query_entities(
    cx: &mut PluginContext,
    query_types: DynamicQueryTypes,
) -> anyhow::Result<(usize, WasmPtr<u8>)> {
    let game = cx.game_mut();
    let query = game.ecs.query_dynamic(query_types);

    let num_entities = query.iter_entities().count();

    // Allocate a buffer to hold `num_entities` EntityIds.
    let layout = Layout::array::<EntityId>(num_entities)?;
    let ptr = cx.allocate(layout)?;

    // Copy the entity IDs to plugin memory.
    for (i, entity) in query.iter_entities().enumerate() {
        let id = EntityId(entity.to_bits());
        cx.write_to_memory(id, ptr.add(i * size_of::<EntityId>()))?;
    }

    drop(game);
    cx.push_query_garbage(ptr, layout);

    Ok((num_entities, ptr))
}

fn write_query_components(
    cx: &mut PluginContext,
    query_types: DynamicQueryTypes,
) -> anyhow::Result<WasmPtr<u8>> {
    let game = cx.game_mut();
    let query = game.ecs.query_dynamic(query_types);
    let num_entities = query.iter_entities().count();

    let mut garbage = Vec::new();

    let component_ptrs_layout = Layout::array::<u32>(query_types.read_types().len())?;
    let component_ptrs = cx.allocate(component_ptrs_layout)?;
    garbage.push((component_ptrs, component_ptrs_layout));

    for (component_index, &component_type) in query_types.read_types().iter().enumerate() {
        let mut allocation: Option<WasmPtr<u8>> = None;
        let mut i = 0;
        // Copy each slice of component data into plugin memory.
        for slice in query.iter_component_slices(component_type) {
            let allocation = match allocation {
                Some(alloc) => alloc,
                None => {
                    let layout = Layout::from_size_align(
                        slice.component_layout().size() * num_entities,
                        slice.component_layout().align(),
                    )?;
                    allocation = Some(cx.allocate(layout)?);
                    let ptr = allocation.unwrap();
                    garbage.push((ptr, layout));
                    ptr
                }
            };
            unsafe {
                cx.write_raw_to_memory(
                    slice.ptr().as_ptr(),
                    slice.len_in_bytes(),
                    allocation.add(i),
                )?;
            }
            i += slice.len_in_bytes();
        }
        cx.write_to_memory(
            allocation.unwrap().offset(),
            component_ptrs.add(size_of::<u32>() * component_index),
        )?;
    }

    drop(game);
    for (ptr, layout) in garbage {
        cx.push_query_garbage(ptr, layout);
    }

    Ok(component_ptrs)
}

fn query_end(cx: &mut PluginContext) -> anyhow::Result<()> {
    cx.deallocate_query_garbage()
}

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
