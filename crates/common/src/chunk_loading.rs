//! Chunk loading and unloading based on player `View`s.

use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use ahash::AHashMap;
use base::ChunkPosition;
use ecs::{Entity, SysResult, SystemExecutor};

use crate::{events::ViewUpdateEvent, Game};

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(ChunkLoadState::default());
    systems
        .group::<ChunkLoadState>()
        .add_system(update_tickets_for_players)
        .add_system(unload_chunks);
}

/// Amount of time to wait after a chunk has
/// no tickets until it is unloaded.
const UNLOAD_DELAY: Duration = Duration::from_secs(10);

#[derive(Default)]
struct ChunkLoadState {
    /// Chunks that have been queued for unloading.
    chunk_unload_queue: VecDeque<QueuedChunkUnload>,

    chunk_tickets: ChunkTickets,
}

#[derive(Copy, Clone, Debug)]
struct QueuedChunkUnload {
    pos: ChunkPosition,
    /// Time after which the chunk should be unloaded.
    unload_at_time: Instant,
}

impl QueuedChunkUnload {
    pub fn new(pos: ChunkPosition) -> Self {
        Self {
            pos,
            unload_at_time: Instant::now() + UNLOAD_DELAY,
        }
    }
}

/// Maintains a list of "tickets" for each loaded chunk.
/// A chunk is queued for unloading when it has no more tickets.
#[derive(Default)]
struct ChunkTickets {
    tickets: AHashMap<ChunkPosition, Vec<Ticket>>,
}

impl ChunkTickets {
    pub fn insert_ticket(&mut self, chunk: ChunkPosition, ticket: Ticket) {
        self.tickets.entry(chunk).or_default().push(ticket);
    }

    pub fn remove_ticket(&mut self, chunk: ChunkPosition, ticket: Ticket) {
        if let Some(vec) = self.tickets.get_mut(&chunk) {
            let index = vec.iter().position(|t| *t == ticket);
            if let Some(index) = index {
                vec.swap_remove(index);
            }
        }
    }

    pub fn num_tickets(&self, chunk: ChunkPosition) -> usize {
        match self.tickets.get(&chunk) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    pub fn remove_chunk(&mut self, pos: ChunkPosition) {
        self.tickets.remove(&pos);
    }
}

/// ID of a chunk ticket that keeps a chunk loaded.
///
/// Currently just represents an entity, the player
/// that is keeping this chunk loaded.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Ticket(Entity);

/// System to populate chunk tickets based on players' views.
fn update_tickets_for_players(game: &mut Game, state: &mut ChunkLoadState) -> SysResult {
    for (player, event) in game.ecs.query::<&ViewUpdateEvent>().iter() {
        let player_ticket = Ticket(player);

        // Remove old tickets
        for old_chunk in event.old_view.difference(event.new_view) {
            state.chunk_tickets.remove_ticket(old_chunk, player_ticket);

            // If this was the last ticket, then queue the chunk to be
            // unloaded.
            if state.chunk_tickets.num_tickets(old_chunk) == 0 {
                state.chunk_tickets.remove_chunk(old_chunk);
                state
                    .chunk_unload_queue
                    .push_back(QueuedChunkUnload::new(old_chunk));
            }
        }

        // Create new tickets
        for new_chunk in event.new_view.difference(event.old_view) {
            state.chunk_tickets.insert_ticket(new_chunk, player_ticket);

            // Load if needed
            if !game.world.is_chunk_loaded(new_chunk) && !game.world.is_chunk_loading(new_chunk) {
                game.world.queue_chunk_load(new_chunk);
            }
        }
    }
    Ok(())
}

/// System to unload chunks from the `ChunkUnloadQueue`.
fn unload_chunks(game: &mut Game, state: &mut ChunkLoadState) -> SysResult {
    while let Some(&unload) = state.chunk_unload_queue.get(0) {
        if unload.unload_at_time > Instant::now() {
            // None of the remaining chunks in the queue are
            // ready for unloading, because the queue is ordered
            // by time.
            break;
        }

        state.chunk_unload_queue.pop_front();

        // If the chunk has acquired new tickets, then abort unloading it.
        if state.chunk_tickets.num_tickets(unload.pos) > 0 {
            continue;
        }

        game.world.unload_chunk(unload.pos);
    }
    Ok(())
}
