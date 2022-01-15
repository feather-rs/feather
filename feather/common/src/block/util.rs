use base::{
    categories::SupportType, BlockId, BlockKind, BlockPosition, FacingCardinal,
    FacingCardinalAndDown, FacingCubic,
};
use libcraft_core::BlockFace;

use crate::World;

pub trait AdjacentBlockHelper {
    fn adjacent_block(&self, pos: BlockPosition, face: BlockFace) -> Option<BlockId>;

    fn adjacent_block_cubic(&self, pos: BlockPosition, dir: FacingCubic) -> Option<BlockId>;

    fn adjacent_block_cardinal(&self, pos: BlockPosition, dir: FacingCardinal) -> Option<BlockId>;

    fn adjacent_block_cardinal_and_down(
        &self,
        pos: BlockPosition,
        dir: FacingCardinalAndDown,
    ) -> Option<BlockId>;

    fn get_facing_block(&self, pos: BlockPosition) -> Option<BlockId>;

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

    fn get_facing_block(&self, pos: BlockPosition) -> Option<BlockId> {
        let block = self.block_at(pos)?;
        let dir = if block.has_facing_cardinal() {
            block.facing_cardinal().unwrap().to_facing_cubic()
        } else if block.has_facing_cardinal_and_down() {
            block.facing_cardinal_and_down().unwrap().to_facing_cubic()
        } else {
            block.facing_cubic()?
        };
        self.adjacent_block_cubic(pos, dir)
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
        Some(if let Some(support_type) = block.support_type() {
            let block_under = self.block_at(pos.down());
            let block_up = self.block_at(pos.up());
            let block_facing = self.get_facing_block(pos);
            match support_type {
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
                        Dirt | CoarseDirt | Podzol | Sand | RedSand
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
            }
        } else {
            true
        })
    }
}
