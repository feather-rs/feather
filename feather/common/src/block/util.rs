use std::convert::TryFrom;

use crate::World;
use base::{
    categories::SupportType, BlockId, BlockKind, BlockPosition, EastNlt, FacingCardinal,
    FacingCardinalAndDown, FacingCubic, NorthNlt, SouthNlt, WestNlt,
};
use libcraft_core::BlockFace;
use std::convert::TryInto;

/// Utility enum that represents `EastNlt`, `WestNlt`, `NorthNlt` and `SouthNlt`. These enums have identical discriminants and binary representations, making it safe to convert between them.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Nlt {
    None,
    Low,
    Tall,
}
impl TryFrom<u16> for Nlt {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Nlt::None),
            1u16 => Ok(Nlt::Low),
            2u16 => Ok(Nlt::Tall),
            x => Err(anyhow::anyhow!("invalid value {} for Nlt", x)),
        }
    }
}
macro_rules! impl_conversions {
    ($ty:ident) => {
        impl From<Nlt> for $ty {
            fn from(nlt: Nlt) -> Self {
                (nlt as u16).try_into().unwrap()
            }
        }
        impl From<$ty> for Nlt {
            fn from(other_nlt: $ty) -> Self {
                (other_nlt as u16).try_into().unwrap()
            }
        }
    };
    ($($ty:ident),+) => {
        $(
            impl_conversions!($ty);
        )+
    }
}
impl_conversions!(EastNlt, WestNlt, NorthNlt, SouthNlt);

/// Trait that implements helper function for adjacency. This is an extension to `World` that tries to keep this utility logic away from the main implementation
pub trait AdjacentBlockHelper {
    fn adjacent_block(&self, pos: BlockPosition, face: BlockFace) -> Option<BlockId>;

    fn adjacent_block_cubic(&self, pos: BlockPosition, dir: FacingCubic) -> Option<BlockId>;

    fn adjacent_block_cardinal(&self, pos: BlockPosition, dir: FacingCardinal) -> Option<BlockId>;

    fn adjacent_block_cardinal_and_down(
        &self,
        pos: BlockPosition,
        dir: FacingCardinalAndDown,
    ) -> Option<BlockId>;

    fn get_facing_direction(&self, block: BlockId) -> Option<FacingCubic>;

    fn set_block_adjacent_cubic(
        &self,
        pos: BlockPosition,
        block: BlockId,
        dir: FacingCubic,
    ) -> bool;

    fn set_block_adjacent_cardinal(
        &self,
        pos: BlockPosition,
        block: BlockId,
        dir: FacingCardinal,
    ) -> bool;

    fn set_block_adjacent_cardinal_and_down(
        &self,
        pos: BlockPosition,
        block: BlockId,
        dir: FacingCardinalAndDown,
    ) -> bool;

    fn check_block_stability(
        &self,
        block: BlockId,
        pos: BlockPosition,
        light_level: u8,
    ) -> Option<bool>;
}
impl AdjacentBlockHelper for World {
    fn adjacent_block(&self, pos: BlockPosition, face: BlockFace) -> Option<BlockId> {
        self.block_at(pos.adjacent(face))
    }

    fn adjacent_block_cubic(&self, pos: BlockPosition, dir: FacingCubic) -> Option<BlockId> {
        self.adjacent_block(
            pos,
            match dir {
                FacingCubic::North => BlockFace::North,
                FacingCubic::East => BlockFace::East,
                FacingCubic::South => BlockFace::South,
                FacingCubic::West => BlockFace::West,
                FacingCubic::Up => BlockFace::Top,
                FacingCubic::Down => BlockFace::Bottom,
            },
        )
    }

    fn adjacent_block_cardinal(&self, pos: BlockPosition, dir: FacingCardinal) -> Option<BlockId> {
        self.adjacent_block_cubic(pos, dir.to_facing_cubic())
    }

    fn adjacent_block_cardinal_and_down(
        &self,
        pos: BlockPosition,
        dir: FacingCardinalAndDown,
    ) -> Option<BlockId> {
        self.adjacent_block_cubic(pos, dir.to_facing_cubic())
    }

