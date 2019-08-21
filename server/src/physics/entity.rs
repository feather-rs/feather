//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use specs::{Join, Read, ReadStorage, System, WriteStorage};

use feather_core::world::ChunkMap;

use crate::entity::{EntityType, PositionComponent, VelocityComponent};
use crate::physics::{block_impacted_by_ray, BlockFace, BoundingBoxComponent};
use feather_core::Block;

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
        for (position, velocity, bounding_box, ty) in
            (&mut positions, &mut velocities, &bounding_boxes, &types).join()
        {
            let mut pending_position = position.current + velocity.0;

            // Check for blocks along path between old position and pending position.
            // This prevents entities from flying through blocks when their
            // velocity is sufficiently high.
            let origin = position.previous.into();
            let direction = (pending_position - position.previous).into();
            let distance_squared = pending_position.distance_squared(position.previous);

            if let Some(impacted) =
                block_impacted_by_ray(&chunk_map, origin, direction, distance_squared as f32)
            {
                // Set velocities along correct axis to 0 and then set position
                // to just before the bbox would have impacted the block.
                let face = impacted.face;
                let block = impacted.block;

                if face.contains(BlockFace::EAST) || face.contains(BlockFace::WEST) {
                    velocity.x = 0.0;
                    pending_position.x =
                        f64::from(block.x) + bounding_box.size().x * face.as_vector().x;
                }
                if face.contains(BlockFace::NORTH) || face.contains(BlockFace::SOUTH) {
                    velocity.z = 0.0;
                    pending_position.z =
                        f64::from(block.z) + bounding_box.size().z * face.as_vector().z;
                }
                if face.contains(BlockFace::TOP) || face.contains(BlockFace::BOTTOM) {
                    velocity.y = 0.0;
                    pending_position.y =
                        f64::from(block.y) + bounding_box.size().y * face.as_vector().y;
                }
                if face.contains(BlockFace::TOP) {
                    pending_position.on_ground = true;
                }
            }

            // TODO check if blocks intersect with bounding box

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

            // Check for block below the bbox.
            let search_distance = bounding_box.size().y;

            let block_below =
                (pending_position - glm::vec3(0.0, search_distance as f32, 0.0)).block_pos();
            let block_below = chunk_map.block_at(block_below);
            if let Some(block) = block_below {
                if block != Block::Air {
                    pending_position.on_ground = true;
                    velocity.0.y = 0.0;
                } else {
                    pending_position.on_ground = false;
                }
            }

            // Set new position.
            // A move event is triggered through FlaggedStorage.
            position.current = pending_position;
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
