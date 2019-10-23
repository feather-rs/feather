//! Bounding boxes for every non-cubic block.

use crate::physics::component::bbox;
use feather_blocks::Block;
use ncollide3d::bounding_volume::AABB;

/// Returns the bounding box for the given block.
///
/// Non-solid blocks have no bounding box,
/// and the bounding box for a non-solid block
/// is undefined.
pub fn bbox_for_block(block: &Block) -> AABB<f64> {
    match block {
        Block::WhiteBed(_)
        | Block::OrangeBed(_)
        | Block::MagentaBed(_)
        | Block::LightBlueBed(_)
        | Block::YellowBed(_)
        | Block::LimeBed(_)
        | Block::PinkBed(_)
        | Block::GrayBed(_)
        | Block::LightGrayBed(_)
        | Block::CyanBed(_)
        | Block::PurpleBed(_)
        | Block::BlueBed(_)
        | Block::BrownBed(_)
        | Block::GreenBed(_)
        | Block::RedBed(_)
        | Block::BlackBed(_)
        | Block::PrismarineSlab(_)
        | Block::PrismarineBrickSlab(_)
        | Block::DarkPrismarineSlab(_)
        | Block::OakSlab(_)
        | Block::SpruceSlab(_)
        | Block::BirchSlab(_)
        | Block::JungleSlab(_)
        | Block::AcaciaSlab(_)
        | Block::DarkOakSlab(_)
        | Block::StoneSlab(_)
        | Block::SandstoneSlab(_)
        | Block::PetrifiedOakSlab(_)
        | Block::CobblestoneSlab(_)
        | Block::BrickSlab(_)
        | Block::StoneBrickSlab(_)
        | Block::NetherBrickSlab(_)
        | Block::QuartzSlab(_)
        | Block::RedSandstoneSlab(_)
        | Block::PurpurSlab(_) => bbox(1.0, 0.5, 1.0),
        _ => bbox(1.0, 1.0, 1.0),
    }
}
