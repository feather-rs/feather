use std::convert::TryInto;

use super::util::{is_door, AdjacentBlockHelper};
use super::wall::update_wall_connections;
use crate::chunk::entities::ChunkEntities;
use crate::entities::player::HotbarSlot;
use crate::events::BlockChangeEvent;
use crate::{Game, World};
use base::{
    Area, BlockId, BlockPosition, Face, FacingCardinal, FacingCardinalAndDown, FacingCubic,
    Gamemode, HalfTopBottom, HalfUpperLower, Hinge, Inventory, Item, ItemStack, Position,
    SimplifiedBlockKind, SlabKind, Vec3d,
};
use blocks::BlockKind;
use ecs::{Ecs, SysResult};
use libcraft_core::{BlockFace, EntityKind};
use libcraft_items::InventorySlot;
use quill_common::components::CanBuild;
use quill_common::events::BlockPlacementEvent;
use utils::continue_on_none;
use vek::Rect3;
/// A system that handles block placement events.
pub fn block_placement(game: &mut Game) -> SysResult {
    let mut events = vec![];
    for (_, (event, gamemode, inv, hotbar, player_pos, can_build)) in game
        .ecs
        .query::<(
            &BlockPlacementEvent,
            &Gamemode,
            &mut Inventory,
            &HotbarSlot,
            &Position,
            &CanBuild,
        )>()
        .iter()
    {
        // Get selected slot
        let mut slot = match event.hand {
            libcraft_core::Hand::Main => inv.item(Area::Hotbar, hotbar.get()),
            libcraft_core::Hand::Offhand => inv.item(Area::Offhand, 0),
        }
        .unwrap();
        // Check that the player has an item in hand and can build
        if !can_build.0 || slot.is_empty() {
            continue;
        }
        let block = continue_on_none!(item_to_block(slot.item_kind().unwrap()));
        continue_on_none!(place_block(
            &mut game.world,
            &game.chunk_entities,
            &game.ecs,
            *player_pos,
            block,
            event,
            0,
            &mut events
        ));
        if matches!(*gamemode, Gamemode::Survival | Gamemode::Adventure) {
            decrease_slot(&mut slot)
        }
    }
    for e in events {
        game.ecs.insert_event(e);
    }
    Ok(())
}
/// Check if the block has a bed portion that can be placed placed in the world. `None` means that while the block has a head, it cannot be placed
fn should_place_bed_head(
    world: &World,
    block: BlockId,
    placement_pos: BlockPosition,
) -> Option<bool> {
    if bed_head(block).is_none() {
        return Some(false);
    }
    if world
        .adjacent_block_cardinal(placement_pos, block.facing_cardinal()?)?
        .is_replaceable()
    {
        Some(true)
    } else {
        None
    }
}
/// Check if the block has an upper half that can be placed in the world. `None` means that while the block has an upper half, it cannot be placed
fn should_place_upper_half(
    world: &World,
    block: BlockId,
    placement_pos: BlockPosition,
) -> Option<bool> {
    if top_half(block).is_none() {
        return Some(false);
    }
    if world.block_at(placement_pos.up())?.is_replaceable() {
        Some(true)
    } else {
        None
    }
}
/// Check if the block collides with any entity.
/// This works but there is a discrepancy between when the place block event is fired and getting the entity location.
/// that makes it possible to place a block at the right exact moment and have the server believe it wasn't blocked.
fn entity_collisions_ok(
    chunk_entities: &ChunkEntities,
    ecs: &Ecs,
    placement_pos: BlockPosition,
) -> bool {
    !chunk_entities
        .entities_in_chunk(placement_pos.chunk())
        .iter()
        .any(|&entity| {
            let entity_position = ecs.get::<Position>(entity).unwrap();
            let entity_kind = *ecs.get::<EntityKind>(entity).unwrap();
            let block_rect: Rect3<f64, f64> = vek::Rect3 {
                x: placement_pos.x.into(),
                y: placement_pos.y.into(),
                z: placement_pos.z.into(),
                w: 1.0,
                h: 1.0,
                d: 1.0,
            };

            let mut entity_rect = entity_kind.bounding_box().into_rect3();
            entity_rect.x = entity_position.x - (entity_rect.w / 2.0);
            entity_rect.y = entity_position.y;
            entity_rect.z = entity_position.z - (entity_rect.d / 2.0);

            block_rect.collides_with_rect3(entity_rect)
        })
}
/// Place multiple blocks and generate corresponding events.
fn multi_place(
    world: &mut World,
    placements: &[(BlockId, BlockPosition)],
    event_buffer: &mut Vec<BlockChangeEvent>,
) {
    for &(block, pos) in placements {
        event_buffer.push(BlockChangeEvent::single(
            pos.try_into().expect("valid block positions only"),
        ));
        assert!(world.set_block_at(pos, block), "block has to be loaded");
    }
}
/// Try placing the block by merging (=placing to the same position as the target block). Includes merging slabs, waterlogging and replacing blocks like grass.
/// Only one of the following conditions can be met at a time, so short-circuiting is ok.
/// The combinations include top slab & bottom slab, waterloggable block & water and any block & a replaceable block
fn basic_place(block: &mut BlockId, target_block: BlockId) -> bool {
    merge_slabs_in_place(block, target_block)
        || waterlog(block, target_block)
        || can_replace(*block, target_block)
}
/// Check if `block` can replace `target`.
fn can_replace(block: BlockId, target: BlockId) -> bool {
    (block.kind() != target.kind()) && target.is_replaceable()
}
/// Blocks get changed if they are getting waterlogged or when they are slabs turning into full blocks.
/// Blocks get replaced in-place when possible, while changing an adjacent block has a lower priority.
#[allow(clippy::too_many_arguments)]
fn place_block(
    world: &mut World,
    chunk_entities: &ChunkEntities,
    ecs: &Ecs,
    player_pos: Position,
    mut block: BlockId,
    placement: &BlockPlacementEvent,
    light_level: u8,
    event_buffer: &mut Vec<BlockChangeEvent>,
) -> Option<()> {
    let target1 = placement.location;
    let target_block1 = world.block_at(target1)?;
    let target2 = target1.adjacent(placement.face);
    let target_block2 = world.block_at(target2);
    // Cannot build on air
    if target_block1.is_air() {
        return None;
    }
    let player_dir_ordered = ordered_directions(player_pos.direction());
    slab_to_place(&mut block, target_block1, placement);
    // Select where to place the block
    let target = if basic_place(&mut block, target_block1) {
        target1
    } else if basic_place(&mut block, target_block2?) {
        target2
    } else {
        // Cannot place block
        return None;
    };
    set_face(&mut block, &player_dir_ordered, placement);
    rotate_8dir(&mut block, player_pos.yaw);
    door_hinge(&mut block, target, &placement.cursor_position, world);
    if !entity_collisions_ok(chunk_entities, ecs, target)
        || !world.check_block_stability(block, target, light_level)?
    {
        return None;
    }
    let primary_placement = (block, target);
    if should_place_upper_half(world, block, target)? {
        let secondary_placement = (
            block.with_half_upper_lower(HalfUpperLower::Upper),
            target.up(),
        );
        multi_place(
            world,
            &[primary_placement, secondary_placement],
            event_buffer,
        )
    } else if should_place_bed_head(world, block, target)? {
        let face = match block.facing_cardinal()? {
            FacingCardinal::North => BlockFace::North,
            FacingCardinal::South => BlockFace::South,
            FacingCardinal::West => BlockFace::West,
            FacingCardinal::East => BlockFace::East,
        };
        let secondary_placement = (block.with_part(base::Part::Head), target.adjacent(face));
        multi_place(
            world,
            &[primary_placement, secondary_placement],
            event_buffer,
        )
    } else {
        multi_place(world, &[primary_placement], event_buffer);
    };
    update_wall_connections(world, target).unwrap();
    Some(())
}
/// Sets the hinge position on a door block. The door attempts to connect to other doors first, then to solid blocks. Otherwise, the hinge position is determined by the click position.
#[allow(clippy::float_cmp)]
fn door_hinge(
    block: &mut BlockId,
    pos: BlockPosition,
    cursor_pos: &[f32],
    world: &World,
) -> Option<()> {
    let cardinal = block.facing_cardinal()?;
    let left = cardinal.left();
    let right = cardinal.right();
    let lb = world.adjacent_block_cardinal(pos, left)?;
    let rb = world.adjacent_block_cardinal(pos, right)?;
    if is_door(lb) && lb.kind() == block.kind() {
        block.set_hinge(Hinge::Right);
        return Some(());
    }
    if is_door(rb) && rb.kind() == block.kind() {
        block.set_hinge(Hinge::Left);
        return Some(());
    }
    let lt = world.adjacent_block_cardinal(pos.up(), left)?;
    let rt = world.adjacent_block_cardinal(pos.up(), right)?;
    let solid_left = is_block_solid(lb) || is_block_solid(lt);
    let solid_right = is_block_solid(rb) || is_block_solid(rt);
    if solid_left && !solid_right {
        block.set_hinge(Hinge::Left);
        return Some(());
    }
    if solid_right && !solid_left {
        block.set_hinge(Hinge::Right);
        return Some(());
    }
    let relevant_axis = match cardinal {
        FacingCardinal::North => cursor_pos[0],
        FacingCardinal::South => 1.0 - cursor_pos[0],
        FacingCardinal::West => 1.0 - cursor_pos[2],
        FacingCardinal::East => cursor_pos[2],
    };
    let hinge = if relevant_axis < 0.5 {
        Hinge::Left
    } else {
        Hinge::Right
    };
    block.set_hinge(hinge);
    Some(())
}

