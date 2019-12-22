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
//! * A system listens for player move events and checks if the player
//! crossed a chunk boundary. If so, a `ViewUpdateEvent` is triggered.
//! * Various systems listen to `ViewUpdateEvent` and send necessary packets.
//! This includes systems to load/unload chunks and send entities.

use crate::chunk_logic;
use crate::chunk_logic::{
    ChunkHolder, ChunkHolderReleaseEvent, ChunkHolders, ChunkLoadEvent, ChunkWorkerHandle,
};
use crate::config::Config;
use crate::entity::{EntityMoveEvent, PreviousPosition};
use crate::network::Network;
use crate::player::PlayerJoinEvent;
use crate::state::State;
use chashmap::CHashMap;
use feather_core::network::packet::implementation::{ChunkData, UnloadChunk};
use feather_core::{Chunk, ChunkPosition, Position};
use hashbrown::HashSet;
use legion::entity::Entity;
use legion::query::{Read, Write};
use parking_lot::Mutex;
use rayon::prelude::*;
use smallvec::SmallVec;
use tonks::{PreparedWorld, Query, Trigger};

/// Event triggered when a player's view is updated, i.e. when they
/// cross into a new chunk or when they join.
pub struct ViewUpdateEvent {
    /// The player whose view was updated.
    pub player: Entity,
    /// The new chunk.
    pub new_chunk: ChunkPosition,
    /// The old chunk, or `None` if there was no old chunk
    /// (i.e. this player just joined).
    pub old_chunk: Option<ChunkPosition>,
}

/// Event triggered when a chunk is sent to a player.
#[derive(Debug)]
pub struct ChunkSendEvent {
    pub chunk: ChunkPosition,
    pub player: Entity,
}

/// System which checks for players crossing chunk boundaries
/// and triggers `ViewUpdateEvent`s.
#[event_handler]
fn view_update(
    events: &[EntityMoveEvent],
    _query: &mut Query<(Read<Position>, Read<PreviousPosition>)>,
    world: &mut PreparedWorld,
    trigger: &mut Trigger<ViewUpdateEvent>,
) {
    let trigger = Mutex::new(trigger);
    events.par_iter().for_each(|event| {
        let pos = *world.get_component::<Position>(event.entity).unwrap();
        let prev_pos = world
            .get_component::<PreviousPosition>(event.entity)
            .unwrap()
            .0;

        if pos.chunk_pos() != prev_pos.chunk_pos() {
            // New chunk: trigger view update.
            let event = ViewUpdateEvent {
                player: event.entity,
                new_chunk: pos.chunk_pos(),
                old_chunk: Some(prev_pos.chunk_pos()),
            };
            trigger.lock().trigger(event);
        }
    });
}

/// System which triggers `ViewUpdateEvent`s on player join.
#[event_handler]
fn view_update_on_join(
    event: &PlayerJoinEvent,
    _query: &mut Query<Read<Position>>,
    world: &mut PreparedWorld,
    trigger: &mut Trigger<ViewUpdateEvent>,
) {
    let position = *world.get_component::<Position>(event.player).unwrap();

    trigger.trigger(ViewUpdateEvent {
        player: event.player,
        new_chunk: position.chunk_pos(),
        old_chunk: None,
    });
}

/// System which sends new chunks and unloads old chunks on the client
/// when the view is updated.
#[event_handler]
fn view_handle_chunks(
    events: &[ViewUpdateEvent],
    _query: &mut Query<(Read<Network>, Write<ChunkHolder>)>,
    world: &mut PreparedWorld,
    holders: &mut ChunkHolders,
    state: &State,
    chunks_to_send: &ChunksToSend,
    handle: &ChunkWorkerHandle,
    holder_release_trigger: &mut Trigger<ChunkHolderReleaseEvent>,
    chunk_send_trigger: &mut Trigger<ChunkSendEvent>,
) {
    events.iter().for_each(|event| {
        // Find the old chunks and new chunks.
        let new_chunks = chunks_within_view_distance(&state.config, event.new_chunk);
        let old_chunks = match event.old_chunk {
            Some(chunk) => chunks_within_view_distance(&state.config, chunk),
            None => HashSet::new(),
        };

        let to_send = new_chunks.difference(&old_chunks);
        let to_unload = old_chunks.difference(&new_chunks);

        let network = world.get_component::<Network>(event.player).unwrap();
        let mut holder =
            unsafe { world.get_component_mut_unchecked::<ChunkHolder>(event.player) }.unwrap();

        // Sort sent chunks so that closer chunks are sent first.
        let mut to_send = to_send.into_iter().copied().collect::<Vec<_>>();
        to_send.sort_unstable_by_key(|chunk| {
            chunk.manhattan_distance(event.new_chunk);
        });

        // Send new chunks.
        to_send.into_iter().for_each(|chunk| {
            send_chunk_to_player(
                state,
                event.player,
                &network,
                &mut holder,
                holders,
                chunk,
                chunks_to_send,
                handle,
                chunk_send_trigger,
            );
        });

        // Unload old chunks on client.
        to_unload.into_iter().for_each(|chunk| {
            unload_chunk_for_player(
                event.player,
                &network,
                holder_release_trigger,
                &mut holder,
                holders,
                *chunk,
            );
        });
    });
}

