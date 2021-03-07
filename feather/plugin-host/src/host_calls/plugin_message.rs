use feather_common::events::PluginMessageEvent;
use feather_ecs::Entity;
use feather_plugin_host_macros::host_function;

use crate::context::{PluginContext, PluginPtr};

#[host_function]
pub fn plugin_message_send(
    cx: &PluginContext,
    entity: u64,
    channel_ptr: PluginPtr<u8>,
    channel_len: u32,
    data_ptr: PluginPtr<u8>,
    data_len: u32,
) -> anyhow::Result<()> {
    let channel = cx.read_string(channel_ptr, channel_len)?;
    let data = cx.read_bytes(data_ptr, data_len)?;

    let entity = Entity::from_bits(entity);
    let event = PluginMessageEvent { channel, data };
    cx.game_mut().ecs.insert_entity_event(entity, event)?;

    Ok(())
}
