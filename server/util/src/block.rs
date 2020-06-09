//! Assorted functionality relating to blocks, including:
//! * The block notify system, where a block update "notifies"
//! adjacent blocks of the update. This is used for spawning
//! falling blocks, for example.
//!
//! The block notify system works as follows: when a block
//! is updated, `on_block_update_notify_adjacent` is called,
//! which checks the blocks adjacent to the updated block.
//! For each adjacent block, `notify_entity_for_block` is called
//! which returns an `Option<EntityBuilder>` containing the components
//! to create for the notify entity. For example, `Some(EntityBuilder::new().with(FallingBlockNotify)`
//! could be returned for `Sand` and `Gravel` variants.
//!
//! `on_block_update_notify_adjacent` then creates an entity with those components.
//! The "notify entity," in this case,
//! acts as a sort of event, as other systems can check for these entities
//! and perform actions based on their components.

use crate::adjacent_blocks;
use feather_core::blocks::{BlockId, BlockKind, Face};
use feather_core::chunk_map::chunk_relative_pos;
use feather_core::util::BlockPosition;
use feather_server_types::{BlockUpdateEvent, Game};
use fecs::{EntityBuilder, World};
use std::cmp::max;
use std::iter;

/// Marker component stating that an entity is a notify entity.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotify;

/// Component storing the position of a block for a block notify entity.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifyPosition(pub BlockPosition);

/// Component storing the type of block notified.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifyBlock(pub BlockId);

/// Marker component for block notify entities created for falling
/// blocks, such as sand and gravel.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifyFallingBlock;

/// Marker component for block notify entities created for falling
/// blocks, such as sand and gravel.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifySupportedBlock;

/// Returns an `EntityBuilder` to create the block notify entity for
/// the given block type.
fn notify_entity_for_block(block: BlockId, pos: BlockPosition) -> Option<EntityBuilder> {
    let builder = EntityBuilder::new()
        .with(BlockNotify)
        .with(BlockNotifyPosition(pos))
        .with(BlockNotifyBlock(block));

    if block.can_fall() {
        Some(builder.with(BlockNotifyFallingBlock))
    } else if does_block_need_support(block.kind()) {
        Some(builder.with(BlockNotifySupportedBlock))
    } else {
        None
    }
}

/// When a block is updated, spawns notify entities
/// for adjacent blocks.
#[fecs::event_handler]
pub fn on_block_update_notify_adjacent(
    event: &BlockUpdateEvent,
    game: &mut Game,
    world: &mut World,
) {
    adjacent_blocks(event.pos)
        .into_iter()
        .chain(iter::once(event.pos))
        .filter_map(|adjacent_pos| {
            if let Some(adjacent_block) = game.block_at(adjacent_pos) {
                Some((adjacent_block, adjacent_pos))
            } else {
                None
            }
        })
        .filter_map(|(adjacent_block, adjacent_pos)| {
            notify_entity_for_block(adjacent_block, adjacent_pos)
        })
        .for_each(|builder| {
            builder.build().spawn_in(world);
        })
}

/// This checks whether a block of a specific BlockId
/// can be placed at a specific position in the world.
/// For example blocks like torches, snow, grass need
/// supported blocks beneath/beside them.
pub fn is_block_supported_at(block_id: BlockId, game: &Game, pos: BlockPosition) -> bool {
    match block_support_check(block_id.kind()) {
        Some(func) => func(block_id, game, pos).unwrap_or(false), // return value of None means tried to check a block in an unloaded chunk TODO how to handle?
        None => true,
    }
}

/// This reports whether a support block might be
/// needed for a block of this BlockKind.
/// See also: `is_block_supported_at`
pub fn does_block_need_support(block_kind: BlockKind) -> bool {
    match block_support_check(block_kind) {
        Some(..) => true,
        None => false,
    }
}

const NORTH: BlockPosition = BlockPosition { x: 0, y: 0, z: -1 };
const EAST: BlockPosition = BlockPosition { x: 1, y: 0, z: 0 };
const SOUTH: BlockPosition = BlockPosition { x: 0, y: 0, z: 1 };
const WEST: BlockPosition = BlockPosition { x: -1, y: 0, z: 0 };
const UP: BlockPosition = BlockPosition { x: 0, y: 1, z: 0 };
const DOWN: BlockPosition = BlockPosition { x: 0, y: -1, z: 0 };

fn facing_offset(id: BlockId) -> BlockPosition {
    id.facing_cardinal().unwrap().opposite().offset()
}

fn face_facing_offset(id: BlockId) -> BlockPosition {
    match id.face().unwrap() {
        Face::Floor => DOWN,
        Face::Ceiling => UP,
        Face::Wall => facing_offset(id),
    }
}

