use crate::{BlockId, BlockKind};

impl BlockId {
    pub fn is_solid(self) -> bool {
        self.kind.solid()
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

    pub fn is_replaceable(self) -> bool {
        self.is_air()
            || self.is_fluid()
            || matches!(
                self.kind(),
                BlockKind::Grass | BlockKind::TallGrass | BlockKind::Snow
            )
    }

    pub fn is_shulker_box(self) -> bool {
        matches!(
            self.kind(),
            BlockKind::ShulkerBox
                | BlockKind::WhiteShulkerBox
                | BlockKind::OrangeShulkerBox
                | BlockKind::MagentaShulkerBox
                | BlockKind::LightBlueShulkerBox
                | BlockKind::YellowShulkerBox
                | BlockKind::LimeShulkerBox
                | BlockKind::PinkShulkerBox
                | BlockKind::GrayShulkerBox
                | BlockKind::LightGrayShulkerBox
                | BlockKind::CyanShulkerBox
                | BlockKind::PurpleShulkerBox
                | BlockKind::BlueShulkerBox
                | BlockKind::BrownShulkerBox
                | BlockKind::GreenShulkerBox
                | BlockKind::RedShulkerBox
                | BlockKind::BlackShulkerBox
        )
    }

    pub fn is_bed(self) -> bool {
        matches!(
            self.kind(),
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
        )
    }

    pub fn is_button(self) -> bool {
        matches!(
            self.kind(),
            BlockKind::StoneButton
                | BlockKind::OakButton
                | BlockKind::SpruceButton
                | BlockKind::BirchButton
                | BlockKind::JungleButton
                | BlockKind::AcaciaButton
                | BlockKind::DarkOakButton
        )
    }

    pub fn is_stairs(self) -> bool {
        matches!(
            self.kind(),
            BlockKind::OakStairs
                | BlockKind::CobblestoneStairs
                | BlockKind::BrickStairs
                | BlockKind::StoneBrickStairs
                | BlockKind::NetherBrickStairs
                | BlockKind::SandstoneStairs
                | BlockKind::SpruceStairs
                | BlockKind::BirchStairs
                | BlockKind::JungleStairs
                | BlockKind::QuartzStairs
                | BlockKind::AcaciaStairs
                | BlockKind::DarkOakStairs
                | BlockKind::PrismarineStairs
                | BlockKind::PrismarineBrickStairs
                | BlockKind::DarkPrismarineStairs
                | BlockKind::RedSandstoneStairs
                | BlockKind::PurpurStairs
        )
    }

    pub fn is_door(self) -> bool {
        matches!(
            self.kind(),
            BlockKind::OakDoor
                | BlockKind::IronDoor
                | BlockKind::SpruceDoor
                | BlockKind::BirchDoor
                | BlockKind::JungleDoor
                | BlockKind::AcaciaDoor
                | BlockKind::DarkOakDoor
        )
    }

    pub fn is_fence_gate(self) -> bool {
        matches!(
            self.kind(),
            BlockKind::OakFenceGate
                | BlockKind::SpruceFenceGate
                | BlockKind::BirchFenceGate
                | BlockKind::JungleFenceGate
                | BlockKind::AcaciaFenceGate
                | BlockKind::DarkOakFenceGate
        )
    }

    pub fn is_slab(self) -> bool {
        matches!(
            self.kind(),
            BlockKind::PrismarineSlab
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
                | BlockKind::PurpurSlab
        )
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
