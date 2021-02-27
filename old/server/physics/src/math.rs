//! A bunch of math-related functions for use with
//! the physics system.

use crate::block_bboxes::bbox_for_block;
use bitflags::bitflags;
use feather_core::blocks::BlockId;
use feather_core::util::{BlockPosition, Position};
use feather_server_types::{AABBExt, Game};

use glm::{vec3, DVec3, Vec3};
use heapless::consts::*;
use nalgebra::{Isometry3, Point3};
use ncollide3d::bounding_volume::AABB;
use ncollide3d::query;
use ncollide3d::query::{Ray, RayCast};
use ncollide3d::shape::{Compound, Cuboid, ShapeHandle};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::f64::INFINITY;

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
    game: &Game,
    origin: DVec3,
    ray: DVec3,
    max_distance_squared: f64,
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

    let mut dist_traveled = glm::vec3(0.0f64, 0.0, 0.0);

    let mut step = glm::vec3(0, 0, 0);
    let mut delta = glm::vec3(INFINITY, INFINITY, INFINITY);
    let mut next = glm::vec3(INFINITY, INFINITY, INFINITY);

    // TODO this implementation does not properly
    // handle when a ray hits multiple faces.
    // In practice, this should not be an issue,
    // but it may causes subtle issues in the future.
    let mut face = Side::NONE;

    match direction.x.partial_cmp(&0.0).unwrap() {
        Ordering::Greater => {
            step.x = 1;
            delta.x = 1.0 / direction.x;
            next.x = ((origin.x + 1.0).floor() - origin.x) / direction.x; // Brings X position to next integer
        }
        Ordering::Less => {
            step.x = -1;
            delta.x = (1.0 / direction.x).abs();
            next.x = ((origin.x - (origin.x - 1.0).ceil()) / direction.x).abs();
        }
        _ => (),
    }

    match direction.y.partial_cmp(&0.0).unwrap() {
        Ordering::Greater => {
            step.y = 1;
            delta.y = 1.0 / direction.y;
            next.y = ((origin.y + 1.0).floor() - origin.y) / direction.y;
        }
        Ordering::Less => {
            step.y = -1;
            delta.y = (1.0 / direction.y).abs();
            next.y = ((origin.y - (origin.y - 1.0).ceil()) / direction.y).abs();
        }
        _ => (),
    }

    match direction.z.partial_cmp(&0.0).unwrap() {
        Ordering::Greater => {
            step.z = 1;
            delta.z = 1.0 / direction.z;
            next.z = ((origin.z + 1.0).floor() - origin.z) / direction.z;
        }
        Ordering::Less => {
            step.z = -1;
            delta.z = (1.0 / direction.z).abs();
            next.z = ((origin.z - (origin.z - 1.0).ceil()) / direction.z).abs();
        }
        _ => (),
    }

    let mut current_pos = Position::from(origin).block();

    while dist_traveled.magnitude_squared() < max_distance_squared {
        if let Some(block) = game.block_at(current_pos) {
            if block.is_solid() {
                // Calculate world-space position of
                // impact using `ncollide`.
                let ray = Ray::new(Point3::from(origin), direction);
                let shape = block_shape(block);
                let isometry = block_isometry(current_pos);

                if let Some(impact) = shape.toi_and_normal_with_ray(&isometry, &ray, 1000.0, true) {
                    let pos = Position::from(origin + impact.toi * direction);

                    return Some(RayImpact {
                        block: current_pos,
                        pos,
                        face,
                    });
                }
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
    game: &Game,
    mut from: Position,
    mut dest: Position,
    bbox: &AABB<f64>,
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

    // Vector of axis and signs to pass to `adjacent_to_bbox()`.
    let axis = [(1, 1), (1, -1), (0, 1), (0, -1), (2, 1), (2, -1)];

    // Compute a vector of compound shapes and axis normals representing adjacent blocks.
    let mut blocks: SmallVec<[Compound<f64>; 4]> = SmallVec::new();

    // Don't check the same block twice.
    let mut checked = heapless::FnvIndexSet::new();

    for (axis, sign) in &axis {
        let compound = adjacent_to_bbox(*axis, *sign, bbox, dest, &game, &mut checked);
        blocks.push(compound);
    }

    // Go through blocks and check for time of impact from original
    // position to the block. If the time of impact is <= 1, the entity
    // has collided with the block; update the position accordingly.
    let velocity = (dest - from).into();
    let bbox_shape = bbox_to_cuboid(&bbox);

    for compound in blocks {
        let toi = match query::time_of_impact(
            &Isometry3::translation(0.0, 0.0, 0.0),
            &vec3(0.0, 0.0, 0.0),
            &compound,
            &Isometry3::new(from.into(), vec3(0.0, 0.0, 0.0)),
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

        let normal = {
            let x_diff = absolute_offset.x.abs();
            let y_diff = absolute_offset.y.abs();
            let z_diff = absolute_offset.z.abs();

            if x_diff > y_diff && x_diff > z_diff {
                vec3(1.0, 0.0, 0.0)
            } else if y_diff > x_diff && y_diff > z_diff {
                vec3(0.0, 1.0, 0.0)
            } else {
                vec3(0.0, 0.0, 1.0)
            }
        };

        result.offset += <Position as Into<DVec3>>::into(absolute_offset).component_mul(&normal);

        if normal.x != 0.0 {
            result.x = true;
        }
        if normal.y != 0.0 {
            result.y = true;
        }
        if normal.z != 0.0 {
            result.z = true;
        }
    }

    result
}

/// Returns a `Compound` representing up to four blocks
/// adjacent to a bounding box along the provided axis.
///
/// Any block positions in `checked` will not be added to the compound.
/// `checked` will also be updated to account for any added blocks.
///
/// `axis` must be one of the following:
/// * `0` for the X axis;
/// * `1` for the Y axis; or
/// * `2` for the Z axis.
///
/// `sign` must be either -1 or 1.
pub fn adjacent_to_bbox(
    axis: usize,
    sign: i32,
    bbox: &AABB<f64>,
    pos: Position,
    game: &Game,
    checked: &mut heapless::FnvIndexSet<BlockPosition, U32>,
) -> Compound<f64> {
    assert!(axis <= 2);
    assert!(sign == -1 || sign == 1);

    let sign = f64::from(sign);

    let size = bbox.size() / 2.0;
    let mut blocks: SmallVec<[(BlockPosition, BlockId); 4]> = SmallVec::new();

    let other_axis1 = match axis {
        0 => 1,
        1 => 2,
        2 => 0,
        _ => unreachable!(),
    };
    let other_axis2 = match axis {
        0 => 2,
        1 => 0,
        2 => 1,
        _ => unreachable!(),
    };

    let offsets = {
        let mut offsets = [vec3(0.0, 0.0, 0.0); 4];

        // Offset for upper right corner, upper left, bottom right, and bottom left.

        // Upper right
        offsets[0][axis] = size[axis] * sign;
        offsets[0][other_axis1] = size[other_axis1];
        offsets[0][other_axis2] = size[other_axis2];

        // Upper left
        offsets[1][axis] = size[axis] * sign;
        offsets[1][other_axis1] = size[other_axis1] * -1.0;
        offsets[1][other_axis2] = size[other_axis2];

        // Bottom right
        offsets[2][axis] = size[axis] * sign;
        offsets[2][other_axis1] = size[other_axis1];
        offsets[2][other_axis2] = size[other_axis2] * -1.0;

        // Bottom left
        offsets[3][axis] = size[axis] * sign;
        offsets[3][other_axis1] = size[other_axis1] * -1.0;
        offsets[3][other_axis2] = size[other_axis2] * -1.0;

        offsets
    };

    // Go through offsets and append block position if the block is solid.
    for offset in &offsets {
        let block_pos = (pos + *offset).block();

        if checked.contains(&block_pos) {
            continue;
        }

        match game.block_at(block_pos) {
            Some(block) => {
                if block.is_solid() {
                    checked.insert(block_pos).unwrap();
                    blocks.push((block_pos, block));
                }
            }
            None => continue,
        }
    }

    let mut shapes = Vec::with_capacity(4);

    for (block_pos, block) in &blocks {
        let isometry = block_isometry(*block_pos);
        let shape = block_shape(*block);
        shapes.push((isometry, ShapeHandle::new(shape)));
    }

    Compound::new(shapes)
}

/// Returns an `ncollide` `Cuboid` corresponding to the given block.
pub fn block_shape(block: BlockId) -> Cuboid<f64> {
    let bbox = bbox_for_block(block);
    Cuboid::new(bbox.half_extents())
}

/// Returns an `Isometry` representing a block's translation.
pub fn block_isometry(pos: BlockPosition) -> Isometry3<f64> {
    Isometry3::new(
        vec3(
            f64::from(pos.x) + 0.5,
            f64::from(pos.y) + 0.5,
            f64::from(pos.z) + 0.5,
        ),
        vec3(0.0, 0.0, 0.0),
    )
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
            1000.0,
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

/* TODO: update
#[cfg(test)]
mod tests {
    use super::*;
    use feather_core::world::ChunkPosition;
    use feather_core::Block;
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
            block_impacted_by_ray(&map, vec3(0.0, 65.0, 0.0), vec3(0.0, 1.0, 0.0), 256.0),
            None
        );

        assert_eq!(
            block_impacted_by_ray(&map, vec3(0.0, 70.0, 0.0), vec3(0.0, -1.0, 0.0), 5.0),
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

        let e1 = test::create(&mut w, position!(0.0, 0.0, 0.0)).build();
        let e2 = test::create(&mut w, position!(-100.0, 0.0, 50.0)).build();
        let e3 = test::create(&mut w, position!(100.0, 50.0, 50.0)).build();
        let e4 = test::create(&mut w, position!(100.0, 1.0, -50.0)).build();

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

    #[test]
    fn test_blocks_intersecting_bbox() {
        let chunk_map = chunk_map();

        let froms = [
            position!(0.0, 66.0, 0.0),
            position!(100.0, 65.0, 0.0),
            position!(0.0, 100.0, 0.0),
        ];

        let dests = [
            position!(0.0, 65.0, 0.0),
            position!(100.0, 65.0, 0.0),
            position!(0.0, 90.0, 0.0),
        ];

        let results = [
            position!(0.0, 65.0, 0.0),
            position!(100.0, 65.0, 0.0),
            position!(0.0, 90.0, 0.0),
        ];

        let bbox = crate::physics::component::bbox(0.25, 0.25, 0.25);

        for ((from, dest), result) in froms.iter().zip(&dests).zip(&results) {
            let intersect = blocks_intersecting_bbox(&chunk_map, *from, *dest, &bbox);
            let mut pos = *dest;
            intersect.apply_to(&mut pos);

            assert_pos_eq!(pos, result);
        }
    }

    #[test]
    fn test_adjacent_to_bbox() {
        let chunk_map = chunk_map();

        let bbox = crate::physics::component::bbox(0.25, 0.25, 0.25);

        let pos = position!(0.0, 65.0, 0.0);

        let axis = 1;
        let sign = -1;

        let mut checked = heapless::FnvIndexSet::new();

        let _ = adjacent_to_bbox(axis, sign, &bbox, pos, &chunk_map, &mut checked);

        assert!(checked.contains(&BlockPosition::new(0, 64, 0)));
    }
}
*/
