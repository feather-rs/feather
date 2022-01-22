use crate::context::{PluginContext, PluginPtr};
use crate::host_calls::component::{InsertComponentVisitor, SetComponentAction};
use anyhow::Context;
use feather_ecs::Entity;
use feather_plugin_host_macros::host_function;
use quill_common::HostComponent;

#[host_function]
pub fn entity_add_event(
    cx: &PluginContext,
    entity: u64,
    event: u32,
    bytes_ptr: PluginPtr<u8>,
    bytes_len: u32,
) -> anyhow::Result<()> {
    let entity = Entity::from_bits(entity);
    let event = HostComponent::from_u32(event).context("invalid component")?;
    let visitor = InsertComponentVisitor {
        cx,
        bytes_ptr,
        bytes_len,
        action: SetComponentAction::AddEntityEvent(entity),
    };
    event.visit(visitor)
}

#[host_function]
pub fn add_event(
    cx: &PluginContext,
    event: u32,
    bytes_ptr: PluginPtr<u8>,
    bytes_len: u32,
) -> anyhow::Result<()> {
    let event = HostComponent::from_u32(event).context("invalid component")?;
    let visitor = InsertComponentVisitor {
        cx,
        bytes_ptr,
        bytes_len,
        action: SetComponentAction::AddEvent,
    };
    event.visit(visitor)
}