/// Resource containing a mapping from chunks -> sets of players indicating
/// which chunks are pending to send to a given player.
#[derive(Default, Resource)]
pub struct ChunksToSend(CHashMap<ChunkPosition, SmallVec<[Entity; 2]>>);

/// Asynchronously sends a chunk to a player.
fn send_chunk_to_player(
    state: &State,
    player: Entity,
    network: &Network,
    holder: &mut ChunkHolder,
    holders: &mut ChunkHolders,
    chunk: ChunkPosition,
    chunks_to_send: &ChunksToSend,
    handle: &ChunkWorkerHandle,
    trigger: &mut Trigger<ChunkSendEvent>,
) {
    // Ensure that the chunk isn't unloaded while the player has it loaded.
    chunk_logic::hold_chunk(player, holder, holders, chunk);

    // If the chunk is already loaded, send it. Otherwise, we need to
    // queue it for loading.
    if let Some(chunk) = state.chunk_at(chunk) {
        network.send(create_chunk_data(&chunk));
        trigger.trigger(ChunkSendEvent {
            chunk: chunk.position(),
            player,
        });
    } else {
        let contains = chunks_to_send.0.contains_key(&chunk);

        let mut vec = match chunks_to_send.0.get_mut(&chunk) {
            Some(vec) => vec,
            None => {
                chunks_to_send.0.insert(chunk, smallvec![]);
                chunks_to_send.0.get_mut(&chunk).unwrap()
            }
        };
        vec.push(player);

        if !contains {
            // Queue chunk for loading if it isn't already.
            chunk_logic::load_chunk(handle, chunk);
        }
    }
}

/// Unloads a chunk on a client.
fn unload_chunk_for_player(
    player: Entity,
    network: &Network,
    trigger: &mut Trigger<ChunkHolderReleaseEvent>,
    holder: &mut ChunkHolder,
    holders: &mut ChunkHolders,
    chunk: ChunkPosition,
) {
    // Release hold on chunk so it can be unloaded on the server
    chunk_logic::release_chunk(player, holder, holders, chunk, trigger);

    // Send Unload Chunk packet.
    network.send(UnloadChunk {
        chunk_x: chunk.x,
        chunk_z: chunk.z,
    });
}

/// System which sends chunks to pending players when a chunk is loaded.
#[event_handler]
fn chunk_send(
    event: &ChunkLoadEvent,
    state: &State,
    to_send: &ChunksToSend,
    _query: &mut Query<Read<Network>>,
    world: &mut PreparedWorld,
    trigger: &mut Trigger<ChunkSendEvent>,
) {
    if let Some(players) = to_send.0.get(&event.pos) {
        let chunk = state
            .chunk_at(event.pos)
            .expect("chunk not loaded, but load event was triggered");
        players.iter().for_each(|player| {
            let network = world.get_component::<Network>(*player).unwrap();
            network.send(create_chunk_data(&chunk));
            trigger.trigger(ChunkSendEvent {
                chunk: chunk.position(),
                player: *player,
            });
        });
    }

    to_send.0.remove(&event.pos);
}

/// Creates a chunk data packet for the given chunk.
fn create_chunk_data(chunk: &Chunk) -> ChunkData {
    ChunkData {
        chunk: chunk.clone(), // TODO: optimize
    }
}

/// Finds all chunks within the view distance of a given chunk.
fn chunks_within_view_distance(config: &Config, position: ChunkPosition) -> HashSet<ChunkPosition> {
    let view_distance = config.server.view_distance as i32;

    let dimensions = view_distance * 2 + 1;

    let mut set = HashSet::with_capacity((dimensions * dimensions) as usize);

    for x in -view_distance..=view_distance {
        for z in -view_distance..=view_distance {
            set.insert(position + ChunkPosition::new(x, z));
        }
    }

    set
}
