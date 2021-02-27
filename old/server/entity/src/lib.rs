//! Dealing with entities, including associated components and events.
//! Submodules here are implementations of specific entities, such as items,
//! block entities, monsters, etc. Player entities are handled in `crate::player`,
//! not here.

#[macro_use]
extern crate feather_core;

mod broadcasters;
pub mod drops;
mod fall_damage;
mod inventory;
mod mob;
mod object;
pub mod particle;

pub use self::inventory::InventoryExt;
pub use broadcasters::*;
pub use drops::on_block_break_drop_loot;
pub use fall_damage::update_blocks_fallen;
pub use mob::*;
pub use object::falling_block::{on_entity_land_remove_falling_block, spawn_falling_blocks};
pub use object::item::{item_collect, on_item_drop_spawn_item_entity};
pub use object::*;

extern crate nalgebra_glm as glm;

use feather_core::util::Position;
use feather_server_types::{
    ChunkCrossEvent, Game, NetworkId, PreviousPosition, PreviousVelocity, Velocity,
};
use fecs::{EntityBuilder, IntoQuery, Read, World, Write};
use std::sync::atomic::{AtomicI32, Ordering};

/// Entity ID counter, used to create new entity IDs.
pub static ENTITY_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

#[fecs::system]
pub fn previous_position_velocity_reset(world: &mut World) {
    <(Read<Position>, Write<PreviousPosition>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(pos, mut previous_pos)| {
            previous_pos.0.replace(*pos);
        },
    );
    <(Read<Velocity>, Write<PreviousVelocity>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(vel, mut previous_vel)| {
            previous_vel.0.replace(vel.0);
        },
    );
}

#[fecs::event_handler]
pub fn on_chunk_cross_mark_modified(event: &ChunkCrossEvent, game: &mut Game) {
    if let Some(pos) = event.old {
        if let Some(mut old_chunk) = game.chunk_map.chunk_at_mut(pos) {
            old_chunk.set_modified()
        }
    }
}

/// Inserts the base components for an entity into an `EntityBuilder`.
///
/// This currently includes:
/// * Velocity (0) and PreviousVelocity
/// * Entity ID for the protocol
pub fn base() -> EntityBuilder {
    let id = new_id();
    EntityBuilder::new()
        .with(NetworkId(id))
        .with(Velocity::default())
        .with(PreviousVelocity::default())
        .with(PreviousPosition::default())
}

/// Returns a new entity ID.
pub fn new_id() -> i32 {
    ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
