//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use specs::{Join, Read, ReadStorage, System, WriteStorage};

use feather_core::world::ChunkMap;

use crate::entity::{EntityType, PositionComponent, VelocityComponent};
use crate::physics::{
    bbox_front, block_impacted_by_ray, blocks_intersecting_bbox, Side, BoundingBoxComponent,
};

/// System for updating all entities' positions and velocities
/// each tick.
pub struct EntityPhysicsSystem;

impl<'a> System<'a> for EntityPhysicsSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, VelocityComponent>,
        ReadStorage<'a, BoundingBoxComponent>,
        ReadStorage<'a, EntityType>,
        Read<'a, ChunkMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut velocities, bounding_boxes, types, chunk_map) = data;

        // Go through entities and update their positions according
        // to their velocities.

        // Unfortunately, we are currently not able to parallel
        // join over the position storage due to slide-rs/specs#541.
        // When this issue is resolved, the join below should be switched to a parallel
        // join.

        // A restricted storage is used for `velocity` so as to avoid
        // triggering a velocity update event when it is not actually
        // modified.
        for (position, mut restrict_velocity, bounding_box, ty) in (
            &mut positions,
            &mut velocities.restrict_mut(),
            &bounding_boxes,
            &types,
        )
            .join()
        {
            let mut velocity = restrict_velocity.get_unchecked().clone();

            let mut pending_position = position.current + velocity.0;

            // Check for blocks around the bbox and apply offset
            // to position to stop the bbox from intersecting blocks.
            let intersect =
                blocks_intersecting_bbox(&chunk_map, position.current, bounding_box);
            intersect.apply_to(&mut pending_position);

            // Check for blocks along path between old position and pending position.
            // This prevents entities from flying through blocks when their
            // velocity is sufficiently high.

            // The origin is the "leading point" of the bounding box:
            // the point at the front.
            let origin = (position.current + bbox_front(&bounding_box.0, velocity.0)).into();
            let direction = (pending_position - position.previous).into();
            let distance_squared = pending_position.distance_squared(position.previous);

            if let Some(impacted) =
                block_impacted_by_ray(&chunk_map, origin, direction, distance_squared as f32)
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

            // Apply drag and gravity.
            // TODO account for liquid
            let gravity = gravitational_acceleration(*ty);
            let drag = drag_force(*ty);

            let slip_multiplier = 0.6;
            if pending_position.on_ground {
                velocity.0.x *= slip_multiplier;
                velocity.0.z *= slip_multiplier;
            } else {
                velocity.0.y = drag * velocity.0.y + gravity;
                velocity.0.x *= drag;
                velocity.0.z *= drag;
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
    if ty.is_living() || ty.is_item() {
        0.98
    } else {
        0.0
    }
}
