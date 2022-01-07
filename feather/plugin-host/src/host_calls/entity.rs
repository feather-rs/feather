use feather_base::Text;
use feather_ecs::Entity;
use feather_plugin_host_macros::host_function;

use crate::context::{PluginContext, PluginPtr};
use quill_common::components::{ChatKind, ChatMessage};

#[host_function]
pub fn entity_exists(cx: &PluginContext, entity: u64) -> anyhow::Result<u32> {
    Ok(cx.game_mut().ecs.entity(Entity::from_bits(entity)).is_ok()).map(|b| b as u32)
}

#[host_function]
pub fn entity_send_message(
    cx: &PluginContext,
    entity: u64,
    message_ptr: PluginPtr<u8>,
) -> anyhow::Result<()> {
    let message = message_ptr.as_native() as *const Text;
    let entity = Entity::from_bits(entity);
    let _ = cx.game_mut().send_message(
        entity,
        ChatMessage::new(ChatKind::System, unsafe { &*message }.clone()),
    );
    Ok(())
}

#[host_function]
pub fn entity_send_title(
    cx: &PluginContext,
    entity: u64,
    title_ptr: PluginPtr<u8>,
    title_len: u32,
) -> anyhow::Result<()> {
    let title = cx.read_bincode(title_ptr, title_len)?;
    let entity = Entity::from_bits(entity);
    cx.game_mut().send_title(entity, title);
    Ok(())
}