    fn get_facing_direction(&self, block: BlockId) -> Option<FacingCubic> {
        let dir = if block.has_facing_cardinal() {
            block.facing_cardinal().unwrap().to_facing_cubic()
        } else if block.has_facing_cardinal_and_down() {
            block.facing_cardinal_and_down().unwrap().to_facing_cubic()
        } else {
            block.facing_cubic()?
        };
        Some(dir.opposite())
    }

    fn set_block_adjacent_cubic(
        &self,
        pos: BlockPosition,
        block: BlockId,
        dir: FacingCubic,
    ) -> bool {
        self.set_block_at(
            pos.adjacent(match dir {
                FacingCubic::North => BlockFace::North,
                FacingCubic::East => BlockFace::East,
                FacingCubic::South => BlockFace::South,
                FacingCubic::West => BlockFace::West,
                FacingCubic::Up => BlockFace::Top,
                FacingCubic::Down => BlockFace::Bottom,
            }),
            block,
        )
    }

    fn set_block_adjacent_cardinal(
        &self,
        pos: BlockPosition,
        block: BlockId,
        dir: FacingCardinal,
    ) -> bool {
        self.set_block_adjacent_cubic(pos, block, dir.to_facing_cubic())
    }

    fn set_block_adjacent_cardinal_and_down(
        &self,
        pos: BlockPosition,
        block: BlockId,
        dir: FacingCardinalAndDown,
    ) -> bool {
        self.set_block_adjacent_cubic(pos, block, dir.to_facing_cubic())
    }

