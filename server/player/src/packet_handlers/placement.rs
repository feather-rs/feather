//! Handling of player block placement packets.

use crate::IteratorExt;
use entity::InventoryExt;
use feather_core::blocks::categories::PlacementType;
use feather_core::blocks::{
    BlockId, BlockKind, Face, FacingCardinal, FacingCardinalAndDown, FacingCubic, HalfTopBottom,
    HalfUpperLower, Hinge, Part, SimplifiedBlockKind, SlabKind, StairsShape,
};
use feather_core::inventory::{slot, Area, Inventory};
use feather_core::item_block::ItemToBlock;
use feather_core::network::packets::PlayerBlockPlacement;
use feather_core::util::{BlockPosition, Gamemode, Position, Vec3d};
use feather_server_types::{
    BlockUpdateCause, Game, HeldItem, InteractionHandler, InventoryUpdateEvent, OpenWindowCount,
    PacketBuffers,
};
use feather_server_util::is_block_supported_at;
use fecs::{Entity, World};
use once_cell::sync::Lazy;
use smallvec::smallvec;
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::Arc;

type PacketFace = feather_core::network::packets::Face;

#[allow(dead_code)]
static INTERACTION_HANDLERS: Lazy<HashMap<BlockKind, &'static dyn InteractionHandler>> =
    Lazy::new(|| {
        let mut handlers_hashmap: HashMap<BlockKind, &'static dyn InteractionHandler> =
            HashMap::new();

        for handler in inventory::iter::<Box<dyn InteractionHandler>> {
            let kind = handler.block_kind();
            handlers_hashmap.insert(kind, &**handler);
        }

        handlers_hashmap
    });

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
///
/// Also handles block interactions because they are handled with the same packet.
#[fecs::system]
pub fn handle_player_block_placement(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<PlayerBlockPlacement>()
        .for_each_valid(world, |world, (player, packet)| {
            let target_block = match game.block_at(packet.location) {
                Some(block) => block,
                None => {
                    game.disconnect(
                        player,
                        world,
                        "Attempted to interact with block in unloaded chunk",
                    );
                    return;
                }
            };

            // Decide whether the player should place a block or interact with the block they are targeting
            // TODO: Maybe player shifting may need to be taken into account (shift click on interactable block)
            if let Some(interaction_handler) = INTERACTION_HANDLERS.get(&target_block.kind()) {
                let window_id = {
                    if let Some(mut window_count) = world.try_get_mut::<OpenWindowCount>(player) {
                        window_count.get_increment()
                    } else {
                        panic!("Unable to get OpenWindowCount for player {}", player);
                    }
                };

                // Interact with the block
                interaction_handler.handle_interaction(
                    game,
                    world,
                    packet.location,
                    player,
                    window_id,
                );
            } else {
                // Try to place a block
                handle_block_placement(game, world, player, target_block, packet);
            }
        });
}

