use crate::{BlockId, BlockKind};

impl BlockId {
    pub fn is_solid(self) -> bool {
        // TODO: there are likely a few missing in this list
        match self.kind() {
            BlockKind::Air
            | BlockKind::OakSapling
            | BlockKind::SpruceSapling
            | BlockKind::BirchSapling
            | BlockKind::JungleSapling
            | BlockKind::AcaciaSapling
            | BlockKind::DarkOakSapling
            | BlockKind::Water
            | BlockKind::Lava
            | BlockKind::Grass
            | BlockKind::Fern
            | BlockKind::DeadBush
            | BlockKind::Seagrass
            | BlockKind::TallSeagrass
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
            | BlockKind::Torch
            | BlockKind::WallTorch
            | BlockKind::Fire
            | BlockKind::Wheat
            | BlockKind::Sign
            | BlockKind::Ladder
            | BlockKind::Rail
            | BlockKind::WallSign
            | BlockKind::Lever
            | BlockKind::StonePressurePlate
            | BlockKind::OakPressurePlate
            | BlockKind::SprucePressurePlate
            | BlockKind::BirchPressurePlate
            | BlockKind::JunglePressurePlate
            | BlockKind::AcaciaPressurePlate
            | BlockKind::DarkOakPressurePlate
            | BlockKind::RedstoneTorch
            | BlockKind::RedstoneWallTorch
            | BlockKind::StoneButton
            | BlockKind::Snow
            | BlockKind::SugarCane
            | BlockKind::Repeater
            | BlockKind::AttachedMelonStem
            | BlockKind::AttachedPumpkinStem
            | BlockKind::MelonStem
            | BlockKind::PumpkinStem
            | BlockKind::Vine
            | BlockKind::Carrots
            | BlockKind::Potatoes
            | BlockKind::OakButton
            | BlockKind::SpruceButton
            | BlockKind::BirchButton
            | BlockKind::JungleButton
            | BlockKind::AcaciaButton
            | BlockKind::DarkOakButton
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
            | BlockKind::DriedKelpBlock
            | BlockKind::VoidAir
            | BlockKind::CaveAir => false,
            _ => true,
        }
    }

    pub fn is_opaque(self) -> bool {
        self.kind.opaque()
    }

    pub fn is_air(self) -> bool {
        match self.kind() {
            BlockKind::Air | BlockKind::CaveAir | BlockKind::VoidAir => true,
            _ => false,
        }
    }

    pub fn is_fluid(self) -> bool {
        match self.kind() {
            BlockKind::Water | BlockKind::Lava => true,
            _ => false,
        }
    }

    pub fn is_leaves(self) -> bool {
        match self.kind() {
            BlockKind::AcaciaLeaves
            | BlockKind::BirchLeaves
            | BlockKind::DarkOakLeaves
            | BlockKind::JungleLeaves
            | BlockKind::OakLeaves
            | BlockKind::SpruceLeaves => true,
            _ => false,
        }
    }

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
}
