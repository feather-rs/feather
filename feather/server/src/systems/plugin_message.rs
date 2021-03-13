use crate::{ClientId, Server};
use common::{events::PluginMessageEvent, Game};
use ecs::{SysResult, SystemExecutor};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(send_plugin_message_packets);
}

fn send_plugin_message_packets(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&client_id, event)) in game.ecs.query::<(&ClientId, &PluginMessageEvent)>().iter() {
        if let Some(client) = server.clients.get(client_id) {
            client.send_plugin_message(event.channel.clone(), event.data.clone());
        }
    }

    Ok(())
}
