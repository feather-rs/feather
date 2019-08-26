//! A bunch of math-related functions for use with
//! the physics system.

use crate::entity::{ChunkEntities, PositionComponent};
use crate::physics::BoundingBoxComponent;
use feather_core::world::{BlockPosition, ChunkMap, Position};
use feather_core::{BlockExt, ChunkPosition};
use glm::{vec3, DVec3, Vec3};
use nalgebra::{Isometry3, Point3};
use ncollide3d::bounding_volume::AABB;
use ncollide3d::query::{Ray, RayCast};
use ncollide3d::shape::Cuboid;
use smallvec::SmallVec;
use specs::storage::GenericReadStorage;
use specs::Entity;
use std::f32::INFINITY;

// TODO is a bitflag really the most
// idiomatic way to do this?
bitflags! {
    /// A face of a block.
    ///
    /// * East is on the positive X side.
    /// * West is on the negative X side.
    /// * North is on the positive Z side.
    /// * South is on the positive Z side.
    /// * Top is on the positive Y side.
    /// * Bottom is on the negative Y side.
    pub struct BlockFace: u8 {
        const EAST = 0x01;
        const WEST = 0x02;
        const NORTH = 0x04;
        const SOUTH = 0x08;
        const TOP = 0x10;
        const BOTTOM = 0x20;
        const NONE = 0x40;
    }
}

impl BlockFace {
    /// Returns a vector with coordinates set to 1.0
    /// where the face is toward the positive axis
    /// and to -1.0 where the face is toward the negative
    /// axis.
    pub fn as_vector(self) -> DVec3 {
        let mut vector = glm::vec3(0.0, 0.0, 0.0);

        if self.contains(BlockFace::EAST) {
            vector.x = 1.0;
        } else if self.contains(BlockFace::WEST) {
            vector.x = -1.0;
        }

        if self.contains(BlockFace::NORTH) {
            vector.z = 1.0;
        } else if self.contains(BlockFace::SOUTH) {
            vector.z = -1.0;
        }

        if self.contains(BlockFace::TOP) {
            vector.y = 1.0;
        } else if self.contains(BlockFace::BOTTOM) {
            vector.y = -1.0;
        }

        vector
    }
}

/// The position at which a ray impacts a block.
#[derive(Debug, Clone, PartialEq)]
pub struct RayImpact {
    /// The position of the block which was impacted.
    pub block: BlockPosition,
    /// The exact position, in world coordinates, at
    /// which the ray met the block.
    pub pos: Position,
    /// The face(s) of the block where the ray impacted.
    pub face: BlockFace,
}

/// Finds the first block impacted by the given ray.
///
/// Traces up to `max_distance` before returning `None`
/// if no block was found.
pub fn block_impacted_by_ray(
    chunk_map: &ChunkMap,
    origin: Vec3,
    ray: Vec3,
    max_distance_squared: f32,
) -> Option<RayImpact> {
    if ray == vec3(0.0, 0.0, 0.0) {
        return None;
    }

    // Go along path of ray and find all points
    // where one or more coordinates are integers.
    // Any position with an integer component
    // is a block boundary, which means a block
    // could be found at the position.
    //
    // This algorithm is based on "A Fast Voxel Traversal Algorithm for Ray Tracing"
    // by John Amanatides and Andrew Woo and has been adapted
    // to our purposes.

    let direction = ray.normalize();

    let mut dist_traveled = glm::vec3(0.0f32, 0.0, 0.0);

    let mut step = glm::vec3(0, 0, 0);
    let mut delta = glm::vec3(INFINITY, INFINITY, INFINITY);
    let mut next = glm::vec3(INFINITY, INFINITY, INFINITY);

    // TODO this implementation does not properly
    // handle when a ray hits multiple faces.
    // In practice, this should not be an issue,
    // but it may causes subtle issues in the future.
    let mut face = BlockFace::NONE;

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
        if let Some(block) = chunk_map.block_at(current_pos) {
            if block.is_solid() {
                // Calculate world-space position of
                // impact using `ncollide`.
                let ray = Ray::new(Point3::from(origin), direction);
                let shape = block_shape();
                let isometry = block_isometry(current_pos);

                let impact = shape
                    .toi_and_normal_with_ray(&isometry, &ray, true)
                    .unwrap(); // Unwrap is safe because we know the ray intersects the block
                let pos = Position::from(origin + impact.toi * direction);

                return Some(RayImpact {
                    block: current_pos,
                    pos,
                    face,
                });
            }
        } else {
            // Traveled outside loaded chunks - no blocks found
            return None;
        }

        if next.x < next.y {
            if next.x < next.z {
                next.x += delta.x;
                current_pos.x += step.x;
                dist_traveled.x += 1.0;
                face = if step.x == 1 {
                    BlockFace::WEST
                } else {
                    BlockFace::EAST
                }
            } else {
                next.z += delta.z;
                current_pos.z += step.z;
                dist_traveled.z += 1.0;
                face = if step.z == 1 {
                    BlockFace::SOUTH
                } else {
                    BlockFace::NORTH
                }
            }
        } else if next.y < next.z {
            next.y += delta.y;
            current_pos.y += step.y;
            dist_traveled.y += 1.0;
            face = if step.y == 1 {
                BlockFace::BOTTOM
            } else {
                BlockFace::TOP
            }
        } else {
            next.z += delta.z;
            current_pos.z += step.z;
            dist_traveled.z += 1.0;
            face = if step.z == 1 {
                BlockFace::SOUTH
            } else {
                BlockFace::NORTH
            }
        }
    }

    None
}

