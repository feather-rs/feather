use anyhow::{bail, Context};
use quill_common::{component::ComponentVisitor, HostComponent};
use wasmer::WasmPtr;

use crate::context::PluginContext;

pub fn entity_builder_new(
    cx: &mut PluginContext,
    position: WasmPtr<u8>,
    entity_init_ptr: WasmPtr<u8>,
    entity_init_len: u32,
) -> anyhow::Result<u32> {
    let position = cx.read_pod(position)?;
    let init = cx.read_bincode(entity_init_ptr, entity_init_len)?;
    let builder = cx.game_mut().create_entity_builder(position, init);
    let id = cx.entity_builders.insert(builder);

    if id > u32::MAX as usize {
        bail!("created too many entity builders");
    }

    Ok(id as u32)
}

struct BuilderAddComponentVisitor<'a> {
    builder: u32,
    cx: &'a mut PluginContext,
    bytes_ptr: WasmPtr<u8>,
    bytes_len: u32,
}

impl<'a> ComponentVisitor<anyhow::Result<()>> for BuilderAddComponentVisitor<'a> {
    fn visit<T: quill_common::Component>(self) -> anyhow::Result<()> {
        let component = self
            .cx
            .read_component::<T>(self.bytes_ptr, self.bytes_len)?;
        self.cx
            .entity_builders
            .get_mut(self.builder as usize)
            .context("invalid entity builder")?
            .add(component);
        Ok(())
    }
}

pub fn entity_builder_add_component(
    cx: &mut PluginContext,
    builder: u32,
    component: u32,
    bytes_ptr: WasmPtr<u8>,
    bytes_len: u32,
) -> anyhow::Result<()> {
    let component = HostComponent::from_u32(component).context("invalid component")?;
    let visitor = BuilderAddComponentVisitor {
        builder,
        cx,
        bytes_ptr,
        bytes_len,
    };
    component.visit(visitor)
}

pub fn entity_builder_finish(cx: &mut PluginContext, builder: u32) -> anyhow::Result<u64> {
    let builder = cx
        .entity_builders
        .remove(builder as usize)
        .context("invalid entity builder")?;

    let entity = cx.game_mut().spawn_entity(builder);
    Ok(entity.to_bits())
}
