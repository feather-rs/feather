//! Handling of inventory update packets.
//! This currently includes Creative Inventory Action, Held Item
//! Change, and the venerable Click Window.

use crate::IteratorExt;
use feather_core::inventory::{Area, Inventory, SlotIndex, Window};
use feather_core::items::ItemStack;
use feather_core::network::packets::{
    ClickWindow, ConfirmTransactionClientbound, CreativeInventoryAction, HeldItemChangeServerbound,
};
use feather_core::util::Gamemode;
use feather_server_types::{
    Game, HeldItem, InventoryUpdateEvent, ItemDropEvent, Network, PacketBuffers,
};
use fecs::{Entity, World};
use smallvec::smallvec;
use std::convert::TryFrom;
use std::sync::Arc;
use thiserror::Error;

/// System for handling Creative Inventory Action packets.
#[fecs::system]
pub fn handle_creative_inventory_action(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<CreativeInventoryAction>()
        .for_each_valid(world, |world, (player, packet)| {
            // Creative Inventory Action can only be used in creative
            // mode.
            let gamemode = *world.get::<Gamemode>(player);
            if gamemode != Gamemode::Creative {
                game.disconnect(
                    player,
                    world,
                    "attempted to use Creative Inventory Action outside of creative mode",
                );
                return;
            }

            // Slot -1 means that the user clicked outside the window,
            // dropping the item.
            if packet.slot == -1 {
                match &packet.clicked_item {
                    Some(stack) => {
                        // Cause item to be dropped
                        let event = ItemDropEvent {
                            slot: None,
                            stack: *stack,
                            player,
                        };
                        game.handle(world, event);

                        // No need to update inventory
                        return;
                    }
                    None => (),
                }
            }

            let inventory = world.get::<Inventory>(player);
            let window = world.get::<Window>(player);

            let accessor = match window.accessor(world) {
                Ok(a) => a,
                Err(_) => return, // silently fail
            };

            let slot = packet.clicked_item;

            if let Err(e) = accessor.set_slot_at(packet.slot as usize, slot) {
                drop(inventory);
                drop(accessor);
                drop(window);
                game.disconnect(player, world, format!("Slot index out of bounds: {}", e));
                return;
            }

            // Trigger inventory update event
            let index = window.convert_network(packet.slot as usize).unwrap(); // already checked above
            let event = InventoryUpdateEvent {
                slots: smallvec![index.into()],
                player,
            };
            drop(inventory);
            drop(accessor);
            drop(window);
            game.handle(world, event);
        });
}

/// System for handling Held Item Change packets.
#[fecs::system]
pub fn handle_held_item_change(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<HeldItemChangeServerbound>()
        .for_each_valid(world, |world, (player, packet)| {
            if packet.slot as usize >= 9 {
                game.disconnect(player, world, "Hotbar index out of bounds");
                return;
            }

            let mut held_item = world.get_mut::<HeldItem>(player);
            held_item.0 = packet.slot as usize;

            // Trigger event
            let event = InventoryUpdateEvent {
                slots: smallvec![SlotIndex {
                    area: Area::Hotbar,
                    slot: held_item.0
                }],
                player,
            };
            drop(held_item);
            game.handle(world, event);
        });
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MouseButton {
    Left,
    Right,
}

/// Mode of a Click Window packet.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    /// Simple left or right mouse click
    SingleClick(MouseButton),
    /// Shift + either left or right click
    ///
    /// Both clicks perform the same action
    ShiftClick,
    /// Number key on interval `[1, 9]`
    NumberKey(u8),
    // TODO: middle click for "non-player inventories"
    // (probably blocked on impl of block entities?)
    ItemDrop {
        /// Whether the full stack should be dropped
        /// (this is the case for CTRL+Q)
        ///
        /// If `false`, only one item should be dropped.
        full_stack: bool,
    },

    /// A paint mode action, where the user
    /// drags their cursor over multiple
    /// slots and "paints" items into their
    /// inventory.
    Paint(PaintAction),

    /// A double click on a slot.
    DoubleClick,
}