/// Returns all entities within the given distance of the given
/// position.
///
/// # Panics
/// Panics if either coordinate of the radius is negative.
pub fn nearby_entities<S>(
    chunk_entities: &ChunkEntities,
    positions: &S,
    pos: Position,
    radius: DVec3,
) -> SmallVec<[Entity; 4]>
where
    S: GenericReadStorage<Component = PositionComponent>,
{
    assert!(radius.x >= 0.0);
    assert!(radius.y >= 0.0);
    assert!(radius.z >= 0.0);

    let mut result = smallvec![];

    for chunk in chunks_within_distance(pos, radius) {
        let entities = chunk_entities.entities_in_chunk(chunk);
        entities
            .iter()
            .copied()
            .filter(|e| {
                let epos = positions.get(*e);
                if let Some(epos) = epos {
                    let epos = epos.current;
                    (epos.x - pos.x).abs() <= radius.x
                        && (epos.y - pos.y).abs() <= radius.y
                        && (epos.z - pos.z).abs() <= radius.z
                } else {
                    false
                }
            })
            .for_each(|e| result.push(e));
    }

    result
}

/// Returns a vector containing `1.0` for each axis where
/// there are no blocks intersecting the bounding box and `0.0` for
/// where there are.
///
/// NOTE: This implementation only covers the most basic cases.
/// It does not correctly work when the bounding box is larger
/// than one block in any length.
pub fn blocks_intersecting_bbox(
    chunk_map: &ChunkMap,
    pos: Position,
    bbox: &BoundingBoxComponent,
) -> Vec3 {
    let bbox_size = bbox.size();

    let mut result = vec3(1.0, 1.0, 1.0);

    let offsets = [
        vec3(bbox_size.x as f32, 0.0, 0.0),
        vec3(-bbox_size.x as f32, 0.0, 0.0),
        vec3(0.0, bbox_size.y as f32, 0.0),
        vec3(0.0, -bbox_size.y as f32, 0.0),
        vec3(0.0, 0.0, bbox_size.z as f32),
        vec3(0.0, 0.0, -bbox_size.z as f32),
    ];
    let masks = [
        vec3(0.0f32, 1.0, 1.0),
        vec3(0.0, 1.0, 1.0),
        vec3(1.0, 0.0, 1.0),
        vec3(1.0, 0.0, 1.0),
        vec3(1.0, 1.0, 0.0),
        vec3(1.0, 1.0, 0.0),
    ];

    for (offset, mask) in offsets.iter().zip(masks.iter()) {
        let block_pos = (pos + *offset).block_pos();

        if let Some(block) = chunk_map.block_at(block_pos) {
            if block.is_solid() {
                result.x *= mask.x;
                result.y *= mask.y;
                result.z *= mask.z;
            }
        }
    }

    result
}

