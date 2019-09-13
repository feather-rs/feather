//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::entity::{EntityDestroyEvent, EntityType, PositionComponent, VelocityComponent};
use crate::physics::{block_impacted_by_ray, blocks_intersecting_bbox, BoundingBoxComponent, Side};
use feather_core::world::ChunkMap;
use feather_core::Position;
use feather_core::{Block, BlockExt};
use shrev::EventChannel;

/// System for updating all entities' positions and velocities
/// each tick.
pub struct EntityPhysicsSystem;

impl<'a> System<'a> for EntityPhysicsSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, VelocityComponent>,
        ReadStorage<'a, BoundingBoxComponent>,
        ReadStorage<'a, EntityType>,
        Write<'a, EventChannel<EntityDestroyEvent>>,
        Read<'a, ChunkMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut positions,
            mut velocities,
            bounding_boxes,
            types,
            mut entity_destroy_events,
            chunk_map,
            entities,
        ) = data;

        // Go through entities and update their positions according
        // to their velocities.

        // Unfortunately, we are currently not able to parallel
        // join over the position storage due to slide-rs/specs#541.
        // When this issue is resolved, the join below should be switched to a parallel
        // join.

        // A restricted storage is used for `velocity` so as to avoid
        // triggering a velocity update event when it is not actually
        // modified.
        for (position, mut restrict_velocity, bounding_box, ty, entity) in (
            &mut positions,
            &mut velocities.restrict_mut(),
            &bounding_boxes,
            &types,
            &entities,
        )
            .join()
        {
            let mut velocity = restrict_velocity.get_unchecked().clone();

            let mut pending_position = position.current + velocity.0;

            // Check for blocks along path between old position and pending position.
            // This prevents entities from flying through blocks when their
            // velocity is sufficiently high.
            let origin = position.current.into();
            let direction = (pending_position - position.current).into();
            let distance_squared = pending_position.distance_squared(position.current);

            if let Some(impacted) =
                block_impacted_by_ray(&chunk_map, origin, direction, distance_squared)
            {
                // Set velocities along correct axis to 0 and then set position
                // to just before the bbox would have impacted the block.
                let face = impacted.face;
                let impact = impacted.pos;

                if face.contains(Side::EAST) || face.contains(Side::WEST) {
                    velocity.x = 0.0;
                    pending_position.x = impact.x + bounding_box.size().x * face.as_vector().x;
                }
                if face.contains(Side::NORTH) || face.contains(Side::SOUTH) {
                    velocity.z = 0.0;
                    pending_position.z = impact.z + bounding_box.size().z * face.as_vector().z;
                }
                if face.contains(Side::TOP) || face.contains(Side::BOTTOM) {
                    velocity.y = 0.0;
                    pending_position.y = impact.y + bounding_box.size().y * face.as_vector().y;
                }
                if face.contains(Side::TOP) {
                    pending_position.on_ground = true;
                }
            }

            // Check for blocks around the bbox and apply offset
            // to position to stop the bbox from intersecting blocks.
            let intersect = blocks_intersecting_bbox(
                &chunk_map,
                position.current,
                pending_position,
                bounding_box,
            );
            intersect.apply_to(&mut pending_position);

            if intersect.x_affected() {
                velocity.x = 0.0;
            }

            if intersect.y_affected() {
                velocity.y = 0.0;
            }

            if intersect.z_affected() {
                velocity.z = 0.0;
            }

            // Delete entity if it has gone into unloaded chunks.
            let block_at_pos = match chunk_map.block_at(pending_position.block_pos()) {
                Some(block) => block,
                None => {
                    // Delete entity.
                    let event = EntityDestroyEvent { entity };
                    entity_destroy_events.single_write(event);

                    entities.delete(entity).unwrap();
                    continue;
                }
            };

            // Set on ground status
            pending_position.on_ground = match chunk_map.block_at(
                position!(
                    pending_position.x,
                    pending_position.y - bounding_box.size().y / 2.0 - 0.01,
                    pending_position.z
                )
                .block_pos(),
            ) {
                Some(block) => block.is_solid(),
                None => false,
            };

            // Apply drag and gravity.
            let gravity = gravitational_acceleration(*ty);
            let drag = drag_force(*ty);

            // In water and lava, gravity is four times less, and velocity is multiplied by a special drag force.
            let liquid_drag = 0.8;
            match block_at_pos {
                Block::Water(_) => {
                    velocity.0 *= liquid_drag;
                    velocity.0.y += gravity / 4.0;
                }
                Block::Lava(_) => {
                    velocity.0 *= liquid_drag - 0.3;
                    velocity.0.y += gravity / 4.0;
                }
                _ => {
                    let slip_multiplier = slip_multiplier(*ty);
                    if pending_position.on_ground {
                        velocity.0.x *= slip_multiplier;
                        velocity.0.z *= slip_multiplier;
                    } else {
                        velocity.0.y = drag * velocity.0.y + gravity;
                        velocity.0.x *= drag;
                        velocity.0.z *= drag;
                    }
                }
            }

            // Set new position.
            // A move event is triggered through FlaggedStorage.
            position.current = pending_position;

            // Update velocity, if it changed.
            if velocity != *restrict_velocity.get_unchecked() {
                *restrict_velocity.get_mut_unchecked() = velocity;
            }
        }
    }
}

fn slip_multiplier(ty: EntityType) -> f64 {
    if ty.is_arrow() {
        0.0
    } else {
        0.6
    }
}

/// Retrieves the gravitational acceleration in blocks per tick squared
/// for a given entity type.
///
/// This information was fetched from
/// [the Minecraft wiki](https://minecraft.gamepedia.com/Entity#Motion_of_entities).
fn gravitational_acceleration(ty: EntityType) -> f64 {
    if ty.is_living() {
        -0.08
    } else if ty.is_item() || ty == EntityType::FallingBlock {
        -0.04
    } else if ty.is_arrow() {
        -0.05
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
fn drag_force(ty: EntityType) -> f64 {
    if ty.is_living() || ty.is_item() || ty == EntityType::FallingBlock {
        0.98
    } else if ty.is_arrow() {
        0.99
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use specs::WorldExt;

    #[test]
    fn test_physics_basic() {
        let (mut w, mut d) = t::builder().with(EntityPhysicsSystem, "").build();

        t::populate_with_air(&mut w);

        let item = t::add_entity_with_pos_and_vel(
            &mut w,
            EntityType::Item,
            position!(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0),
            false,
        );

        let bbox = crate::physics::component::bbox(0.25, 0.25);
        w.write_component::<BoundingBoxComponent>()
            .insert(item, BoundingBoxComponent(bbox))
            .unwrap();

        d.dispatch(&w);
        w.maintain();

        let pos = t::entity_pos(&w, item);
        let vel = t::entity_vel(&w, item).unwrap();

        assert_pos_eq!(pos, position!(0.0, 1.0, 0.0));
        assert_float_eq!(vel.x, 0.0);
        assert_float_eq!(vel.y, 0.94);
        assert_float_eq!(vel.z, 0.0);
    }

    #[test]
    fn test_unloaded_chunk() {
        let (mut w, mut d) = t::builder().with(EntityPhysicsSystem, "").build();

        let entity = t::add_entity_with_pos(
            &mut w,
            EntityType::Item,
            position!(1000.0, 100.0, 1000.0),
            false,
        );

        let bbox = crate::physics::component::bbox(0.25, 0.25);
        w.write_component::<BoundingBoxComponent>()
            .insert(entity, BoundingBoxComponent(bbox))
            .unwrap();

        d.dispatch(&w);
        w.maintain();

        t::assert_removed(&w, entity);
    }
}