#[derive(Debug, Error)]
enum ModeParseError {
    #[error("invalid mode ID {0}")]
    InvalidModeId(i32),
    #[error("invalid button ID {0}")]
    InvalidButtonId(u8),
    #[error("number key ID {0} is outside of the interval [0, 8]")]
    InvalidNumberKeyId(u8),
    #[error("invalid paint action ID {0}")]
    InvalidPaintActionId(u8),
    #[error("unhandled parse mode; should skip")]
    Unhandled,
}

// https://wiki.vg/index.php?title=Protocol&diff=14889&oldid=14881#Click_Window
impl<'a> TryFrom<&'a ClickWindow> for Mode {
    type Error = ModeParseError;

    fn try_from(value: &'a ClickWindow) -> Result<Self, Self::Error> {
        match value.mode {
            0 => {
                let button = parse_button(value.button)?;

                Ok(Mode::SingleClick(button))
            }
            1 => {
                // ensure button is valid
                let _button = parse_button(value.button)?;

                Ok(Mode::ShiftClick)
            }
            2 => {
                if value.button > 8 {
                    return Err(ModeParseError::InvalidNumberKeyId(value.button));
                }

                Ok(Mode::NumberKey(value.button + 1))
            }
            3 => Err(ModeParseError::Unhandled),
            4 => {
                if value.slot == -999 {
                    return Err(ModeParseError::Unhandled);
                }

                let full_stack = value.button == 1;

                Ok(Mode::ItemDrop { full_stack })
            }
            5 => {
                let action = match value.button {
                    0 => PaintAction::Start(MouseButton::Left),
                    4 => PaintAction::Start(MouseButton::Right),
                    8 => return Err(ModeParseError::Unhandled),
                    1 | 5 | 9 => PaintAction::AddSlot,
                    2 | 6 | 10 => PaintAction::Finish,
                    x => return Err(ModeParseError::InvalidPaintActionId(x)),
                };

                Ok(Mode::Paint(action))
            }
            6 => Ok(Mode::DoubleClick),
            x => Err(ModeParseError::InvalidModeId(x)),
        }
    }
}

fn parse_button(x: u8) -> Result<MouseButton, ModeParseError> {
    match x {
        0 => Ok(MouseButton::Left),
        1 => Ok(MouseButton::Right),
        x => Err(ModeParseError::InvalidButtonId(x)),
    }
}

/// Action for `Mode::Paint`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PaintAction {
    /// Start dragging with some mouse
    Start(MouseButton),
    /// Add slot for the current drag
    AddSlot,
    /// Finish dragging
    Finish,
}

/// System for handling Click Window packets.
///
/// This is a bulky one
#[fecs::system]
pub fn handle_click_windows(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
) {
    packet_buffers
        .received::<ClickWindow>()
        .for_each_valid(world, |world, (player, packet)| {
            let action_number = packet.action_number;
            // Packet format is documented at https://wiki.vg/index.php?title=Protocol&diff=14889&oldid=14881#Click_Window.
            // In effect, the `mode` field determines
            // the action to take, and the `button` field
            // then indicates what type of action.
            let mode = match Mode::try_from(&packet) {
                Ok(mode) => mode,
                Err(e) => match e {
                    ModeParseError::Unhandled => return, // ignore the error - this packet doesn't need to be handled
                    e => {
                        game.disconnect(
                            player,
                            world,
                            format!("error parsing Click Window packet: {}", e),
                        );
                        return;
                    }
                },
            };

            if let Err(e) = handle_click_window(game, world, player, packet, mode) {
                game.disconnect(
                    player,
                    world,
                    format!("error handling Click Window packet: {}", e),
                );
                return;
            }

            // Send Confirm Transaction to verify success
            let packet = ConfirmTransactionClientbound {
                window_id: 0,
                action_number,
                accepted: true, // TODO: correctly account for lag
            };
            world.get::<Network>(player).send(packet);
        });
}

