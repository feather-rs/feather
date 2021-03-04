//! Bounding boxes for every non-cubic block.

use feather_core::blocks::{BlockId, SimplifiedBlockKind};
use nalgebra::Point3;
use ncollide3d::bounding_volume::AABB;

/// Returns the bounding box for the given block.
///
/// Non-solid blocks have no bounding box,
/// and the bounding box for a non-solid block
/// is undefined.
pub fn bbox_for_block(block: BlockId) -> AABB<f64> {
    if matches!(
        block.simplified_kind(),
        SimplifiedBlockKind::Bed | SimplifiedBlockKind::Slab
    ) {
        bbox(1.0, 0.5, 1.0)
    } else {
        bbox(1.0, 1.0, 1.0)
    }
}

fn bbox(x: f64, y: f64, z: f64) -> AABB<f64> {
    AABB::new(Point3::from([0.0, 0.0, 0.0]), Point3::from([x, y, z]))
}
