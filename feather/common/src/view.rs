use ahash::{AHashMap, AHashSet};
use itertools::Either;
use libcraft::ChunkPosition;
use quill::components::EntityWorld;
use quill::components::{EntityPosition, Name};
use quill::events::PlayerJoinEvent;
use quill::world::ChunkTicket;
use quill::WorldId;
use vane::{Component, SysResult, SystemExecutor};

use crate::{events::ViewUpdateEvent, Game};

/// Registers systems to update the `View` of a player.
pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .add_system(update_player_views)
        .add_system(update_view_on_join)
        .add_system(update_tickets_for_players);
}

/// Updates players' views when they change chunks.
fn update_player_views(game: &mut Game) -> SysResult {
    let mut events = Vec::new();
    for (player, (mut view, position, name, world)) in game
        .ecs
        .query::<(&mut View, &EntityPosition, &Name, &EntityWorld)>()
        .iter()
    {
        if position.chunk() != view.center() {
            let old_view = view.clone();
            let new_view = View::new(position.chunk(), old_view.view_distance, world.0);

            let event = ViewUpdateEvent::new(&old_view, &new_view);
            events.push((player, event));

            *view = new_view;
            log::trace!("View of {} has been updated", name);
        }
    }

    for (player, event) in events {
        game.ecs.insert_entity_event(player, event)?;
    }
    Ok(())
}

/// Triggers a ViewUpdateEvent when a player joins the game.
fn update_view_on_join(game: &mut Game) -> SysResult {
    let mut events = Vec::new();
    for (player, (view, name, world, _)) in game
        .ecs
        .query::<(&View, &Name, &EntityWorld, &PlayerJoinEvent)>()
        .iter()
    {
        let event = ViewUpdateEvent::new(&View::empty(world.0), &view);
        events.push((player, event));
        log::trace!("View of {} has been updated (player joined)", name);
    }
    for (player, event) in events {
        game.ecs.insert_entity_event(player, event)?;
        game.ecs.insert(player, PlayerChunkTickets::default())?;
    }
    Ok(())
}

#[derive(Default)]
struct PlayerChunkTickets {
    map: AHashMap<ChunkPosition, ChunkTicket>,
}

impl Component for PlayerChunkTickets {}

fn update_tickets_for_players(game: &mut Game) -> SysResult {
    for (_, (event, mut chunk_tickets, world_id)) in game
        .ecs
        .query::<(&ViewUpdateEvent, &mut PlayerChunkTickets, &EntityWorld)>()
        .iter()
    {
        // Remove old tickets
        for old_chunk in &event.old_chunks {
            // Dropping the ticket automatically removes it from the world
            chunk_tickets.map.remove(old_chunk);
        }

        // Create new tickets
        let mut world = game.world_mut(world_id.0)?;
        for &new_chunk in &event.new_chunks {
            let ticket = world.create_chunk_ticket(new_chunk);
            chunk_tickets.map.insert(new_chunk, ticket);
        }
    }
    Ok(())
}

/// The view of a player, representing the set of chunks
/// within their view distance.
#[derive(Clone, Debug)]
pub struct View {
    center: ChunkPosition,
    view_distance: u32,
    world: WorldId,
}

impl Component for View {}

impl View {
    /// Creates a `View` from a center chunk (the position of the player)
    /// and the view distance.
    pub fn new(center: ChunkPosition, view_distance: u32, world: WorldId) -> Self {
        Self {
            center,
            view_distance,
            world,
        }
    }

    /// Gets the empty view, i.e., the view containing no chunks.
    pub fn empty(world: WorldId) -> Self {
        Self::new(ChunkPosition::new(0, 0), 0, world)
    }

    /// Determines whether this is the empty view.
    pub fn is_empty(&self) -> bool {
        self.view_distance == 0
    }

    pub fn center(&self) -> ChunkPosition {
        self.center
    }

    pub fn view_distance(&self) -> u32 {
        self.view_distance
    }

    pub fn set_center(&mut self, center: ChunkPosition) {
        self.center = center;
    }

    pub fn set_view_distance(&mut self, view_distance: u32) {
        self.view_distance = view_distance;
    }

    /// Iterates over chunks visible to the player.
    pub fn iter(&self) -> impl Iterator<Item = ChunkPosition> {
        if self.is_empty() {
            Either::Left(std::iter::empty())
        } else {
            Either::Right(Self::iter_2d(
                self.min_x(),
                self.min_z(),
                self.max_x(),
                self.max_z(),
            ))
        }
    }

    /// Returns the set of chunks that are in `self` but not in `other`.
    pub fn difference(&self, other: &View) -> Vec<ChunkPosition> {
        if self.world != other.world {
            self.iter().collect()
        } else {
            // PERF: consider analytical approach instead of sets
            let self_chunks: AHashSet<_> = self.iter().collect();
            let other_chunks: AHashSet<_> = other.iter().collect();
            self_chunks.difference(&other_chunks).copied().collect()
        }
    }

    /// Determines whether the given chunk is visible.
    pub fn contains(&self, pos: ChunkPosition) -> bool {
        pos.x >= self.min_x()
            && pos.x <= self.max_x()
            && pos.z >= self.min_z()
            && pos.z <= self.max_z()
    }

    fn iter_2d(
        min_x: i32,
        min_z: i32,
        max_x: i32,
        max_z: i32,
    ) -> impl Iterator<Item = ChunkPosition> {
        (min_x..=max_x)
            .flat_map(move |x| (min_z..=max_z).map(move |z| (x, z)))
            .map(|(x, z)| ChunkPosition { x, z })
    }

    /// Returns the minimum X chunk coordinate.
    pub fn min_x(&self) -> i32 {
        // I don't know why but it's loading a 3x3 area with view_distance=2,
        // there should be a better way to fix this
        self.center.x - self.view_distance as i32 - 1
    }

    /// Returns the minimum Z coordinate.
    pub fn min_z(&self) -> i32 {
        self.center.z - self.view_distance as i32 - 1
    }

    /// Returns the maximum X coordinate.
    pub fn max_x(&self) -> i32 {
        self.center.x + self.view_distance as i32 + 1
    }

    /// Returns the maximum Z coordinate.
    pub fn max_z(&self) -> i32 {
        self.center.z + self.view_distance as i32 + 1
    }

    pub fn world(&self) -> WorldId {
        self.world
    }
}
