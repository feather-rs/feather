//! Implements the `entity_query` host call.

use std::{alloc::Layout, any::TypeId, mem::size_of, ptr};

use anyhow::Context;
use feather_ecs::{DynamicQuery, DynamicQueryTypes, Ecs};
use feather_plugin_host_macros::host_function;
use quill_common::{
    component::{ComponentVisitor, SerializationMethod},
    entity::QueryData,
    Component, EntityId, HostComponent, PointerMut,
};

use crate::context::{PluginContext, PluginPtr, PluginPtrMut};

#[host_function]
pub fn entity_query(
    cx: &PluginContext,
    components_ptr: PluginPtr<u32>,
    components_len: u32,
    query_data_out: PluginPtrMut<QueryData>,
) -> anyhow::Result<()> {
    let mut components = Vec::with_capacity(components_len as usize);
    for i in 0..components_len {
        let ptr = unsafe { components_ptr.add(i as usize) };
        let id = cx.read_pod(ptr)?;

        let component = HostComponent::from_u32(id).context("bad component type")?;
        components.push(component);
    }

    let game = cx.game_mut();
    let query_data = create_query_data(cx, &game.ecs, &components)?;
    cx.write_pod(query_data_out, query_data)?;

    Ok(())
}

struct WrittenComponentData {
    pointer: PluginPtrMut<u8>,
    len: u32,
}

/// `ComponentVisitor` implementation used to write
/// component data to plugin memory.
struct WriteComponentsVisitor<'a> {
    query: &'a DynamicQuery<'a>,
    cx: &'a PluginContext,
    num_entities: usize,
}

impl<'a> ComponentVisitor<anyhow::Result<WrittenComponentData>> for WriteComponentsVisitor<'a> {
    fn visit<T: Component>(self) -> anyhow::Result<WrittenComponentData> {
        let components = self.query.iter_component_slices(TypeId::of::<T>());

        // Write each component.
        // We use a different strategy depending
        // on how the component is serialized.
        let (buffer, len) = match T::SERIALIZATION_METHOD {
            SerializationMethod::Bytemuck => {
                // Allocate enough memory to hold all the components.
                let layout = Layout::array::<T>(self.num_entities)?;
                let buffer = self.cx.bump_allocate(layout)?;

                if size_of::<T>() != 0 {
                    // Copy the components into the buffer.
                    let mut byte_index = 0;
                    for component_slice in components {
                        for component in component_slice.as_slice::<T>() {
                            let bytes = component.as_bytes();

                            unsafe {
                                self.cx.write_bytes(buffer.add(byte_index), bytes)?;
                            }

                            byte_index += bytes.len();
                        }
                    }
                }

                (buffer, self.num_entities * size_of::<T>())
            }
            SerializationMethod::Bincode => {
                // Memory will need to be allocated dynamically,
                // but we can approximate a minimum capacity.
                let mut bytes = Vec::with_capacity(self.num_entities * size_of::<T>());

                // Write components into the buffer.
                for component_slice in components {
                    for component in component_slice.as_slice::<T>() {
                        component.to_bytes(&mut bytes);
                    }
                }

                let buffer = self.cx.bump_allocate_and_write_bytes(&bytes)?;
                (buffer, bytes.len())
            }
        };

        Ok(WrittenComponentData {
            pointer: buffer,
            len: len as u32,
        })
    }
}

fn create_query_data(
    cx: &PluginContext,
    ecs: &Ecs,
    types: &[HostComponent],
) -> anyhow::Result<QueryData> {
    let query_types: Vec<TypeId> = types.iter().copied().map(HostComponent::type_id).collect();
    let query = ecs.query_dynamic(DynamicQueryTypes::new(&query_types, &[]));

    let num_entities = query.iter_entities().count();
    if num_entities == 0 {
        return Ok(QueryData {
            num_entities: 0,
            entities_ptr: PointerMut::new(ptr::null_mut()),
            component_ptrs: PointerMut::new(ptr::null_mut()),
            component_lens: PointerMut::new(ptr::null_mut()),
        });
    }

    let component_ptrs = cx.bump_allocate(Layout::array::<PluginPtrMut<u8>>(types.len())?)?;
    let component_lens = cx.bump_allocate(Layout::array::<u32>(types.len())?)?;
    for (i, &typ) in types.iter().enumerate() {
        let data = typ.visit(WriteComponentsVisitor {
            query: &query,
            cx,
            num_entities,
        })?;

        unsafe {
            cx.write_pod(component_ptrs.cast().add(i), data.pointer)?;
            cx.write_pod(component_lens.cast().add(i), data.len)?;
        }
    }

    let entities_ptr = cx.bump_allocate(Layout::array::<EntityId>(num_entities)?)?;
    for (i, entity) in query.iter_entities().enumerate() {
        let bits = entity.to_bits();
        unsafe {
            cx.write_pod(entities_ptr.cast().add(i), bits)?;
        }
    }

    Ok(QueryData {
        num_entities: num_entities as u64,
        entities_ptr: PointerMut::new(entities_ptr.as_native().cast()),
        component_ptrs: PointerMut::new(component_ptrs.as_native().cast()),
        component_lens: PointerMut::new(component_lens.as_native().cast()),
    })
}
