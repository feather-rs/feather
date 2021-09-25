use crate::{BlockId, BlockKind};

impl BlockId {
    pub fn to_wall_block(self) -> Option<BlockId> {
        match self.kind() {
            BlockKind::Torch => Some(BlockId::wall_torch()),
            BlockKind::RedstoneTorch => Some(BlockId::redstone_wall_torch()),
            BlockKind::OakSign => Some(BlockId::oak_wall_sign()),
            BlockKind::SpruceSign => Some(BlockId::spruce_wall_sign()),
            BlockKind::AcaciaSign => Some(BlockId::acacia_wall_sign()),
            BlockKind::BirchSign => Some(BlockId::birch_wall_sign()),
            BlockKind::CrimsonSign => Some(BlockId::crimson_wall_sign()),
            BlockKind::DarkOakSign => Some(BlockId::dark_oak_wall_sign()),
            BlockKind::JungleSign => Some(BlockId::jungle_wall_sign()),
            BlockKind::WarpedSign => Some(BlockId::warped_wall_sign()),
            BlockKind::SkeletonSkull => Some(BlockId::skeleton_wall_skull()),
            BlockKind::WitherSkeletonSkull => Some(BlockId::wither_skeleton_wall_skull()),
            BlockKind::ZombieHead => Some(BlockId::zombie_wall_head()),
            BlockKind::CreeperHead => Some(BlockId::creeper_wall_head()),
            BlockKind::PlayerHead => Some(BlockId::player_wall_head()),
            BlockKind::DragonHead => Some(BlockId::dragon_wall_head()),
            BlockKind::WhiteBanner => Some(BlockId::white_wall_banner()),
            BlockKind::OrangeBanner => Some(BlockId::orange_wall_banner()),
            BlockKind::MagentaBanner => Some(BlockId::magenta_wall_banner()),
            BlockKind::LightBlueBanner => Some(BlockId::light_blue_wall_banner()),
            BlockKind::YellowBanner => Some(BlockId::yellow_wall_banner()),
            BlockKind::LimeBanner => Some(BlockId::lime_wall_banner()),
            BlockKind::PinkBanner => Some(BlockId::pink_wall_banner()),
            BlockKind::GrayBanner => Some(BlockId::gray_wall_banner()),
            BlockKind::LightGrayBanner => Some(BlockId::light_gray_wall_banner()),
            BlockKind::CyanBanner => Some(BlockId::cyan_wall_banner()),
            BlockKind::PurpleBanner => Some(BlockId::purple_wall_banner()),
            BlockKind::BlueBanner => Some(BlockId::blue_wall_banner()),
            BlockKind::BrownBanner => Some(BlockId::brown_wall_banner()),
            BlockKind::GreenBanner => Some(BlockId::green_wall_banner()),
            BlockKind::RedBanner => Some(BlockId::red_wall_banner()),
            BlockKind::BlackBanner => Some(BlockId::black_wall_banner()),
            _ => None,
        }
    }
}
