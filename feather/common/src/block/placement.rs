use std::convert::TryInto;

use super::util::AdjacentBlockHelper;
use super::wall::update_wall_connections;
use crate::events::BlockChangeEvent;
use crate::Game;
use libcraft::block::{AttachedFace, BedPart, BlockHalf, DoorHinge, SlabType};
use libcraft::blocks::SimplifiedBlockKind;
use libcraft::{BlockFace, BlockKind, BlockPosition, BlockState, Item, ItemStack, Position, Vec3d};
use quill::block_data::{
    Bed, Campfire, Directional, Door, FaceAttachable, Orientable, Slab, Waterlogged,
};
use quill::components::{EntityKindComponent, EntityPosition};
use quill::events::BlockPlacementEvent;
use quill::InventorySlot;
use quill::World;
use vek::Rect3;

/// Check if the block has a bed portion that can be placed placed in the world. `None` means that while the block has a head, it cannot be placed
fn should_place_bed_head(
    world: &dyn World,
    block: BlockState,
    placement_pos: BlockPosition,
) -> Option<bool> {
    if bed_head(block).is_none() {
        return Some(false);
    }
    if world
        .adjacent_block(placement_pos, block.data_as::<Directional>()?.facing())?
        .is_replaceable()
    {
        Some(true)
    } else {
        None
    }
}
/// Check if the block has an upper half that can be placed in the world. `None` means that while the block has an upper half, it cannot be placed
fn should_place_upper_half(
    world: &dyn World,
    block: BlockState,
    placement_pos: BlockPosition,
) -> Option<bool> {
    if top_half(block).is_none() {
        return Some(false);
    }
    if world.block_at(placement_pos.up()).ok()?.is_replaceable() {
        Some(true)
    } else {
        None
    }
}
/// Check if the block collides with any entity.
fn entity_collisions_ok(game: &Game, placement_pos: BlockPosition) -> bool {
    !game
        .chunk_entities
        .entities_in_chunk(placement_pos.chunk())
        .iter()
        .any(|&entity| {
            let entity_position = game.ecs.get::<EntityPosition>(entity).unwrap().0;
            let _entity_kind = game.ecs.get::<EntityKindComponent>(entity).unwrap().0;
            let block_rect: Rect3<f64, f64> = vek::Rect3 {
                x: placement_pos.x.into(),
                y: placement_pos.y.into(),
                z: placement_pos.z.into(),
                w: 1.0,
                h: 1.0,
                d: 1.0,
            };

            // TODO use entity-specific bounding box
            let mut entity_rect = Rect3 {
                x: 0.,
                y: 0.,
                z: 0.,
                w: 0.5,
                h: 2.0,
                d: 0.5,
            };
            entity_rect.x = entity_position.x - (entity_rect.w / 2.0);
            entity_rect.y = entity_position.y;
            entity_rect.z = entity_position.z - (entity_rect.d / 2.0);

            block_rect.collides_with_rect3(entity_rect)
        })
}
/// Place multiple blocks and generate corresponding events.
fn multi_place(
    world: &mut dyn World,
    placements: &[(BlockState, BlockPosition)],
    event_buffer: &mut Vec<BlockChangeEvent>,
) {
    for &(block, pos) in placements {
        event_buffer.push(BlockChangeEvent::single(
            pos.try_into().expect("valid block positions only"),
            world.id(),
        ));
        world
            .set_block_at(pos, block)
            .expect("block has to be loaded");
    }
}
/// Try placing the block by merging (=placing to the same position as the target block). Includes merging slabs, waterlogging and replacing blocks like grass.
/// Only one of the following conditions can be met at a time, so short-circuiting is ok.
/// The combinations include top slab & bottom slab, waterloggable block & water and any block & a replaceable block
fn basic_place(block: &mut BlockState, target_block: BlockState, face: BlockFace) -> bool {
    merge_slabs_in_place(block, target_block, face)
        || waterlog(block, target_block)
        || can_replace(*block, target_block)
}
/// Check if `block` can replace `target`.
fn can_replace(block: BlockState, target: BlockState) -> bool {
    (block.kind() != target.kind()) && target.is_replaceable()
}
/// Blocks get changed if they are getting waterlogged or when they are slabs turning into full blocks.
/// Blocks get replaced in-place when possible, while changing an adjacent block has a lower priority.
pub fn place_block(
    game: &Game,
    world: &mut dyn World,
    player_pos: Position,
    mut block: BlockState,
    placement: &BlockPlacementEvent,
    light_level: u8,
    event_buffer: &mut Vec<BlockChangeEvent>,
) -> Option<()> {
    let target1 = placement.location;
    let target_block1 = world.block_at(target1).ok()?;
    let target2 = target1.adjacent(placement.face);
    let target_block2 = world.block_at(target2);
    // Cannot build on air
    if target_block1.kind().is_air() {
        return None;
    }
    let player_dir_ordered = ordered_directions(player_pos.direction());
    slab_to_place(&mut block, target_block1, placement);
    // Select where to place the block
    let target = if basic_place(&mut block, target_block1, placement.face) {
        target1
    } else if basic_place(&mut block, target_block2.ok()?, placement.face) {
        target2
    } else {
        // Cannot place block
        return None;
    };
    set_face(&mut block, &player_dir_ordered, placement);
    // rotate_8dir(&mut block, player_pos.yaw);
    door_hinge(&mut block, target, &placement.cursor_position, world);
    if !entity_collisions_ok(game, target)
        || !(&*world).check_block_stability(block, target, light_level)?
    {
        return None;
    }
    let primary_placement = (block, target);
    if should_place_upper_half(world, block, target)? {
        let top_block =
            block.with_data(block.data_as::<Door>().unwrap().with_half(BlockHalf::Upper));
        let secondary_placement = (top_block, target.up());
        multi_place(
            world,
            &[primary_placement, secondary_placement],
            event_buffer,
        )
    } else if should_place_bed_head(world, block, target)? {
        let face = block.data_as::<Directional>().unwrap().facing();
        let secondary_placement = (
            block.with_data(block.data_as::<Bed>().unwrap().with_part(BedPart::Head)),
            target.adjacent(face),
        );
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
    block: &mut BlockState,
    pos: BlockPosition,
    cursor_pos: &[f32],
    world: &dyn World,
) -> Option<()> {
    if let Some(facing) = block.data_as::<Directional>().map(|dir| dir.facing()) {
        let left = facing.left();
        let right = facing.right();
        let lb = world.adjacent_block(pos, left)?;
        let rb = world.adjacent_block(pos, right)?;

        if let Some(door) = block.data_as::<Door>() {
            if lb.kind() == block.kind() {
                block.set_data(door.with_hinge(DoorHinge::Right));
                return Some(());
            } else if rb.kind() == block.kind() {
                block.set_data(door.with_hinge(DoorHinge::Left));
                return Some(());
            }

            let lt = world.adjacent_block(pos.up(), left)?;
            let rt = world.adjacent_block(pos.up(), right)?;
            let solid_left = is_block_solid(lb) || is_block_solid(lt);
            let solid_right = is_block_solid(rb) || is_block_solid(rt);
            if solid_left && !solid_right {
                block.set_data(door.with_hinge(DoorHinge::Left));
                return Some(());
            }
            if solid_right && !solid_left {
                block.set_data(door.with_hinge(DoorHinge::Right));
                return Some(());
            }
            let relevant_axis = match facing {
                BlockFace::North => cursor_pos[0],
                BlockFace::South => 1.0 - cursor_pos[0],
                BlockFace::West => 1.0 - cursor_pos[2],
                BlockFace::East => cursor_pos[2],
                _ => unreachable!(),
            };
            let hinge = if relevant_axis < 0.5 {
                DoorHinge::Left
            } else {
                DoorHinge::Right
            };
            block.set_data(door.with_hinge(hinge));
        }

        Some(())
    } else {
        None
    }
}

fn is_block_solid(block: BlockState) -> bool {
    block.kind().solid()
        && !matches!(
            block.data_as::<Slab>().map(|s| s.slab_type()),
            Some(SlabType::Bottom) | Some(SlabType::Top)
        )
}

/// Gets the top half of the block. Works with doors, flowers, tall grass etc.
fn top_half(block: BlockState) -> Option<BlockState> {
    if let Some(door) = block.data_as::<Door>() {
        Some(block.with_data(door.with_half(BlockHalf::Upper)))
    } else {
        None
    }
}

/// If applicable, this function returns a matching bed head to its foot part.
fn bed_head(block: BlockState) -> Option<BlockState> {
    if let Some(bed) = block.data_as::<Bed>() {
        Some(block.with_data(bed.with_part(BedPart::Head)))
    } else {
        None
    }
}

/*
/// If applicable, rotates 8-directional blocks like banners and signs. The yaw is a property of the placing entity.
fn rotate_8dir(block: &mut BlockState, yaw: f32) {
    if let Some(mut rot) = block.data_as::<Rotatable>() {

    }
    block.set_rotation(((yaw + 180.0) / 22.5).round() as i32);
}
*/

/// Orders all the cubic directions(`FacingCubic`) by how well they represent the direction.
pub fn ordered_directions(direction: Vec3d) -> [BlockFace; 6] {
    let x_dir = if direction[0] > 0.0 {
        BlockFace::East
    } else {
        BlockFace::West
    };
    let y_dir = if direction[1] > 0.0 {
        BlockFace::Top
    } else {
        BlockFace::Bottom
    };
    let z_dir = if direction[2] > 0.0 {
        BlockFace::South
    } else {
        BlockFace::North
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
fn merge_slabs_in_place(block: &mut BlockState, target: BlockState, face: BlockFace) -> bool {
    if block.kind() != target.kind() {
        return false;
    }
    let opt = match block
        .data_as::<Slab>()
        .and_then(|s1| target.data_as::<Slab>().map(move |s2| (s1, s2)))
    {
        Some((slab, target_slab)) => match (slab.slab_type(), target_slab.slab_type()) {
            (SlabType::Top, SlabType::Bottom) if face == BlockFace::Top => {
                Some(block.with_data(slab.with_slab_type(SlabType::Double)))
            }
            (SlabType::Bottom, SlabType::Top) if face == BlockFace::Bottom => {
                Some(block.with_data(slab.with_slab_type(SlabType::Double)))
            }

            _ => None,
        },
        _ => None,
    };
    *block = opt.unwrap_or(*block);
    opt.is_some()
}
/// Determine what kind of slab to place. Returns `true` if the slab
/// placed would be merged with the other slab.
/// `try_merge_slabs` should always succeed if this function returns `true`.
#[allow(clippy::float_cmp)]
fn slab_to_place(
    block: &mut BlockState,
    target: BlockState,
    placement: &BlockPlacementEvent,
) -> bool {
    if let Some(mut slab) = target.data_as::<Slab>() {
        let k = slab.slab_type();
        let (slab_kind, place_adjacent) = match placement.cursor_position[1] {
            y if y == 0.5 => {
                if block.kind() != target.kind() {
                    match k {
                        SlabType::Top => (SlabType::Top, false),
                        SlabType::Bottom => (SlabType::Bottom, false),
                        SlabType::Double => return false,
                    }
                } else {
                    match k {
                        SlabType::Top => (SlabType::Bottom, true),
                        SlabType::Bottom => (SlabType::Top, true),
                        SlabType::Double => return false,
                    }
                }
            }
            y if y == 0.0 => (SlabType::Top, false),
            y if matches!(placement.face, BlockFace::Top) || y == 1.0 => (SlabType::Bottom, false),
            y if y < 0.5 => (SlabType::Bottom, false),
            y if y < 1.0 => (SlabType::Top, false),
            _ => return false,
        };
        slab.set_slab_type(slab_kind);
        block.set_data(slab);
        place_adjacent
    } else {
        false
    }
}

/// This function determines the result of combining the 2 blocks. If one is water and the other is waterloggable, the target is waterlogged. Has no effect otherwise.
fn waterlog(target_block: &mut BlockState, to_place: BlockState) -> bool {
    // Select the non-water block or return
    let mut waterloggable = if matches!(target_block.kind(), BlockKind::Water) {
        to_place
    } else if matches!(to_place.kind(), BlockKind::Water) {
        *target_block
    } else {
        return false;
    };
    // Refuse to waterlog double slabs and blocks that are already waterlogged
    if let Some(slab) = waterloggable.data_as::<Slab>() {
        if slab.slab_type() == SlabType::Double {
            return false;
        }
    }
    if let Some(mut waterlogged) = waterloggable.data_as::<Waterlogged>() {
        if waterlogged.waterlogged() {
            return false;
        }

        waterlogged.set_waterlogged(true);
        waterloggable.set_data(waterlogged);
        if let Some(campfire) = target_block.data_as::<Campfire>() {
            target_block.set_data(campfire.with_lit(false));
        }
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

/// Changes the block facing as necessary. This includes calling `make_wall_block`,
/// determining how the block is placed, setting its
/// facing, cubic, cardinal, cardinal&down, top/bottom and the xyz axis.
/// Blocks have only one of these orientations.
fn set_face(
    block: &mut BlockState,
    player_directions: &[BlockFace],
    placement: &BlockPlacementEvent,
) {
    if !matches!(placement.face, BlockFace::Top) {
        make_wall_block(block);
    }
    let player_relative = is_player_relative(block.simplified_kind());
    let facing = match player_relative {
        true => player_directions[0].opposite(),
        false => placement.face,
    };
    let facing = if is_reverse_placed(block.simplified_kind()) {
        facing.opposite()
    } else {
        facing
    };
    if let Some(attach) = block.data_as::<FaceAttachable>() {
        block.set_data(attach.with_attached_face(match placement.face {
            BlockFace::Bottom => AttachedFace::Ceiling,
            BlockFace::Top => AttachedFace::Floor,
            BlockFace::North => AttachedFace::Wall,
            BlockFace::South => AttachedFace::Wall,
            BlockFace::West => AttachedFace::Wall,
            BlockFace::East => AttachedFace::Wall,
        }));
    }

    let facing_cardinal = facing.to_cardinal().unwrap_or_else(|| {
        if player_relative {
            if is_reverse_placed(block.simplified_kind()) {
                player_directions[1]
            } else {
                player_directions[1].opposite()
            }
            .to_cardinal()
            .unwrap()
        } else {
            player_directions[0]
                .to_cardinal()
                .unwrap_or_else(|| player_directions[1].to_cardinal().unwrap())
        }
    });
    if let Some(mut dir) = block.data_as::<Directional>() {
        if !dir.valid_facing().contains(&BlockFace::Top) {
            block.set_data(dir.with_facing(match block.simplified_kind() {
                SimplifiedBlockKind::Anvil => facing_cardinal.left(),
                _ => facing_cardinal,
            }));
        } else {
            dir.set_facing(facing);
        }
    }

    if let Some(orient) = block.data_as::<Orientable>() {
        block.set_data(orient.with_axis(facing.axis()));
    }

    if let Some(door) = block.data_as::<Door>() {
        block.set_data(door.with_half(match placement.face {
            BlockFace::Top => BlockHalf::Lower,
            BlockFace::Bottom => BlockHalf::Upper,
            _ => match placement.cursor_position[1] {
                y if y <= 0.5 => BlockHalf::Lower,
                _ => BlockHalf::Upper,
            },
        }));
    }
}

/// If possible, turns a free standing block to its wall mounted counterpart, as they are considered different. This applies to torches, signs and banners.
fn make_wall_block(block: &mut BlockState) {
    *block = match block.kind() {
        BlockKind::Torch => BlockState::new(BlockKind::WallTorch),
        BlockKind::RedstoneTorch => BlockState::new(BlockKind::RedstoneWallTorch),
        BlockKind::SoulTorch => BlockState::new(BlockKind::SoulWallTorch),
        BlockKind::OakSign => BlockState::new(BlockKind::OakWallSign),
        BlockKind::BirchSign => BlockState::new(BlockKind::BirchWallSign),
        BlockKind::AcaciaSign => BlockState::new(BlockKind::AcaciaWallSign),
        BlockKind::JungleSign => BlockState::new(BlockKind::JungleWallSign),
        BlockKind::SpruceSign => BlockState::new(BlockKind::SpruceWallSign),
        BlockKind::WarpedSign => BlockState::new(BlockKind::WarpedWallSign),
        BlockKind::CrimsonSign => BlockState::new(BlockKind::CrimsonWallSign),
        BlockKind::DarkOakSign => BlockState::new(BlockKind::DarkOakWallSign),
        BlockKind::RedBanner => BlockState::new(BlockKind::RedWallBanner),
        BlockKind::BlueBanner => BlockState::new(BlockKind::BlueWallBanner),
        BlockKind::CyanBanner => BlockState::new(BlockKind::CyanWallBanner),
        BlockKind::GrayBanner => BlockState::new(BlockKind::GrayWallBanner),
        BlockKind::LimeBanner => BlockState::new(BlockKind::LimeWallBanner),
        BlockKind::PinkBanner => BlockState::new(BlockKind::PinkWallBanner),
        BlockKind::BlackBanner => BlockState::new(BlockKind::BlackWallBanner),
        BlockKind::BrownBanner => BlockState::new(BlockKind::BrownWallBanner),
        BlockKind::GreenBanner => BlockState::new(BlockKind::GreenWallBanner),
        BlockKind::WhiteBanner => BlockState::new(BlockKind::WhiteWallBanner),
        BlockKind::OrangeBanner => BlockState::new(BlockKind::OrangeWallBanner),
        BlockKind::PurpleBanner => BlockState::new(BlockKind::PurpleWallBanner),
        BlockKind::YellowBanner => BlockState::new(BlockKind::YellowWallBanner),
        BlockKind::MagentaBanner => BlockState::new(BlockKind::MagentaWallBanner),
        BlockKind::LightBlueBanner => BlockState::new(BlockKind::LightBlueWallBanner),
        BlockKind::LightGrayBanner => BlockState::new(BlockKind::LightGrayWallBanner),
        _ => *block,
    };
}

/// Attempts to convert an item to its placeable counterpart. Buckets to their respective contents, redstone dust to redstone wire and string to tripwire
pub fn item_to_block(item: Item) -> Option<BlockState> {
    Some(match item {
        Item::WaterBucket => BlockState::new(BlockKind::Water),
        Item::LavaBucket => BlockState::new(BlockKind::Lava),
        Item::Redstone => BlockState::new(BlockKind::RedstoneWire),
        Item::FlintAndSteel => BlockState::new(BlockKind::Fire),
        Item::String => BlockState::new(BlockKind::Tripwire),
        i => {
            let mut name = "minecraft:".to_owned();
            name.push_str(i.name());
            BlockState::new(BlockKind::from_namespaced_id(&name)?)
        }
    })
}

/// Reduces the amount of items in a slot. Full buckets are emptied instead. There are no checks if the item can be placed.
pub fn decrease_slot(slot: &mut InventorySlot) {
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
            let _ = slot.try_take(1);
        }
    }
}
