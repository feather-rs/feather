//! This module handles the monolithic Player Digging packet.
//!
//! The packet's name is rather misleading, as it is also sent
//! for actions mostly unrelated to digging including eating, shooting bows,
//! swapping items out to the offhand, and dropping items.

use crate::{ItemTimedUse, IteratorExt};
use entity::InventoryExt;
use feather_core::blocks::{BlockId, HalfUpperLower, Part, SimplifiedBlockKind};
use feather_core::inventory::{slot, Area, Inventory, Slot, SlotIndex};
use feather_core::items::{Item, ItemStack};
use feather_core::network::packets::{PlayerDigging, PlayerDiggingStatus};
use feather_core::util::{BlockPosition, Gamemode, Position};
use feather_definitions::Tool;
use feather_server_types::{
    BlockUpdateCause, CanBreak, CanInstaBreak, EntitySpawnEvent, Game, HeldItem,
    InventoryUpdateEvent, ItemDamageEvent, ItemDropEvent, PacketBuffers, Velocity,
    PLAYER_EYE_HEIGHT, TPS,
};
use feather_server_util::{charge_from_ticks_held, compute_projectile_velocity};
use fecs::{Entity, IntoQuery, Read, World, Write};
use smallvec::smallvec;
use std::sync::Arc;

/// Stores the "digging status" of a player.
///
/// If this component exists for an entity,
/// then it is currently digging a block. The
/// corresponding animation must be displayed.
#[derive(Copy, Clone, Debug)]
pub struct Digging {
    /// The position of the block being dug
    pub pos: BlockPosition,
    /// The total time (in seconds) of digging needed
    pub time: f64,
    /// Progress made, in seconds (better tools increase this
    /// value faster)
    pub progress: f64,
}

/// System responsible for polling for PlayerDigging
/// packets and writing the corresponding events.
#[fecs::system]
pub fn handle_player_digging(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    use PlayerDiggingStatus::*;

    packet_buffers
        .received::<PlayerDigging>()
        .for_each_valid(world, |world, (player, packet)| match packet.status {
            StartedDigging | FinishedDigging | CancelledDigging => {
                handle_digging(game, world, player, packet)
            }
            DropItem | DropItemStack => handle_drop_item_stack(game, world, player, packet),
            ConsumeItem => handle_consume_item(game, world, player, packet),
            status => log::warn!("Unhandled Player Digging status {:?}", status),
        });
}

fn handle_digging(game: &mut Game, world: &mut World, player: Entity, packet: PlayerDigging) {
    if !world.has::<CanBreak>(player) {
        log::trace!(
            "Player cannot break blocks but sent player digging status {:?}",
            packet.status
        );
        return;
    }

    match packet.status {
        PlayerDiggingStatus::StartedDigging => handle_started_digging(game, world, player, packet),
        PlayerDiggingStatus::CancelledDigging => handle_cancelled_digging(game, world, player),
        PlayerDiggingStatus::FinishedDigging => {
            handle_finished_digging(game, world, player, packet)
        }
        _ => unreachable!(),
    }
}

const MAX_DIG_RADIUS_SQUARED: f64 = 36.0;

/// Event triggered when the `Digging` component is added to a player.
///
/// Not triggered in the case of insta-breaks.
#[derive(Copy, Clone, Debug)]
pub struct StartDiggingEvent {
    pub player: Entity,
}

/// Event triggered when a player finished digging (the `Digging` component
/// is removed). This event is triggered event if digging was canceled.
#[derive(Copy, Clone, Debug)]
pub struct FinishDiggingEvent {
    /// The player who finished digging
    pub player: Entity,
    /// The `Digging` component which was removed
    pub digging: Digging,
}

fn handle_started_digging(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: PlayerDigging,
) {
    // Delete old `Digging`, if it exists
    let _ = world.remove::<Digging>(player);

    // Check the distance isn't too far.
    if packet
        .location
        .position()
        .distance_squared_to(*world.get::<Position>(player))
        > MAX_DIG_RADIUS_SQUARED
    {
        // Ignore the packet.
        log::trace!("player {:?} tried to dig too far", player);
        return;
    }

    // If the player can insta-break, or the block has hardness 0, then they can already break the block.
    if world.has::<CanInstaBreak>(player)
        || game
            .block_at(packet.location)
            .unwrap_or_default()
            .kind()
            .hardness()
            < 0.01
    {
        dig(game, world, player, packet.location);
    } else {
        // Insert new `Digging`.
        let block = game.block_at(packet.location).unwrap_or_default();
        let hardness = block.kind().hardness();

        world
            .add(
                player,
                Digging {
                    pos: packet.location,
                    time: hardness,
                    progress: 0.0,
                },
            )
            .unwrap();
        game.handle(world, StartDiggingEvent { player });
    }
}

