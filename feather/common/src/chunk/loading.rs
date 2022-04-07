//! Chunk loading and unloading based on player `View`s.

use std::{
    collections::VecDeque,
    mem,
    time::{Duration, Instant},
};

use ahash::AHashMap;

use anyhow::Context;
use libcraft::ChunkPosition;
use quill::events::EntityRemoveEvent;
use utils::vec_remove_item;
use vane::{Entity, SysResult, SystemExecutor};

use crate::{chunk::worker::LoadRequest, events::ViewUpdateEvent, Game};
use quill::components::{EntityDimension, EntityWorld};

use crate::world::Dimensions;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(ChunkLoadState::default());
    systems
        .group::<ChunkLoadState>()
        .add_system(remove_dead_entities)
        .add_system(update_tickets_for_players)
        .add_system(unload_chunks)
        .add_system(load_chunks);
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

impl ChunkLoadState {
    pub fn remove_ticket(
        &mut self,
        chunk: ChunkPosition,
        ticket: Ticket,
        world: EntityWorld,
        dimension: EntityDimension,
    ) {
        self.chunk_tickets.remove_ticket(chunk, ticket);

        // If this was the last ticket, then queue the chunk to be
        // unloaded.
        if self.chunk_tickets.num_tickets(chunk) == 0 {
            self.chunk_tickets.remove_chunk(chunk);
            self.chunk_unload_queue
                .push_back(QueuedChunkUnload::new(chunk, world, dimension));
        }
    }
}

#[derive(Clone, Debug)]
struct QueuedChunkUnload {
    pos: ChunkPosition,
    /// Time after which the chunk should be unloaded.
    unload_at_time: Instant,
    world: EntityWorld,
    dimension: EntityDimension,
}

impl QueuedChunkUnload {
    pub fn new(pos: ChunkPosition, world: EntityWorld, dimension: EntityDimension) -> Self {
        Self {
            pos,
            unload_at_time: Instant::now() + UNLOAD_DELAY,
            world,
            dimension,
        }
    }
}

/// Maintains a list of "tickets" for each loaded chunk.
/// A chunk is queued for unloading when it has no more tickets.
#[derive(Default)]
struct ChunkTickets {
    tickets: AHashMap<ChunkPosition, Vec<Ticket>>,
    by_entity: AHashMap<Ticket, Vec<ChunkPosition>>,
}

impl ChunkTickets {
    pub fn insert_ticket(&mut self, chunk: ChunkPosition, ticket: Ticket) {
        self.tickets.entry(chunk).or_default().push(ticket);
        self.by_entity.entry(ticket).or_default().push(chunk);
    }

    pub fn remove_ticket(&mut self, chunk: ChunkPosition, ticket: Ticket) {
        if let Some(vec) = self.tickets.get_mut(&chunk) {
            vec_remove_item(vec, &ticket);
        }
        vec_remove_item(self.by_entity.get_mut(&ticket).unwrap(), &chunk);
    }

    pub fn num_tickets(&self, chunk: ChunkPosition) -> usize {
        match self.tickets.get(&chunk) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    pub fn take_entity_tickets(&mut self, ticket: Ticket) -> Vec<ChunkPosition> {
        self.by_entity
            .get_mut(&ticket)
            .map(mem::take)
            .unwrap_or_default()
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
    for (player, (event, world, dimension)) in game
        .ecs
        .query::<(&ViewUpdateEvent, &EntityWorld, &EntityDimension)>()
        .iter()
    {
        let player_ticket = Ticket(player);

        // Remove old tickets
        for &old_chunk in &event.old_chunks {
            state.remove_ticket(old_chunk, player_ticket, *world, dimension.clone());
        }

        // Create new tickets
        for &new_chunk in &event.new_chunks {
            state.chunk_tickets.insert_ticket(new_chunk, player_ticket);

            // Load if needed
            let mut dimensions = game.ecs.get_mut::<Dimensions>(event.new_world.0)?;
            let dimension = dimensions
                .get_mut(&*event.new_dimension)
                .context("missing dimension")?;
            if !dimension.is_chunk_loaded(new_chunk) && !dimension.is_chunk_loading(new_chunk) {
                dimension.queue_chunk_load(LoadRequest { pos: new_chunk });
            }
        }
    }
    Ok(())
}

/// System to unload chunks from the `ChunkUnloadQueue`.
fn unload_chunks(game: &mut Game, state: &mut ChunkLoadState) -> SysResult {
    while let Some(unload) = state.chunk_unload_queue.get(0) {
        if unload.unload_at_time > Instant::now() {
            // None of the remaining chunks in the queue are
            // ready for unloading, because the queue is ordered
            // by time.
            break;
        }

        let unload = state.chunk_unload_queue.pop_front().unwrap();

        // If the chunk has acquired new tickets, then abort unloading it.
        if state.chunk_tickets.num_tickets(unload.pos) > 0 {
            continue;
        }

        game.ecs
            .query::<&mut Dimensions>()
            .iter()
            .find(|(world, _dimensions)| *world == *unload.world)
            .unwrap()
            .1
            .get_mut(&*unload.dimension)
            .unwrap()
            .unload_chunk(unload.pos)?;
    }
    for (_, mut dimensions) in game.ecs.query::<&mut Dimensions>().iter() {
        for dimension in dimensions.iter_mut() {
            dimension.cache.purge_unused();
        }
    }
    Ok(())
}

fn remove_dead_entities(game: &mut Game, state: &mut ChunkLoadState) -> SysResult {
    for (entity, (_event, world, dimension)) in game
        .ecs
        .query::<(&EntityRemoveEvent, &EntityWorld, &EntityDimension)>()
        .iter()
    {
        let entity_ticket = Ticket(entity);
        for chunk in state.chunk_tickets.take_entity_tickets(entity_ticket) {
            state.remove_ticket(chunk, entity_ticket, *world, dimension.clone());
        }
    }
    Ok(())
}

/// System to call `World::load_chunks` each tick
fn load_chunks(game: &mut Game, _state: &mut ChunkLoadState) -> SysResult {
    let mut events = Vec::new();
    for (_, mut dimensions) in game.ecs.query::<&mut Dimensions>().iter() {
        for dimension in dimensions.iter_mut() {
            events.extend(dimension.load_chunks()?)
        }
    }
    events
        .into_iter()
        .for_each(|event| game.ecs.insert_event(event));
    Ok(())
}