pub fn handle_block_placement(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    target_block: BlockId,
    packet: PlayerBlockPlacement,
) {
    let gamemode = *world.get::<Gamemode>(player);

    let item = {
        let inventory = world.get::<Inventory>(player);
        match inventory.item_in_main_hand(player, world) {
            Some(item) => item,
            // Offhand?
            None => return, // No block to place
        }
    };

    let block = match item.ty.to_block() {
        Some(block) => block,
        None => return, // Item is not a block
    };

    if !handle_slab_placement(game, world, block, packet.location, packet.face) {
        // TODO: waterlogged blocks, more
        let pos = if target_block.is_replaceable() {
            packet.location
        } else {
            packet.location + packet.face.placement_offset()
        };

        let current_block = match game.block_at(pos) {
            Some(block) => block,
            None => return,
        };

        if !current_block.is_replaceable() {
            return;
        }

        // Deny replacing grass with grass for example
        if current_block.is_replaceable()
            && !current_block.is_air()
            && !current_block.is_fluid()
            && block.is_replaceable()
        {
            return;
        }

        let block = update_block_state_for_placement(
            game,
            block,
            pos,
            *world.get::<Position>(player),
            &packet,
        );

        // Abort if block that needs support wouldn't have the needed support blocks
        if !is_block_supported_at(block, game, pos) {
            return;
        }

        // handle multi-block placements (i.e. doors and beds)
        if let Some((other_pos, other_block)) = match block.simplified_kind() {
            SimplifiedBlockKind::Bed => {
                let mut head = block;
                head.set_part(Part::Head);
                Some((pos + block.facing_cardinal().unwrap().offset(), head))
            }
            SimplifiedBlockKind::IronDoor | SimplifiedBlockKind::WoodenDoor => {
                let mut upper = block;
                upper.set_half_upper_lower(HalfUpperLower::Upper);
                Some((pos.up(), upper))
            }
            _ => None,
        } {
            game.set_block_at(
                world,
                other_pos,
                other_block,
                BlockUpdateCause::Entity(player),
            );
        }

        game.set_block_at(world, pos, block, BlockUpdateCause::Entity(player));
    }

    // Update player's inventory if in survival
    let event = {
        if gamemode != Gamemode::Creative {
            if item.amount == 0 {
                game.disconnect(
                    player,
                    world,
                    "Attempted to place block with zero-sized item stack.",
                );
                return;
            }

            let held_item = world.get::<HeldItem>(player).0;
            let inventory = world.get::<Inventory>(player);

            let item = item.of_amount(item.amount - 1);
            inventory
                .set_item_at(Area::Hotbar, held_item, item)
                .unwrap();

            Some(InventoryUpdateEvent {
                slots: smallvec![slot(Area::Hotbar, held_item)],
                entity: player,
            })
        } else {
            None
        }
    };

    if let Some(event) = event {
        // Only send the event to decrement the held stack if the player's gamemode is survival
        game.handle(world, event);
    }
}

// returns true if placement handling should stop
fn handle_slab_placement(
    game: &mut Game,
    world: &mut World,
    block_to_place: BlockId,
    mut target_block_pos: BlockPosition,
    placement_face: PacketFace,
) -> bool {
    if block_to_place.simplified_kind() != SimplifiedBlockKind::Slab {
        return false;
    }

    let mut target_block = game.block_at(target_block_pos).unwrap();
    if target_block.simplified_kind() == SimplifiedBlockKind::Slab
        && target_block.slab_kind().unwrap() != SlabKind::Double
        && matches!(placement_face, PacketFace::Bottom | PacketFace::Top)
    {
        let target_block_slab_kind = target_block.slab_kind().unwrap();
        if (target_block_slab_kind == SlabKind::Bottom && placement_face == PacketFace::Bottom)
            || (target_block_slab_kind == SlabKind::Top && placement_face == PacketFace::Top)
        {
            return false;
        }
    } else {
        target_block_pos = target_block_pos + placement_face.placement_offset();
        if let Some(block) = game.block_at(target_block_pos) {
            target_block = block;
        } else {
            return false;
        }

        if target_block.simplified_kind() != SimplifiedBlockKind::Slab {
            return false;
        }
    }

    if target_block.kind() != block_to_place.kind() {
        return false;
    }

    target_block.set_slab_kind(SlabKind::Double);

    game.set_block_at(
        world,
        target_block_pos,
        target_block,
        BlockUpdateCause::Unknown,
    );

    true
}

