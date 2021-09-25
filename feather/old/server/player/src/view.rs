//! Handling of a player's "view."
//!
//! This module includes systems and components
//! which handle sending new data as
//! a player moves through the world.
//!
//! When a player crosses a chunk boundary, its
//! view has changed: some chunks are no longer visible,
//! while others now are. To account for this, we
//! must send the new chunks, unload the old
//! chunks on the client, send new entities, and
//! delete old ones.
//!
//! This is handled as follows:
//! * A system queries all position components which have changed
//! and adds a `CrossedChunk` component to these entities.
//! * Other systems query for added `CrossedChunk` components
//! and perform updates on these players' views.

use crate::Player;
use ahash::AHashMap;
use feather_core::chunk::Chunk;
use feather_core::network::packets::{ChunkData, DestroyEntities, UnloadChunk};
use feather_core::util::{ChunkPosition, Position};
use feather_server_types::{
    BumpVec, ChunkCrossEvent, ChunkLoadEvent, ChunkSendEvent, EntityClientRemoveEvent,
    EntitySendEvent, Game, HoldChunkRequest, LoadChunkRequest, Network, NetworkId, PlayerJoinEvent,
    PreviousPosition, ReleaseChunkRequest, SpawnPacketCreator,
};
use fecs::{Entity, IntoQuery, Read, World};
use itertools::Either;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::iter;
use std::ops::Add;
use std::sync::Arc;

/// System which polls for updated positions and
/// calls `Game::on_chunk_cross()` accordingly.
#[fecs::system]
pub fn check_crossed_chunks(world: &mut World, game: &mut Game) {
    let mut crossed = BumpVec::new_in(game.bump());
    for (entity, (pos, prev_pos)) in
        <(Read<Position>, Read<PreviousPosition>)>::query().iter_entities(world.inner())
    {
        if let Some(prev_pos) = prev_pos.0 {
            if pos.chunk() != prev_pos.chunk() {
                crossed.push((entity, pos.chunk(), prev_pos.chunk()));
            }
        }
    }

    for (entity, new, old) in crossed {
        game.handle(
            world,
            ChunkCrossEvent {
                entity,
                old: Some(old),
                new,
            },
        );
    }
}

/// Triggers a chunk cross when a new player joins.
#[fecs::event_handler]
pub fn on_player_join_trigger_chunk_cross(
    event: &PlayerJoinEvent,
    game: &mut Game,
    world: &mut World,
) {
    let chunk = world.get::<Position>(event.player).chunk();
    game.handle(
        world,
        ChunkCrossEvent {
            old: None,
            new: chunk,
            entity: event.player,
        },
    );
}

/// System which sends new chunks and unloads old chunks on the client
/// when the view is updated.
#[fecs::event_handler]
pub fn on_chunk_cross_update_chunks(
    event: &ChunkCrossEvent,
    game: &mut Game,
    #[default] chunks_to_send: &mut ChunksToSend,
    world: &mut World,
) {
    if world.try_get::<Player>(event.entity).is_none() {
        return;
    }

    // The client likes it if we send closer chunks first,
    // so we'll sort by the Manhattan distance to the player.
    let mut pending_send = BumpVec::new_in(game.bump());
    pending_send.extend(find_new_chunks(
        event.old,
        event.new,
        game.config.server.view_distance,
    ));
    pending_send.sort_unstable_by_key(|chunk| chunk.manhattan_distance_to(event.new));

    for chunk in pending_send {
        send_chunk_to_player(game, world, chunks_to_send, event.entity, chunk);
    }

    for chunk in find_old_chunks(event.old, event.new, game.config.server.view_distance) {
        unload_chunk_for_player(game, world, chunk, event.entity);
    }
}

/// System which sends new entities and removes old entities
/// when a player crosses into a new view.
#[fecs::event_handler]
pub fn on_chunk_cross_update_entities(event: &ChunkCrossEvent, game: &mut Game, world: &mut World) {
    let network = match world.try_get::<Network>(event.entity) {
        Some(net) => net,
        None => return, // not a player
    };

    // Send newly visible entities.
    let mut sends_to_trigger = vec![];
    for other in find_new_chunks(event.old, event.new, game.config.server.view_distance)
        .flat_map(|chunk| game.chunk_entities.entities_in_chunk(chunk))
        .filter(|other| **other != event.entity)
    // don't send player to themselves!
    {
        if let Some(creator) = world.try_get::<SpawnPacketCreator>(*other) {
            let accessor = world
                .entity(*other)
                .expect("entity in chunk entities does not exist");
            let packet = creator.get(&accessor);

            network.send_boxed(packet);
            sends_to_trigger.push((*other, event.entity));
        }

        // if this `other` is a player, also send `entity` to other
        if let Some(network) = world.try_get::<Network>(*other) {
            if let Some(creator) = world.try_get::<SpawnPacketCreator>(event.entity) {
                let accessor = world.entity(event.entity).expect("entity does not exist");
                let packet = creator.get(&accessor);

                network.send_boxed(packet);
                sends_to_trigger.push((event.entity, *other));
            }
        }
    }

    // Tell the client to despawn entities which are no longer visible.
    let mut to_client_remove_trigger = vec![];
    to_client_remove_trigger.extend(
        find_old_chunks(event.old, event.new, game.config.server.view_distance)
            .flat_map(|chunk| game.chunk_entities.entities_in_chunk(chunk))
            .map(|other| (*other, event.entity)),
    );

    // Despawn this entity on other visible clients.
    find_old_chunks(event.old, event.new, game.config.server.view_distance)
        .flat_map(|chunk| game.chunk_entities.entities_in_chunk(chunk))
        .filter_map(|entity| world.try_get::<Network>(*entity).map(|net| (*entity, net)))
        .for_each(|(other, network)| {
            let packet = DestroyEntities {
                entity_ids: vec![world.get::<NetworkId>(event.entity).0],
            };
            network.send(packet);
            to_client_remove_trigger.push((event.entity, other));
        });

    let to_destroy = to_client_remove_trigger
        .iter()
        .filter_map(|(other, _)| world.try_get::<NetworkId>(*other).map(|id| id.0))
        .collect::<Vec<_>>();

    if !to_destroy.is_empty() {
        let packet = DestroyEntities {
            entity_ids: to_destroy,
        };
        network.send(packet);
    }

    drop(network);

    // Trigger on_entity_send
    for (entity, client) in sends_to_trigger {
        game.handle(world, EntitySendEvent { entity, client });
    }

    // Trigger on_entity_client_remove
    for (other, to) in to_client_remove_trigger {
        game.handle(
            world,
            EntityClientRemoveEvent {
                entity: other,
                client: to,
            },
        );
    }
}