fn handle_click_window(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: ClickWindow,
    mode: Mode,
) -> anyhow::Result<()> {
    match mode {
        Mode::SingleClick(button) => handle_single_click(game, world, player, packet, button),
        Mode::DoubleClick => handle_double_click(game, world, player, packet),
        Mode::ShiftClick => handle_shift_click(game, world, player, packet),
        Mode::NumberKey(key) => handle_number_key(game, world, player, packet, key),
        Mode::ItemDrop { full_stack } => handle_item_drop(game, world, player, packet, full_stack),
        Mode::Paint(action) => handle_paint(game, world, player, packet, action),
    }
}

/// Stores an item currently picked by
/// a player's cursor.
#[derive(Copy, Clone, Debug)]
struct PickedItem(ItemStack);

fn handle_single_click(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: ClickWindow,
    button: MouseButton,
) -> anyhow::Result<()> {
    if let Some(picked) = world.try_get::<PickedItem>(player).map(|i| *i) {
        // Put down the item on the clicked slot. Based on the mouse button:
        // * left => whole stack
        // * right => single item
        //
        // But if there already exists an item of a different
        // type in the slot, then the picked up item and
        // the item in the slot are swapped.

        let window = world.get::<Window>(player);
        let accessor = window.accessor(world)?;
        let current_item = accessor.item_at(packet.slot as usize)?;

        if let Some(current_item) = current_item.and_then(|item| {
            if item.ty == picked.0.ty {
                None
            } else {
                Some(item)
            }
        }) {
            // Different items - swap
            accessor.set_item_at(packet.slot as usize, picked.0)?;
            drop(accessor);
            drop(window);
            world.get_mut::<PickedItem>(player).0 = current_item;
        } else {
            // Place item on slot
            let count = match button {
                MouseButton::Left => picked.0.amount,
                MouseButton::Right => 1,
            };

            let current_count = current_item.map(|stack| stack.amount).unwrap_or(0);
            let new_count = (count + current_count).min(picked.0.ty.stack_size() as u8);

            accessor.set_item_at(packet.slot as usize, ItemStack::new(picked.0.ty, new_count))?;

            drop(accessor);
            drop(window);

            world.get_mut::<PickedItem>(player).0.amount -= new_count - current_count;

            if world.get::<PickedItem>(player).0.amount == 0 {
                world.remove::<PickedItem>(player).unwrap();
            }
        }
    } else {
        let window = world.get::<Window>(player);
        let accessor = window.accessor(world)?;

        // Pick up the item in the slot
        let picked_up = accessor.item_at(packet.slot as usize)?;
        let mut count = picked_up.map(|item| item.amount).unwrap_or(0);
        if button == MouseButton::Right {
            count = (count + 1) / 2;
        }
        if let Some(item) = picked_up {
            accessor.set_item_at(
                packet.slot as usize,
                ItemStack::new(item.ty, item.amount - count),
            )?;

            drop(accessor);
            drop(window);
            world
                .add(player, PickedItem(ItemStack::new(item.ty, count)))
                .unwrap();
        }
    }

    let window = world.get::<Window>(player);
    let slots = smallvec![window
        .convert_network(packet.slot as usize)
        .ok_or_else(|| anyhow::anyhow!("invalid slot index"))?
        .into()];
    drop(window);
    game.handle(world, InventoryUpdateEvent { player, slots });

    Ok(())
}