fn update_block_state_for_placement(
    game: &Game,
    mut block: BlockId,
    block_pos: BlockPosition,
    player_pos: Position,
    packet: &PlayerBlockPlacement,
) -> BlockId {
    let face = packet.face.face();
    if face == Face::Wall {
        if let Some(wall_block) = block.to_wall_block() {
            block = wall_block
        }
    }

    if block.has_face() {
        block.set_face(face);
    }

    if block.has_facing_cardinal() {
        let player_direction = facing_directions(player_pos.direction())
            .iter()
            .find(|dir| dir.is_horizontal())
            .unwrap()
            .to_facing_cardinal()
            .unwrap();

        block.set_facing_cardinal(match block.placement_type() {
            Some(PlacementType::TargetedFace) => {
                if face == Face::Wall {
                    packet.face.facing_cardinal()
                } else {
                    FacingCardinal::North
                }
            }
            Some(PlacementType::PlayerDirection) => player_direction,
            Some(PlacementType::PlayerDirectionRightAngle) => player_direction.right(),
            None => player_direction.opposite(),
        });
    }

    if block.has_facing_cardinal_and_down() {
        block.set_facing_cardinal_and_down(match face {
            Face::Wall => packet.face.facing_cardinal_and_down().opposite().unwrap(),
            _ => FacingCardinalAndDown::Down,
        });
    }

    if block.has_facing_cubic() {
        let player_direction = facing_directions(player_pos.direction())[0];

        block.set_facing_cubic(match block.placement_type() {
            Some(PlacementType::TargetedFace) => packet.face.facing_cubic(),
            Some(PlacementType::PlayerDirection) => player_direction,
            None => player_direction.opposite(),
            _ => unreachable!(),
        });
    }

    if block.has_slab_kind() {
        block.set_slab_kind(if is_placed_top(face, packet.cursor_position_y) {
            SlabKind::Top
        } else {
            SlabKind::Bottom
        });
    }

    if block.has_half_top_bottom() {
        block.set_half_top_bottom(if is_placed_top(face, packet.cursor_position_y) {
            HalfTopBottom::Top
        } else {
            HalfTopBottom::Bottom
        });
    }

    if block.has_stairs_shape() {
        block.set_stairs_shape(get_stairs_shape(
            game,
            block_pos,
            block.facing_cardinal().unwrap(),
            block.half_top_bottom().unwrap(),
        ));
    }

    if block.has_axis_xyz() {
        block.set_axis_xyz(packet.face.facing_cubic().axis());
    }

    if block.has_hinge() {
        block.set_hinge(get_hinge_side(
            game,
            block.kind(),
            block_pos,
            block.facing_cardinal().unwrap(),
            packet.cursor_position_x,
            packet.cursor_position_z,
        ));
    }

    block
}

fn is_placed_top(face: Face, cursor_position_y: f32) -> bool {
    face == Face::Ceiling || (face == Face::Wall && cursor_position_y > 0.5)
}

fn get_hinge_side(
    game: &Game,
    block_kind: BlockKind,
    block_pos: BlockPosition,
    block_facing_cardinal: FacingCardinal,
    cursor_position_x: f32,
    cursor_position_z: f32,
) -> Hinge {
    let right_pos = block_pos + block_facing_cardinal.right().offset();
    let left_pos = block_pos + block_facing_cardinal.left().offset();

    let score = (
        // check right side
        game.block_at(right_pos).unwrap().is_opaque() as i8
            + game.block_at(right_pos.up()).unwrap().is_opaque() as i8
    ) - (
        // check left side
        game.block_at(left_pos).unwrap().is_opaque() as i8
            + game.block_at(left_pos.up()).unwrap().is_opaque() as i8
    );

    let (door_on_right, door_on_left) = {
        let is_door = |pos: BlockPosition| {
            game.block_at(pos).and_then(|block| {
                if block.kind() == block_kind {
                    block.half_upper_lower()
                } else {
                    None
                }
            }) == Some(HalfUpperLower::Lower)
        };

        (is_door(right_pos), is_door(left_pos))
    };

    if (door_on_left && !door_on_right) || score > 0 {
        return Hinge::Right;
    }

    if (door_on_right && !door_on_left) || score < 0 {
        return Hinge::Left;
    }

    if (block_facing_cardinal == FacingCardinal::West && cursor_position_z < 0.5)
        || (block_facing_cardinal == FacingCardinal::East && cursor_position_z > 0.5)
        || (block_facing_cardinal == FacingCardinal::South && cursor_position_x > 0.5)
        || (block_facing_cardinal == FacingCardinal::North && cursor_position_x < 0.5)
    {
        return Hinge::Right;
    }

    Hinge::Left
}

