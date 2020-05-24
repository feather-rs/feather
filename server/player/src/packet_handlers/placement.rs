//! Handling of player block placement packets.

use crate::IteratorExt;
use entity::InventoryExt;
use feather_core::blocks::{BlockId, BlockKind, Face, FacingCardinalAndDown, FacingCubic};
use feather_core::inventory::{slot, Area, Inventory};
use feather_core::item_block::ItemToBlock;
use feather_core::items::ItemStack;
use feather_core::network::packets::PlayerBlockPlacement;
use feather_core::util::{Gamemode, Position, Vec3d};
use feather_server_types::{BlockUpdateCause, Game, HeldItem, InventoryUpdateEvent, PacketBuffers};
use fecs::World;
use smallvec::smallvec;
use std::sync::Arc;

/// System for handling Player Block Placement packets
/// and updating the world accordingly.
#[fecs::system]
pub fn handle_player_block_placement(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<PlayerBlockPlacement>()
        .for_each_valid(world, |world, (player, packet)| {
            // TODO: handle slabs, blocks with directions, etc.
            let gamemode = *world.get::<Gamemode>(player);
            let player_pos = *world.get::<Position>(player);
            let inventory = world.get::<Inventory>(player);

            let item = match inventory.item_in_main_hand(player, world) {
                Some(item) => item,
                None => return, // No block to place
            };

            drop(inventory);

            let face = packet.face.face();
            let mut block = match item.ty.to_block() {
                Some(block) => {
                    if face == Face::Wall {
                        match block.kind() {
                            BlockKind::Torch => BlockId::wall_torch(),
                            BlockKind::RedstoneTorch => BlockId::redstone_wall_torch(),
                            BlockKind::Sign => BlockId::wall_sign(),
                            BlockKind::SkeletonSkull => BlockId::skeleton_wall_skull(),
                            BlockKind::WitherSkeletonSkull => BlockId::wither_skeleton_wall_skull(),
                            BlockKind::ZombieHead => BlockId::zombie_wall_head(),
                            BlockKind::CreeperHead => BlockId::creeper_wall_head(),
                            BlockKind::PlayerHead => BlockId::player_wall_head(),
                            BlockKind::DragonHead => BlockId::dragon_wall_head(),
                            BlockKind::WhiteWallBanner => BlockId::white_wall_banner(),
                            BlockKind::OrangeWallBanner => BlockId::orange_wall_banner(),
                            BlockKind::MagentaWallBanner => BlockId::magenta_wall_banner(),
                            BlockKind::LightBlueWallBanner => BlockId::light_blue_wall_banner(),
                            BlockKind::YellowWallBanner => BlockId::yellow_wall_banner(),
                            BlockKind::LimeWallBanner => BlockId::lime_wall_banner(),
                            BlockKind::PinkWallBanner => BlockId::pink_wall_banner(),
                            BlockKind::GrayWallBanner => BlockId::gray_wall_banner(),
                            BlockKind::LightGrayWallBanner => BlockId::light_gray_wall_banner(),
                            BlockKind::CyanWallBanner => BlockId::cyan_wall_banner(),
                            BlockKind::PurpleWallBanner => BlockId::purple_wall_banner(),
                            BlockKind::BlueWallBanner => BlockId::blue_wall_banner(),
                            BlockKind::BrownWallBanner => BlockId::brown_wall_banner(),
                            BlockKind::GreenWallBanner => BlockId::green_wall_banner(),
                            BlockKind::RedWallBanner => BlockId::red_wall_banner(),
                            BlockKind::BlackWallBanner => BlockId::black_wall_banner(),
                            _ => block,
                        }
                    } else {
                        block
                    }
                }
                None => return, // Item is not a block
            };

            let placed_on = match game.block_at(packet.location) {
                Some(block) => block,
                None => {
                    game.disconnect(player, world, "attempted to place block in unloaded chunk");
                    return;
                }
            };

            // TODO: waterlogged blocks, more
            let pos = match placed_on.kind() {
                BlockKind::Grass | BlockKind::TallGrass | BlockKind::Water | BlockKind::Lava => {
                    packet.location
                }
                _ => packet.location + packet.face.placement_offset(),
            };

            if block.has_face() {
                block.set_face(face);
            }

            if block.has_facing_cardinal() {
                block.set_facing_cardinal(match block.kind() {
                    BlockKind::WallTorch | BlockKind::RedstoneWallTorch => {
                        packet.face.facing_cardinal()
                    }
                    kind => {
                        let direction = facing_directions(player_pos.direction())
                            .iter()
                            .find(|dir| dir.is_horizontal())
                            .unwrap()
                            .to_facing_cardinal();
                        match kind {
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
                            | BlockKind::OakStairs
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
                            | BlockKind::OakDoor
                            | BlockKind::IronDoor
                            | BlockKind::SpruceDoor
                            | BlockKind::BirchDoor
                            | BlockKind::JungleDoor
                            | BlockKind::AcaciaDoor
                            | BlockKind::DarkOakDoor
                            | BlockKind::OakFenceGate
                            | BlockKind::SpruceFenceGate
                            | BlockKind::BirchFenceGate
                            | BlockKind::JungleFenceGate
                            | BlockKind::AcaciaFenceGate
                            | BlockKind::DarkOakFenceGate => direction.opposite(),
                            BlockKind::Anvil
                            | BlockKind::ChippedAnvil
                            | BlockKind::DamagedAnvil => direction.left(),
                            _ => direction,
                        }
                    }
                });
            }

            if block.has_facing_cardinal_and_down() {
                block.set_facing_cardinal_and_down(match face {
                    Face::Ceiling | Face::Floor => FacingCardinalAndDown::Down,
                    _ => packet.face.facing_cardinal_and_down().opposite(),
                });
            }

            if block.has_facing_cubic() {
                block.set_facing_cubic(match block.kind() {
                    BlockKind::EndRod
                    | BlockKind::ShulkerBox
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
                    | BlockKind::BlackShulkerBox => packet.face.facing_cubic(),
                    kind => {
                        let direction = facing_directions(player_pos.direction())[0];
                        match kind {
                            BlockKind::Observer => direction,
                            _ => direction.opposite(),
                        }
                    }
                });
            }

            game.set_block_at(world, pos, block, BlockUpdateCause::Entity(player));

            let held_item = world.get::<HeldItem>(player).0;
            let inventory = world.get::<Inventory>(player);

            // Update player's inventory if in survival
            if gamemode == Gamemode::Survival {
                if item.amount == 0 {
                    drop(inventory);
                    game.disconnect(
                        player,
                        world,
                        "attempted to place block with zero-sized item stack",
                    );
                    return;
                }

                let item = ItemStack::new(item.ty, item.amount - 1);
                inventory
                    .set_item_at(Area::Hotbar, held_item, item)
                    .unwrap();

                let event = InventoryUpdateEvent {
                    slots: smallvec![slot(Area::Hotbar, held_item)],
                    player,
                };
                drop(inventory);
                game.handle(world, event);
            }
        });
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
