//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`View`].

use ahash::{AHashMap, AHashSet};
use base::ChunkPosition;
use common::{
    events::{ChunkLoadEvent, ViewUpdateEvent},
    view::View,
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
    for (player, (&client_id, event)) in game.ecs.query::<(&ClientId, &ViewUpdateEvent)>().iter() {
        let client = server.clients.get(client_id).unwrap();
        client.update_own_chunk(event.new_view.center());
        update_chunks(
            game,
            player,
            client,
            event.old_view,
            event.new_view,
            &mut server.waiting_chunks,
        )?;
    }
    Ok(())
}

fn update_chunks(
    game: &Game,
    player: Entity,
    client: &Client,
    old_view: View,
    new_view: View,
    waiting_chunks: &mut WaitingChunks,
) -> SysResult {
    let old_chunks: AHashSet<ChunkPosition> = old_view.iter().collect();
    let new_chunks: AHashSet<ChunkPosition> = new_view.iter().collect();

    // Send chunks that are in the new view but not the old view.
    let mut chunks_to_send = Vec::new();
    chunks_to_send.extend(new_chunks.difference(&old_chunks));
    chunks_to_send.sort_unstable();

    // Send the chunks closest to the player first.
    chunks_to_send.sort_unstable_by_key(|chunk: &ChunkPosition| {
        chunk.manhattan_distance_to(new_view.center()).abs()
    });

    for pos in chunks_to_send {
        if let Some(chunk) = game.world.chunk_map().chunk_handle_at(pos) {
            client.send_chunk(&chunk);
        } else {
            waiting_chunks.insert(player, pos);
        }
    }

    // Unsend the chunks that are in the old view but not the new view.
    for &pos in old_chunks.difference(&new_chunks) {
        client.unload_chunk(pos);
    }

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
                let client = server.clients.get(*client_id).unwrap();
                client.send_chunk(&event.chunk);
            }
        }
    }
    Ok(())
}
