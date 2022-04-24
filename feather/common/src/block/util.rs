use libcraft::{
    block::AttachedFace, blocks::SupportType, BlockFace, BlockKind, BlockPosition, BlockState,
};
use quill::World;

use quill::block_data::{Directional, FaceAttachable, Waterlogged};

/// Trait that implements helper function for adjacency. This is an extension to `World` that tries to keep this utility logic away from the main implementation
pub trait AdjacentBlockHelper {
    fn adjacent_block(&self, pos: BlockPosition, dir: BlockFace) -> Option<BlockState>;

    fn set_block_adjacent_facing(
        &self,
        pos: BlockPosition,
        block: BlockState,
        dir: BlockFace,
    ) -> bool;

    fn check_block_stability(
        &self,
        block: BlockState,
        pos: BlockPosition,
        light_level: u8,
    ) -> Option<bool>;
}

impl<'a> AdjacentBlockHelper for &'a dyn World {
    fn adjacent_block(&self, pos: BlockPosition, face: BlockFace) -> Option<BlockState> {
        self.block_at(pos.adjacent(face)).ok()
    }

    fn set_block_adjacent_facing(
        &self,
        pos: BlockPosition,
        block: BlockState,
        dir: BlockFace,
    ) -> bool {
        self.set_block_at(pos.adjacent(dir), block).is_ok()
    }

    fn check_block_stability(
        &self,
        block: BlockState,
        pos: BlockPosition,
        light_level: u8,
    ) -> Option<bool> {
        use libcraft::blocks::SimplifiedBlockKind::*;
        let support_type = match block.support_type() {
            Some(s) => s,
            None => return Some(true),
        };
        let block_under = self.block_at(pos.down()).ok();
        let block_up = self.block_at(pos.up()).ok();
        let block_facing = block
            .data_as::<Directional>()
            .map(|f| self.adjacent_block(pos, f.facing()))
            .flatten();
        let is_supported = match support_type {
            SupportType::OnSolid => block_under?.kind().solid(),
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
            SupportType::FacingSolid => block_facing?.kind().solid(),
            SupportType::FacingJungleWood => matches!(
                block_facing?.kind(),
                BlockKind::JungleLog
                    | BlockKind::StrippedJungleLog
                    | BlockKind::JungleWood
                    | BlockKind::StrippedJungleWood
            ),
            SupportType::OnOrFacingSolid => self
                .block_at(
                    pos.adjacent(match block.data_as::<FaceAttachable>()?.attached_face() {
                        AttachedFace::Floor => BlockFace::Bottom,
                        AttachedFace::Wall => block.data_as::<Directional>()?.facing().opposite(),
                        AttachedFace::Ceiling => BlockFace::Top,
                    }),
                )
                .ok()?
                .kind()
                .solid(),
            SupportType::CactusLike => {
                matches!(block_under?.simplified_kind(), Sand | RedSand | Cactus) && {
                    let mut ok = true;
                    for face in [
                        BlockFace::North,
                        BlockFace::South,
                        BlockFace::West,
                        BlockFace::East,
                    ] {
                        let block = self.block_at(pos.adjacent(face)).ok()?;
                        ok &= !block.kind().solid() && block.simplified_kind() != Cactus
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
                .filter_map(|&face| self.block_at(pos.adjacent(face)).ok())
                .map(BlockState::simplified_kind);
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
                    .filter_map(|&f| self.block_at(pos.adjacent(f)).ok())
                    .map(BlockState::simplified_kind);
                let horizontal_down = n
                    .iter()
                    .filter_map(|&f| self.block_at(pos.down().adjacent(f)).ok())
                    .map(BlockState::simplified_kind);
                if horizontal.clone().count() != 4 || horizontal_down.clone().count() != 4 {
                    return None;
                }
                let has_horizontal = horizontal.clone().any(|b| b == ChorusPlant);
                let has_vertical =
                    matches!(block_up?.simplified_kind(), ChorusPlant | ChorusFlower);
                let is_connected = horizontal
                    .zip(horizontal_down)
                    .any(|(h, hd)| h == ChorusPlant && matches!(hd, ChorusPlant | EndStone));
                is_connected && !(has_vertical && has_horizontal && !block_under?.kind().is_air())
            }
            SupportType::MushroomLike => block_under?.kind().solid() && light_level < 13,
            SupportType::SnowLike => {
                block_under?.kind().solid()
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
                        let block = self.block_at(pos.down().adjacent(face)).ok()?;
                        ok |= matches!(block.simplified_kind(), FrostedIce | Water);
                        ok |= block
                            .data_as::<Waterlogged>()
                            .map(|w| w.waterlogged())
                            .unwrap_or(false);
                    }
                    ok
                }
            }
            SupportType::TripwireHookLike => {
                block_facing?.kind().solid()
                    && !matches!(block_facing?.simplified_kind(), RedstoneBlock | Observer)
            }
            SupportType::VineLike => {
                matches!(self.block_at(pos.up()).ok()?.simplified_kind(), Vine) || {
                    let mut ok = false;
                    for face in [
                        BlockFace::North,
                        BlockFace::South,
                        BlockFace::West,
                        BlockFace::East,
                        BlockFace::Top,
                    ] {
                        let block = self.block_at(pos.down().adjacent(face)).ok()?;
                        ok |= block.kind().solid();
                    }
                    ok
                }
            }
        };
        Some(is_supported)
    }
}

/// Checks if the block is a wall. `SimplifiedBlockKind` does not have a common type for walls at this time, making this function neccessary.
pub fn is_wall(block: BlockState) -> bool {
    use libcraft::blocks::SimplifiedBlockKind::*;
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