/// Returns new chunks visible from a new chunk position.
fn find_new_chunks(
    old: Option<ChunkPosition>,
    new: ChunkPosition,
    view_distance: u8,
) -> impl Iterator<Item = ChunkPosition> {
    let within_view_distance = chunks_within_view_distance(new, view_distance);
    if let Some(old) = old {
        Either::Left(within_view_distance.filter(move |chunk| {
            (chunk.x - old.x).abs() >= view_distance as i32
                || (chunk.z - old.z).abs() >= view_distance as i32
        }))
    } else {
        Either::Right(within_view_distance)
    }
}

/// Returns chunks which are no longer visible from a new chunk position.
fn find_old_chunks(
    old: Option<ChunkPosition>,
    new: ChunkPosition,
    view_distance: u8,
) -> impl Iterator<Item = ChunkPosition> {
    if let Some(old) = old {
        Either::Left(
            chunks_within_view_distance(old, view_distance).filter(move |chunk| {
                (chunk.x - new.x).abs() >= view_distance as i32
                    || (chunk.z - new.z).abs() >= view_distance as i32
            }),
        )
    } else {
        Either::Right(iter::empty())
    }
}

/// Finds all chunks within the view distance of a given chunk.
fn chunks_within_view_distance(
    chunk: ChunkPosition,
    view_distance: u8,
) -> impl Iterator<Item = ChunkPosition> {
    let view_distance = i32::from(view_distance);

    (-view_distance..=view_distance).flat_map(move |x| {
        (-view_distance..=view_distance).map(move |z| chunk.add(ChunkPosition::new(x, z)))
    })
}

/// Resource containing a mapping from chunks -> sets of players indicating
/// which chunks are pending to send to a given player.
#[derive(Default)]
pub struct ChunksToSend(AHashMap<ChunkPosition, SmallVec<[Entity; 2]>>);

/// Asynchronously sends a chunk to a player.
fn send_chunk_to_player(
    game: &mut Game,
    world: &mut World,
    chunks_to_send: &mut ChunksToSend,
    player: Entity,
    chunk_pos: ChunkPosition,
) {
    if !world.is_alive(player) {
        return;
    }

    // Ensure that the chunk isn't unloaded while the player has it loaded.
    game.handle(
        world,
        HoldChunkRequest {
            player,
            chunk: chunk_pos,
        },
    );

    // If the chunk is already loaded, send it. Otherwise, we need to
    // queue it for loading.
    if let Some(chunk) = game.chunk_map.chunk_handle_at(chunk_pos) {
        world.get::<Network>(player).send(create_chunk_data(chunk));
        game.handle(
            world,
            ChunkSendEvent {
                player,
                chunk: chunk_pos,
            },
        );
    } else {
        let contains = chunks_to_send.0.contains_key(&chunk_pos);

        let vec = match chunks_to_send.0.get_mut(&chunk_pos) {
            Some(vec) => vec,
            None => {
                chunks_to_send.0.insert(chunk_pos, SmallVec::new());
                chunks_to_send.0.get_mut(&chunk_pos).unwrap()
            }
        };
        vec.push(player);

        if !contains {
            // Queue chunk for loading if it isn't already.
            game.handle(world, LoadChunkRequest { chunk: chunk_pos });
        }
    }
}

/// Unloads a chunk on a client.
fn unload_chunk_for_player(
    game: &mut Game,
    world: &mut World,
    chunk: ChunkPosition,
    player: Entity,
) {
    // Release hold on chunk so it can be unloaded on the server
    game.handle(world, ReleaseChunkRequest { player, chunk });

    // Send Unload Chunk packet.
    world.get::<Network>(player).send(UnloadChunk {
        chunk_x: chunk.x,
        chunk_z: chunk.z,
    });
}

/// System which sends chunks to pending players when a chunk is loaded.
#[fecs::event_handler]
pub fn on_chunk_load_send_to_clients(
    event: &ChunkLoadEvent,
    game: &mut Game,
    world: &mut World,
    chunks_to_send: &mut ChunksToSend,
) {
    if let Some(players) = chunks_to_send.0.get(&event.chunk) {
        let chunk = game
            .chunk_map
            .chunk_handle_at(event.chunk)
            .expect("chunk not loaded, but load event was triggered");
        for player in players {
            if !world.is_alive(*player) {
                continue;
            }

            world
                .get::<Network>(*player)
                .send(create_chunk_data(Arc::clone(&chunk)));
            game.handle(
                world,
                ChunkSendEvent {
                    chunk: event.chunk,
                    player: *player,
                },
            );
        }
    }

    chunks_to_send.0.remove(&event.chunk);
}

/// Creates a chunk data packet for the given chunk.
fn create_chunk_data(chunk: ChunkHandle) -> ChunkData {
    ChunkData { chunk }
}
