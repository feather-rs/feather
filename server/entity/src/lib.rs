//! Dealing with entities, including associated components and events.
//! Submodules here are implementations of specific entities, such as items,
//! block entities, monsters, etc. Player entities are handled in `crate::player`,
//! not here.

#[macro_use]
extern crate feather_core;

mod broadcasters;
mod inventory;
mod mob;
mod object;

pub use broadcasters::*;
pub use mob::*;
pub use object::*;

extern crate nalgebra_glm as glm;

use feather_core::util::Position;
use feather_server_types::{EntityId, PreviousPosition, PreviousVelocity, Velocity};
use fecs::{EntityBuilder, IntoQuery, Read, World, Write};
use std::sync::atomic::{AtomicI32, Ordering};

/// Entity ID counter, used to create new entity IDs.
pub static ENTITY_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

#[fecs::system]
pub fn previous_position_velocity_reset(world: &mut World) {
    <(Read<Position>, Write<PreviousPosition>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(pos, mut previous_pos)| {
            previous_pos.0 = *pos;
        },
    );
    <(Read<Velocity>, Write<PreviousVelocity>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(vel, mut previous_vel)| {
            previous_vel.0 = vel.0;
        },
    );
}

/// Inserts the base components for an entity into an `EntityBuilder`.
///
/// This currently includes:
/// * Velocity (0) and PreviousVelocity
/// * Entity ID for the protocol
pub fn base() -> EntityBuilder {
    let id = new_id();
    EntityBuilder::new()
        .with(EntityId(id))
        .with(Velocity::default())
        .with(PreviousVelocity::default())
        .with(PreviousPosition(position!(0.0, 0.0, 0.0)))
}

/// Returns a new entity ID.
pub fn new_id() -> i32 {
    ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
