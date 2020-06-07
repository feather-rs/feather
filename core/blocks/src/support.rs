use crate::{BlockId, BlockKind, FacingCubic, Face};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SupportType {
    FullBlock(FacingCubic),
    // TODO dont use these horrible arrays
    FullBlockWhitelist(FacingCubic, [Option<BlockKind>; 1]),
    FullBlockBlacklist(FacingCubic, [Option<BlockKind>; 2]),
}

impl SupportType {
    pub fn direction(self) -> FacingCubic {
        match self {
            SupportType::FullBlock(facing) => facing,
            SupportType::FullBlockWhitelist(facing, ..) => facing,
            SupportType::FullBlockBlacklist(facing, ..) => facing,
        }
    }

    pub fn is_valid_support(self, kind: BlockKind) -> bool {
        kind.full_block() && match self {
            SupportType::FullBlockWhitelist(.., whitelist) =>
                whitelist.iter().any(|&k| Some(kind) == k),
            SupportType::FullBlockBlacklist(.., blacklist) =>
                !blacklist.iter().any(|&k| Some(kind) == k),
            _ => true,
        }
    }
}

impl BlockId {
    pub fn can_fall(self) -> bool {
        match self.kind() {
            BlockKind::Sand
            | BlockKind::Gravel
            | BlockKind::RedSand
            | BlockKind::Anvil
            | BlockKind::ChippedAnvil
            | BlockKind::DamagedAnvil => true,
            _ => false,
        }
    }