fn is_block_solid(block: BlockId) -> bool {
    block.is_solid()
        && !matches!(
            block.slab_kind(),
            Some(SlabKind::Bottom) | Some(SlabKind::Top)
        )
}

/// Gets the top half of the block. Works with doors, flowers, tall grass etc.
fn top_half(block: BlockId) -> Option<BlockId> {
    if block.has_half_upper_lower() {
        Some(block.with_half_upper_lower(HalfUpperLower::Upper))
    } else {
        None
    }
}

/// If applicable, this function returns a matching bed head to its foot part.
fn bed_head(block: BlockId) -> Option<BlockId> {
    if block.has_part() {
        Some(block.with_part(base::Part::Head))
    } else {
        None
    }
}

/// If applicable, rotates 8-directional blocks like banners and signs. The yaw is a property of the placing entity.
fn rotate_8dir(block: &mut BlockId, yaw: f32) {
    block.set_rotation(((yaw + 180.0) / 22.5).round() as i32);
}

/// Orders all the cubic directions(`FacingCubic`) by how well they represent the direction.
pub fn ordered_directions(direction: Vec3d) -> [FacingCubic; 6] {
    let x_dir = if direction[0] > 0.0 {
        FacingCubic::East
    } else {
        FacingCubic::West
    };
    let y_dir = if direction[1] > 0.0 {
        FacingCubic::Up
    } else {
        FacingCubic::Down
    };
    let z_dir = if direction[2] > 0.0 {
        FacingCubic::South
    } else {
        FacingCubic::North
    };
    let abs_x = direction[0].abs();
    let abs_y = direction[1].abs();
    let abs_z = direction[2].abs();
    let t = match (abs_x > abs_y, abs_y > abs_z, abs_z > abs_x) {
        (true, true, true) => unreachable!(),
        (true, true, false) => [x_dir, y_dir, z_dir], // 2 1 0
        (true, false, true) => [z_dir, x_dir, y_dir], // 1 0 2
        (true, false, false) => [x_dir, z_dir, y_dir], // 2 0 1
        (false, true, true) => [y_dir, z_dir, x_dir], // 0 2 1
        (false, true, false) => [y_dir, x_dir, z_dir], // 1 2 0
        (false, false, true) => [z_dir, y_dir, x_dir], // 0 1 2
        (false, false, false) => unreachable!(),
    };
    [
        t[0],
        t[1],
        t[2],
        t[2].opposite(),
        t[1].opposite(),
        t[0].opposite(),
    ]
}

