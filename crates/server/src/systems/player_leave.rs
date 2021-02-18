use base::Text;
use common::{chat::ChatKind, Game};
use ecs::{SysResult, SystemExecutor};
use quill_common::components::Name;

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(remove_disconnected_clients);
}

fn remove_disconnected_clients(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities_to_remove = Vec::new();
    for (player, (&client_id, name)) in game.ecs.query::<(&ClientId, &Name)>().iter() {
        let client = server.clients.get(client_id).unwrap();
        if client.is_disconnected() {
            server.remove_client(client_id);
            entities_to_remove.push(player);
            broadcast_player_leave(game, name);
        }
    }

    for player in entities_to_remove {
        game.remove_entity(player)?;
    }

    Ok(())
}

fn broadcast_player_leave(game: &Game, username: &Name) {
    let message = Text::translate_with("multiplayer.player.left", vec![username.to_string()]);
    game.broadcast_chat(ChatKind::System, message);
}