/// Returns an `ncollide` `Cuboid` corresponding to a block.
pub fn block_shape() -> Cuboid<f32> {
    Cuboid::new(vec3(0.5, 0.5, 0.5))
}

/// Returns an `Isometry` representing a block's translation.
pub fn block_isometry(pos: BlockPosition) -> Isometry3<f32> {
    Isometry3::new(
        vec3(pos.x as f32 + 0.5, pos.y as f32 + 0.5, pos.z as f32 + 0.5),
        vec3(0.0, 0.0, 0.0),
    )
}

/// Finds all chunks within a given distance (in blocks)
/// of a position.
///
/// The Y coordinate of `distance` is ignored.
fn chunks_within_distance(mut pos: Position, mut distance: DVec3) -> SmallVec<[ChunkPosition; 9]> {
    assert!(distance.x >= 0.0);
    assert!(distance.z >= 0.0);

    let mut result = smallvec![];

    let mut x_len = 0;
    let mut z_len = 0;

    let center_chunk_pos = pos.chunk_pos();

    loop {
        let needed = ((pos.x + 16.0) / 16.0).floor() * 16.0 - pos.x;
        if needed > distance.x {
            break;
        }

        distance.x -= needed;
        pos.x += needed;
        x_len += 1;
    }

    loop {
        let needed = ((pos.z + 16.0) / 16.0).floor() * 16.0 - pos.z;
        if needed > distance.z {
            break;
        }

        distance.z -= needed;
        pos.z += needed;
        z_len += 1;
    }

    for x in -x_len..=x_len {
        for z in -z_len..=z_len {
            result.push(ChunkPosition::new(
                x + center_chunk_pos.x,
                z + center_chunk_pos.z,
            ));
        }
    }

    result
}

/// Returns a point at the "front" of the bounding
/// box when it is traveling in the given direction.
///
/// The direction vector is expected to be normalized.
pub fn bbox_front(bbox: &AABB<f64>, direction: Vec3) -> Position {
    let direction = DVec3::new(
        f64::from(direction.x),
        f64::from(direction.y),
        f64::from(direction.z),
    );
    let cuboid = bbox_to_cuboid(bbox);

    let origin = Point3::from([0.0, 0.0, 0.0]);

    let ray = Ray::new(origin, direction);

    let toi = cuboid
        .toi_with_ray(
            &Isometry3::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0)),
            &ray,
            false,
        )
        .unwrap();

    Position::from(direction * toi)
}