fn handle_double_click(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    _packet: ClickWindow,
) -> anyhow::Result<()> {
    // Double click gathers items into the picked stack until it hits the max stack size, or it has picked up all possible items

    // Keep track of what slots are modified so the client can be informed
    let mut modified_slots: smallvec::SmallVec<[SlotIndex; 2]> = smallvec![];

    let stack_size;

    let new_picked_count = {
        let mut current_count;
        let item_type;

        // Get information about the currently picked item
        if let Some(picked) = world.try_get_mut::<PickedItem>(player) {
            stack_size = picked.0.ty.stack_size() as u8;
            current_count = picked.0.amount;
            item_type = picked.0.ty;
        } else {
            // Immediately return if there is no item picked
            return Ok(());
        };

        // Get the current inventory
        let inventory = world.get::<Inventory>(player);

        // Iterate through all inventory slots, picking up items of the same type
        for (index, slot) in inventory.enumerate() {
            if let Some(slot) = slot {
                // Remove items from the inventory until the player's PickedItem has reached its max stack size
                if slot.ty == item_type && slot.amount != stack_size {
                    if let Some(mut item_stack) =
                        inventory.remove_item_at(index.area, index.slot)?
                    {
                        // Push the slot that was modified so it will be updated on the client
                        modified_slots.push(index);

                        current_count += item_stack.amount;

                        if current_count >= stack_size {
                            // Put the extra items back into the slot that had its items removed and break

                            item_stack.amount = current_count - stack_size;
                            current_count = stack_size;

                            if item_stack.amount > 0 {
                                inventory.set_item_at(index.area, index.slot, item_stack)?;
                            }

                            break;
                        }
                    }
                }
            }
        }

        // Return how many items the picked stack should now have
        current_count
    };

    // Ensure there are no situations where new items are created and the PickedItem stack size is larger than the maximum
    assert!(new_picked_count <= stack_size);

    // Update the picked item
    world.get_mut::<PickedItem>(player).0.amount = new_picked_count;

    game.handle(
        world,
        InventoryUpdateEvent {
            player,
            slots: modified_slots,
        },
    );

    Ok(())
}

fn handle_shift_click(
    _game: &mut Game,
    _world: &mut World,
    _player: Entity,
    _packet: ClickWindow,
) -> anyhow::Result<()> {
    // TODO
    Ok(())
}

fn handle_number_key(
    game: &mut Game,
    world: &mut World,
    player: Entity,
    packet: ClickWindow,
    key: u8,
) -> anyhow::Result<()> {
    let slot: usize = packet.slot as usize;
    let hotbar_slot_index = (key - 1) as usize;

    let window = world.get::<Window>(player);
    let accessor = window.accessor(world)?;

    let inventory = world.get::<Inventory>(player);

    // The slot index of the target hotbar slot
    let hotbar_slot = SlotIndex {
        area: Area::Hotbar,
        slot: hotbar_slot_index,
    };

    // Perform the swap
    if let Some(hotbar_slot_stack) = inventory.remove_item_at(Area::Hotbar, hotbar_slot_index)? {
        // Handles the case where there is an item in both target slots

        let stack_under_cursor = accessor.remove_item_at(slot)?;

        accessor.set_item_at(slot, hotbar_slot_stack)?;

        if let Some(stack) = stack_under_cursor {
            inventory.set_item_at(Area::Hotbar, hotbar_slot_index, stack)?;
        };
    } else {
        // Handles the case where there is an item in only the slot below the cursor

        let stack_under_cursor = accessor.remove_item_at(slot)?;

        if let Some(stack) = stack_under_cursor {
            inventory.set_item_at(Area::Hotbar, hotbar_slot_index, stack)?;
        };
    };

    drop(inventory);
    drop(accessor);

    // Create a list of slots that were updated with this operation so they can be sent to handlers
    let slots = smallvec![
        window
            .convert_network(slot as usize)
            .ok_or_else(|| anyhow::anyhow!("invalid slot index"))?
            .into(),
        hotbar_slot
    ];

    drop(window);

    game.handle(world, InventoryUpdateEvent { player, slots });

    Ok(())
}

fn handle_item_drop(
    _game: &mut Game,
    _world: &mut World,
    _player: Entity,
    _packet: ClickWindow,
    _full_stack: bool,
) -> anyhow::Result<()> {
    // TODO
    Ok(())
}

fn handle_paint(
    _game: &mut Game,
    _world: &mut World,
    _player: Entity,
    _packet: ClickWindow,
    _action: PaintAction,
) -> anyhow::Result<()> {
    // TODO
    Ok(())
}