/// System to advance the digging progress.
#[fecs::system]
pub fn advance_dig_progress(game: &mut Game, world: &mut World) {
    <(Write<Digging>, Read<Inventory>, Read<HeldItem>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(mut digging, inventory, held_item)| {
            // Advance progress depends on tool and the
            // block kind: https://minecraft.gamepedia.com/Breaking#Speed
            // * If the block requires some tool to harvest (i.e. it requires a tool to get the item after it breaks),
            // then if that tool is not held, progress is hindered by a factor of 5. Otherwise, the hindrance
            // is only a factor of 1.5.
            // * If the player's tool helps dig the block (e.g. shovel => dirt, pickaxe => cobblestone),
            // then a constant mutliplier is applied to the dig speed depending on the tool's material.
            // This is retrieved through the `dig_multiplier` property on `ToolMaterial`.
            let block = game.block_at(digging.pos).unwrap_or_default();
            let best_tool = block.kind().best_tool();
            let best_tool_required = block.kind().best_tool_required();

            let item_in_main_hand: Slot = inventory
                .item_at(Area::Hotbar, held_item.0)
                .expect("held item out of bounds");
            let held_tool = item_in_main_hand.map(|item| item.ty.tool()).flatten();

            let multiplier = if best_tool == held_tool && best_tool.is_some() {
                let dig_multiplier = item_in_main_hand
                    .unwrap()
                    .ty
                    .tool_material()
                    .map(|mat| mat.dig_multiplier())
                    .unwrap_or_else(|| {
                        // Missing data in feather-definitions;
                        // panic. (TODO: maybe this should just be a log message)
                        panic!(
                            "no tool material for item {:?}, even though it has a tool",
                            item_in_main_hand
                        )
                    });

                (1.0 / 1.5) * dig_multiplier
            } else if best_tool_required {
                1.0 / 5.0
            } else {
                1.0 / 1.5
            };

            digging.progress += (1.0 / TPS as f64) * multiplier;
        },
    );
}

fn handle_cancelled_digging(game: &mut Game, world: &mut World, player: Entity) {
    let digging = world.try_get::<Digging>(player).map(|d| *d);
    let _ = world.remove::<Digging>(player);

    if let Some(digging) = digging {
        game.handle(world, FinishDiggingEvent { player, digging });
    }
}

fn handle_finished_digging(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: PlayerDigging,
) {
    let digging = match world.try_get::<Digging>(player) {
        Some(digging) => *digging,
        None => {
            if world.has::<CanInstaBreak>(player) {
                // Can insta-break - no `StartedDigging` needed
                Digging {
                    pos: packet.location,
                    time: 0.0,
                    progress: 0.0,
                }
            } else {
                // Player can't insta-break and has
                // not sent StartedDigging.
                // They cannot finish.
                return;
            }
        }
    };

    let _ = world.remove::<Digging>(player);

    if digging.pos != packet.location {
        return;
    }

    // Attempt to break the block
    dig(game, world, player, digging.pos);

    // Finished
    game.handle(world, FinishDiggingEvent { player, digging });
}

fn dig(game: &mut Game, world: &mut World, player: Entity, pos: BlockPosition) {
    let block = match game.block_at(pos) {
        Some(block) => block,
        None => {
            game.disconnect(
                player,
                world,
                format!(
                    "Attempted to break block in unloaded chunk (position: {:?})",
                    pos
                ),
            );

            return;
        }
    };

    damage_tool(player, block, game, world);

    // Handle multi-block destruction (i.e. doors and beds)
    if let Some(other_pos) = match block.simplified_kind() {
        SimplifiedBlockKind::Bed => {
            let direction = block.facing_cardinal().unwrap();
            Some(match block.part().unwrap() {
                Part::Head => pos - direction.offset(),
                Part::Foot => pos + direction.offset(),
            })
        }
        SimplifiedBlockKind::WoodenDoor | SimplifiedBlockKind::IronDoor => {
            Some(match block.half_upper_lower().unwrap() {
                HalfUpperLower::Upper => pos.down(),
                HalfUpperLower::Lower => pos.up(),
            })
        }
        _ => None,
    } {
        if game.block_at(other_pos).unwrap().kind() == block.kind() {
            game.set_block_at(
                world,
                other_pos,
                BlockId::air(),
                BlockUpdateCause::Entity(player),
            );
        };
    }

    game.set_block_at(world, pos, BlockId::air(), BlockUpdateCause::Entity(player));
}