fn get_stairs_shape(
    game: &Game,
    block_pos: BlockPosition,
    block_facing_cardinal: FacingCardinal,
    block_half_top_bottom: HalfTopBottom,
) -> StairsShape {
    if let Some(adjacent_block) = game.block_at(block_pos + block_facing_cardinal.offset()) {
        if adjacent_block.simplified_kind() == SimplifiedBlockKind::Stairs
            && adjacent_block.half_top_bottom().unwrap() == block_half_top_bottom
        {
            let adjacent_block_facing_cardinal = adjacent_block.facing_cardinal().unwrap();
            if adjacent_block_facing_cardinal.axis() != block_facing_cardinal.axis()
                && is_different_stairs(
                    block_facing_cardinal,
                    block_half_top_bottom,
                    game.block_at(block_pos + adjacent_block_facing_cardinal.opposite().offset()),
                )
            {
                if adjacent_block_facing_cardinal == block_facing_cardinal.left() {
                    return StairsShape::OuterLeft;
                }

                return StairsShape::OuterRight;
            }
        }
    }

    if let Some(adjacent_block) = game.block_at(block_pos + block_facing_cardinal.offset()) {
        if adjacent_block.simplified_kind() == SimplifiedBlockKind::Stairs
            && adjacent_block.half_top_bottom().unwrap() == block_half_top_bottom
        {
            let adjacent_block_facing_cardinal = adjacent_block.facing_cardinal().unwrap();
            if adjacent_block_facing_cardinal.axis() != block_facing_cardinal.axis()
                && is_different_stairs(
                    block_facing_cardinal,
                    block_half_top_bottom,
                    game.block_at(block_pos + adjacent_block_facing_cardinal.offset()),
                )
            {
                if adjacent_block_facing_cardinal == block_facing_cardinal.left() {
                    return StairsShape::InnerLeft;
                }

                return StairsShape::InnerRight;
            }
        }
    }

    StairsShape::Straight
}

fn is_different_stairs(
    block_facing_cardinal: FacingCardinal,
    block_half_top_bottom: HalfTopBottom,
    test_block: Option<BlockId>,
) -> bool {
    match test_block {
        Some(test_block) => {
            test_block.simplified_kind() != SimplifiedBlockKind::Stairs
                || block_facing_cardinal != test_block.facing_cardinal().unwrap()
                || block_half_top_bottom != test_block.half_top_bottom().unwrap()
        }
        None => true,
    }
}

fn facing_directions(d: Vec3d) -> [FacingCubic; 6] {
    let x_dir = if d.x > 0.0 {
        FacingCubic::East
    } else {
        FacingCubic::West
    };

    let y_dir = if d.y > 0.0 {
        FacingCubic::Up
    } else {
        FacingCubic::Down
    };

    let z_dir = if d.z > 0.0 {
        FacingCubic::South
    } else {
        FacingCubic::North
    };

    let x = d.x.abs();
    let y = d.y.abs();
    let z = d.z.abs();

    let dirs = if x > z {
        if y > x {
            [y_dir, x_dir, z_dir]
        } else if z > y {
            [x_dir, z_dir, y_dir]
        } else {
            [x_dir, y_dir, z_dir]
        }
    } else if y > z {
        [y_dir, z_dir, x_dir]
    } else if x > y {
        [z_dir, x_dir, y_dir]
    } else {
        [z_dir, y_dir, x_dir]
    };

    [
        dirs[0],
        dirs[1],
        dirs[2],
        dirs[2].opposite(),
        dirs[1].opposite(),
        dirs[0].opposite(),
    ]
}
