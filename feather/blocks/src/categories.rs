use crate::{BlockId, BlockKind};
use libcraft_blocks::SimplifiedBlockKind;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlacementType {
    TargetedFace,
    PlayerDirection,
    PlayerDirectionRightAngle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SupportType {
    OnSolid,
    OnDesertBlocks,
    OnDirtBlocks,
    OnFarmland,
    OnSoulSand,
    OnWater,

    FacingSolid,
    FacingJungleWood,

    OnOrFacingSolid,

    CactusLike,
    ChorusFlowerLike,
    ChorusPlantLike,
    MushroomLike,
    SnowLike,
    SugarCaneLike,
    TripwireHookLike,
    VineLike,
}

impl BlockId {
    #[inline]
    pub fn is_solid(self) -> bool {
        self.kind().solid()
    }

    #[inline]
    pub fn is_opaque(self) -> bool {
        !self.kind().transparent()
    }

    #[inline]
    pub fn is_air(self) -> bool {
        self.simplified_kind() == SimplifiedBlockKind::Air
    }

    #[inline]
    pub fn is_fluid(self) -> bool {
        matches!(
            self.simplified_kind(),
            SimplifiedBlockKind::Water | SimplifiedBlockKind::Lava
        )
    }

    #[inline]
    pub fn is_replaceable(self) -> bool {
        matches!(
            self.simplified_kind(),
            SimplifiedBlockKind::Air
                | SimplifiedBlockKind::Water
                | SimplifiedBlockKind::Lava
                | SimplifiedBlockKind::Grass
                | SimplifiedBlockKind::TallGrass
                | SimplifiedBlockKind::Snow
                | SimplifiedBlockKind::Vine
                | SimplifiedBlockKind::DeadBush
        )
    }

    #[inline]
    pub fn light_emission(self) -> u8 {
        match self.kind() {
            BlockKind::Beacon
            | BlockKind::EndGateway
            | BlockKind::EndPortal
            | BlockKind::Fire
            | BlockKind::Glowstone
            | BlockKind::JackOLantern
            | BlockKind::Lava
            | BlockKind::SeaLantern
            | BlockKind::Conduit => 15,
            BlockKind::RedstoneLamp => {
                if self.lit().unwrap() {
                    15
                } else {
                    0
                }
            }
            BlockKind::EndRod | BlockKind::Torch => 14,
            BlockKind::Furnace => 13,
            BlockKind::NetherPortal => 11,
            BlockKind::EnderChest | BlockKind::RedstoneTorch => 7,
            BlockKind::SeaPickle => 6,
            BlockKind::MagmaBlock => 3,
            BlockKind::BrewingStand
            | BlockKind::BrownMushroom
            | BlockKind::DragonEgg
            | BlockKind::EndPortalFrame => 1,
            _ => 0,
        }
    }

    #[inline]
    pub fn can_fall(self) -> bool {
        matches!(
            self.simplified_kind(),
            SimplifiedBlockKind::Sand
                | SimplifiedBlockKind::Gravel
                | SimplifiedBlockKind::RedSand
                | SimplifiedBlockKind::Anvil
        )
    }

    #[inline]
    pub fn support_type(self) -> Option<SupportType> {
        Some(match self.simplified_kind() {
            SimplifiedBlockKind::Torch
            | SimplifiedBlockKind::RedstoneTorch
            | SimplifiedBlockKind::RedstoneWire
            | SimplifiedBlockKind::Repeater
            | SimplifiedBlockKind::Rail
            | SimplifiedBlockKind::ActivatorRail
            | SimplifiedBlockKind::DetectorRail
            | SimplifiedBlockKind::PoweredRail
            | SimplifiedBlockKind::Comparator
            | SimplifiedBlockKind::Seagrass
            | SimplifiedBlockKind::TallSeagrass
            | SimplifiedBlockKind::Kelp
            | SimplifiedBlockKind::KelpPlant
            | SimplifiedBlockKind::Sign
            | SimplifiedBlockKind::Coral
            | SimplifiedBlockKind::CoralFan
            | SimplifiedBlockKind::Banner
            | SimplifiedBlockKind::Carpet
            | SimplifiedBlockKind::WoodenDoor
            | SimplifiedBlockKind::IronDoor
            | SimplifiedBlockKind::StonePressurePlate
            | SimplifiedBlockKind::WoodenPressurePlate
            | SimplifiedBlockKind::HeavyWeightedPressurePlate
            | SimplifiedBlockKind::LightWeightedPressurePlate => SupportType::OnSolid,
            SimplifiedBlockKind::WallTorch
            | SimplifiedBlockKind::RedstoneWallTorch
            | SimplifiedBlockKind::Ladder
            | SimplifiedBlockKind::WallSign
            | SimplifiedBlockKind::CoralWallFan
            | SimplifiedBlockKind::WallBanner => SupportType::FacingSolid,
            SimplifiedBlockKind::Lever
            | SimplifiedBlockKind::StoneButton
            | SimplifiedBlockKind::WoodenButton => SupportType::OnOrFacingSolid,
            SimplifiedBlockKind::TripwireHook => SupportType::TripwireHookLike,
            SimplifiedBlockKind::AttachedMelonStem
            | SimplifiedBlockKind::AttachedPumpkinStem
            | SimplifiedBlockKind::MelonStem
            | SimplifiedBlockKind::PumpkinStem
            | SimplifiedBlockKind::Carrots
            | SimplifiedBlockKind::Potatoes
            | SimplifiedBlockKind::Beetroots
            | SimplifiedBlockKind::Wheat => SupportType::OnFarmland,
            SimplifiedBlockKind::Snow => SupportType::SnowLike,
            SimplifiedBlockKind::LilyPad => SupportType::OnWater,
            SimplifiedBlockKind::Cocoa => SupportType::FacingJungleWood,
            SimplifiedBlockKind::Grass
            | SimplifiedBlockKind::Fern
            | SimplifiedBlockKind::Sunflower
            | SimplifiedBlockKind::Lilac
            | SimplifiedBlockKind::RoseBush
            | SimplifiedBlockKind::Peony
            | SimplifiedBlockKind::TallGrass
            | SimplifiedBlockKind::LargeFern
            | SimplifiedBlockKind::Sapling
            | SimplifiedBlockKind::Flower => SupportType::OnDirtBlocks,
            SimplifiedBlockKind::Mushroom => SupportType::MushroomLike,
            SimplifiedBlockKind::DeadBush => SupportType::OnDesertBlocks,
            SimplifiedBlockKind::SugarCane => SupportType::SugarCaneLike,
            SimplifiedBlockKind::Vine => SupportType::VineLike,
            SimplifiedBlockKind::Cactus => SupportType::CactusLike,
            SimplifiedBlockKind::NetherWart => SupportType::OnSoulSand,
            SimplifiedBlockKind::ChorusFlower => SupportType::ChorusFlowerLike,
            SimplifiedBlockKind::ChorusPlant => SupportType::ChorusPlantLike,
            _ => return None,
        })
    }

    #[inline]
    pub fn placement_type(self) -> Option<PlacementType> {
        match self.simplified_kind() {
            SimplifiedBlockKind::WallTorch
            | SimplifiedBlockKind::RedstoneWallTorch
            | SimplifiedBlockKind::Lever
            | SimplifiedBlockKind::EndRod
            | SimplifiedBlockKind::StoneButton
            | SimplifiedBlockKind::WoodenButton
            | SimplifiedBlockKind::ShulkerBox => Some(PlacementType::TargetedFace),
            SimplifiedBlockKind::Observer
            | SimplifiedBlockKind::Bed
            | SimplifiedBlockKind::Fence
            | SimplifiedBlockKind::FenceGate
            | SimplifiedBlockKind::IronDoor
            | SimplifiedBlockKind::Stairs
            | SimplifiedBlockKind::WoodenDoor => Some(PlacementType::PlayerDirection),
            SimplifiedBlockKind::Anvil => Some(PlacementType::PlayerDirectionRightAngle),
            _ => None,
        }
    }

    #[inline]
    pub fn is_full_block(self) -> bool {
        self.kind().solid()
    }
}
