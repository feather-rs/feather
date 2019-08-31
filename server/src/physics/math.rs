//! A bunch of math-related functions for use with
//! the physics system.

use crate::entity::{ChunkEntities, PositionComponent};
use crate::physics::BoundingBoxComponent;
use feather_core::world::{BlockPosition, ChunkMap, Position};
use feather_core::{BlockExt, ChunkPosition};
use glm::{vec3, DVec3, Vec3};
use heapless::consts::U8;
use nalgebra::{Isometry3, Point3};
use ncollide3d::bounding_volume::AABB;
use ncollide3d::query;
use ncollide3d::query::{Ray, RayCast};
use ncollide3d::shape::Cuboid;
use smallvec::SmallVec;
use specs::storage::GenericReadStorage;
use specs::Entity;
use std::f32::INFINITY;

// TODO is a bitflag really the most
// idiomatic way to do this?
bitflags! {
    /// A side.
    ///
    /// * East is on the positive X side.
    /// * West is on the negative X side.
    /// * North is on the positive Z side.
    /// * South is on the positive Z side.
    /// * Top is on the positive Y side.
    /// * Bottom is on the negative Y side.
    pub struct Side: u8 {
        const EAST = 0x01;
        const WEST = 0x02;
        const NORTH = 0x04;
        const SOUTH = 0x08;
        const TOP = 0x10;
        const BOTTOM = 0x20;
        const NONE = 0x40;
    }
}

