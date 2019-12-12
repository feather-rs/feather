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

use crate::entity::{EntityMoveEvent, PreviousPosition};
use crate::player::Player;
use feather_core::{ChunkPosition, Position};
use legion::entity::Entity;
use rayon::prelude::*;
use tonks::{PreparedQuery, PreparedWorld, Read, TriggerOwned};

/// Event triggered when a player's view is updated, i.e. when they
/// cross into a new chunk.
pub struct ViewUpdateEvent {
    /// The player whose view was updated.
    pub player: Entity,
    /// The new chunk.
    pub new_chunk: ChunkPosition,
    /// The old chunk.
    pub old_chunk: ChunkPosition,
}

/// System which checks for players crossing chunk boundaries
/// and triggers `ViewUpdateEvent`s.
#[event_handler]
fn view_update(
    events: &[EntityMoveEvent],
    query: PreparedQuery<(Read<Position>, Read<PreviousPosition>)>,
    world: PreparedWorld,
    trigger: TriggerOwned<ViewUpdateEvent>,
) {
    events.iter().for_each(|event| {
        let (pos, prev_pos) = query.find_immutable(event.entity, &world).unwrap();

        if pos.chunk_pos() != prev_pos.chunk_pos() {
            // New chunk: trigger view update.
            let event = ViewUpdateEvent {
                player: event.entity,
                new_chunk: pos.chunk_pos(),
                old_chunk: prev_pos.chunk_pos(),
            };
            trigger.trigger(event);
        }
    });
}
