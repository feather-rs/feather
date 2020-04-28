//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use crate::{block_impacted_by_ray, blocks_intersecting_bbox, Side};
use feather_core::blocks::BlockKind;
use feather_core::position;
use feather_core::util::Position;
use feather_server_types::{AABBExt, EntityLandEvent, Game, Physics, Velocity};
use fecs::{IntoQuery, Read, World, Write};
use parking_lot::Mutex;

/// System for updating all entities' positions and velocities
/// each tick.
#[fecs::system]
pub fn entity_physics(game: &mut Game, world: &mut World) {
    // Go through entities and update their positions according
    // to their velocities.
    let land_events = Mutex::new(vec![]);

    let query = <(Write<Position>, Write<Velocity>, Read<Physics>)>::query();
    query.par_entities_for_each_mut(
        world.inner_mut(),
        |(entity, (mut position, mut velocity, physics))| {
            let mut pending_position = *position + velocity.0;

            // Check for blocks along path between old position and pending position.
            // This prevents entities from flying through blocks when their
            // velocity is sufficiently high.
            let origin = (*position).into();
            let direction = (pending_position - *position).into();
            let distance_squared = pending_position.distance_squared_to(*position);

            if let Some(impacted) = block_impacted_by_ray(game, origin, direction, distance_squared)
            {
                // Set velocities along correct axis to 0 and then set position
                // to just before the bbox would have impacted the block.
                let face = impacted.face;
                let impact = impacted.pos;

                if face.contains(Side::EAST) || face.contains(Side::WEST) {
                    velocity.0.x = 0.0;
                    pending_position.x = impact.x + physics.bbox.size().x * face.as_vector().x;
                }
                if face.contains(Side::NORTH) || face.contains(Side::SOUTH) {
                    velocity.0.z = 0.0;
                    pending_position.z = impact.z + physics.bbox.size().z * face.as_vector().z;
                }
                if face.contains(Side::TOP) || face.contains(Side::BOTTOM) {
                    velocity.0.y = 0.0;
                    pending_position.y = impact.y + physics.bbox.size().y * face.as_vector().y;
                }
                if face.contains(Side::TOP) {
                    pending_position.on_ground = true;
                }
            }

            // Check for blocks around the bbox and apply offset
            // to position to stop the bbox from intersecting blocks.
            let intersect =
                blocks_intersecting_bbox(game, *position, pending_position, &physics.bbox);
            intersect.apply_to(&mut pending_position);

            if intersect.x_affected() {
                velocity.0.x = 0.0;
            }

            if intersect.y_affected() {
                velocity.0.y = 0.0;
            }

            if intersect.z_affected() {
                velocity.0.z = 0.0;
            }

            // Delete entity if it has gone into unloaded chunks.
            let block_at_pos = match game.block_at(pending_position.block()) {
                Some(block) => block,
                None => {
                    // TODO: delete entity
                    return;
                }
            };

            // Set on ground status.
            pending_position.on_ground = match game.block_at(
                position!(
                    pending_position.x,
                    pending_position.y - physics.bbox.size().y / 2.0 - 0.01,
                    pending_position.z
                )
                .block(),
            ) {
                Some(block) => block.is_solid(),
                None => false,
            };
            if pending_position.on_ground && !position.on_ground {
                land_events.lock().push(EntityLandEvent {
                    entity,
                    pos: pending_position,
                });
            }

            // Apply drag and gravity.

            // In water and lava, gravity is four times less, and velocity is multiplied by a special drag force.
            let liquid_drag = 0.8;
            match block_at_pos.kind() {
                BlockKind::Water => {
                    velocity.0 *= liquid_drag;
                    velocity.0.y += physics.gravity / 4.0;
                }
                BlockKind::Lava => {
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
            *position = pending_position;
        },
    );

    // Trigger land events.
    for event in land_events.into_inner() {
        game.handle(world, event);
    }
}
