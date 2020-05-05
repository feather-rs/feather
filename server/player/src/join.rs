//! Join logic for players.

use feather_core::network::packets::{JoinGame, PlayerPositionAndLookClientbound, SpawnPosition};
use feather_core::util::{BlockPosition, Difficulty, Dimension, Gamemode, Position};
use feather_server_network::{ListenerToServerMessage, NetworkIoManager, ServerToListenerMessage};
use feather_server_types::{
    BumpVec, ChunkSendEvent, Game, Network, NetworkId, PlayerJoinEvent, WorkerToServerMessage,
};
use fecs::{IntoQuery, Read, World};
use std::iter;

/// System which polls for player disconnects.
#[fecs::system]
pub fn poll_player_disconnect(game: &mut Game, world: &mut World) {
    // For each player with a Network component,
    // check their channel for disconnects.
    let mut to_despawn = BumpVec::new_in(game.bump());
    <Read<Network>>::query()
        .iter_entities(world.inner())
        .for_each(|(entity, network)| {
            while let Ok(msg) = network.rx.lock().try_recv() {
                match msg {
                    WorkerToServerMessage::NotifyDisconnected { reason } => {
                        to_despawn.push((entity, reason));
                    }
                }
            }
        });

    to_despawn.into_iter().for_each(|(player, reason)| {
        game.disconnect(player, world, reason);
    });
}

/// System which polls for new clients from the listener task.
#[fecs::system]
pub fn poll_new_clients(game: &mut Game, world: &mut World, io_handle: &mut NetworkIoManager) {
    while let Ok(msg) = io_handle.rx.lock().try_recv() {
        match msg {
            ListenerToServerMessage::NewClient(info) => {
                crate::create(game, world, info);
            }
            ListenerToServerMessage::RequestEntity => {
                let entity = world.spawn(iter::once(()))[0];
                let _ = io_handle.tx.send(ServerToListenerMessage::Entity(entity));
            }
            ListenerToServerMessage::DeleteEntity(entity) => {
                // no need to use `Game::despawn` here as
                // the entity hasn't actually "existed" yet;
                // it has no components
                world.despawn(entity);
            }
        }
    }
}

// After chunks are sent to a client, we complete the login sequence
// by sending Spawn Position, Player Position and Look, and inventory,
// among others. This is handled by the event handler below;

/// Component indicating that a player has completed the join sequence.
#[derive(Default, Debug)]
pub struct Joined;

/// System to run the join sequence. To determine when a player is ready to join,
/// we wait for the chunk that the player is in to be sent—this appears to work
/// well with the client.
#[fecs::event_handler]
pub fn on_chunk_send_join_player(event: &ChunkSendEvent, game: &Game, world: &mut World) {
    if world.try_get::<Joined>(event.player).is_some() {
        return; // already joined
    }

    let pos = {
        let pos = world.get::<Position>(event.player);

        if pos.chunk() != event.chunk {
            return;
        }

        *pos
    };

    // Run the join sequence.
    world.add(event.player, Joined).unwrap();

    let network = world.get::<Network>(event.player);

    let packet = SpawnPosition {
        location: BlockPosition::new(game.level.spawn_x, game.level.spawn_y, game.level.spawn_z),
    };
    network.send(packet);

    let packet = PlayerPositionAndLookClientbound {
        x: pos.x,
        y: pos.y,
        z: pos.z,
        yaw: pos.yaw,
        pitch: pos.pitch,
        flags: 0,
        teleport_id: 0,
    };
    network.send(packet);
}

#[fecs::event_handler]
pub fn on_player_join_send_join_game(event: &PlayerJoinEvent, game: &Game, world: &mut World) {
    let network = world.get::<Network>(event.player);
    let id = world.get::<NetworkId>(event.player);

    // TODO
    let packet = JoinGame {
        entity_id: id.0,
        gamemode: Gamemode::Creative.id(),
        dimension: Dimension::Overwold.id(),
        difficulty: Difficulty::Medium.id(),
        max_players: game.config.server.max_players as u8,
        level_type: game.level.generator_name.clone(),
        reduced_debug_info: false,
    };
    network.send(packet);
}
