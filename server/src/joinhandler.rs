//! The join handler, in contrast to the initial handler,
//! takes over after the login sequence has completed.
//! It's responsible for asyncrhonously loading the player's
//! data (inventory, chunks, etc.) and then sending the necessary
//! packets to join the player. After completion, the component is
//! removed.

use std::sync::atomic::Ordering;
use std::sync::Arc;

use shrev::EventChannel;
use specs::{
    Component, Entities, Entity, HashMapStorage, Join, LazyUpdate, Read, ReadStorage, System,
    Write, WriteStorage,
};

use feather_core::network::packet::implementation::{
    JoinGame, PlayerPositionAndLookClientbound, SpawnPosition,
};
use feather_core::world::{BlockPosition, ChunkMap, ChunkPosition, Position};
use feather_core::{Difficulty, Dimension, Gamemode};

use crate::chunk_logic::{ChunkHolderComponent, ChunkHolders, ChunkWorkerHandle};
use crate::config::Config;
use crate::network::NetworkComponent;
use crate::player::ChunkPendingComponent;
use crate::PlayerCount;

/// For now, we use a fixed spawn position.
/// In the future, the spawn position should
/// be loaded asynchronously from the world save.
pub const SPAWN_POSITION: Position = Position {
    x: 0.0,
    y: 64.0,
    z: 0.0,
    pitch: 0.0,
    yaw: 0.0,
};

/// See `SPAWN_POSITION`
const COMPASS_SPAWN_POSITION: BlockPosition = BlockPosition { x: 0, y: 64, z: 0 };

pub struct JoinHandlerComponent {
    stage: Stage,
}

impl JoinHandlerComponent {
    pub fn new() -> Self {
        Self {
            stage: Stage::Initial,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    Initial,
    AwaitChunkSends,
}

impl Component for JoinHandlerComponent {
    type Storage = HashMapStorage<Self>;
}

/// Event which is triggered when a player
/// completes the join process (i.e. when
/// all chunks have been sent).
pub struct PlayerJoinEvent {
    pub player: Entity,
}

/// System for join handling.
pub struct JoinHandlerSystem;

impl<'a> System<'a> for JoinHandlerSystem {
    type SystemData = (
        WriteStorage<'a, JoinHandlerComponent>,
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, ChunkPendingComponent>,
        Write<'a, EventChannel<PlayerJoinEvent>>,
        Read<'a, ChunkWorkerHandle>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Arc<Config>>,
        Read<'a, Arc<PlayerCount>>,
        Read<'a, ChunkMap>,
        Write<'a, ChunkHolders>,
        WriteStorage<'a, ChunkHolderComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut joincomps,
            netcomps,
            pending_chunks,
            mut join_events,
            worker_handle,
            entities,
            lazy,
            config,
            player_count,
            chunk_map,
            mut holders,
            mut holder_comps,
        ) = data;

        let mut to_remove = vec![];

        for (player, net, join_handler, pending_chunks) in
            (&entities, &netcomps, &mut joincomps, &pending_chunks).join()
        {
            match join_handler.stage {
                Stage::Initial => {
                    // Send Join Game, then queue chunks for loading + sending.
                    let join_game = JoinGame::new(
                        player.id() as i32,
                        Gamemode::Creative.get_id(),
                        Dimension::Overwold.get_id(),
                        Difficulty::Medium.get_id(),
                        0,                     // Max players - not used
                        "default".to_string(), // Level type
                        false,                 // Reduced debug info
                    );
                    crate::network::send_packet_to_player(net, join_game);

                    let mut holder_comp = ChunkHolderComponent::new();

                    // Queue chunks
                    let view_distance = config.server.view_distance as i32;
                    for x in -view_distance..=view_distance {
                        for y in -view_distance..=view_distance {
                            let pos = ChunkPosition::new(x, y);
                            crate::player::send_chunk_to_player(
                                pos,
                                net,
                                player,
                                &chunk_map,
                                &worker_handle,
                                &mut holders,
                                &mut holder_comp,
                                &lazy,
                            );
                        }
                    }

                    holder_comps.insert(player, holder_comp).unwrap();

                    // Increment player count
                    player_count.0.fetch_add(1, Ordering::SeqCst);

                    join_handler.stage = Stage::AwaitChunkSends;
                }
                Stage::AwaitChunkSends => {
                    // If 0 chunks have yet to be sent, join the player by sending spawn position.
                    // See https://wiki.vg/Protocol_FAQ
                    if pending_chunks.len() != 0 {
                        continue;
                    }
                    let spawn_position = SpawnPosition::new(COMPASS_SPAWN_POSITION);
                    crate::network::send_packet_to_player(net, spawn_position);

                    let position_and_look = PlayerPositionAndLookClientbound::new(
                        SPAWN_POSITION.x,
                        SPAWN_POSITION.y,
                        SPAWN_POSITION.z,
                        SPAWN_POSITION.yaw,
                        SPAWN_POSITION.pitch,
                        0, // Flags - unused by us
                        0, // Teleport ID - unused by us
                    );
                    crate::network::send_packet_to_player(net, position_and_look);

                    // Trigger event
                    let event = PlayerJoinEvent { player };
                    join_events.single_write(event);

                    // We're finished here.
                    to_remove.push(player);
                }
            }
        }

        to_remove.into_iter().for_each(|player| {
            joincomps.remove(player);
        });
    }
}
