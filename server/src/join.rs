//! After chunks are sent to a client, we complete the login sequence
//! by sending Spawn Position, Player Position and Look, and inventory,
//! among others. This is handled by the event handler `join`.

use crate::entity::EntityId;
use crate::game::Game;
use crate::network::Network;
use feather_core::network::packet::implementation::{
    JoinGame, PlayerPositionAndLookClientbound, SpawnPosition,
};
use feather_core::{BlockPosition, ChunkPosition, Difficulty, Dimension, Gamemode, Position};
use fecs::{Entity, World};

/// Component indicating that a player has completed the join sequence.
#[derive(Default, Debug)]
pub struct Joined;

/// System to run the join sequence. To determine when a player is ready to join,
/// we wait for the chunk that the player is in to be sent—this appears to work
/// well with the client.
pub fn on_chunk_send_join_player(
    game: &Game,
    world: &mut World,
    chunk: ChunkPosition,
    player: Entity,
) {
    if world.try_get::<Joined>(player).is_some() {
        return; // already joined
    }

    let pos = {
        let pos = world.get::<Position>(player);

        if pos.chunk() != chunk {
            return;
        }

        *pos
    };

    // Run the join sequence.
    world.add(player, Joined).unwrap();

    let network = world.get::<Network>(player);

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

pub fn on_player_join_send_join_game(game: &Game, world: &World, player: Entity) {
    let network = world.get::<Network>(player);
    let id = world.get::<EntityId>(player);

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
