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
    Component, Entities, Entity, HashMapStorage, Join, LazyUpdate, Read, ReadExpect, ReadStorage,
    System, Write, WriteStorage,
};

use feather_core::level::LevelData;
use feather_core::network::packet::implementation::{
    JoinGame, PlayerPositionAndLookClientbound, SpawnPosition,
};
use feather_core::world::{BlockPosition, ChunkMap, ChunkPosition};
use feather_core::{Difficulty, Dimension};

use crate::chunk_logic::{ChunkHolderComponent, ChunkHolders, ChunkWorkerHandle};
use crate::config::Config;
use crate::entity::{EntitySpawnEvent, EntityType, PlayerComponent, PositionComponent};
use crate::network::NetworkComponent;
use crate::player::{ChunkPendingComponent, InventoryUpdateEvent, LoadedChunksComponent};
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
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        ReadExpect<'a, ChunkWorkerHandle>,
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
            mut inv_events,
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

                    let level_type = &level.generator_name;

                    // Send Join Game, then queue chunks for loading + sending.
                    let join_game = JoinGame::new(
                        player.id() as i32,
                        playercomp.gamemode.get_id(),
                        Dimension::Overwold.get_id(),
                        Difficulty::Medium.get_id(),
                        0, // Max players - not used
                        level_type.to_string(),
                        false, // Reduced debug info
                    );
                    crate::network::send_packet_to_player(net, join_game);

                    let mut holder_comp = ChunkHolderComponent::new();
                    let mut loaded_chunks_comp = LoadedChunksComponent::default();

                    let player_pos = positions.get(player).unwrap().current.chunk_pos();

                    // Offsets from the origin to center view distance on
                    let chunk_offset_x = player_pos.x;
                    let chunk_offset_z = player_pos.z;

                    // Queue chunks for sending.
                    let view_distance = i32::from(config.server.view_distance);
                    let mut chunks = Vec::with_capacity((view_distance * view_distance) as usize);
                    for x in -view_distance..=view_distance {
                        for z in -view_distance..=view_distance {
                            let chunk = ChunkPosition::new(x + chunk_offset_x, z + chunk_offset_z);
                            chunks.push(chunk);
                        }
                    }

                    // Sort chunks so that closest chunks are sent first.
                    chunks.sort_unstable_by(|a, b| {
                        a.manhattan_distance(player_pos)
                            .cmp(&b.manhattan_distance(player_pos))
                    });

                    // Queue chunks for loading + sending
                    chunks.into_iter().for_each(|chunk| {
                        crate::player::send_chunk_to_player(
                            chunk,
                            net,
                            player,
                            &chunk_map,
                            &worker_handle,
                            &mut holders,
                            &mut holder_comp,
                            &mut loaded_chunks_comp,
                            &lazy,
                        );
                    });

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

                    // SpawnPosition packet: world spawn (used for compass)
                    let level_spawn_block_pos =
                        BlockPosition::new(level.spawn_x, level.spawn_y, level.spawn_z);
                    let level_spawn_position = SpawnPosition::new(level_spawn_block_pos);
                    crate::network::send_packet_to_player(net, level_spawn_position);

                    // Initial position/rotation for the player when they spawn
                    let player_pos = positions.get(player).unwrap().current;
                    let position_and_look = PlayerPositionAndLookClientbound::new(
                        player_pos.x,
                        player_pos.y,
                        player_pos.z,
                        player_pos.yaw,
                        player_pos.pitch,
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

                    // Trigger inventory update event on the entire inventory
                    let event = InventoryUpdateEvent {
                        slots: (0..46).collect(),
                        player,
                    };
                    inv_events.single_write(event);

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
