//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use crate::entity::{EntityComponent, EntityType, VelocityComponent};
use feather_core::world::ChunkMap;
use nalgebra::Vector3;
use rayon::prelude::*;
use shrev::EventChannel;
use specs::{Entities, ParJoin, Read, ReadStorage, System, Write, WriteStorage};

/// System for updating all entities' positions and velocities
/// each tick.
pub struct EntityPhysicsSystem;

impl<'a> System<'a> for EntityPhysicsSystem {
    type SystemData = (
        WriteStorage<'a, EntityComponent>,
        WriteStorage<'a, VelocityComponent>,
        ReadStorage<'a, EntityType>,
        Write<'a, EventChannel<EntityMoveEvent>>,
        Read<'a, ChunkMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut entity_comps, mut velocities, types, mut move_events, chunk_map, entities) = data;

        // We can't use a vector for this, since it has to be mutated
        // across threads.
        // This channel will act as a vector of `EntityMoveEvents` to trigger.
        let (tx, rx) = crossbeam::unbounded();

        // Go through entities and update position and velocity in parallel.
        (&mut entity_comps, &mut velocities, &entities, &types)
            .par_join()
            .for_each(|(mut entity_comp, mut velocity, entity, ty)| {
                let mut new_pos = entity_comp.position.clone();
                // First, add velocity to position, accounting for blocks
                // at the new position.
                let pending_new_pos = new_pos + velocity;
                if chunk_map.block_at(pending_new_pos.block_pos()).is_some() {
                    velocity.velocity = Vector3::new(0.0, 0.0, 0.0);
                } else {
                    new_pos = pending_new_pos;
                }

                // Depending on the type of entity, apply drag and gravity.
                // See https://minecraft.gamepedia.com/Entity for more information.
            });
    }
}

/// Retrieves the gravitational acceleration in blocks per tick
/// for a given entity type.
///
/// This information was fetched from
/// [the Minecraft wiki](https://minecraft.gamepedia.com/Entity#Motion_of_entities).
fn gravitational_acceleration(ty: EntityType) -> f64 {
    if ty.is_living() {
        -0.08
    } else if ty.is_item() {
        -0.04
    } else if ty.is_other() {
        0.0
    }
}

/// Retrieves the terminal velocity in blocks per tick
/// for a given entity type.
fn terminal_velocity(ty: EntityType) -> f64 {
    if ty.is_living() {
        -3.92
    } else if ty.is_item() {
        -1.96
    } else if ty.is_other() {
        0.0
    }
}

/// Retrieves the drag force for a given entity type.
fn drag_force(ty: EntityType) -> f64 {
    if ty.is_living() {
        0.98
    } else if ty.is_item() {
        0.04
    } else if ty.is_other() {
        0.0
    }
}
