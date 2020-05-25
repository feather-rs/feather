use crate::{BlockId, BlockKind};

impl BlockId {
    pub fn to_wall_block(self) -> Option<BlockId> {
        match self.kind() {
            BlockKind::Torch => Some(BlockId::wall_torch()),
            BlockKind::RedstoneTorch => Some(BlockId::redstone_wall_torch()),
            BlockKind::Sign => Some(BlockId::wall_sign()),
            BlockKind::SkeletonSkull => Some(BlockId::skeleton_wall_skull()),
            BlockKind::WitherSkeletonSkull => Some(BlockId::wither_skeleton_wall_skull()),
            BlockKind::ZombieHead => Some(BlockId::zombie_wall_head()),
            BlockKind::CreeperHead => Some(BlockId::creeper_wall_head()),
            BlockKind::PlayerHead => Some(BlockId::player_wall_head()),
            BlockKind::DragonHead => Some(BlockId::dragon_wall_head()),
            BlockKind::WhiteWallBanner => Some(BlockId::white_wall_banner()),
            BlockKind::OrangeWallBanner => Some(BlockId::orange_wall_banner()),
            BlockKind::MagentaWallBanner => Some(BlockId::magenta_wall_banner()),
            BlockKind::LightBlueWallBanner => Some(BlockId::light_blue_wall_banner()),
            BlockKind::YellowWallBanner => Some(BlockId::yellow_wall_banner()),
            BlockKind::LimeWallBanner => Some(BlockId::lime_wall_banner()),
            BlockKind::PinkWallBanner => Some(BlockId::pink_wall_banner()),
            BlockKind::GrayWallBanner => Some(BlockId::gray_wall_banner()),
            BlockKind::LightGrayWallBanner => Some(BlockId::light_gray_wall_banner()),
            BlockKind::CyanWallBanner => Some(BlockId::cyan_wall_banner()),
            BlockKind::PurpleWallBanner => Some(BlockId::purple_wall_banner()),
            BlockKind::BlueWallBanner => Some(BlockId::blue_wall_banner()),
            BlockKind::BrownWallBanner => Some(BlockId::brown_wall_banner()),
            BlockKind::GreenWallBanner => Some(BlockId::green_wall_banner()),
            BlockKind::RedWallBanner => Some(BlockId::red_wall_banner()),
            BlockKind::BlackWallBanner => Some(BlockId::black_wall_banner()),
            _ => None,
        }
    }
}