fn damage_tool(player: Entity, block: BlockId, game: &mut Game, world: &mut World) {
    if block.kind().hardness() == 0.0 || world.has::<CanInstaBreak>(player) {
        return; // Instant break should not cause damage
    }

    let held_item = world.get::<HeldItem>(player).0;
    let inventory = world.get::<Inventory>(player);

    let item_in_main_hand: Slot = inventory
        .item_at(Area::Hotbar, held_item)
        .expect("held item out of bounds");

    if let Some(item) = item_in_main_hand {
        let damage_taken = if item.ty == Item::Trident {
            2
        } else {
            match item.ty.tool() {
                // Note: it looks like hoes do not take damage when breaking blocks in 1.13.2
                // but in some later version this was changed so that they take 1 damage.
                None | Some(Tool::Hoe) => return,
                Some(Tool::Sword) => 2,
                Some(_) => 1,
            }
        };

        drop(inventory);
        let damage_event = ItemDamageEvent {
            player,
            slot: slot(Area::Hotbar, held_item),
            damage_taken,
        };
        game.handle(world, damage_event);
    }
}

fn handle_drop_item_stack(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: PlayerDigging,
) {
    assert!(
        packet.status == PlayerDiggingStatus::DropItem
            || packet.status == PlayerDiggingStatus::DropItemStack
    );

    let held_item = world.get::<HeldItem>(player).0;
    let inventory = world.get::<Inventory>(player);

    let stack = {
        if let Some(item) = inventory.item_at(Area::Hotbar, held_item).unwrap() {
            item
        } else {
            // Silently fail - no item stack to drop
            return;
        }
    };

    let amnt = match packet.status {
        PlayerDiggingStatus::DropItem => {
            if stack.amount == 0 {
                inventory.remove_item_at(Area::Hotbar, held_item).unwrap();
                0
            } else if stack.amount == 1 {
                inventory.remove_item_at(Area::Hotbar, held_item).unwrap();
                1
            } else {
                inventory
                    .set_item_at(Area::Hotbar, held_item, stack.of_amount(stack.amount - 1))
                    .unwrap();
                1
            }
        }
        PlayerDiggingStatus::DropItemStack => {
            inventory.remove_item_at(Area::Hotbar, held_item).unwrap();
            stack.amount
        }
        _ => unreachable!(), // Assertion above
    };

    drop(inventory);

    let idx = SlotIndex {
        area: Area::Hotbar,
        slot: held_item,
    };
    let inv_update = InventoryUpdateEvent {
        slots: smallvec![idx],
        entity: player,
    };
    game.handle(world, inv_update);

    if amnt != 0 {
        let item_drop = ItemDropEvent {
            slot: Some(idx),
            stack: stack.of_amount(amnt),
            player,
        };
        game.handle(world, item_drop);
    }
}

/// Handles food consumption and shooting arrows.
fn handle_consume_item(game: &mut Game, world: &mut World, player: Entity, packet: PlayerDigging) {
    assert_eq!(packet.status, PlayerDiggingStatus::ConsumeItem);

    // TODO: Fallback to off-hand if main-hand is not a consumable
    let inventory = world.get::<Inventory>(player);
    let used_item = inventory.item_in_main_hand(player, world);

    if let Some(item) = used_item {
        if item.ty == Item::Bow {
            drop(inventory);
            handle_shoot_bow(game, world, player);
        }
        // TODO: Food, potions
    }
}

