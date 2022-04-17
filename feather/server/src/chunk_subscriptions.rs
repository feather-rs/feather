use ahash::AHashMap;
use common::{events::ViewUpdateEvent, view::View, Game};
use libcraft::ChunkPosition;
use quill::components::EntityWorld;
use quill::events::EntityRemoveEvent;
use quill::WorldId;
use utils::vec_remove_item;
use vane::{SysResult, SystemExecutor};

use crate::{ClientId, Server};

#[derive(Eq, PartialEq, Hash)]
pub struct ChunkPositionWithWorld {
    pub world: WorldId,
    pub chunk: ChunkPosition,
}

impl ChunkPositionWithWorld {
    pub fn new(world: WorldId, chunk: ChunkPosition) -> Self {
        Self { world, chunk }
    }
}

/// Data structure to query which clients should
/// receive updates from a given chunk, fast.
#[derive(Default)]
pub struct ChunkSubscriptions {
    chunks: AHashMap<ChunkPositionWithWorld, Vec<ClientId>>,
}

impl ChunkSubscriptions {
    pub fn subscriptions_for(&self, chunk: ChunkPositionWithWorld) -> &[ClientId] {
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
    for (_, (event, client_id)) in game.ecs.query::<(&ViewUpdateEvent, &ClientId)>().iter() {
        for new_chunk in event.new_view.difference(&event.old_view) {
            server
                .chunk_subscriptions
                .chunks
                .entry(ChunkPositionWithWorld::new(
                    event.new_view.world(),
                    new_chunk,
                ))
                .or_default()
                .push(*client_id);
        }
        for old_chunk in event.old_view.difference(&event.new_view) {
            remove_subscription(
                server,
                ChunkPositionWithWorld::new(event.old_view.world(), old_chunk),
                *client_id,
            );
        }
    }

    // Update players that have left
    for (_, (_event, client_id, view, world)) in game
        .ecs
        .query::<(&EntityRemoveEvent, &ClientId, &View, &EntityWorld)>()
        .iter()
    {
        for chunk in view.iter() {
            remove_subscription(
                server,
                ChunkPositionWithWorld::new(world.0, chunk),
                *client_id,
            );
        }
    }

    Ok(())
}

fn remove_subscription(server: &mut Server, chunk: ChunkPositionWithWorld, client_id: ClientId) {
    if let Some(vec) = server.chunk_subscriptions.chunks.get_mut(&chunk) {
        vec_remove_item(vec, &client_id);

        if vec.is_empty() {
            server.chunk_subscriptions.chunks.remove(&chunk);
        }
    }
}
