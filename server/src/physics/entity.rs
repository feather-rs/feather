//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use specs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::entity::{EntityDestroyEvent, PositionComponent, VelocityComponent};
use crate::physics::{
    block_impacted_by_ray, blocks_intersecting_bbox, AABBExt, PhysicsComponent, Side,
};
use feather_core::world::ChunkMap;
use feather_core::Position;
use feather_core::{Block, BlockExt};
use shrev::EventChannel;

#[derive(Debug, Clone)]
pub struct EntityPhysicsLandEvent {
    pub entity: Entity,
    pub pos: Position,
}

/// System for updating all entities' positions and velocities
/// each tick.
pub struct EntityPhysicsSystem;

impl<'a> System<'a> for EntityPhysicsSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, VelocityComponent>,
        ReadStorage<'a, PhysicsComponent>,
        Write<'a, EventChannel<EntityDestroyEvent>>,
        Write<'a, EventChannel<EntityPhysicsLandEvent>>,
        Read<'a, ChunkMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut positions,
            mut velocities,
            physics,
            mut entity_destroy_events,
            mut entity_land_events,
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
        for (position, mut restrict_velocity, physics, entity) in (
            &mut positions,
            &mut velocities.restrict_mut(),
            &physics,
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
                    pending_position.x = impact.x + physics.bbox.size().x * face.as_vector().x;
                }
                if face.contains(Side::NORTH) || face.contains(Side::SOUTH) {
                    velocity.z = 0.0;
                    pending_position.z = impact.z + physics.bbox.size().z * face.as_vector().z;
                }
                if face.contains(Side::TOP) || face.contains(Side::BOTTOM) {
                    velocity.y = 0.0;
                    pending_position.y = impact.y + physics.bbox.size().y * face.as_vector().y;
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
                &physics.bbox,
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
                    pending_position.y - physics.bbox.size().y / 2.0 - 0.01,
                    pending_position.z
                )
                .block_pos(),
            ) {
                Some(block) => block.is_solid(),
                None => false,
            };
            if pending_position.on_ground && !position.current.on_ground {
                entity_land_events.single_write(EntityPhysicsLandEvent {
                    entity,
                    pos: pending_position,
                });
            }

            // Apply drag and gravity.

            // In water and lava, gravity is four times less, and velocity is multiplied by a special drag force.
            let liquid_drag = 0.8;
            match block_at_pos {
                Block::Water(_) => {
                    velocity.0 *= liquid_drag;
                    velocity.0.y += physics.gravity / 4.0;
                }
                Block::Lava(_) => {
                    velocity.0 *= liquid_drag - 0.3;
                    velocity.0.y += physics.gravity / 4.0;
                }
                _ => {
                    let slip_multiplier = physics.slip_multiplier;
                    if pending_position.on_ground {
                        velocity.0.x *= slip_multiplier;
                        velocity.0.z *= slip_multiplier;
                    } else {
                        velocity.0.y = physics.drag * velocity.0.y + physics.gravity;
                        velocity.0.x *= physics.drag;
                        velocity.0.z *= physics.drag;
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

        let bbox = crate::physics::component::bbox(0.25, 0.25, 0.25);
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

        let bbox = crate::physics::component::bbox(0.25, 0.25, 0.25);
        w.write_component::<BoundingBoxComponent>()
            .insert(entity, BoundingBoxComponent(bbox))
            .unwrap();

        d.dispatch(&w);
        w.maintain();

        t::assert_removed(&w, entity);
    }
}
