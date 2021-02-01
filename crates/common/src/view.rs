use base::{ChunkPosition, Position};
use ecs::{SysResult, SystemExecutor};

use crate::{
    events::{PlayerJoinEvent, ViewUpdateEvent},
    Game, Name,
};

/// Registers systems to update the `View` of a player.
pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.add_system(update_player_views);
    game.event_bus().add_handler(update_view_on_join);
    dbg!();
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
            events.push(ViewUpdateEvent {
                player,
                old_view,
                new_view,
            });
            log::trace!("View of {} has been updated", name.0);
        }
    }

    for event in events {
        game.trigger_event(event);
    }
    Ok(())
}

/// Triggers a ViewUpdateEvent when a player joins the game.
fn update_view_on_join(game: &mut Game, event: &PlayerJoinEvent) -> SysResult {
    dbg!();
    let view = *game.ecs.get::<View>(event.player)?;
    let event = ViewUpdateEvent {
        old_view: View::empty(),
        new_view: view,
        player: event.player,
    };
    log::trace!(
        "View of {} has been updated",
        game.ecs.get::<Name>(event.player)?.0
    );
    game.trigger_event(event);
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
        Self::iter_3d(self.min_x(), self.min_z(), self.max_x(), self.max_z())
    }

    /// Determines whether the given chunk is visible.
    pub fn contains(&self, pos: ChunkPosition) -> bool {
        pos.x >= self.min_x()
            && pos.x <= self.max_x()
            && pos.z >= self.min_z()
            && pos.z <= self.max_z()
    }

    fn iter_3d(
        min_x: i32,
        min_z: i32,
        max_x: i32,
        max_z: i32,
    ) -> impl Iterator<Item = ChunkPosition> {
        (min_x..=max_x)
            .flat_map(move |x| (min_z..=max_z).map(move |y| (x, y)))
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