// Attempts to merge the two blocks, in place.
fn merge_slabs_in_place(block: &mut BlockId, target: BlockId) -> bool {
    if block.kind() != target.kind() {
        return false;
    }
    let opt = match (block.slab_kind(), target.slab_kind()) {
        (Some(slab1), Some(slab2)) => match (slab1, slab2) {
            (SlabKind::Top, SlabKind::Bottom) | (SlabKind::Bottom, SlabKind::Top) => {
                Some(block.with_slab_kind(SlabKind::Double))
            }

            _ => None,
        },
        _ => None,
    };
    *block = opt.unwrap_or(*block);
    opt.is_some()
}
/// Determine what kind of slab to place. Returns `true` if the slab placed would be merged with the other slab. `try_merge_slabs` should always succeed if this function returns `true`.
#[allow(clippy::float_cmp)]
fn slab_to_place(block: &mut BlockId, target: BlockId, placement: &BlockPlacementEvent) -> bool {
    let (slab_kind, place_adjacent) = match placement.cursor_position[1] {
        y if y == 0.5 => {
            if let Some(k) = target.slab_kind() {
                if block.kind() != target.kind() {
                    match k {
                        SlabKind::Top => (SlabKind::Top, false),
                        SlabKind::Bottom => (SlabKind::Bottom, false),
                        SlabKind::Double => return false,
                    }
                } else {
                    match k {
                        SlabKind::Top => (SlabKind::Bottom, true),
                        SlabKind::Bottom => (SlabKind::Top, true),
                        SlabKind::Double => return false,
                    }
                }
            } else {
                return false;
            }
        }
        y if y == 0.0 => (SlabKind::Top, false),
        y if matches!(placement.face, BlockFace::Top) || y == 1.0 => (SlabKind::Bottom, false),
        y if y < 0.5 => (SlabKind::Bottom, false),
        y if y < 1.0 => (SlabKind::Top, false),
        _ => return false,
    };
    block.set_slab_kind(slab_kind);
    place_adjacent
}