fn handle_shoot_bow(game: &mut Game, world: &mut World, player: Entity) {
<<<<<<< HEAD
    let inventory = world.get::<Inventory>(player);
    let arrow_to_consume: Option<(SlotIndex, ItemStack)> = find_arrow(&inventory);

    if player.gamemode == Gamemode::Survival || player.gamemode == Gamemode::Adventure {
        // If no arrow was found, don't shoot
        let arrow_to_consume = arrow_to_consume.clone();
        if arrow_to_consume.is_none() {
            return;
        }
=======
    let gamemode = *world.get::<Gamemode>(player);
>>>>>>> develop

    {
        let inventory = world.get::<Inventory>(player);
        let arrow_to_consume: Option<(SlotIndex, ItemStack)> = find_arrow(&inventory);

<<<<<<< HEAD
        inventory.set_item_at(arrow_slot, arrow_stack);
        game.handle(
            world,
            InventoryUpdateEvent {
                slots: smallvec![arrow_slot],
                entity: player,
            },
        );
    }
=======
        if gamemode == Gamemode::Survival || gamemode == Gamemode::Adventure {
            // If no arrow was found, don't shoot
            if arrow_to_consume.is_none() {
                return;
            }
>>>>>>> develop

            // Consume arrow
            let (arrow_slot, arrow_stack) = arrow_to_consume.unwrap();
            let mut arrow_stack: ItemStack = arrow_stack;
            arrow_stack.amount -= 1;

            inventory
                .set_item_at(arrow_slot.area, arrow_slot.slot, arrow_stack)
                .unwrap();
            drop(inventory);
            game.handle(
                world,
                InventoryUpdateEvent {
                    slots: smallvec![arrow_slot],
                    entity: player,
                },
            );
        }

        let _arrow_type: Item = match arrow_to_consume {
            None => Item::Arrow, // Default to generic arrow in creative mode with none in inventory
            Some((_, arrow_stack)) => arrow_stack.ty,
        };
    }

    let timed_use = world.try_get::<ItemTimedUse>(player);

    // Spam clicking can lead to a scenario where this system is called before the UseItem system adds the component
    // In that case just return.
    if timed_use.is_none() {
        return;
    }

    let timed_use = timed_use.unwrap();

    let mut time_held = game.tick_count - timed_use.tick_start;

    // if bow not held for at least 4 ticks, don't shoot at all
    // to avoid extreme bowspamming
    if time_held < 4 {
        return;
    }

    if time_held > 20 {
        time_held = 20;
    }

    let charge_force = charge_from_ticks_held(time_held as u32);
    log::trace!("Held for {} ticks. Force of {}", time_held, charge_force);

    let init_position = *world.get::<Position>(player) + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);

    let direction = init_position.direction();

    let arrow_velocity = compute_projectile_velocity(
        glm::vec3(direction.x, direction.y, direction.z),
        charge_force as f64,
        0.0,
        &mut *game.rng(),
    );
    log::trace!(
        "Computed exit velocity: {}. Velocity is norm {}",
        arrow_velocity,
        arrow_velocity.norm()
    );

    drop(timed_use);

    world.remove::<ItemTimedUse>(player).unwrap();

    log::trace!("Spawning arrow entity.");
    let entity = entity::arrow::create()
        .with(init_position)
        .with(Velocity(arrow_velocity))
        .build()
        .spawn_in(world);
    game.handle(world, EntitySpawnEvent { entity });
}

fn find_arrow(inventory: &Inventory) -> Option<(SlotIndex, ItemStack)> {
    // Order of priority is: off-hand, hotbar (0 to 8), rest of inventory

    if let Some(offhand) = inventory.item_at(Area::Hotbar, 0).unwrap() {
        if is_arrow_item(offhand.ty) {
            return Some((slot(Area::Offhand, 0), offhand));
        }
    }

    for hotbar_slot in 0..9 {
        if let Some(hotbar_stack) = inventory.item_at(Area::Hotbar, hotbar_slot).unwrap() {
            if is_arrow_item(hotbar_stack.ty) {
                return Some((slot(Area::Hotbar, hotbar_slot), hotbar_stack));
            }
        }
    }

    for inv_slot in 0..27 {
        if let Some(inv_stack) = inventory.item_at(Area::Main, inv_slot).unwrap() {
            if is_arrow_item(inv_stack.ty) {
                return Some((slot(Area::Main, inv_slot), inv_stack));
            }
        }
    }
    None
}

fn is_arrow_item(item: Item) -> bool {
    matches!(item, Item::Arrow | Item::SpectralArrow | Item::TippedArrow)
}
