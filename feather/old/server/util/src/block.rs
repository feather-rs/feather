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
use feather_core::blocks::categories::SupportType;
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
    } else if block.support_type().is_some() {
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
    // return value of None means tried to check a block in an unloaded chunk TODO how to handle?
    check_block_support_at(block_id, game, pos).unwrap_or(false)
}

const NORTH: BlockPosition = BlockPosition { x: 0, y: 0, z: -1 };
const EAST: BlockPosition = BlockPosition { x: 1, y: 0, z: 0 };
const SOUTH: BlockPosition = BlockPosition { x: 0, y: 0, z: 1 };
const WEST: BlockPosition = BlockPosition { x: -1, y: 0, z: 0 };
const UP: BlockPosition = BlockPosition { x: 0, y: 1, z: 0 };
const DOWN: BlockPosition = BlockPosition { x: 0, y: -1, z: 0 };

fn face_facing_offset(id: BlockId) -> BlockPosition {
    match id.face().unwrap() {
        Face::Floor => DOWN,
        Face::Ceiling => UP,
        Face::Wall => id.facing_cardinal().unwrap().opposite().offset(),
    }
}

use feather_core::blocks::SimplifiedBlockKind::*;

fn check_block_support_at(id: BlockId, game: &Game, pos: BlockPosition) -> Option<bool> {
    // TODO leaves are technically a full block, but e.g. torches can't be placed on them https://minecraft.gamepedia.com/Opacity/Placement
    let block_down = game.block_at(pos + DOWN);
    let block_facing = if id.has_facing_cardinal() {
        game.block_at(pos + id.facing_cardinal().unwrap().opposite().offset())
    } else {
        None
    };

    match id.support_type() {
        Some(support_type) => match support_type {
            SupportType::OnSolid => Some(block_down?.is_full_block()),
            SupportType::OnDirtBlocks => Some(matches!(
                block_down?.simplified_kind(),
                Dirt | GrassBlock | CoarseDirt | Podzol | Farmland
            )),
            SupportType::OnDesertBlocks => Some(matches!(
                block_down?.simplified_kind(),
                Sand | RedSand | Dirt | CoarseDirt | Podzol | Terracotta
            )),
            SupportType::OnFarmland => Some(block_down?.simplified_kind() == Farmland),
            SupportType::OnSoulSand => Some(block_down?.simplified_kind() == SoulSand),
            SupportType::OnWater => Some(matches!(
                block_down?.simplified_kind(),
                Water | Ice | FrostedIce
            )),

            SupportType::FacingSolid => Some(block_facing?.is_full_block()),
            SupportType::FacingJungleWood => Some(matches!(
                block_facing?.kind(),
                BlockKind::JungleLog
                    | BlockKind::StrippedJungleLog
                    | BlockKind::JungleWood
                    | BlockKind::StrippedJungleWood
            )),

            SupportType::OnOrFacingSolid => {
                let block_face_facing = game.block_at(pos + face_facing_offset(id))?;

                Some(block_face_facing.is_full_block())
            }

            SupportType::SnowLike => {
                let is_supported = block_down?.is_full_block()
                    && !matches!(block_down?.simplified_kind(), Ice | PackedIce);

                Some(is_supported)
            }
            SupportType::TripwireHookLike => {
                let is_supported = block_facing?.is_full_block()
                    && !matches!(block_facing?.simplified_kind(), RedstoneBlock | Observer);

                Some(is_supported)
            }

            SupportType::CactusLike => block_support_cactus_like(game, pos),
            SupportType::ChorusFlowerLike => block_support_chorus_flower_like(game, pos),
            SupportType::ChorusPlantLike => block_support_chorus_plant_like(game, pos),
            SupportType::MushroomLike => block_support_mushroom_like(game, pos),
            SupportType::SugarCaneLike => block_support_sugar_cane_like(game, pos),
            SupportType::VineLike => block_support_vine_like(game, pos),
        },
        None => Some(true),
    }
}

