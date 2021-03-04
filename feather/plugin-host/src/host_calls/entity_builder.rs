use anyhow::{bail, Context};
use feather_base::Position;
use feather_plugin_host_macros::host_function;
use quill_common::{component::ComponentVisitor, HostComponent};

use crate::context::{PluginContext, PluginPtr};

#[host_function]
pub fn entity_builder_new_empty(cx: &PluginContext) -> anyhow::Result<u32> {
    let builder = cx.game_mut().create_empty_entity_builder();
    let id = cx.entity_builders.borrow_mut().insert(builder);

    if id > u32::MAX as usize {
        bail!("created too many entity builders!");
    }

    Ok(id as u32)
}

#[host_function]
pub fn entity_builder_new(
    cx: &PluginContext,
    position: PluginPtr<Position>,
    entity_init_ptr: PluginPtr<u8>,
    entity_init_len: u32,
) -> anyhow::Result<u32> {
    let position = cx.read_pod(position)?;
    let init = cx.read_bincode(entity_init_ptr, entity_init_len)?;
    let builder = cx.game_mut().create_entity_builder(position, init);
    let id = cx.entity_builders.borrow_mut().insert(builder);

    if id > u32::MAX as usize {
        bail!("created too many entity builders");
    }

    Ok(id as u32)
}

struct BuilderAddComponentVisitor<'a> {
    builder: u32,
    cx: &'a PluginContext,
    bytes_ptr: PluginPtr<u8>,
    bytes_len: u32,
}

impl<'a> ComponentVisitor<anyhow::Result<()>> for BuilderAddComponentVisitor<'a> {
    fn visit<T: quill_common::Component>(self) -> anyhow::Result<()> {
        let component = self
            .cx
            .read_component::<T>(self.bytes_ptr, self.bytes_len)?;
        self.cx
            .entity_builders
            .borrow_mut()
            .get_mut(self.builder as usize)
            .context("invalid entity builder")?
            .add(component);
        Ok(())
    }
}

#[host_function]
pub fn entity_builder_add_component(
    cx: &PluginContext,
    builder: u32,
    component: u32,
    bytes_ptr: PluginPtr<u8>,
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

#[host_function]
pub fn entity_builder_finish(cx: &PluginContext, builder: u32) -> anyhow::Result<u64> {
    let builder = cx
        .entity_builders
        .borrow_mut()
        .remove(builder as usize)
        .context("invalid entity builder")?;

    let entity = cx.game_mut().spawn_entity(builder);
    Ok(entity.to_bits())
}