fn block_support_check(
    block_kind: BlockKind,
) -> Option<&'static dyn Fn(BlockId, &Game, BlockPosition) -> Option<bool>> {
    use BlockKind::*;

    // TODO leaves are technically a full block, but e.g. torches can't be placed on them https://minecraft.gamepedia.com/Opacity/Placement
    match block_kind {
        BlockKind::Torch
        | BlockKind::RedstoneTorch
        | BlockKind::RedstoneWire
        | BlockKind::Repeater
        | BlockKind::Rail
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
        | BlockKind::WhiteBanner
        | BlockKind::OrangeBanner
        | BlockKind::MagentaBanner
        | BlockKind::LightBlueBanner
        | BlockKind::YellowBanner
        | BlockKind::LimeBanner
        | BlockKind::PinkBanner
        | BlockKind::GrayBanner
        | BlockKind::LightGrayBanner
        | BlockKind::CyanBanner
        | BlockKind::PurpleBanner
        | BlockKind::BlueBanner
        | BlockKind::BrownBanner
        | BlockKind::GreenBanner
        | BlockKind::RedBanner
        | BlockKind::BlackBanner
        | BlockKind::Seagrass
        | BlockKind::TallSeagrass
        | BlockKind::Kelp
        | BlockKind::KelpPlant
        | BlockKind::Sign
        | BlockKind::StonePressurePlate
        | BlockKind::OakPressurePlate
        | BlockKind::SprucePressurePlate
        | BlockKind::BirchPressurePlate
        | BlockKind::JunglePressurePlate
        | BlockKind::AcaciaPressurePlate
        | BlockKind::DarkOakPressurePlate
        | BlockKind::OakDoor
        | BlockKind::SpruceDoor
        | BlockKind::BirchDoor
        | BlockKind::JungleDoor
        | BlockKind::AcaciaDoor
        | BlockKind::DarkOakDoor
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
        | BlockKind::DeadTubeCoralFan => {
            Some(&|_id, game, pos| Some({ game.block_at(pos + DOWN)?.is_full_block() }))
        }
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
        | BlockKind::DeadTubeCoralWallFan => {
            Some(&|id, game, pos| Some({ game.block_at(pos + facing_offset(id))?.is_full_block() }))
        }
        BlockKind::OakButton
        | BlockKind::SpruceButton
        | BlockKind::BirchButton
        | BlockKind::JungleButton
        | BlockKind::AcaciaButton
        | BlockKind::DarkOakButton
        | BlockKind::StoneButton
        | BlockKind::Lever => Some(&|id, game, pos| {
            Some({ game.block_at(pos + face_facing_offset(id))?.is_full_block() })
        }),
        BlockKind::TripwireHook => Some(&|id, game, pos| {
            Some({
                let support = game.block_at(pos + facing_offset(id))?;

                support.is_full_block() && !matches!(support.kind(), RedstoneBlock | Observer)
            })
        }),
        BlockKind::AttachedMelonStem
        | BlockKind::AttachedPumpkinStem
        | BlockKind::MelonStem
        | BlockKind::PumpkinStem
        | BlockKind::Carrots
        | BlockKind::Potatoes
        | BlockKind::Beetroots
        | BlockKind::Wheat => {
            Some(&|_id, game, pos| Some({ matches!(game.block_at(pos + DOWN)?.kind(), Farmland) }))
        }
        BlockKind::Snow => Some(&|_id, game, pos| {
            Some({
                let support = game.block_at(pos + DOWN)?;

                support.is_full_block() && !matches!(support.kind(), Ice | PackedIce)
            })
        }),
        BlockKind::LilyPad => Some(&|_id, game, pos| {
            Some({ matches!(game.block_at(pos + DOWN)?.kind(), Water | Ice | FrostedIce) })
        }),
        BlockKind::Cocoa => Some(&|id, game, pos| {
            Some({
                matches!(
                    game.block_at(pos + facing_offset(id))?.kind(),
                    JungleLog | StrippedJungleLog | JungleWood | StrippedJungleWood
                )
            })
        }),
        BlockKind::Grass
        | BlockKind::Fern
        | BlockKind::Sunflower
        | BlockKind::Lilac
        | BlockKind::RoseBush
        | BlockKind::Peony
        | BlockKind::TallGrass
        | BlockKind::LargeFern
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
        | BlockKind::OxeyeDaisy => Some(&|_id, game, pos| {
            Some({
                matches!(
                    game.block_at(pos + DOWN)?.kind(),
                    Dirt | GrassBlock | CoarseDirt | Podzol | Farmland
                )
            })
        }),
        BlockKind::BrownMushroom | BlockKind::RedMushroom => Some(&|_id, game, pos| {
            Some({
                let chunk = game.chunk_map.chunk_at(pos.chunk())?;
                let (x, y, z) = chunk_relative_pos(pos + DOWN);

                game.block_at(pos + DOWN)?.is_full_block()
                    && max(chunk.sky_light_at(x, y, z), chunk.block_light_at(x, y, z)) < 13
            })
        }),
        BlockKind::DeadBush => Some(&|_id, game, pos| {
            Some({
                matches!(
                    game.block_at(pos + DOWN)?.kind(),
                    Sand | RedSand
                        | Dirt
                        | CoarseDirt
                        | Podzol
                        | Terracotta
                        | WhiteTerracotta
                        | OrangeTerracotta
                        | MagentaTerracotta
                        | LightBlueTerracotta
                        | YellowTerracotta
                        | LimeTerracotta
                        | PinkTerracotta
                        | GrayTerracotta
                        | LightGrayTerracotta
                        | CyanTerracotta
                        | PurpleTerracotta
                        | BlueTerracotta
                        | BrownTerracotta
                        | GreenTerracotta
                        | RedTerracotta
                        | BlackTerracotta
                        | WhiteCarpet
                        | OrangeCarpet
                        | MagentaCarpet
                        | LightBlueCarpet
                        | YellowCarpet
                        | LimeCarpet
                        | PinkCarpet
                        | GrayCarpet
                        | LightGrayCarpet
                        | CyanCarpet
                        | PurpleCarpet
                        | BlueCarpet
                        | BrownCarpet
                        | GreenCarpet
                        | RedCarpet
                        | BlackCarpet
                )
            })
        }),
        BlockKind::SugarCane => Some(&|_id, game, pos| {
            Some({
                let support = game.block_at(pos + DOWN)?.kind();

                support == SugarCane
                    || (matches!(
                        support,
                        GrassBlock | Dirt | CoarseDirt | Podzol | Sand | RedSand
                    ) && (matches!(
                        game.block_at(pos + DOWN + NORTH)?.kind(),
                        Water | FrostedIce
                    ) || matches!(
                        game.block_at(pos + DOWN + EAST)?.kind(),
                        Water | FrostedIce
                    ) || matches!(
                        game.block_at(pos + DOWN + SOUTH)?.kind(),
                        Water | FrostedIce
                    ) || matches!(
                        game.block_at(pos + DOWN + WEST)?.kind(),
                        Water | FrostedIce
                    )))
            })
        }),
        BlockKind::Vine => Some(&|_id, game, pos| {
            Some({
                let up = game.block_at(pos + UP)?;

                up.is_full_block()
                    || matches!(up.kind(), Vine)
                    || game.block_at(pos + NORTH)?.is_full_block()
                    || game.block_at(pos + EAST)?.is_full_block()
                    || game.block_at(pos + SOUTH)?.is_full_block()
                    || game.block_at(pos + WEST)?.is_full_block()
            })
        }),
        BlockKind::Cactus => Some(&|_id, game, pos| {
            Some({
                let north = game.block_at(pos + NORTH)?;
                let east = game.block_at(pos + EAST)?;
                let south = game.block_at(pos + SOUTH)?;
                let west = game.block_at(pos + WEST)?;

                matches!(game.block_at(pos + DOWN)?.kind(), Cactus | Sand | RedSand)
                    && !matches!(north.kind(), Cactus)
                    && !north.is_full_block()
                    && !matches!(east.kind(), Cactus)
                    && !east.is_full_block()
                    && !matches!(south.kind(), Cactus)
                    && !south.is_full_block()
                    && !matches!(west.kind(), Cactus)
                    && !west.is_full_block()
            })
        }),
        BlockKind::NetherWart => {
            Some(&|_id, game, pos| Some({ matches!(game.block_at(pos + DOWN)?.kind(), SoulSand) }))
        }
        BlockKind::ChorusFlower => Some(&|_id, game, pos| {
            Some({
                let north = game.block_at(pos + NORTH)?;
                let east = game.block_at(pos + EAST)?;
                let south = game.block_at(pos + SOUTH)?;
                let west = game.block_at(pos + WEST)?;

                let neighbours = [north, east, south, west];
                let neighbouring_chorus = neighbours
                    .iter()
                    .filter(|&id| id.kind() == ChorusPlant)
                    .count();
                let neighbouring_air = neighbours.iter().filter(|&id| id.is_air()).count();

                matches!(game.block_at(pos + DOWN)?.kind(), EndStone | ChorusPlant)
                    || (neighbouring_chorus == 1 && neighbouring_air == 3)
            })
        }),
        BlockKind::ChorusPlant => Some(&|_id, game, pos| {
            Some({
                let north = game.block_at(pos + NORTH)?;
                let east = game.block_at(pos + EAST)?;
                let south = game.block_at(pos + SOUTH)?;
                let west = game.block_at(pos + WEST)?;

                let north_down = game.block_at(pos + NORTH + DOWN)?;
                let east_down = game.block_at(pos + EAST + DOWN)?;
                let south_down = game.block_at(pos + SOUTH + DOWN)?;
                let west_down = game.block_at(pos + WEST + DOWN)?;

                let down = game.block_at(pos + DOWN)?;
                let up = game.block_at(pos + UP)?;

                let horizontal = [north, east, south, west];
                let has_horizontal = horizontal.iter().any(|&id| id.kind() == ChorusPlant);
                let has_vertical = matches!(up.kind(), ChorusPlant | ChorusFlower);

                let horizontal_support = [north_down, east_down, south_down, west_down];
                let is_connected = matches!(down.kind(), ChorusPlant | EndStone)
                    || horizontal
                        .iter()
                        .zip(horizontal_support.iter())
                        .any(|(&b, &b_down)| {
                            b.kind() == ChorusPlant
                                && matches!(b_down.kind(), ChorusPlant | EndStone)
                        });

                is_connected && !(has_vertical && has_horizontal && !down.is_air())
            })
        }),
        _ => None,
    }
}