/// This function determines the result of combining the 2 blocks. If one is water and the other is waterloggable, the target is waterlogged. Has no effect otherwise.
fn waterlog(target_block: &mut BlockId, to_place: BlockId) -> bool {
    // Select the non-water block or return
    let mut waterloggable = if matches!(target_block.kind(), BlockKind::Water) {
        to_place
    } else if matches!(to_place.kind(), BlockKind::Water) {
        *target_block
    } else {
        return false;
    };
    // Refuse to waterlog double slabs and blocks that are already waterlogged
    if waterloggable
        .slab_kind()
        .map(|e| matches!(e, SlabKind::Double))
        .unwrap_or(false)
        | waterloggable.waterlogged().unwrap_or(false)
    {
        return false;
    }

    if waterloggable.set_waterlogged(true) {
        *target_block = waterloggable;
        // Campfires are extinguished when waterlogged
        target_block.set_lit(false);
        true
    } else {
        false
    }
}

/// Checks if the block facing is relative to the player. If this function returns `false`, the block facing is determined by the face of the block this one was placed on.
fn is_player_relative(kind: SimplifiedBlockKind) -> bool {
    use SimplifiedBlockKind::*;
    matches!(
        kind,
        Dispenser
            | StickyPiston
            | Piston
            | CommandBlock
            | Observer
            | Dropper
            | Furnace
            | Smoker
            | Chest
            | TrappedChest
            | BlastFurnace
            | CarvedPumpkin
            | JackOLantern
            | BeeNest
            | Beehive
            | EndPortalFrame
            | Anvil
            | EnderChest
            | Bed
            | Loom
            | Banner
            | Sign
            | FenceGate
            | Repeater
            | Comparator
            | Lectern
            | Rail
            | PoweredRail
            | ActivatorRail
            | DetectorRail
            | Stonecutter
            | WoodenDoor
            | IronDoor
            | WarpedDoor
            | CrimsonDoor
            | Stairs
    )
}

/// Checks if the block is to be placed with an opposite facing relative to the player.
fn is_reverse_placed(kind: SimplifiedBlockKind) -> bool {
    use SimplifiedBlockKind::*;
    matches!(
        kind,
        Observer | Bed | FenceGate | WoodenDoor | IronDoor | WarpedDoor | CrimsonDoor | Stairs
    )
}

/// Changes the block facing as necessary. This includes calling `make_wall_block`, determining how the block is placed, setting its facing, cubic, cardinal, cardinal&down, top/bottom and the xyz axis.
/// Blocks have only one of these orientations.
fn set_face(
    block: &mut BlockId,
    player_directions: &[FacingCubic],
    placement: &BlockPlacementEvent,
) {
    if !matches!(placement.face, BlockFace::Top) {
        make_wall_block(block);
    }
    let player_relative = is_player_relative(block.simplified_kind());
    let cubic_facing = match player_relative {
        true => player_directions[0].opposite(),
        false => match placement.face {
            BlockFace::Bottom => FacingCubic::Down,
            BlockFace::Top => FacingCubic::Up,
            BlockFace::North => FacingCubic::North,
            BlockFace::South => FacingCubic::South,
            BlockFace::West => FacingCubic::West,
            BlockFace::East => FacingCubic::East,
        },
    };
    let cubic_facing = {
        if is_reverse_placed(block.simplified_kind()) {
            cubic_facing.opposite()
        } else {
            cubic_facing
        }
    };
    block.set_face(match placement.face {
        BlockFace::Bottom => Face::Ceiling,
        BlockFace::Top => Face::Floor,
        BlockFace::North => Face::Wall,
        BlockFace::South => Face::Wall,
        BlockFace::West => Face::Wall,
        BlockFace::East => Face::Wall,
    });
    let cardinal = cubic_facing.to_facing_cardinal().unwrap_or_else(|| {
        if player_relative {
            if is_reverse_placed(block.simplified_kind()) {
                player_directions[1]
            } else {
                player_directions[1].opposite()
            }
            .to_facing_cardinal()
            .unwrap()
        } else {
            player_directions[0]
                .to_facing_cardinal()
                .unwrap_or_else(|| player_directions[1].to_facing_cardinal().unwrap())
        }
    });
    block.set_facing_cardinal(match block.simplified_kind() {
        SimplifiedBlockKind::Anvil => cardinal.left(),
        _ => cardinal,
    });
    block.set_axis_xyz(cubic_facing.axis());
    block.set_facing_cardinal_and_down(
        cubic_facing
            .opposite()
            .to_facing_cardinal_and_down()
            .unwrap_or(FacingCardinalAndDown::Down),
    );
    block.set_facing_cubic(cubic_facing);
    block.set_half_top_bottom(match placement.face {
        BlockFace::Top => HalfTopBottom::Bottom,
        BlockFace::Bottom => HalfTopBottom::Top,
        _ => match placement.cursor_position[1] {
            y if y <= 0.5 => HalfTopBottom::Bottom,
            _ => HalfTopBottom::Top,
        },
    });
}