/// Converts an axis-aligned bounding box to a cuboid shape.
pub fn bbox_to_cuboid(bbox: &AABB<f64>) -> Cuboid<f64> {
    let lengths = bbox.maxs() - bbox.mins();

    let half_lengths = vec3(lengths.x / 2.0, lengths.y / 2.0, lengths.z / 2.0);

    Cuboid::new(half_lengths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityType;
    use crate::testframework as t;
    use feather_core::world::chunk::Chunk;
    use feather_core::world::ChunkPosition;
    use feather_core::Block;
    use specs::WorldExt;
    use std::collections::HashSet;

    #[test]
    fn test_block_impacted_by_ray() {
        let mut map = chunk_map();

        assert_eq!(
            block_impacted_by_ray(&map, vec3(0.0, 65.0, 0.0), vec3(0.0, -1.0, 0.0), 5.0),
            Some(RayImpact {
                block: BlockPosition::new(0, 64, 0),
                pos: position!(0.0, 65.0, 0.0),
                face: BlockFace::TOP,
            })
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
            Some(RayImpact {
                block: BlockPosition::new(1, 65, 1),
                pos: position!(1.0, 65.0, 1.0),
                face: BlockFace::WEST, // This should be three faces—see the TODO above
            })
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

    #[test]
    fn test_nearby_entities() {
        let (mut w, mut d) = t::init_world();

        t::populate_with_air(&mut w); // Prevents entities from getting despawned for being outside loaded chunks

        let e1 = t::add_entity_with_pos(&mut w, EntityType::Player, position!(0.0, 0.0, 0.0), true);
        let e2 = t::add_entity_with_pos(
            &mut w,
            EntityType::Player,
            position!(-100.0, 0.0, 50.0),
            true,
        );
        let e3 = t::add_entity_with_pos(
            &mut w,
            EntityType::Player,
            position!(100.0, 50.0, 50.0),
            true,
        );
        let e4 = t::add_entity_with_pos(
            &mut w,
            EntityType::Player,
            position!(100.0, 1.0, -50.0),
            true,
        );

        d.dispatch(&w);
        w.maintain();

        let entities = nearby_entities(
            &w.fetch(),
            &w.read_component(),
            position!(0.0, 0.0, 0.0),
            vec3(100.0, 1.0, 50.0),
        )
        .into_iter()
        .collect::<HashSet<_>>();

        assert_eq!(entities.len(), 3);

        assert!(entities.contains(&e1));
        assert!(entities.contains(&e2));
        assert!(!entities.contains(&e3));
        assert!(entities.contains(&e4));
    }

    #[test]
    fn test_chunks_within_distance_basic() {
        let pos = position!(0.0, 0.0, 0.0);
        let distance = vec3(16.0, 0.0, 16.0);

        let chunks = chunks_within_distance(pos, distance);

        dbg!(chunks.clone());

        let set = chunks.into_iter().collect::<HashSet<_>>();

        for x in -1..=1 {
            for z in -1..=1 {
                assert!(set.contains(&ChunkPosition::new(x, z)));
            }
        }

        assert_eq!(set.len(), 9);
    }

    #[test]
    fn test_chunks_within_distance_complex() {
        let pos = position!(32.0, 0.0, -32.0);

        let distance = vec3(32.0, 0.0, 31.0);

        let chunks = chunks_within_distance(pos, distance);

        dbg!(chunks.clone());
        assert_eq!(chunks.len(), 15);

        let set = chunks.into_iter().collect::<HashSet<_>>();

        for x in 0..=4 {
            for z in -3..=-1 {
                assert!(set.contains(&ChunkPosition::new(x, z)));
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_chunks_within_distance_negative_distance() {
        let pos = position!(16.0, 0.0, 16.0);
        let distance = vec3(-0.1, -50.0, 0.0);
        chunks_within_distance(pos, distance);
    }

    #[test]
    fn test_blocks_intersecting_bbox() {
        let chunk_map = chunk_map();

        assert_eq!(
            blocks_intersecting_bbox(
                &chunk_map,
                position!(0.0, 32.0, 0.0),
                &BoundingBoxComponent(AABB::new(
                    Point3::from(vec3(0.0, 0.0, 0.0)),
                    Point3::from(vec3(0.5, 0.5, 0.5))
                )),
            ),
            vec3(0.0, 0.0, 0.0)
        );

        assert_eq!(
            blocks_intersecting_bbox(
                &chunk_map,
                position!(0.0, 65.0, 0.0),
                &BoundingBoxComponent(AABB::new(
                    Point3::from(vec3(0.0, 0.0, 0.0)),
                    Point3::from(vec3(0.5, 1.5, 0.5))
                )),
            ),
            vec3(1.0, 0.0, 1.0)
        );
    }

    #[test]
    fn test_bbox_front() {
        let bbox = AABB::new(Point3::from([0.0, 0.0, 0.0]), Point3::from([1.0, 2.0, 3.0]));

        let direction = vec3(1.0, 0.0, 0.0);

        assert_eq!(bbox_front(&bbox, direction), position!(0.5, 0.0, 0.0),);
    }

    #[test]
    fn test_bbox_to_cuboid() {
        let bbox = AABB::new(Point3::from([0.0, 0.0, 0.0]), Point3::from([1.0, 2.0, 3.0]));

        let half_extents = *bbox_to_cuboid(&bbox).half_extents();

        assert_float_eq!(half_extents.x, 0.5);
        assert_float_eq!(half_extents.y, 1.0);
        assert_float_eq!(half_extents.z, 1.5);
    }
}