fn block_support_cactus_like(game: &Game, pos: BlockPosition) -> Option<bool> {
    let north = game.block_at(pos + NORTH)?;
    let east = game.block_at(pos + EAST)?;
    let south = game.block_at(pos + SOUTH)?;
    let west = game.block_at(pos + WEST)?;

    let is_supported = matches!(
        game.block_at(pos + DOWN)?.simplified_kind(),
        Cactus | Sand | RedSand
    ) && north.simplified_kind() != Cactus
        && !north.is_full_block()
        && east.simplified_kind() != Cactus
        && !east.is_full_block()
        && south.simplified_kind() != Cactus
        && !south.is_full_block()
        && west.simplified_kind() != Cactus
        && !west.is_full_block();

    Some(is_supported)
}

fn block_support_chorus_flower_like(game: &Game, pos: BlockPosition) -> Option<bool> {
    let north = game.block_at(pos + NORTH)?;
    let east = game.block_at(pos + EAST)?;
    let south = game.block_at(pos + SOUTH)?;
    let west = game.block_at(pos + WEST)?;

    let neighbours = [north, east, south, west];
    let neighbouring_chorus = neighbours
        .iter()
        .filter(|&id| id.simplified_kind() == ChorusPlant)
        .count();
    let neighbouring_air = neighbours.iter().filter(|&id| id.is_air()).count();

    let is_supported = matches!(
        game.block_at(pos + DOWN)?.simplified_kind(),
        EndStone | ChorusPlant
    ) || (neighbouring_chorus == 1 && neighbouring_air == 3);

    Some(is_supported)
}

fn block_support_chorus_plant_like(game: &Game, pos: BlockPosition) -> Option<bool> {
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
    let has_horizontal = horizontal
        .iter()
        .any(|&id| id.simplified_kind() == ChorusPlant);
    let has_vertical = matches!(up.simplified_kind(), ChorusPlant | ChorusFlower);

    let horizontal_support = [north_down, east_down, south_down, west_down];
    let is_connected = matches!(down.simplified_kind(), ChorusPlant | EndStone)
        || horizontal
            .iter()
            .zip(horizontal_support.iter())
            .any(|(&b, &b_down)| {
                b.simplified_kind() == ChorusPlant
                    && matches!(b_down.simplified_kind(), ChorusPlant | EndStone)
            });

    let is_supported = is_connected && !(has_vertical && has_horizontal && !down.is_air());

    Some(is_supported)
}

fn block_support_mushroom_like(game: &Game, pos: BlockPosition) -> Option<bool> {
    let chunk = game.chunk_map.chunk_at(pos.chunk())?;
    let (x, y, z) = chunk_relative_pos(pos + DOWN);

    let is_supported = game.block_at(pos + DOWN)?.is_full_block()
        && max(chunk.sky_light_at(x, y, z), chunk.block_light_at(x, y, z)) < 13;

    Some(is_supported)
}

fn block_support_sugar_cane_like(game: &Game, pos: BlockPosition) -> Option<bool> {
    let support = game.block_at(pos + DOWN)?.simplified_kind();

    let is_supported = support == SugarCane
        || (matches!(
            support,
            GrassBlock | Dirt | CoarseDirt | Podzol | Sand | RedSand
        ) && (matches!(
            game.block_at(pos + DOWN + NORTH)?.simplified_kind(),
            Water | FrostedIce
        ) || matches!(
            game.block_at(pos + DOWN + EAST)?.simplified_kind(),
            Water | FrostedIce
        ) || matches!(
            game.block_at(pos + DOWN + SOUTH)?.simplified_kind(),
            Water | FrostedIce
        ) || matches!(
            game.block_at(pos + DOWN + WEST)?.simplified_kind(),
            Water | FrostedIce
        )));

    Some(is_supported)
}

fn block_support_vine_like(game: &Game, pos: BlockPosition) -> Option<bool> {
    let up = game.block_at(pos + UP)?;

    let is_supported = up.is_full_block()
        || up.simplified_kind() == Vine
        || game.block_at(pos + NORTH)?.is_full_block()
        || game.block_at(pos + EAST)?.is_full_block()
        || game.block_at(pos + SOUTH)?.is_full_block()
        || game.block_at(pos + WEST)?.is_full_block();

    Some(is_supported)
}
