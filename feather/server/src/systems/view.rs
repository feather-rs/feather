//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`common::view::View`].

use ahash::AHashMap;
use common::{events::ViewUpdateEvent, view::View, Game};
use libcraft::{ChunkPosition, Position};
use quill::{
    components::{EntityPosition, EntityWorld},
    events::ChunkLoadEvent,
};
use vane::{Entity, SysResult, SystemExecutor};

use crate::{Client, ClientId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(send_new_chunks);
}

fn send_new_chunks(game: &mut Game, server: &mut Server) -> SysResult {
    for (player, (client_id, event, position)) in game
        .ecs
        .query::<(&ClientId, &ViewUpdateEvent, &EntityPosition)>()
        .iter()
    {
        // As ecs removes the client one tick after it gets removed here, it can
        // happen that a client is still listed in the ecs but actually removed here so
        // we need to check if the client is actually still there.
        if let Some(client) = server.clients.get_mut(*client_id) {
            client.update_own_chunk(event.new_view.center());
            update_chunks(game, player, client, &event, position.0)?;
        }
    }

    Ok(())
}

fn update_chunks(
    game: &Game,
    player: Entity,
    client: &mut Client,
    event: &ViewUpdateEvent,
    position: Position,
) -> SysResult {
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

