//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`common::view::View`].

use ahash::AHashMap;
use base::{ChunkPosition, Position};
use common::{
    events::{ChunkLoadEvent, ViewUpdateEvent},
    Game,
};
use ecs::{Entity, SysResult, SystemExecutor};

use crate::{Client, ClientId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(send_new_chunks)
        .add_system(send_loaded_chunks);
}

/// Stores the players waiting on chunks that are currently being loaded.
#[derive(Default)]
pub struct WaitingChunks(AHashMap<ChunkPosition, Vec<Entity>>);

impl WaitingChunks {
    pub fn drain_players_waiting_for(&mut self, chunk: ChunkPosition) -> Vec<Entity> {
        self.0.remove(&chunk).unwrap_or_default()
    }

    pub fn insert(&mut self, player: Entity, chunk: ChunkPosition) {
        self.0.entry(chunk).or_default().push(player);
    }
}

fn send_new_chunks(game: &mut Game, server: &mut Server) -> SysResult {
    for (player, (&client_id, event, &position)) in game
        .ecs
        .query::<(&ClientId, &ViewUpdateEvent, &Position)>()
        .iter()
    {
        // As ecs removes the client one tick after it gets removed here, it can
        // happen that a client is still listed in the ecs but actually removed here so
        // we need to check if the client is actually still there.
        if let Some(client) = server.clients.get(client_id) {
            client.update_own_chunk(event.new_view.center());
            update_chunks(
                game,
                player,
                client,
                event,
                position,
                &mut server.waiting_chunks,
            )?;
        }
    }
    Ok(())
}

fn update_chunks(
    game: &Game,
    player: Entity,
    client: &Client,
    event: &ViewUpdateEvent,
    position: Position,
    waiting_chunks: &mut WaitingChunks,
) -> SysResult {
    // Send chunks that are in the new view but not the old view.
    for &pos in &event.new_chunks {
        if let Some(chunk) = game.world.chunk_map().chunk_handle_at(pos) {
            client.send_chunk(&chunk);
        } else {
            waiting_chunks.insert(player, pos);
        }
    }

    // Unsend the chunks that are in the old view but not the new view.
    for &pos in &event.old_chunks {
        client.unload_chunk(pos);
    }

    spawn_client_if_needed(client, position);

    Ok(())
}

/// Sends newly loaded chunks to players currently
/// waiting for those chunks to load.
fn send_loaded_chunks(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, event) in game.ecs.query::<&ChunkLoadEvent>().iter() {
        for player in server
            .waiting_chunks
            .drain_players_waiting_for(event.position)
        {
            if let Ok(client_id) = game.ecs.get::<ClientId>(player) {
                if let Some(client) = server.clients.get(*client_id) {
                    client.send_chunk(&event.chunk);
                    spawn_client_if_needed(client, *game.ecs.get::<Position>(player)?);
                }
            }
        }
    }
    Ok(())
}

fn spawn_client_if_needed(client: &Client, pos: Position) {
    if !client.knows_own_position() && client.known_chunks() >= 9 * 9 {
        log::debug!("Sent all chunks to {}; now spawning", client.username());
        client.update_own_position(pos);
    }
}
