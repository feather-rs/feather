//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use crate::entity::{EntityComponent, EntityMoveEvent, EntityType, VelocityComponent};
use feather_core::world::block::Block;
use feather_core::world::ChunkMap;
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
            .for_each(|(entity_comp, velocity, entity, ty)| {
                let position = entity_comp.position;
                // First, add velocity to position, accounting for blocks
                // at the new position.
                let pending_position = position + velocity.0;
                let pending_block = chunk_map.block_at(pending_position.block_pos());

                // If block is `None`, the chunk at pending_position isn't
                // loading. If so, unload the entity.
                if let Some(block) = pending_block {
                    // Check that block isn't solid.
                    // If it is, update velocity accordingly.
                    if block != Block::Air {
                        let pending_pos_x = position + glm::vec3(velocity.0.x, 0.0, 0.0);
                        if chunk_map.block_at(pending_pos_x.block_pos()) != Some(Block::Air) {
                            velocity.0.x = 0.0;
                        }

                        let pending_pos_y = position + glm::vec3(velocity.0.y, 0.0, 0.0);
                        if chunk_map.block_at(pending_pos_y.block_pos()) != Some(Block::Air) {
                            velocity.0.y = 0.0;
                        }

                        let pending_pos_z = position + glm::vec3(velocity.0.z, 0.0, 0.0);
                        if chunk_map.block_at(pending_pos_z.block_pos()) != Some(Block::Air) {
                            velocity.0.z = 0.0;
                        }
                    } else {
                        // Depending on the type of entity, apply drag and gravity.
                        // See https://minecraft.gamepedia.com/Entity for more information.
                        let gravity = gravitational_acceleration(*ty);
                        // let terminal = terminal_velocity(*ty);
                        let drag = drag_force(*ty);

                        velocity.0.y = drag * velocity.0.y + gravity;
                    }

                    // Set new position and trigger event.
                    let event = EntityMoveEvent {
                        entity,
                        old_pos: position,
                        new_pos: pending_position,
                    };

                    tx.send(event).unwrap();

                    entity_comp.position = pending_position;
                } else {
                    // Chunk isn't loaded. Unload entity.
                    entities.delete(entity).unwrap();
                    return;
                }
            });

        // Go through events and trigger them.
        while let Ok(event) = rx.try_recv() {
            move_events.single_write(event);
        }
    }
}

/// Retrieves the gravitational acceleration in blocks per tick
/// for a given entity type.
///
/// This information was fetched from
/// [the Minecraft wiki](https://minecraft.gamepedia.com/Entity#Motion_of_entities).
fn gravitational_acceleration(ty: EntityType) -> f32 {
    if ty.is_living() {
        -0.08
    } else if ty.is_item() {
        -0.04
    } else if ty.is_other() {
        0.0
    } else {
        0.0
    }
}

/*
/// Retrieves the terminal velocity in blocks per tick
/// for a given entity type.
fn terminal_velocity(ty: EntityType) -> f32 {
    if ty.is_living() {
        -3.92
    } else if ty.is_item() {
        -1.96
    } else if ty.is_other() {
        0.0
    } else {
        0.0
    }
}
*/

/// Retrieves the drag force for a given entity type.
fn drag_force(ty: EntityType) -> f32 {
    if ty.is_living() {
        0.98
    } else if ty.is_item() {
        0.04
    } else if ty.is_other() {
        0.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::world::Position;

    #[test]
    fn test_entity_physics_basic() {
        let (mut w, mut d) = t::init_world();

        let pos = Position::new(0.0, 0.0, 0.0);
        let vel = glm::vec3(0.0, 1.0, 0.0);
        let entity = t::add_entity_with_pos_and_vel(&mut w, pos, vel);
    }
}
