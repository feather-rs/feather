//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`View`].

use ahash::AHashSet;
use base::{ChunkPosition, Position};
use common::{events::ViewUpdateEvent, view::View, Game, Name};
use ecs::{SysResult, SystemExecutor};

use crate::{Client, ClientId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(update_player_view);
}

fn update_player_view(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&client_id, event, name, position)) in game
        .ecs
        .query::<(&ClientId, &ViewUpdateEvent, &Name, &Position)>()
        .iter()
    {
        let client = server.clients.get(client_id).unwrap();
        client.update_own_chunk(event.new_view.center());
        update_chunks(game, client, event.old_view, event.new_view, name, position)?;
    }
    Ok(())
}

fn update_chunks(
    game: &Game,
    client: &Client,
    old_view: View,
    new_view: View,
    name: &Name,
    position: &Position,
) -> SysResult {
    let old_chunks: AHashSet<ChunkPosition> = old_view.iter().collect();
    let new_chunks: AHashSet<ChunkPosition> = new_view.iter().collect();

    // Send chunks that are in the new view but not the old view.
    let mut chunks_to_send = Vec::new();
    chunks_to_send.extend(new_chunks.difference(&old_chunks));

    // Send the chunks closest to the player first.
    chunks_to_send.sort_unstable_by_key(|chunk: &ChunkPosition| {
        chunk.manhattan_distance_to(new_view.center()).abs()
    });

    let mut sent = 0;
    for pos in chunks_to_send {
        if let Some(chunk) = game.world.chunk_handle_at(pos) {
            client.send_chunk(&chunk);
            sent += 1;
        }
    }
    log::debug!("Sent {} chunks to {}", sent, name);

    // Unsend the chunks that are in the old view but not the new view.
    let mut unsent = 0;
    for &pos in old_chunks.difference(&new_chunks) {
        client.unload_chunk(pos);
        unsent += 1;
    }
    log::debug!("Unloaded {} chunks for {}", unsent, name);

    if old_view.is_empty() {
        // Player just joined - send them their position now
        // that chunks have been sent
        client.update_own_position(*position);
    }

    Ok(())
}
