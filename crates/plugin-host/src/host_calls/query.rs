//! Implements the `entity_query` host call.

use std::{alloc::Layout, any::TypeId, mem::size_of};

use anyhow::Context;
use feather_ecs::{DynamicQuery, DynamicQueryTypes, Ecs};
use quill_common::{
    component::{ComponentVisitor, SerializationMethod},
    entity::QueryData,
    Component, EntityId, HostComponent,
};
use wasmer::WasmPtr;

use crate::{context::PluginContext, wasm_ptr_ext::WasmPtrExt};

pub fn entity_query(
    cx: &mut PluginContext,
    components_ptr: WasmPtr<u32>,
    components_len: u32,
) -> anyhow::Result<WasmPtr<u8>> {
    let mut components = Vec::with_capacity(components_len as usize);
    for i in 0..components_len {
        let id = components_ptr
            .add(i as usize * size_of::<HostComponent>())
            .deref(cx.memory())
            .context("bad pointer")?;
        let component = HostComponent::from_u32(id.get()).context("bad component type")?;
        components.push(component);
    }

    let game = cx.game_mut();
    let query_data = create_query_data(cx, &game.ecs, &components)?;

    let ptr = cx.insert_to_memory(query_data)?;
    Ok(ptr)
}

struct WrittenComponentData {
    pointer: WasmPtr<u8>,
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
                            self.cx
                                .write_slice_to_memory(bytes, buffer.add(byte_index))?;
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
                let mut stride = None;
                // Write components into the buffer.
                for component_slice in components {
                    for component in component_slice.as_slice::<T>() {
                        component.to_bytes(&mut bytes);
                        if stride.is_none() {
                            stride = Some(bytes.len());
                        }
                    }
                }

                let buffer = self.cx.bump_allocate(Layout::array::<u8>(bytes.len())?)?;
                self.cx.write_slice_to_memory(&bytes, buffer)?;
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
            entities_ptr: 0,
            component_ptrs: 0,
            component_lens: 0,
        });
    }

    let component_ptrs = cx.bump_allocate(Layout::array::<u32>(types.len())?)?;
    let component_lens = cx.bump_allocate(Layout::array::<u32>(types.len())?)?;
    for (i, &typ) in types.iter().enumerate() {
        let data = typ.visit(WriteComponentsVisitor {
            query: &query,
            cx,
            num_entities,
        })?;

        cx.write_to_memory(
            data.pointer.offset(),
            component_ptrs.add(i * size_of::<u32>()),
        )?;
        cx.write_to_memory(data.len, component_lens.add(i * size_of::<u32>()))?;
    }

    let entities_ptr = cx.bump_allocate(Layout::array::<EntityId>(num_entities)?)?;
    for (i, entity) in query.iter_entities().enumerate() {
        let bits = entity.to_bits();
        cx.write_to_memory(bits, entities_ptr.add(i * size_of::<EntityId>()))?;
    }

    Ok(QueryData {
        num_entities: num_entities as u32,
        entities_ptr: entities_ptr.offset(),
        component_ptrs: component_ptrs.offset(),
        component_lens: component_lens.offset(),
    })
}
