use std::alloc::Layout;

use anyhow::Context;
use feather_ecs::Entity;
use quill_common::{component::ComponentVisitor, entity::GetComponentResult, HostComponent};
use wasmer::WasmPtr;

use crate::context::PluginContext;

struct GetComponentVisitor<'a> {
    cx: &'a mut PluginContext,
    entity: Entity,
}

impl<'a> ComponentVisitor<anyhow::Result<u64>> for GetComponentVisitor<'a> {
    fn visit<T: quill_common::Component>(self) -> anyhow::Result<u64> {
        let game = self.cx.game_mut();
        let component = match game.ecs.get::<T>(self.entity) {
            Ok(c) => c,
            Err(_) => return Ok(0), // corresponds to `None`
        };
        let bytes = component.to_cow_bytes();
        let buffer = self.cx.bump_allocate(Layout::array::<u8>(bytes.len())?)?;
        self.cx.write_slice_to_memory(&bytes, buffer)?;

        Ok(GetComponentResult::new(buffer.offset(), bytes.len() as u32).to_bits())
    }
}

pub fn entity_get_component(
    cx: &mut PluginContext,
    entity: u64,
    component: u32,
) -> anyhow::Result<u64> {
    let component = HostComponent::from_u32(component).context("invalid component")?;
    let entity = Entity::from_bits(entity);
    let visitor = GetComponentVisitor { cx, entity };
    component.visit(visitor)
}

struct SetComponentVisitor<'a> {
    cx: &'a mut PluginContext,
    entity: Entity,
    bytes_ptr: WasmPtr<u8>,
    bytes_len: u32,
}

impl<'a> ComponentVisitor<anyhow::Result<()>> for SetComponentVisitor<'a> {
    fn visit<T: quill_common::Component>(self) -> anyhow::Result<()> {
        let component = self
            .cx
            .read_component::<T>(self.bytes_ptr, self.bytes_len)?;
        let mut game = self.cx.game_mut();
        
        let existing_component = game.ecs.get_mut::<T>(self.entity);
        if let Ok(mut existing_component) = existing_component {
            *existing_component = component;
        } else {
            drop(existing_component);
            let _= game.ecs.insert(self.entity, component);
        }

        Ok(())
    }
}

pub fn entity_set_component(
    cx: &mut PluginContext,
    entity: u64,
    component: u32,
    bytes_ptr: WasmPtr<u8>,
    bytes_len: u32,
) -> anyhow::Result<()> {
    let entity = Entity::from_bits(entity);
    let component = HostComponent::from_u32(component).context("invalid component")?;
    let visitor = SetComponentVisitor {
        cx,
        entity,
        bytes_ptr,
        bytes_len,
    };
    component.visit(visitor)
}
