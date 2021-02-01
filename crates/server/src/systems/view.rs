//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`View`].

use ahash::AHashSet;
use base::ChunkPosition;
use common::{events::ViewUpdateEvent, view::View, Game, Name};
use ecs::{EntityRef, SysResult, SystemExecutor};

use crate::{Client, ClientId, Server};

pub fn register(game: &mut Game, _systems: &mut SystemExecutor<Game>) {
    game.event_bus()
        .group::<Server>()
        .add_handler(update_player_view);
}

fn update_player_view(game: &mut Game, server: &mut Server, event: &ViewUpdateEvent) -> SysResult {
    let player = game.ecs.entity(event.player)?;
    let client = server.clients.get(*player.get::<ClientId>()?).unwrap();

    client.update_own_chunk(event.new_view.center());
    update_chunks(game, player, client, event.old_view, event.new_view)
}

fn update_chunks(
    game: &Game,
    player: EntityRef,
    client: &Client,
    old_view: View,
    new_view: View,
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
    log::debug!("Sent {} chunks to {}", sent, &**player.get::<Name>()?);

    // Unsend the chunks that are in the old view but not the new view.
    let mut unsent = 0;
    for &pos in old_chunks.difference(&new_chunks) {
        client.unload_chunk(pos);
        unsent += 1;
    }
    log::debug!(
        "Unloaded {} chunks for {}",
        unsent,
        &**player.get::<Name>()?
    );

    Ok(())
}