impl Side {
    /// Returns a vector with coordinates set to 1.0
    /// where the face is toward the positive axis
    /// and to -1.0 where the face is toward the negative
    /// axis.
    pub fn as_vector(self) -> DVec3 {
        let mut vector = glm::vec3(0.0, 0.0, 0.0);

        if self.contains(Side::EAST) {
            vector.x = 1.0;
        } else if self.contains(Side::WEST) {
            vector.x = -1.0;
        }

        if self.contains(Side::NORTH) {
            vector.z = 1.0;
        } else if self.contains(Side::SOUTH) {
            vector.z = -1.0;
        }

        if self.contains(Side::TOP) {
            vector.y = 1.0;
        } else if self.contains(Side::BOTTOM) {
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
    pub face: Side,
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
    let mut face = Side::NONE;

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
                face = if step.x == 1 { Side::WEST } else { Side::EAST }
            } else {
                next.z += delta.z;
                current_pos.z += step.z;
                dist_traveled.z += 1.0;
                face = if step.z == 1 {
                    Side::SOUTH
                } else {
                    Side::NORTH
                }
            }
        } else if next.y < next.z {
            next.y += delta.y;
            current_pos.y += step.y;
            dist_traveled.y += 1.0;
            face = if step.y == 1 { Side::BOTTOM } else { Side::TOP }
        } else {
            next.z += delta.z;
            current_pos.z += step.z;
            dist_traveled.z += 1.0;
            face = if step.z == 1 {
                Side::SOUTH
            } else {
                Side::NORTH
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

/// The offsets which need to be applied to a position
/// to prevent it from intersecting with a block.
#[derive(Debug, Clone)]
pub struct BlockIntersect {
    offset: DVec3,
    x: bool,
    y: bool,
    z: bool,
}

impl BlockIntersect {
    /// Applies this offset to the given position.
    pub fn apply_to(&self, pos: &mut Position) {
        pos.x += self.offset.x;
        pos.y += self.offset.y;
        pos.z += self.offset.z;
    }

    /// Returns whether the X axis is affected.
    pub fn x_affected(&self) -> bool {
        self.x
    }

    /// Returns whether the Y axis is affected.
    pub fn y_affected(&self) -> bool {
        self.y
    }

    /// Returns whether the Z axis is affected.
    pub fn z_affected(&self) -> bool {
        self.z
    }
}

/// Returns a struct containing position offsets which
/// must be applied to prevent blocks from intersecting
/// the bounding box. Call `BlockIntersect::apply` to
/// apply the offsets to a position.
///
/// `prev` should be the entity's position on the previous
/// tick. This is used to calculate impact points.
///
/// # Restrictions
/// Currently, bounding boxes with side lengths greater
/// than 1 are not supported. If the bounding box's size
/// is more than 1, this function will panic.
pub fn blocks_intersecting_bbox(
    chunk_map: &ChunkMap,
    mut from: Position,
    mut dest: Position,
    bbox: &BoundingBoxComponent,
) -> BlockIntersect {
    let bbox_size = bbox.size() / 2.0;

    // Center along Y axis of bounding box is at bottom, not center.
    // This is a quick fix to get around this.
    from.y += bbox_size.y;
    dest.y += bbox_size.y;

    assert!(bbox_size.x <= 1.0);
    assert!(bbox_size.y <= 1.0);
    assert!(bbox_size.z <= 1.0);

    let mut result = BlockIntersect {
        offset: vec3(0.0, 0.0, 0.0),
        x: false,
        y: false,
        z: false,
    };

    let offsets = [
        vec3(0.0, bbox_size.y, 0.0),
        vec3(0.0, -bbox_size.y, 0.0),
        vec3(bbox_size.x, 0.0, 0.0),
        vec3(-bbox_size.x, 0.0, 0.0),
        vec3(0.0, 0.0, bbox_size.z),
        vec3(0.0, 0.0, -bbox_size.z),
        vec3(0.0, 0.0, 0.0),
    ];
    let normals = [
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 0.0),
    ];

    // Compute a vector of isometries and axis normals representing adjacent block locations.
    let mut blocks: SmallVec<[(Isometry3<f64>, DVec3); 16]> = smallvec![];

    // Prevent same block being checked twice.
    let mut checked = heapless::FnvIndexSet::<BlockPosition, U8>::new();

    for (offset, normal) in offsets.iter().zip(normals.iter()) {
        let block_pos = (dest + *offset).block_pos();

        if checked.contains(&block_pos) {
            continue; // Already added this block
        }

        checked.insert(block_pos).unwrap(); // Unwrap is safe because set has capacity 8 and there are at most 7 blocks

        let block = match chunk_map.block_at(block_pos) {
            Some(block) => block,
            None => continue, // Unloaded chunk
        };
        if !block.is_solid() {
            continue; // Not a solid block
        }

        let isometry = block_isometry_64(block_pos);
        blocks.push((isometry, *normal));
    }

    // Go through blocks and check for time of impact from original
    // position to the block. If the time of impact is <= 1, the entity
    // has collided with the block; update the position accordingly.
    let block_shape = block_shape_64();
    let velocity = (dest - from).as_vec();
    let bbox_shape = bbox_to_cuboid(&bbox.0);

    for (block_isometry, normal) in blocks {
        let toi = match query::time_of_impact(
            &block_isometry,
            &vec3(0.0, 0.0, 0.0),
            &block_shape,
            &Isometry3::new(from.as_vec(), vec3(0.0, 0.0, 0.0)),
            &velocity,
            &bbox_shape,
            1.0,
            0.0,
        ) {
            Some(toi) => toi,
            None => continue, // No impact
        };

        let world_pos = from + velocity * toi.toi;
        let absolute_offset = world_pos - dest;

        result.offset += absolute_offset.as_vec().component_mul(&normal);

        if normal.x != 0.0 {
            result.x = true;
        }
        if normal.y != 0.0 {
            result.y = true;
        }
        if normal.z != 0.0 {
            result.z = true;
        }

        debug!("normal {:?}", normal);

        break; // Only check along one axis
    }

    debug!("{:?}", result);

    result
}

/// Returns an `ncollide` `Cuboid` corresponding to a block.
pub fn block_shape() -> Cuboid<f32> {
    Cuboid::new(vec3(0.5, 0.5, 0.5))
}

pub fn block_shape_64() -> Cuboid<f64> {
    Cuboid::new(vec3(0.5, 0.5, 0.5))
}

/// Returns an `Isometry` representing a block's translation.
pub fn block_isometry(pos: BlockPosition) -> Isometry3<f32> {
    Isometry3::new(
        vec3(pos.x as f32 + 0.5, pos.y as f32 + 0.5, pos.z as f32 + 0.5),
        vec3(0.0, 0.0, 0.0),
    )
}

pub fn block_isometry_64(pos: BlockPosition) -> Isometry3<f64> {
    Isometry3::new(
        vec3(
            f64::from(pos.x) + 0.5,
            f64::from(pos.y) + 0.5,
            f64::from(pos.z) + 0.5,
        ),
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
                face: Side::TOP,
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
                face: Side::WEST, // This should be three faces—see the TODO above
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
