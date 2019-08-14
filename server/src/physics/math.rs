//! A bunch of math-related functions for use with
//! the physics system.

use feather_core::world::block::Block;
use feather_core::world::{BlockPosition, ChunkMap, Position};
use glm::{vec3, Vec3};
use std::f32::INFINITY;

/// Finds the first block impacted by the given ray.
///
/// Traces up to `max_distance` before returning `None`
/// if no block was found.
pub fn block_impacted_by_ray(
    chunk_map: &ChunkMap,
    origin: Vec3,
    ray: Vec3,
    max_distance: f32,
) -> Option<BlockPosition> {
    assert_ne!(ray, vec3(0.0, 0.0, 0.0));

    // Go along path of ray and find all points
    // where one or more coordinates are integers.
    // Any position with an integer component
    // is a block boundary, which means a block
    // could be found at the position.
    //
    // This algorithm is based on "A Fast Voxel Traversal Algorithm for Ray Tracing"
    // by John Amanatides and Andrew Woo and has been adapted
    // to our purposes.

    let max_distance_squared = max_distance * max_distance;

    let direction = ray.normalize();

    let mut dist_traveled = glm::vec3(0.0f32, 0.0, 0.0);

    let mut step = glm::vec3(0, 0, 0);
    let mut delta = glm::vec3(INFINITY, INFINITY, INFINITY);
    let mut next = glm::vec3(INFINITY, INFINITY, INFINITY);

    if direction.x > 0.0 {
        step.x = 1;
        delta.x = 1.0 / direction.x;
        next.x = ((origin.x + 1.0).floor() - origin.x) / direction.x; // Brings X position to next integer
    } else if direction.x < 0.0 {
        step.x = -1;
        delta.x = (1.0 / direction.x).abs();
        next.x = ((origin.x - (origin.x - 1.0).ceil()) / direction.x).abs();
    }

    if direction.y > 0.0 {
        step.y = 1;
        delta.y = 1.0 / direction.y;
        next.y = ((origin.y + 1.0).floor() - origin.y) / direction.y;
    } else if direction.y < 0.0 {
        step.y = -1;
        delta.y = (1.0 / direction.y).abs();
        next.y = ((origin.y - (origin.y - 1.0).ceil()) / direction.y).abs();
    }

    if direction.z > 0.0 {
        step.z = 1;
        delta.z = 1.0 / direction.z;
        next.z = ((origin.z + 1.0).floor() - origin.z) / direction.z;
    } else if direction.z < 0.0 {
        step.z = -1;
        delta.z = (1.0 / direction.z).abs();
        next.z = ((origin.z - (origin.z - 1.0).ceil()) / direction.z).abs();
    }

    let mut current_pos = Position::from(origin).block_pos();

    while dist_traveled.magnitude_squared() < max_distance_squared {
        if next.x < next.y {
            if next.x < next.z {
                next.x += delta.x;
                current_pos.x += step.x;
                dist_traveled.x += 1.0;
            } else {
                next.z += delta.z;
                current_pos.z += step.z;
                dist_traveled.z += 1.0;
            }
        } else if next.y < next.z {
            next.y += delta.y;
            current_pos.y += step.y;
            dist_traveled.y += 1.0;
        } else {
            next.z += delta.z;
            current_pos.z += step.z;
            dist_traveled.z += 1.0;
        }

        if let Some(block) = chunk_map.block_at(current_pos) {
            if block != Block::Air {
                return Some(current_pos);
            }
        } else {
            // Traveled outside loaded chunks - no blocks found
            return None;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_core::world::chunk::Chunk;
    use feather_core::world::ChunkPosition;

    #[test]
    fn test_block_impacted_by_ray() {
        let mut map = chunk_map();

        assert_eq!(
            block_impacted_by_ray(&map, vec3(0.0, 65.0, 0.0), vec3(0.0, -1.0, 0.0), 5.0),
            Some(BlockPosition::new(0, 64, 0))
        );

        assert_eq!(
            block_impacted_by_ray(&map, vec3(0.0, 65.0, 0.0), vec3(0.0, 1.0, 0.0), 256.0,),
            None
        );

        assert_eq!(
            block_impacted_by_ray(&map, vec3(0.0, 70.0, 0.0), vec3(0.0, -1.0, 0.0), 5.0,),
            None
        );

        map.set_block_at(BlockPosition::new(1, 65, 1), Block::Stone)
            .unwrap();

        assert_eq!(
            block_impacted_by_ray(&map, vec3(0.0, 66.0, 0.0), vec3(1.0, -1.0, 1.0), 5.0),
            Some(BlockPosition::new(1, 65, 1))
        );
    }

    fn chunk_map() -> ChunkMap {
        let mut map = ChunkMap::new();

        for x in -2..=2 {
            for z in -2..=2 {
                let pos = ChunkPosition::new(x, z);
                let mut chunk = Chunk::new(pos);

                for x in 0..16 {
                    for y in 0..=64 {
                        for z in 0..16 {
                            chunk.set_block_at(x, y, z, Block::Stone);
                        }
                    }
                }
                map.set_chunk_at(pos, chunk);
            }
        }

        map
    }
}