    fn check_block_stability(
        &self,
        block: BlockId,
        pos: BlockPosition,
        light_level: u8,
    ) -> Option<bool> {
        use blocks::SimplifiedBlockKind::*;
        let support_type = block.support_type();
        if support_type.is_none() {
            return Some(true);
        }
        let support_type = support_type.unwrap();
        let block_under = self.block_at(pos.down());
        let block_up = self.block_at(pos.up());
        let block_facing = self
            .get_facing_direction(block)
            .map(|f| self.adjacent_block_cubic(pos, f))
            .flatten();
        let is_supported = match support_type {
            SupportType::OnSolid => block_under?.is_solid(),
            SupportType::OnDesertBlocks => matches!(
                block_under?.simplified_kind(),
                Sand | RedSand | Dirt | CoarseDirt | Podzol
            ),
            SupportType::OnDirtBlocks => matches!(
                block_under?.simplified_kind(),
                Dirt | GrassBlock | CoarseDirt | Podzol | Farmland
            ),
            SupportType::OnFarmland => block_under?.simplified_kind() == Farmland,
            SupportType::OnSoulSand => block_under?.simplified_kind() == SoulSand,
            SupportType::OnWater => block_under?.simplified_kind() == Water,
            SupportType::FacingSolid => block_facing?.is_solid(),
            SupportType::FacingJungleWood => matches!(
                block_facing?.kind(),
                BlockKind::JungleLog
                    | BlockKind::StrippedJungleLog
                    | BlockKind::JungleWood
                    | BlockKind::StrippedJungleWood
            ),
            SupportType::OnOrFacingSolid => self
                .block_at(pos.adjacent(match block.face()? {
                    base::Face::Floor => BlockFace::Bottom,
                    base::Face::Wall => match block.facing_cardinal()?.opposite() {
                        FacingCardinal::North => BlockFace::North,
                        FacingCardinal::South => BlockFace::South,
                        FacingCardinal::West => BlockFace::West,
                        FacingCardinal::East => BlockFace::East,
                    },
                    base::Face::Ceiling => BlockFace::Top,
                }))?
                .is_full_block(),
            SupportType::CactusLike => {
                matches!(block_under?.simplified_kind(), Sand | RedSand | Cactus) && {
                    let mut ok = true;
                    for face in [
                        BlockFace::North,
                        BlockFace::South,
                        BlockFace::West,
                        BlockFace::East,
                    ] {
                        let block = self.block_at(pos.adjacent(face))?;
                        ok &= !block.is_full_block() && block.simplified_kind() != Cactus
                    }
                    ok
                }
            }
            SupportType::ChorusFlowerLike => {
                let neighbours = [
                    BlockFace::North,
                    BlockFace::South,
                    BlockFace::West,
                    BlockFace::East,
                ]
                .iter()
                .filter_map(|&face| self.block_at(pos.adjacent(face)))
                .map(BlockId::simplified_kind);
                neighbours.clone().filter(|&e| e == Air).count() == 3
                    && neighbours.filter(|&e| e == EndStone).count() == 1
                    || matches!(block_under?.simplified_kind(), EndStone | ChorusPlant)
            }
            SupportType::ChorusPlantLike => {
                let n = [
                    BlockFace::North,
                    BlockFace::South,
                    BlockFace::West,
                    BlockFace::East,
                ];
                let horizontal = n
                    .iter()
                    .filter_map(|&f| self.block_at(pos.adjacent(f)))
                    .map(BlockId::simplified_kind);
                let horizontal_down = n
                    .iter()
                    .filter_map(|&f| self.block_at(pos.down().adjacent(f)))
                    .map(BlockId::simplified_kind);
                if horizontal.clone().count() != 4 || horizontal_down.clone().count() != 4 {
                    return None;
                }
                let has_horizontal = horizontal.clone().any(|b| b == ChorusPlant);
                let has_vertical =
                    matches!(block_up?.simplified_kind(), ChorusPlant | ChorusFlower);
                let is_connected = horizontal
                    .zip(horizontal_down)
                    .any(|(h, hd)| h == ChorusPlant && matches!(hd, ChorusPlant | EndStone));
                is_connected && !(has_vertical && has_horizontal && !block_under?.is_air())
            }
            SupportType::MushroomLike => block_under?.is_full_block() && light_level < 13,
            SupportType::SnowLike => {
                block_under?.is_full_block()
                    && !matches!(block_under?.simplified_kind(), Ice | PackedIce)
            }
            SupportType::SugarCaneLike => {
                matches!(
                    block_under?.simplified_kind(),
                    Grass | Dirt | CoarseDirt | Podzol | Sand | RedSand | SugarCane
                ) && {
                    let mut ok = false;
                    for face in [
                        BlockFace::North,
                        BlockFace::South,
                        BlockFace::West,
                        BlockFace::East,
                    ] {
                        let block = self.block_at(pos.down().adjacent(face))?;
                        ok |= matches!(block.simplified_kind(), FrostedIce | Water);
                        ok |= block.waterlogged().unwrap_or(false);
                    }
                    ok
                }
            }
            SupportType::TripwireHookLike => {
                block_facing?.is_full_block()
                    && !matches!(block_facing?.simplified_kind(), RedstoneBlock | Observer)
            }
            SupportType::VineLike => {
                matches!(self.block_at(pos.up())?.simplified_kind(), Vine) || {
                    let mut ok = false;
                    for face in [
                        BlockFace::North,
                        BlockFace::South,
                        BlockFace::West,
                        BlockFace::East,
                        BlockFace::Top,
                    ] {
                        let block = self.block_at(pos.down().adjacent(face))?;
                        ok |= block.is_full_block();
                    }
                    ok
                }
            }
        };
        Some(is_supported)
    }
}

/// Checks if the block is a wall. `SimplifiedBlockKind` does not have a common type for walls at this time, making this function neccessary.
pub fn is_wall(block: BlockId) -> bool {
    use base::SimplifiedBlockKind::*;
    matches!(
        block.simplified_kind(),
        BrickWall
            | PrismarineWall
            | RedSandstoneWall
            | MossyStoneBrickWall
            | GraniteWall
            | StoneBrickWall
            | NetherBrickWall
            | AndesiteWall
            | RedNetherBrickWall
            | SandstoneWall
            | EndStoneBrickWall
            | DioriteWall
            | CobblestoneWall
            | MossyCobblestoneWall
            | BlackstoneWall
            | PolishedBlackstoneBrickWall
            | PolishedBlackstoneWall
    )
}
pub fn is_door(block: BlockId) -> bool {
    use base::SimplifiedBlockKind::*;
    matches!(
        block.simplified_kind(),
        WoodenDoor | IronDoor | WarpedDoor | CrimsonDoor
    )
}
