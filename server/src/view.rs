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

use crate::entity::{EntityId, PreviousPosition, SpawnPacketCreator};
use crate::game::Game;
use crate::network::Network;
use crate::player::Player;
use crate::{chunk_logic, BumpVec};
use ahash::AHashMap;
use feather_core::network::packet::implementation::{ChunkData, DestroyEntities, UnloadChunk};
use feather_core::{Chunk, ChunkPosition, Position};
use fecs::{Entity, IntoQuery, Read, World};
use itertools::Either;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::iter;
use std::ops::Add;
use std::sync::Arc;

/// System which polls for updated positions and
/// calls `Game::on_chunk_cross()` accordingly.
#[system]
pub fn check_crossed_chunks(world: &mut World, game: &mut Game) {
    let mut crossed = BumpVec::new_in(game.bump());
    for (entity, (pos, prev_pos)) in
        <(Read<Position>, Read<PreviousPosition>)>::query().iter_entities(world.inner())
    {
        if pos.chunk() != prev_pos.0.chunk() {
            crossed.push((entity, pos.chunk(), prev_pos.0.chunk()));
        }
    }

    for (entity, new, old) in crossed {
        game.on_chunk_cross(world, entity, Some(old), new);
    }
}

/// Triggers a chunk cross when a new player joins.
pub fn on_player_join_trigger_chunk_cross(game: &mut Game, world: &mut World, player: Entity) {
    let chunk = world.get::<Position>(player).chunk();
    game.on_chunk_cross(world, player, None, chunk);
}

/// System which sends new chunks and unloads old chunks on the client
/// when the view is updated.
pub fn on_chunk_cross_update_chunks(
    game: &mut Game,
    world: &mut World,
    entity: Entity,
    old: Option<ChunkPosition>,
    new: ChunkPosition,
) {
    if world.try_get::<Player>(entity).is_none() {
        return;
    }

    // The client likes it if we send closer chunks first,
    // so we'll sort on the Manhattan distance to the player.
    let mut chunks_to_send = BumpVec::new_in(game.bump());
    chunks_to_send.extend(find_new_chunks(old, new, game.config.server.view_distance));
    chunks_to_send.sort_unstable_by_key(|chunk| chunk.manhattan_distance_to(new));

    for chunk in chunks_to_send {
        send_chunk_to_player(game, world, entity, chunk);
    }

    for chunk in find_old_chunks(old, new, game.config.server.view_distance) {
        unload_chunk_for_player(game, world, chunk, entity);
    }
}

/// System which sends new entities and removes old entities
/// when a player crosses into a new view.
pub fn on_chunk_cross_update_entities(
    game: &mut Game,
    world: &mut World,
    entity: Entity,
    old: Option<ChunkPosition>,
    new: ChunkPosition,
) {
    let network = match world.try_get::<Network>(entity) {
        Some(net) => net,
        None => return, // not a player
    };

    // Send newly visible entities.
    let mut sends_to_trigger = BumpVec::new_in(game.bump());
    for other in find_new_chunks(old, new, game.config.server.view_distance)
        .flat_map(|chunk| game.chunk_entities.entities_in_chunk(chunk))
        .filter(|other| **other != entity)
    // don't send player to themselves!
    {
        if let Some(creator) = world.try_get::<SpawnPacketCreator>(*other) {
            let accessor = world
                .entity(*other)
                .expect("entity in chunk entities does not exist");
            let packet = creator.get(&accessor);

            network.send_boxed(packet);
            sends_to_trigger.push((*other, entity));
        }

        // if this `other` is a player, also send `entity` to other
        if let Some(network) = world.try_get::<Network>(*other) {
            if let Some(creator) = world.try_get::<SpawnPacketCreator>(entity) {
                let accessor = world.entity(entity).expect("entity does not exist");
                let packet = creator.get(&accessor);

                network.send_boxed(packet);
                sends_to_trigger.push((entity, *other));
            }
        }
    }

    // Tell the client to despawn entities which are no longer visible.
    let mut to_client_remove_trigger = BumpVec::new_in(game.bump());
    to_client_remove_trigger.extend(
        find_old_chunks(old, new, game.config.server.view_distance)
            .flat_map(|chunk| game.chunk_entities.entities_in_chunk(chunk))
            .map(|other| (*other, entity)),
    );

    // Despawn this entity on other visible entities.
    find_old_chunks(old, new, game.config.server.view_distance)
        .flat_map(|chunk| game.chunk_entities.entities_in_chunk(chunk))
        .filter_map(|entity| world.try_get::<Network>(*entity).map(|net| (*entity, net)))
        .for_each(|(other, network)| {
            let packet = DestroyEntities {
                entity_ids: vec![world.get::<EntityId>(entity).0],
            };
            network.send(packet);
            to_client_remove_trigger.push((entity, other));
        });

    let to_destroy = to_client_remove_trigger
        .iter()
        .map(|(other, _)| world.get::<EntityId>(*other).0)
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
        game.on_entity_send(world, entity, client);
    }

    // Trigger on_entity_client_remmove
    for (other, to) in to_client_remove_trigger {
        game.on_entity_client_remove(world, other, to);
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
                (chunk.x - new.x).abs() > view_distance as i32
                    || (chunk.z - new.z).abs() > view_distance as i32
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
    player: Entity,
    chunk_pos: ChunkPosition,
) {
    // Ensure that the chunk isn't unloaded while the player has it loaded.
    chunk_logic::hold_chunk(game, &mut *world.get_mut(player), chunk_pos, player);

    // If the chunk is already loaded, send it. Otherwise, we need to
    // queue it for loading.
    if let Some(chunk) = game.chunk_map.chunk_handle_at(chunk_pos) {
        world.get::<Network>(player).send(create_chunk_data(chunk));
        game.on_chunk_send(world, chunk_pos, player);
    } else {
        let contains = game.chunks_to_send.0.contains_key(&chunk_pos);

        let vec = match game.chunks_to_send.0.get_mut(&chunk_pos) {
            Some(vec) => vec,
            None => {
                game.chunks_to_send.0.insert(chunk_pos, smallvec![]);
                game.chunks_to_send.0.get_mut(&chunk_pos).unwrap()
            }
        };
        vec.push(player);

        if !contains {
            // Queue chunk for loading if it isn't already.
            chunk_logic::load_chunk(&game.chunk_worker_handle, chunk_pos);
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
    chunk_logic::release_chunk(game, world, chunk, player);

    // Send Unload Chunk packet.
    world.get::<Network>(player).send(UnloadChunk {
        chunk_x: chunk.x,
        chunk_z: chunk.z,
    });
}

/// System which sends chunks to pending players when a chunk is loaded.
pub fn on_chunk_load_send_to_clients(game: &mut Game, world: &mut World, chunk_pos: ChunkPosition) {
    if let Some(players) = game.chunks_to_send.0.get(&chunk_pos) {
        let chunk = game
            .chunk_map
            .chunk_handle_at(chunk_pos)
            .expect("chunk not loaded, but load event was triggered");
        for player in players {
            if !world.is_alive(*player) {
                continue;
            }

            world
                .get::<Network>(*player)
                .send(create_chunk_data(Arc::clone(&chunk)));
            game.on_chunk_send(world, chunk_pos, *player);
        }
    }

    game.chunks_to_send.0.remove(&chunk_pos);
}

/// Creates a chunk data packet for the given chunk.
fn create_chunk_data(chunk: Arc<RwLock<Chunk>>) -> ChunkData {
    ChunkData { chunk }
}