/// If possible, turns a free standing block to its wall mounted counterpart, as they are considered different. This applies to torches, signs and banners.
fn make_wall_block(block: &mut BlockId) {
    *block = match block.kind() {
        BlockKind::Torch => BlockId::wall_torch(),
        BlockKind::RedstoneTorch => BlockId::redstone_wall_torch(),
        BlockKind::SoulTorch => BlockId::soul_wall_torch(),
        BlockKind::OakSign => BlockId::oak_wall_sign(),
        BlockKind::BirchSign => BlockId::birch_wall_sign(),
        BlockKind::AcaciaSign => BlockId::acacia_wall_sign(),
        BlockKind::JungleSign => BlockId::jungle_wall_sign(),
        BlockKind::SpruceSign => BlockId::spruce_wall_sign(),
        BlockKind::WarpedSign => BlockId::warped_wall_sign(),
        BlockKind::CrimsonSign => BlockId::crimson_wall_sign(),
        BlockKind::DarkOakSign => BlockId::dark_oak_wall_sign(),
        BlockKind::RedBanner => BlockId::red_wall_banner(),
        BlockKind::BlueBanner => BlockId::blue_wall_banner(),
        BlockKind::CyanBanner => BlockId::cyan_wall_banner(),
        BlockKind::GrayBanner => BlockId::gray_wall_banner(),
        BlockKind::LimeBanner => BlockId::lime_wall_banner(),
        BlockKind::PinkBanner => BlockId::pink_wall_banner(),
        BlockKind::BlackBanner => BlockId::black_wall_banner(),
        BlockKind::BrownBanner => BlockId::brown_wall_banner(),
        BlockKind::GreenBanner => BlockId::green_wall_banner(),
        BlockKind::WhiteBanner => BlockId::white_wall_banner(),
        BlockKind::OrangeBanner => BlockId::orange_wall_banner(),
        BlockKind::PurpleBanner => BlockId::purple_wall_banner(),
        BlockKind::YellowBanner => BlockId::yellow_wall_banner(),
        BlockKind::MagentaBanner => BlockId::magenta_wall_banner(),
        BlockKind::LightBlueBanner => BlockId::light_blue_wall_banner(),
        BlockKind::LightGrayBanner => BlockId::light_gray_wall_banner(),
        _ => *block,
    };
}

/// Attempts to convert an item to its placeable counterpart. Buckets to their respective contents, redstone dust to redstone wire and string to tripwire
fn item_to_block(item: Item) -> Option<BlockId> {
    Some(match item {
        Item::WaterBucket => BlockId::water(),
        Item::LavaBucket => BlockId::lava(),
        Item::Redstone => BlockId::redstone_wire(),
        Item::FlintAndSteel => BlockId::fire(),
        Item::String => BlockId::tripwire(),
        i => {
            let mut name = "minecraft:".to_owned();
            name.push_str(i.name());
            return BlockId::from_identifier(&name);
        }
    })
}

/// Reduces the amount of items in a slot. Full buckets are emptied instead. There are no checks if the item can be placed.
fn decrease_slot(slot: &mut InventorySlot) {
    match slot.item_kind().unwrap() {
        Item::WaterBucket | Item::LavaBucket => {
            *slot = InventorySlot::Filled(ItemStack::new(Item::Bucket, 1).unwrap())
        }
        Item::FlintAndSteel => {
            if match slot {
                InventorySlot::Filled(f) => f.damage(1),
                InventorySlot::Empty => unreachable!(),
            } {
                *slot = InventorySlot::Empty
            }
        }
        _ => {
            // Can always take at least one
            slot.try_take(1);
        }
    }
}
