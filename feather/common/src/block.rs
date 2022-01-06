use std::convert::TryInto;

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
use ecs::{Ecs, SysResult, SystemExecutor};
use libcraft_core::{BlockFace, EntityKind};
use libcraft_items::InventorySlot;
use quill_common::components::CanBuild;
use quill_common::events::BlockPlacementEvent;
use vek::Rect3;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(block_placement);
}

/// A system that handles block placement events.
pub fn block_placement(game: &mut Game) -> SysResult {
    let mut events = vec![];
    for (player, event) in game.ecs.query::<&BlockPlacementEvent>().iter() {
        // Get inventory, gamemode, player position and held item
        let inv = game.ecs.get_mut::<Inventory>(player)?;
        let gamemode = game.ecs.get::<Gamemode>(player)?;
        let hotbar = game.ecs.get::<HotbarSlot>(player)?;
        let pos = game.ecs.get::<Position>(player)?;
        let mut slot = match event.hand {
            libcraft_core::Hand::Main => inv.item(Area::Hotbar, hotbar.get()),
            libcraft_core::Hand::Offhand => inv.item(Area::Offhand, 0),
        }
        .unwrap();
        // Check whether player has an item in hand, can build and that the item is placeable
        if slot.is_empty() || !game.ecs.get::<CanBuild>(player)?.0 {
            continue;
        }
        let block = match item_to_block(slot.item_kind().unwrap()) {
            Some(s) => s,
            None => continue,
        };
        if let Some(mut produced_events) = place_block(
            &mut game.world,
            *pos,
            &game.chunk_entities,
            block,
            event,
            &game.ecs,
            15,
        ) {
            match *gamemode {
                Gamemode::Survival | Gamemode::Adventure => decrease_slot(&mut slot),
                _ => {}
            }
            events.append(&mut produced_events);
        }
    }
    for e in events {
        game.ecs.insert_event(e);
    }

    Ok(())
}
/// Blocks get changed if they are getting waterlogged or when they are slabs turning into full blocks.
/// Blocks get replaced in-place when possible, while changing an adjacent block has a lower priority.
fn place_block(
    world: &mut World,
    player_pos: Position,
    chunk_entities: &ChunkEntities,
    mut block: BlockId,
    placement: &BlockPlacementEvent,
    ecs: &Ecs,
    light_level: u8,
) -> Option<Vec<BlockChangeEvent>> {
    let target1 = placement.location;
    let target_block1 = world.block_at(target1.try_into().unwrap())?;
    if target_block1.is_air() {
        return None;
    }
    let player_dir_ordered = ordered_directions(player_pos.direction());
    set_face(&mut block, &player_dir_ordered, placement);
    rotate_8dir(&mut block, player_pos.yaw);
    // Try placing the block by merging (=placing to the same position as the target block). Includes merging slabs, waterlogging and replacing blocks like grass.
    // Only one of the following conditions can be met at a time, so short-circuiting is ok.
    // The combinations include top slab & bottom slab, waterloggable block & water and any block & a replaceable block
    let target = if slab_to_place(&mut block, target_block1, placement)
        || waterlog(&mut block, target_block1)
        || ((target_block1.kind() != block.kind()) && target_block1.is_replaceable())
    {
        // If the target block was waterlogged or replaced, attempts to create a double slabs have no effect
        merge_slabs_in_place(&mut block, target_block1);
        target1
    } else {
        // Otherwise place the block next to the target
        let target2 = target1.adjacent(placement.face);
        let target_block2 = world.block_at(target2.try_into().unwrap())?;
        if merge_slabs_in_place(&mut block, target_block2)
            || waterlog(&mut block, target_block2)
            || ((target_block2.kind() != block.kind()) && target_block2.is_replaceable())
        {
            target2
        } else {
            return None;
        }
    };
    door_hinge(&mut block, target, &placement.cursor_position, world);
    let place_top = match top_half(block) {
        Some(_) => {
            if !world
                .block_at(target.up().try_into().unwrap())?
                .is_replaceable()
            {
                // Short circuit if upper block is > 256
                return None;
            }
            true
        }
        None => false,
    };
    let place_head = match bed_head(block) {
        Some(_) => {
            if !world
                .adjacent_block_cardinal(target, block.facing_cardinal()?)?
                .is_replaceable()
            {
                return None;
            }
            true
        }
        None => false,
    };
    // This works but there is a discrepancy between when the place block event is fired and getting the entity location.
    // that makes it possible to place a block at the right exact moment and have the server believe it wasn't blocked.
    if chunk_entities
        .entities_in_chunk(target.chunk())
        .iter()
        .any(|&entity| {
            let entity_position = ecs.get::<Position>(entity).unwrap();
            let entity_kind = *ecs.get::<EntityKind>(entity).unwrap();
            let block_rect: Rect3<f64, f64> = vek::Rect3 {
                x: target.x.into(),
                y: target.y.into(),
                z: target.z.into(),
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
    {
        return None;
    }
    if !world.check_block_stability(block, target, light_level)? {
        return None;
    }
    if place_top {
        world.set_block_at(target.try_into().unwrap(), block);
        world.set_block_at(
            target.up().try_into().unwrap(),
            block.with_half_upper_lower(HalfUpperLower::Upper),
        );
        Some(vec![
            BlockChangeEvent::single(target.try_into().unwrap()),
            BlockChangeEvent::single(target.up().try_into().unwrap()),
        ])
    } else if place_head {
        let face = match block.facing_cardinal()? {
            FacingCardinal::North => BlockFace::North,
            FacingCardinal::South => BlockFace::South,
            FacingCardinal::West => BlockFace::West,
            FacingCardinal::East => BlockFace::East,
        };
        world.set_block_at(target.try_into().unwrap(), block);
        world.set_block_adjacent_cardinal(
            target,
            block.with_part(base::Part::Head),
            block.facing_cardinal()?,
        );
        Some(vec![
            BlockChangeEvent::single(target.try_into().unwrap()),
            BlockChangeEvent::single(target.adjacent(face).try_into().unwrap()),
        ])
    } else {
        world.set_block_at(target.try_into().unwrap(), block);
        Some(vec![BlockChangeEvent::single(target.try_into().unwrap())])
    }
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
    let is_door = |block: BlockId| {
        use SimplifiedBlockKind::*;
        matches!(
            block.simplified_kind(),
            WoodenDoor | IronDoor | WarpedDoor | CrimsonDoor
        )
    };
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
    let solid_left = is_block_solid(lb) | is_block_solid(lt);
    let solid_right = is_block_solid(rb) | is_block_solid(rt);
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
        FacingCardinal::West => cursor_pos[2],
        FacingCardinal::East => 1.0 - cursor_pos[2],
    };
    block.set_hinge(if relevant_axis < 0.5 {
        Hinge::Left
    } else {
        Hinge::Right
    });
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
    )
}

/// Checks if the block is to be placed with an opposite facing relative to the player.
fn is_reverse_placed(kind: SimplifiedBlockKind) -> bool {
    use SimplifiedBlockKind::*;
    matches!(
        kind,
        Observer | Bed | FenceGate | WoodenDoor | IronDoor | WarpedDoor | CrimsonDoor
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
        _ => {
            // Can always take at least one
            slot.try_take(1);
        }
    }
}
