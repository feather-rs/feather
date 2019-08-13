//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use crate::entity::{EntityComponent, EntityMoveEvent, EntityType, VelocityComponent};
use crate::physics::BoundingBoxComponent;
use feather_core::world::block::Block;
use feather_core::world::{ChunkMap, Position};
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
        ReadStorage<'a, BoundingBoxComponent>,
        ReadStorage<'a, EntityType>,
        Write<'a, EventChannel<EntityMoveEvent>>,
        Read<'a, ChunkMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut entity_comps, mut velocities, bboxes, types, mut move_events, chunk_map, entities) =
            data;

        // We can't use a vector for this, since it has to be mutated
        // across threads.
        // This channel will act as a vector of `EntityMoveEvents` to trigger.
        let (tx, rx) = crossbeam::unbounded();

        // Go through entities and update position and velocity in parallel.
        (&mut entity_comps, &mut velocities, &entities, &types)
            .par_join()
            .for_each(|(entity_comp, velocity, entity, ty)| {
                let bbox = bboxes.get(entity); // bounding box is optional
                let position = entity_comp.position;
                // First, add velocity to position, accounting for blocks
                // at the new position.
                let mut pending_position = position + velocity.0;
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

                        let pending_pos_y = position + glm::vec3(0.0, velocity.0.y, 0.0);
                        if chunk_map.block_at(pending_pos_y.block_pos()) != Some(Block::Air) {
                            velocity.0.y = 0.0;
                            entity_comp.position.on_ground = true;
                        }

                        let pending_pos_z = position + glm::vec3(0.0, 0.0, velocity.0.z);
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

                        // Friction.
                        if position.on_ground {
                            let slip_multiplier = 0.6;
                            velocity.0.x *= slip_multiplier;
                            velocity.0.y = 0.0;
                            velocity.0.z *= slip_multiplier;
                        } else {
                            velocity.0.x *= drag;
                            velocity.0.z *= drag;
                        }

                        // Set new position and trigger event.
                        let event = EntityMoveEvent {
                            entity,
                            old_pos: position,
                            new_pos: pending_position,
                        };

                        tx.send(event).unwrap();

                        set_entity_location(&mut pending_position, bbox, &chunk_map);

                        entity_comp.position = pending_position;
                    }
                } else {
                    // Chunk isn't loaded. Unload entity.
                    entities.delete(entity).unwrap();
                }
            });

        // Go through events and trigger them.
        while let Ok(event) = rx.try_recv() {
            move_events.single_write(event);
        }
    }
}

/// Performs a number of calculations when an entity's position
/// is updated.
fn set_entity_location(
    pos: &mut Position,
    bbox: Option<&BoundingBoxComponent>,
    chunk_map: &ChunkMap,
) {
    // Set entity to be on ground if a block is detected beneath it
    if !pos.on_ground {
        let mut offset = 0.0;

        if let Some(bbox) = bbox {
            offset = bbox.size().y;
        }

        let check_pos = *pos - position!(0.0, offset, 0.0);
        if let Some(block) = chunk_map.block_at(check_pos.block_pos()) {
            pos.on_ground = block != Block::Air;
        }
    }
}

/// Retrieves the gravitational acceleration in blocks per tick squared
/// for a given entity type.
///
/// This information was fetched from
/// [the Minecraft wiki](https://minecraft.gamepedia.com/Entity#Motion_of_entities).
fn gravitational_acceleration(ty: EntityType) -> f32 {
    if ty.is_living() {
        -0.08
    } else if ty.is_item() {
        -0.04
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
        0.96
    } else {
        0.0
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use crate::entity::{EntityMoveEvent, EntityType};
    use crate::testframework as t;
    use feather_core::world::block::Block;
    use feather_core::world::Position;
    use shrev::ReaderId;
    use specs::{Dispatcher, Entity, World, WorldExt};

    #[test]
    fn test_entity_physics_basic() {
        let (mut w, mut d, entity, mut reader) = setup();

        d.dispatch(&w);
        w.maintain();

        t::assert_not_removed(&w, entity);

        let pos = t::entity_pos(&w, entity);
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 1.0);
        assert_eq!(pos.z, 0.0);

        let vel = t::entity_vel(&w, entity).unwrap();
        assert_eq!(vel.x, 0.0);
        assert_eq!(vel.z, 0.0);

        let events = t::triggered_events::<EntityMoveEvent>(&w, &mut reader);
        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.entity, entity);
        assert_eq!(first.old_pos, position!(0.0, 0.0, 0.0));
        assert_eq!(first.new_pos, position!(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_entity_physics_block_collide() {
        let (mut w, mut d, entity, mut reader) = setup();

        t::set_block(0, 1, 0, Block::Stone, &w);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_removed(&w, entity);

        let pos = t::entity_pos(&w, entity);
        assert_eq!(pos, position!(0.0, 0.0, 0.0));

        let vel = t::entity_vel(&w, entity).unwrap();
        assert_eq!(vel, glm::vec3(0.0, 0.0, 0.0));

        let events = t::triggered_events(&w, &mut reader);
        assert!(events.is_empty());
    }

    #[test]
    fn test_entity_physics_unloaded_chunk() {
        let (mut w, mut d, entity, mut reader) = setup();

        t::set_entity_velocity(&w, entity, glm::vec3(10000.0, 0.0, 0.0));

        d.dispatch(&w);
        w.maintain();

        t::assert_removed(&w, entity);

        let events = t::triggered_events(&w, &mut reader);
        assert!(events.is_empty());
    }

    fn setup<'a, 'b>() -> (World, Dispatcher<'a, 'b>, Entity, ReaderId<EntityMoveEvent>) {
        let (mut w, d) = t::init_world();

        let old_pos = position!(0.0, 0.0, 0.0);
        let vel = glm::vec3(0.0, 1.0, 0.0);
        let entity = t::add_entity_with_pos_and_vel(&mut w, EntityType::Player, old_pos, vel);

        t::populate_with_air(&mut w);

        let reader = t::reader(&w);

        (w, d, entity, reader)
    }
}
