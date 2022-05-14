use ahash::AHashSet;
use base::{ChunkPosition, Position};
use ecs::{SysResult, SystemExecutor};
use itertools::Either;
use quill_common::components::Name;
use quill_common::events::PlayerJoinEvent;

use crate::{events::ViewUpdateEvent, Game};

/// Registers systems to update the `View` of a player.
pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .add_system(update_player_views)
        .add_system(update_view_on_join);
}

/// Updates players' views when they change chunks.
fn update_player_views(game: &mut Game) -> SysResult {
    let mut events = Vec::new();
    for (player, (view, &position, name)) in
        game.ecs.query::<(&mut View, &Position, &Name)>().iter()
    {
        if position.chunk() != view.center() {
            let old_view = *view;
            let new_view = View::new(position.chunk(), old_view.view_distance);

            let event = ViewUpdateEvent::new(old_view, new_view);
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
    for (player, (&view, name, _)) in game.ecs.query::<(&View, &Name, &PlayerJoinEvent)>().iter() {
        let event = ViewUpdateEvent::new(View::empty(), view);
        events.push((player, event));
        log::trace!("View of {} has been updated (player joined)", name);
    }
    for (player, event) in events {
        game.ecs.insert_entity_event(player, event)?;
    }
    Ok(())
}

/// The view of a player, representing the set of chunks
/// within their view distance.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct View {
    center: ChunkPosition,
    view_distance: u32,
}

impl View {
    /// Creates a `View` from a center chunk (the position of the player)
    /// and the view distance.
    pub fn new(center: ChunkPosition, view_distance: u32) -> Self {
        Self {
            center,
            view_distance,
        }
    }

    /// Gets the empty view, i.e., the view containing no chunks.
    pub fn empty() -> Self {
        Self::new(ChunkPosition::new(0, 0), 0)
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
    pub fn iter(self) -> impl Iterator<Item = ChunkPosition> {
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
    pub fn difference(self, other: View) -> impl Iterator<Item = ChunkPosition> {
        // PERF: consider analytical approach instead of sets
        let self_chunks: AHashSet<_> = self.iter().collect();
        let other_chunks: AHashSet<_> = other.iter().collect();
        self_chunks
            .difference(&other_chunks)
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
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
        self.center.x - self.view_distance as i32
    }

    /// Returns the minimum Z coordinate.
    pub fn min_z(&self) -> i32 {
        self.center.z - self.view_distance as i32
    }

    /// Returns the maximum X coordinate.
    pub fn max_x(&self) -> i32 {
        self.center.x + self.view_distance as i32
    }

    /// Returns the maximum Z coordinate.
    pub fn max_z(&self) -> i32 {
        self.center.z + self.view_distance as i32
    }
}
