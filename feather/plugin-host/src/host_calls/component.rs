use anyhow::Context;
use feather_ecs::Entity;
use feather_plugin_host_macros::host_function;
use quill_common::{component::ComponentVisitor, HostComponent};

use crate::context::{PluginContext, PluginPtr, PluginPtrMut};

struct GetComponentVisitor<'a> {
    cx: &'a PluginContext,
    entity: Entity,
}

impl<'a> ComponentVisitor<anyhow::Result<(PluginPtrMut<u8>, u32)>> for GetComponentVisitor<'a> {
    fn visit<T: quill_common::Component>(self) -> anyhow::Result<(PluginPtrMut<u8>, u32)> {
        let game = self.cx.game_mut();
        let component = match game.ecs.get::<T>(self.entity) {
            Ok(c) => c,
            Err(_) => return Ok((unsafe { PluginPtrMut::null() }, 0)),
        };
        let bytes = component.to_cow_bytes();
        let ptr = self.cx.bump_allocate_and_write_bytes(&bytes)?;

        Ok((ptr, bytes.len() as u32))
    }
}

#[host_function]
pub fn entity_get_component(
    cx: &PluginContext,
    entity: u64,
    component: u32,
    bytes_ptr_ptr: PluginPtrMut<PluginPtrMut<u8>>,
    bytes_len_ptr: PluginPtrMut<u32>,
) -> anyhow::Result<()> {
    let component = HostComponent::from_u32(component).context("invalid component")?;
    let entity = Entity::from_bits(entity);
    let visitor = GetComponentVisitor { cx, entity };
    let (bytes_ptr, bytes_len) = component.visit(visitor)?;

    cx.write_pod(bytes_ptr_ptr, bytes_ptr)?;
    cx.write_pod(bytes_len_ptr, bytes_len)?;

    Ok(())
}

pub(crate) struct InsertComponentVisitor<'a> {
    pub cx: &'a PluginContext,
    pub bytes_ptr: PluginPtr<u8>,
    pub bytes_len: u32,
    pub action: SetComponentAction,
}

pub(crate) enum SetComponentAction {
    SetComponent(Entity),
    AddEntityEvent(Entity),
    AddEvent,
}

impl<'a> ComponentVisitor<anyhow::Result<()>> for InsertComponentVisitor<'a> {
    fn visit<T: quill_common::Component>(self) -> anyhow::Result<()> {
        let component = self
            .cx
            .read_component::<T>(self.bytes_ptr, self.bytes_len)?;
        let mut game = self.cx.game_mut();

        match self.action {
            SetComponentAction::SetComponent(entity) => {
                let _ = game.ecs.insert(entity, component);
            }
            SetComponentAction::AddEntityEvent(entity) => {
                let _ = game.ecs.insert_entity_event(entity, component);
            }
            SetComponentAction::AddEvent => {
                game.ecs.insert_event(component);
            }
        }

        Ok(())
    }
}

#[host_function]
pub fn entity_set_component(
    cx: &PluginContext,
    entity: u64,
    component: u32,
    bytes_ptr: PluginPtr<u8>,
    bytes_len: u32,
) -> anyhow::Result<()> {
    let entity = Entity::from_bits(entity);
    let component = HostComponent::from_u32(component).context("invalid component")?;
    let visitor = InsertComponentVisitor {
        cx,
        bytes_ptr,
        bytes_len,
        action: SetComponentAction::SetComponent(entity),
    };
    component.visit(visitor)
}