    pub fn needs_support(self) -> Option<SupportType> {
        // TODO add more, especially handle vines and sugar cane
        match self.kind() {
            BlockKind::Grass
            | BlockKind::Fern
            | BlockKind::Seagrass
            | BlockKind::TallSeagrass
            | BlockKind::DeadBush
            | BlockKind::Torch
            | BlockKind::RedstoneTorch
            | BlockKind::RedstoneWire
            | BlockKind::Repeater
            | BlockKind::Rail
            | BlockKind::WhiteBed
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
            | BlockKind::LightWeightedPressurePlate
            | BlockKind::HeavyWeightedPressurePlate
            | BlockKind::Comparator
            | BlockKind::WhiteCarpet
            | BlockKind::OrangeCarpet
            | BlockKind::MagentaCarpet
            | BlockKind::LightBlueCarpet
            | BlockKind::YellowCarpet
            | BlockKind::LimeCarpet
            | BlockKind::PinkCarpet
            | BlockKind::GrayCarpet
            | BlockKind::LightGrayCarpet
            | BlockKind::CyanCarpet
            | BlockKind::PurpleCarpet
            | BlockKind::BlueCarpet
            | BlockKind::BrownCarpet
            | BlockKind::GreenCarpet
            | BlockKind::RedCarpet
            | BlockKind::BlackCarpet
            | BlockKind::Sunflower
            | BlockKind::Lilac
            | BlockKind::RoseBush
            | BlockKind::Peony
            | BlockKind::TallGrass
            | BlockKind::LargeFern
            | BlockKind::Kelp
            | BlockKind::KelpPlant
            | BlockKind::OakSapling
            | BlockKind::SpruceSapling
            | BlockKind::BirchSapling
            | BlockKind::JungleSapling
            | BlockKind::AcaciaSapling
            | BlockKind::DarkOakSapling
            | BlockKind::Dandelion
            | BlockKind::Poppy
            | BlockKind::BlueOrchid
            | BlockKind::Allium
            | BlockKind::AzureBluet
            | BlockKind::RedTulip
            | BlockKind::OrangeTulip
            | BlockKind::WhiteTulip
            | BlockKind::PinkTulip
            | BlockKind::OxeyeDaisy
            | BlockKind::BrownMushroom
            | BlockKind::RedMushroom
            | BlockKind::Wheat
            | BlockKind::Sign
            | BlockKind::StonePressurePlate
            | BlockKind::OakPressurePlate
            | BlockKind::SprucePressurePlate
            | BlockKind::BirchPressurePlate
            | BlockKind::JunglePressurePlate
            | BlockKind::AcaciaPressurePlate
            | BlockKind::DarkOakPressurePlate
            | BlockKind::SugarCane
            | BlockKind::ActivatorRail
            | BlockKind::DetectorRail
            | BlockKind::PoweredRail
            | BlockKind::BrainCoral
            | BlockKind::BubbleCoral
            | BlockKind::FireCoral
            | BlockKind::HornCoral
            | BlockKind::TubeCoral
            | BlockKind::BrainCoralFan
            | BlockKind::BubbleCoralFan
            | BlockKind::FireCoralFan
            | BlockKind::HornCoralFan
            | BlockKind::TubeCoralFan
            | BlockKind::DeadBrainCoral
            | BlockKind::DeadBubbleCoral
            | BlockKind::DeadFireCoral
            | BlockKind::DeadHornCoral
            | BlockKind::DeadTubeCoral
            | BlockKind::DeadBrainCoralFan
            | BlockKind::DeadBubbleCoralFan
            | BlockKind::DeadFireCoralFan
            | BlockKind::DeadHornCoralFan
            | BlockKind::DeadTubeCoralFan => Some(
                SupportType::FullBlock(FacingCubic::Down)
            ),
            BlockKind::WallTorch
            | BlockKind::RedstoneWallTorch
            | BlockKind::WhiteWallBanner
            | BlockKind::OrangeWallBanner
            | BlockKind::MagentaWallBanner
            | BlockKind::LightBlueWallBanner
            | BlockKind::YellowWallBanner
            | BlockKind::LimeWallBanner
            | BlockKind::PinkWallBanner
            | BlockKind::GrayWallBanner
            | BlockKind::LightGrayWallBanner
            | BlockKind::CyanWallBanner
            | BlockKind::PurpleWallBanner
            | BlockKind::BlueWallBanner
            | BlockKind::BrownWallBanner
            | BlockKind::GreenWallBanner
            | BlockKind::RedWallBanner
            | BlockKind::BlackWallBanner
            | BlockKind::OakButton
            | BlockKind::Ladder
            | BlockKind::WallSign
            | BlockKind::BrainCoralWallFan
            | BlockKind::BubbleCoralWallFan
            | BlockKind::FireCoralWallFan
            | BlockKind::HornCoralWallFan
            | BlockKind::TubeCoralWallFan
            | BlockKind::DeadBrainCoralWallFan
            | BlockKind::DeadBubbleCoralWallFan
            | BlockKind::DeadFireCoralWallFan
            | BlockKind::DeadHornCoralWallFan
            | BlockKind::DeadTubeCoralWallFan => Some(
                SupportType::FullBlock(
                    self.facing_cardinal().unwrap().opposite().to_facing_cubic()
                )
            ),
            BlockKind::SpruceButton
            | BlockKind::BirchButton
            | BlockKind::JungleButton
            | BlockKind::AcaciaButton
            | BlockKind::DarkOakButton
            | BlockKind::StoneButton
            | BlockKind::Lever => Some(
                SupportType::FullBlock(match self.face().unwrap() {
                    Face::Floor => FacingCubic::Down,
                    Face::Ceiling => FacingCubic::Up,
                    Face::Wall => self.facing_cardinal().unwrap().opposite().to_facing_cubic(),
                })
            ),
            BlockKind::AttachedMelonStem
            | BlockKind::AttachedPumpkinStem
            | BlockKind::MelonStem
            | BlockKind::PumpkinStem
            | BlockKind::Carrots
            | BlockKind::Potatoes
            | BlockKind::Beetroots => Some(
                SupportType::FullBlockWhitelist(
                    FacingCubic::Down, [Some(BlockKind::Farmland)]
                )
            ),
            BlockKind::Snow => Some(
                SupportType::FullBlockBlacklist(
                    FacingCubic::Down, [Some(BlockKind::Ice), Some(BlockKind::PackedIce)]
                )
            ),
            _ => None,
        }
    }
}