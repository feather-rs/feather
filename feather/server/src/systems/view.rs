//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`common::view::View`].

use common::{events::ViewUpdateEvent, Game};
use quill::components::EntityPosition;
use vane::{SysResult, SystemExecutor};

use crate::{Client, ClientId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(send_new_chunks);
}

fn send_new_chunks(game: &mut Game, server: &mut Server) -> SysResult {
    for (_player, (client_id, event, _position)) in game
        .ecs
        .query::<(&ClientId, &ViewUpdateEvent, &EntityPosition)>()
        .iter()
    {
        // As ecs removes the client one tick after it gets removed here, it can
        // happen that a client is still listed in the ecs but actually removed here so
        // we need to check if the client is actually still there.
        if let Some(client) = server.clients.get_mut(*client_id) {
            client.update_own_chunk(event.new_view.center());
            update_chunks(client, &event)?;
        }
    }

    Ok(())
}

fn update_chunks(client: &mut Client, event: &ViewUpdateEvent) -> SysResult {
    // Send chunks that are in the new view but not the old view.
    for &pos in &event.new_chunks {
        client.queue_send_chunk(pos);
    }

    // Unsend the chunks that are in the old view but not the new view.
    for &pos in &event.old_chunks {
        client.unload_chunk(pos);
    }

    Ok(())
}
