use ahash::AHashMap;
use base::ChunkPosition;
use common::{events::ViewUpdateEvent, view::View, Game};
use ecs::{SysResult, SystemExecutor};
use quill_common::events::EntityRemoveEvent;
use utils::vec_remove_item;

use crate::{ClientId, Server};

/// Data structure to query which clients should
/// receive updates from a given chunk, fast.
#[derive(Default)]
pub struct ChunkSubscriptions {
    chunks: AHashMap<ChunkPosition, Vec<ClientId>>,
}

impl ChunkSubscriptions {
    pub fn subscriptions_for(&self, chunk: ChunkPosition) -> &[ClientId] {
        self.chunks
            .get(&chunk)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(update_chunk_subscriptions);
}

fn update_chunk_subscriptions(game: &mut Game, server: &mut Server) -> SysResult {
    // Update players whose views have changed
    for (_, (event, &client_id)) in game.ecs.query::<(&ViewUpdateEvent, &ClientId)>().iter() {
        for new_chunk in event.new_view.difference(event.old_view) {
            server
                .chunk_subscriptions
                .chunks
                .entry(new_chunk)
                .or_default()
                .push(client_id);
        }
        for old_chunk in event.old_view.difference(event.new_view) {
            remove_subscription(server, old_chunk, client_id);
        }
    }

    // Update players that have left
    for (_, (_event, &client_id, &view)) in game
        .ecs
        .query::<(&EntityRemoveEvent, &ClientId, &View)>()
        .iter()
    {
        for chunk in view.iter() {
            remove_subscription(server, chunk, client_id);
        }
    }

    Ok(())
}

fn remove_subscription(server: &mut Server, chunk: ChunkPosition, client_id: ClientId) {
    if let Some(vec) = server.chunk_subscriptions.chunks.get_mut(&chunk) {
        vec_remove_item(vec, &client_id);

        if vec.is_empty() {
            server.chunk_subscriptions.chunks.remove(&chunk);
        }
    }
}
