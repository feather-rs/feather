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

use feather_core::level::LevelData;
use feather_core::network::packet::implementation::{
    JoinGame, PlayerPositionAndLookClientbound, SpawnPosition,
};
use feather_core::world::{ChunkMap, ChunkPosition};
use feather_core::{Difficulty, Dimension};

use crate::chunk_logic::{ChunkHolderComponent, ChunkHolders, ChunkWorkerHandle};
use crate::config::Config;
use crate::entity::{EntitySpawnEvent, EntityType, PlayerComponent, PositionComponent};
use crate::network::NetworkComponent;
use crate::player::{ChunkPendingComponent, LoadedChunksComponent};
use crate::PlayerCount;

#[derive(Default)]
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

impl Default for Stage {
    fn default() -> Self {
        Stage::Initial
    }
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
        Write<'a, EventChannel<EntitySpawnEvent>>,
        Read<'a, ChunkWorkerHandle>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Arc<Config>>,
        Read<'a, Arc<PlayerCount>>,
        Read<'a, ChunkMap>,
        Write<'a, ChunkHolders>,
        Read<'a, LevelData>,
        WriteStorage<'a, ChunkHolderComponent>,
        WriteStorage<'a, LoadedChunksComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut joincomps,
            netcomps,
            pending_chunks,
            mut join_events,
            mut spawn_events,
            worker_handle,
            entities,
            lazy,
            config,
            player_count,
            chunk_map,
            mut holders,
            level,
            mut holder_comps,
            mut loaded_chunks_comps,
            playercomps,
            positions,
        ) = data;

        let mut to_remove = vec![];

        for (player, net, join_handler, pending_chunks) in
            (&entities, &netcomps, &mut joincomps, &pending_chunks).join()
        {
            match join_handler.stage {
                Stage::Initial => {
                    let playercomp = playercomps.get(player).unwrap();

                    // Send Join Game, then queue chunks for loading + sending.
                    let join_game = JoinGame::new(
                        player.id() as i32,
                        playercomp.gamemode.get_id(),
                        Dimension::Overwold.get_id(),
                        Difficulty::Medium.get_id(),
                        0,                     // Max players - not used
                        "default".to_string(), // Level type
                        false,                 // Reduced debug info
                    );
                    crate::network::send_packet_to_player(net, join_game);

                    let mut holder_comp = ChunkHolderComponent::new();
                    let mut loaded_chunks_comp = LoadedChunksComponent::default();

                    // Offsets from the origin to center view distance on
                    let chunk_offset_x = level.spawn_x >> 4;
                    let chunk_offset_z = level.spawn_z >> 4;

                    // Queue chunks
                    let view_distance = i32::from(config.server.view_distance);
                    for x in -view_distance..=view_distance {
                        for z in -view_distance..=view_distance {
                            let pos = ChunkPosition::new(x + chunk_offset_x, z + chunk_offset_z);
                            crate::player::send_chunk_to_player(
                                pos,
                                net,
                                player,
                                &chunk_map,
                                &worker_handle,
                                &mut holders,
                                &mut holder_comp,
                                &mut loaded_chunks_comp,
                                &lazy,
                            );
                        }
                    }

                    holder_comps.insert(player, holder_comp).unwrap();
                    loaded_chunks_comps
                        .insert(player, loaded_chunks_comp)
                        .unwrap();

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

                    let pos = positions.get(player).unwrap();
                    let spawn_block_pos = pos.current.block_pos();

                    let spawn_position = SpawnPosition::new(spawn_block_pos);
                    crate::network::send_packet_to_player(net, spawn_position);

                    let spawn_pos = spawn_block_pos.world_pos();

                    let position_and_look = PlayerPositionAndLookClientbound::new(
                        spawn_pos.x,
                        spawn_pos.y,
                        spawn_pos.z,
                        spawn_pos.yaw,
                        spawn_pos.pitch,
                        0, // Flags - unused by us
                        0, // Teleport ID - unused by us
                    );
                    crate::network::send_packet_to_player(net, position_and_look);

                    // Trigger events
                    let event = PlayerJoinEvent { player };
                    join_events.single_write(event);

                    let event = EntitySpawnEvent {
                        entity: player,
                        ty: EntityType::Player,
                    };
                    spawn_events.single_write(event);

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
