//! Bounding boxes for every non-cubic block.

use feather_core::blocks::{BlockId, BlockKind};
use nalgebra::Point3;
use ncollide3d::bounding_volume::AABB;

/// Returns the bounding box for the given block.
///
/// Non-solid blocks have no bounding box,
/// and the bounding box for a non-solid block
/// is undefined.
pub fn bbox_for_block(block: BlockId) -> AABB<f64> {
    match block.kind() {
        BlockKind::WhiteBed
        | BlockKind::OrangeBed
        | BlockKind::MagentaBed
        | BlockKind::LightBlueBed
        | BlockKind::YellowBed
        | BlockKind::LimeBed
        | BlockKind::PinkBed
        | BlockKind::GrayBed
        | BlockKind::LightGrayBed
        | BlockKind::CyanBed
        | BlockKind::PurpleBed
        | BlockKind::BlueBed
        | BlockKind::BrownBed
        | BlockKind::GreenBed
        | BlockKind::RedBed
        | BlockKind::BlackBed
        | BlockKind::PrismarineSlab
        | BlockKind::PrismarineBrickSlab
        | BlockKind::DarkPrismarineSlab
        | BlockKind::OakSlab
        | BlockKind::SpruceSlab
        | BlockKind::BirchSlab
        | BlockKind::JungleSlab
        | BlockKind::AcaciaSlab
        | BlockKind::DarkOakSlab
        | BlockKind::StoneSlab
        | BlockKind::SandstoneSlab
        | BlockKind::PetrifiedOakSlab
        | BlockKind::CobblestoneSlab
        | BlockKind::BrickSlab
        | BlockKind::StoneBrickSlab
        | BlockKind::NetherBrickSlab
        | BlockKind::QuartzSlab
        | BlockKind::RedSandstoneSlab
        | BlockKind::PurpurSlab => bbox(1.0, 0.5, 1.0),
        _ => bbox(1.0, 1.0, 1.0),
    }
}

fn bbox(x: f64, y: f64, z: f64) -> AABB<f64> {
    AABB::new(Point3::from([0.0, 0.0, 0.0]), Point3::from([x, y, z]))
}
