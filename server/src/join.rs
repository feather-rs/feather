//! After chunks are sent to a client, we complete the login sequence
//! by sending Spawn Position, Player Position and Look, and inventory,
//! among others. This is handled by the event handler `join`.

use crate::entity::EntityId;
use crate::network::Network;
use crate::player::PlayerJoinEvent;
use crate::state::State;
use crate::view::ChunkSendEvent;
use feather_core::network::packet::implementation::{
    JoinGame, PlayerPositionAndLookClientbound, SpawnPosition,
};
use feather_core::{BlockPosition, Gamemode, Position};
use legion::query::{Read, Write};
use parking_lot::RwLock;
use rayon::prelude::*;
use tonks::{PreparedWorld, Query};

/// Component indicating whether a player has completed the join sequence.
#[derive(Default, Debug)]
pub struct Joined(pub bool);

/// System to run the join sequence. To determine when a player is ready to join,
/// we wait for the chunk that the player is in to be sent—this appears to work
/// well with the client.
#[event_handler]
fn join(
    events: &[ChunkSendEvent],
    _query: &mut Query<(Write<Joined>, Read<Position>, Read<Network>)>,
    world: &mut PreparedWorld,
    state: &State,
) {
    let world = RwLock::new(world);
    events.par_iter().for_each(|event| {
        let pos = {
            let world = world.read();

            let pos = world.get_component::<Position>(event.player).unwrap();
            let joined = world.get_component::<Joined>(event.player).unwrap();

            if pos.chunk() != event.chunk || joined.0 {
                return;
            }

            *pos
        };

        // Run the join sequence. TODO: inventory.
        world
            .write()
            .get_component_mut::<Joined>(event.player)
            .unwrap()
            .0 = true;

        let world = world.read();
        let network = world.get_component::<Network>(event.player).unwrap();

        let packet = SpawnPosition {
            location: BlockPosition::new(
                state.level.spawn_x,
                state.level.spawn_y,
                state.level.spawn_z,
            ),
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
    });
}

#[event_handler]
fn send_join_game(
    event: &PlayerJoinEvent,
    _query: &mut Query<(Read<EntityId>, Read<Network>)>,
    world: &mut PreparedWorld,
) {
    let network = world.get_component::<Network>(event.player).unwrap();
    let id = world.get_component::<EntityId>(event.player).unwrap();

    // TODO
    let packet = JoinGame {
        entity_id: id.0,
        gamemode: Gamemode::Creative.id(),
        dimension: 0,
        difficulty: 0,
        max_players: 0,
        level_type: "default".to_string(),
        reduced_debug_info: false,
    };
    network.send(packet);
}
