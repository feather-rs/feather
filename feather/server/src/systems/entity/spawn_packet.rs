use ahash::AHashSet;
use anyhow::Context;
use base::Position;
use common::{
    events::{ChunkCrossEvent, ViewUpdateEvent},
    Game,
};
use ecs::{SysResult, SystemExecutor};
use quill_common::events::{EntityCreateEvent, EntityRemoveEvent};

use crate::{entities::SpawnPacketSender, ClientId, NetworkId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(update_visible_entities)
        .add_system(send_entities_when_created)
        .add_system(unload_entities_when_removed)
        .add_system(update_entities_on_chunk_cross);
}

/// System to spawn entities on clients when they become visible,
/// and despawn entities when they become invisible, based on the client's view.
pub fn update_visible_entities(game: &mut Game, server: &mut Server) -> SysResult {
    for (player, (event, &client_id)) in game.ecs.query::<(&ViewUpdateEvent, &ClientId)>().iter() {
        let client = match server.clients.get(client_id) {
            Some(client) => client,
            None => continue,
        };

        // Send newly visible entities
        for &new_chunk in &event.new_chunks {
            for &entity_id in game.chunk_entities.entities_in_chunk(new_chunk) {
                if entity_id != player {
                    let entity_ref = game.ecs.entity(entity_id)?;
                    if let Ok(spawn_packet) = entity_ref.get::<SpawnPacketSender>() {
                        spawn_packet
                            .send(&entity_ref, client)
                            .context("failed to send spawn packet")?;
                    }
                }
            }
        }

        // Unload entities no longer visible
        for &old_chunk in &event.old_chunks {
            for &entity_id in game.chunk_entities.entities_in_chunk(old_chunk) {
                if entity_id != player {
                    if let Ok(network_id) = game.ecs.get::<NetworkId>(entity_id) {
                        client.unload_entity(*network_id);
                    }
                }
            }
        }
    }

    Ok(())
}

/// System to send an entity to clients when it is created.
fn send_entities_when_created(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (_event, &position, spawn_packet)) in game
        .ecs
        .query::<(&EntityCreateEvent, &Position, &SpawnPacketSender)>()
        .iter()
    {
        let entity_ref = game.ecs.entity(entity)?;
        server.broadcast_nearby_with(position, |client| {
            spawn_packet
                .send(&entity_ref, client)
                .expect("failed to create spawn packet")
        });
    }

    Ok(())
}

/// System to unload an entity on clients when it is removed.
fn unload_entities_when_removed(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (_event, &position, &network_id)) in game
        .ecs
        .query::<(&EntityRemoveEvent, &Position, &NetworkId)>()
        .iter()
    {
        server.broadcast_nearby_with(position, |client| client.unload_entity(network_id));
    }

    Ok(())
}

/// System to send/unsend entities on clients when the entity changes chunks.
fn update_entities_on_chunk_cross(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (event, spawn_packet, &network_id)) in game
        .ecs
        .query::<(&ChunkCrossEvent, &SpawnPacketSender, &NetworkId)>()
        .iter()
    {
        let old_clients: AHashSet<_> = server
            .chunk_subscriptions
            .subscriptions_for(event.old_chunk)
            .iter()
            .copied()
            .collect();
        let new_clients: AHashSet<_> = server
            .chunk_subscriptions
            .subscriptions_for(event.new_chunk)
            .iter()
            .copied()
            .collect();

        for left_client in old_clients.difference(&new_clients) {
            if let Some(client) = server.clients.get(*left_client) {
                client.unload_entity(network_id);
            }
        }

        let entity_ref = game.ecs.entity(entity)?;
        for send_client in new_clients.difference(&old_clients) {
            if let Some(client) = server.clients.get(*send_client) {
                spawn_packet.send(&entity_ref, client)?;
            }
        }
    }

    Ok(())
}
