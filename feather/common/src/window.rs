use std::{mem, num::NonZeroU32};

use anyhow::{anyhow, bail};

use base::{Area, Item, ItemStack, ItemStackBuilder, ItemStackError};

use ecs::SysResult;
pub use libcraft_inventory::Window as BackingWindow;
use libcraft_inventory::{WindowError};
use libcraft_items::InventorySlot::{self, Filled, Empty};
use parking_lot::MutexGuard;

/// A player's window. Wraps one or more inventories and handles
/// conversion between protocol and slot indices.
///
/// Also provides high-level methods to interact with the inventory,
/// like [`Window::right_click`], [`Window::shift_click`], etc.
#[derive(Debug)]
pub struct Window {
    /// The backing window (contains the `Inventory`s)
    inner: BackingWindow,
    /// The item currently held by the player's cursor.
    cursor_item: InventorySlot,
    /// Current painting state (mouse drag)
    paint_state: Option<PaintState>,
}

impl Window {
    /// Creates a window from the backing window representation.
    pub fn new(inner: BackingWindow) -> Self {
        Self {
            inner,
            cursor_item: Empty,
            paint_state: None,
        }
    }

    /// Left-click a slot in the window.
    pub fn left_click(&mut self, slot: usize) -> SysResult {
        let mut slot = &mut *self.inner.item(slot)?;

        let mut cursor_slot = self.cursor_item;

        // Cases:
        // * Either the cursor slot or the clicked slot is empty; swap the two.
        // * Both slots are present but are of different types; swap the two.
        // * Both slots are present and have the same type; merge the two.
        match (&mut slot, &mut cursor_slot) {
            (Filled(slot_stack), Filled(cursor_stack)) => {
                if cursor_stack.has_same_type(slot_stack) {
                    slot_stack.merge_with(cursor_stack).unwrap();

                } else {
                    mem::swap(slot_stack, cursor_stack);
                }
            }
            (Filled(_), Empty) => cursor_slot = slot.take_all(),
            (Empty, Filled(_)) => *slot = cursor_slot.take_all(),
            (Empty, Empty) => (),
        }

        drop(slot);

        Ok(())
    }

    /// Right-clicks a slot in the window.
    pub fn right_click(&mut self, slot: usize) -> SysResult {
        let mut slot_item = &mut *self.inner.item(slot)?;
        let mut cursor_slot = &mut self.cursor_item;

        // Cases:
        // * Cursor slot is present and clicked slot has the same item type; drop one item in the clicked slot.
        // * Clicked slot is present but cursor slot is not; move half the items into the cursor slot.
        // * Both slots are present but differ in type; swap the two.
        match (&mut slot_item, &mut cursor_slot) {
            (Filled(slot_stack), Filled(cursor_stack)) => {
                if slot_stack.has_same_type(&cursor_stack) {
                    if let Err(e) = cursor_stack.transfer_to(1, slot_stack) {
                        self.cursor_item = Empty;
                    }
                } else {
                    mem::swap(slot_stack, cursor_stack);
                }
            }
            (Filled(slot_item), Empty) => {
                let (_, right) = slot_item.clone().take_half();
                self.cursor_item = Filled(right);
            }
            (Empty, Filled(cursor_item)) => {
                let mut new_slot_stack = cursor_item.clone();
                new_slot_stack.set_count(1).unwrap();
                *slot_item = Filled(new_slot_stack);
                if let Err(_) = cursor_item.remove(1) {
                    self.cursor_item = Empty;
                };
            }
            (Empty, Empty) => (),
        }

        drop(slot_item);

        Ok(())
    }

    /// Shift-clicks the given slot. (Either right or left click.)
    pub fn shift_click(&mut self, slot: usize) -> SysResult {
        let mut slot_item_guard = &mut *self.inner.item(slot)?;
        let slot_item = match slot_item_guard {
            Filled(item) => item,
            Empty => return Ok(()),
        };

        let (inventory, slot_area, _) = self.inner.index_to_slot(slot).unwrap();
        // TODO: correctly support non-player windows
        let areas_to_try = [
            Area::Hotbar,
            Area::Storage,
            Area::Helmet,
            Area::Chestplate,
            Area::Leggings,
            Area::Boots,
            Area::CraftingInput,
        ];
        for &area in &areas_to_try {
            if area == slot_area || !will_accept(area, &slot_item) {
                continue;
            }

            // Find slot with same type first
            let mut i = 0;
            while let Some(mut stack) = inventory.item(area, i) {
                if let Filled(stack) = &mut *stack {
                    if stack.has_same_type(&slot_item) {
                        slot_item.transfer_to(u32::MAX, stack).unwrap();
                    }
                }
                i += 1;
            }

            // If we still haven't moved all the items, transfer to any empty space
            i = 0;
            while let Some(mut stack) = inventory.item(area, i) {
                if stack.is_empty() {
                    let mut new_stack = slot_item.clone();
                    new_stack.set_count(1).unwrap();
                    slot_item.transfer_to(u32::MAX, &mut new_stack).unwrap();
                    new_stack.remove(1).unwrap();

                    *stack = Filled(new_stack);
                    break;
                }
                i += 1;
            }

            if slot_item.count() == 0 {
                break;
            }
        }

        drop(slot_item_guard);

        Ok(())
    }

    /// Starts a left mouse paint operation.
    pub fn begin_left_mouse_paint(&mut self) {
        self.paint_state = Some(PaintState::new(Mouse::Left));
    }

    /// Starts a right mouse paint operation.
    pub fn begin_right_mouse_paint(&mut self) {
        self.paint_state = Some(PaintState::new(Mouse::Right));
    }

    /// Adds a slot to the current paint operation.
    pub fn add_paint_slot(&mut self, slot: usize) -> SysResult {
        if let Some(state) = &mut self.paint_state {
            state.add_slot(slot)
        } else {
            Err(anyhow!("no paint operation was active"))
        }
    }

    /// Completes and executes the current paint operation.
    pub fn end_paint(&mut self) -> SysResult {
        if let Some(state) = self.paint_state.take() {
            state.finish(self)
        } else {
            Err(anyhow!("no paint operation was active"))
        }
    }

    /// Gets the item currently held in the cursor.
    pub fn cursor_item(&self) -> &InventorySlot {
        &self.cursor_item
    }

    pub fn item(&self, index: usize) -> Result<MutexGuard<InventorySlot>, WindowError> {
        self.inner.item(index)
    }

    pub fn set_item(&self, index: usize, item: InventorySlot) -> Result<(), WindowError> {
        self.inner.set_item(index, item)
    }

    pub fn inner(&self) -> &BackingWindow {
        &self.inner
    }
}

/// Determines whether the given area will accept the given item
/// for shift-click transfer.
fn will_accept(area: Area, stack: &ItemStack) -> bool {
    match area {
        Area::Storage => true,
        Area::CraftingOutput => false,
        Area::CraftingInput => false,
        Area::Helmet => matches!(
            stack.item(),
            Item::LeatherHelmet
                | Item::ChainmailHelmet
                | Item::GoldenHelmet
                | Item::IronHelmet
                | Item::DiamondHelmet
                | Item::NetheriteHelmet
        ),
        Area::Chestplate => matches!(
            stack.item(),
            Item::LeatherChestplate
                | Item::ChainmailChestplate
                | Item::GoldenChestplate
                | Item::IronChestplate
                | Item::DiamondChestplate
                | Item::NetheriteChestplate
        ),
        Area::Leggings => matches!(
            stack.item(),
            Item::LeatherHelmet
                | Item::ChainmailLeggings
                | Item::GoldenLeggings
                | Item::IronLeggings
                | Item::DiamondLeggings
                | Item::NetheriteLeggings
        ),
        Area::Boots => matches!(
            stack.item(),
            Item::LeatherBoots
                | Item::ChainmailBoots
                | Item::GoldenBoots
                | Item::IronBoots
                | Item::DiamondBoots
                | Item::NetheriteBoots
        ),
        Area::Hotbar => true,
        Area::Offhand => true,
        Area::FurnaceIngredient => true,
        Area::FurnaceFuel => true,
        Area::FurnaceOutput => false,
        Area::EnchantmentItem => true,
        Area::EnchantmentLapis => stack.item() == Item::LapisLazuli,
        Area::BrewingBottle => matches!(
            stack.item(),
            Item::GlassBottle | Item::Potion | Item::SplashPotion | Item::LingeringPotion
        ),
        Area::BrewingIngredient => true,
        Area::BrewingBlazePowder => stack.item() == Item::BlazePowder,
        Area::VillagerInput => true,
        Area::VillagerOutput => false,
        Area::BeaconPayment => matches!(
            stack.item(),
            Item::IronIngot
                | Item::GoldIngot
                | Item::Diamond
                | Item::NetheriteIngot
                | Item::Emerald
        ),
        Area::AnvilInput1 => true,
        Area::AnvilInput2 => true,
        Area::AnvilOutput => false,
        Area::Saddle => stack.item() == Item::Saddle,
        Area::HorseArmor => matches!(
            stack.item(),
            Item::LeatherHorseArmor
                | Item::IronHorseArmor
                | Item::GoldenHorseArmor
                | Item::DiamondHorseArmor
        ),
        Area::LlamaCarpet => true,
        Area::CartographyMap => matches!(stack.item(), Item::Map | Item::FilledMap),
        Area::CartographyPaper => stack.item() == Item::Paper,
        Area::CartographyOutput => false,
        Area::GrindstoneInput1 => true,
        Area::GrindstoneInput2 => true,
        Area::GrindstoneOutput => false,
        Area::LecternBook => true,
        Area::LoomBanner => true,
        Area::LoomDye => true,
        Area::LoomPattern => true,
        Area::LoomOutput => false,
        Area::StonecutterInput => true,
        Area::StonecutterOutput => false,
    }
}

/// State for a paint operation (left mouse or right mouse drag).
#[derive(Debug)]
struct PaintState {
    mouse: Mouse,
    slots: Vec<usize>,
}

impl PaintState {
    pub fn new(mouse: Mouse) -> Self {
        Self {
            mouse,
            slots: Vec::new(),
        }
    }

    pub fn add_slot(&mut self, slot: usize) -> SysResult {
        self.slots.push(slot);
        if self.slots.len() > 1000 {
            bail!("too many paint slots! malicious client?");
        }
        Ok(())
    }

    pub fn finish(self, window: &mut Window) -> SysResult {
        let mut cursor_item = match &window.cursor_item {
            Filled(item) => Some(item),
            Empty => bail!("cannot paint without cursor item"),
        };

        match self.mouse {
            Mouse::Left => self.handle_left_drag(window),
            Mouse::Right => self.handle_right_drag(window),
        }
        Ok(())
    }

    /**
        Splits cursor items evenly into every selected slot.
        Remainder of even split ends up in `window.cursor_item`.
    */
    fn handle_left_drag(&self, window: &mut Window) {
        // Number of slots that can contain cursors item kind.
        let slots = self.slots.iter().filter(|s| {
            // unwrap is safe because index is valid.
            match &*window.inner.item(**s).unwrap() {
                Filled(item_stack) => {
                    match &window.cursor_item() {
                        Filled(cursor_stack) => {
                            item_stack.has_same_type(cursor_stack)
                        },
                        Empty => false,
                    }
                },
                Empty => true,
            }
        }).count() as u32;
        
        // If slots is 0 that means there are no slots to put items into.
        // So the cursor keeps all the items.
        if slots == 0 {return};

        let items_cursor = window.cursor_item().count();

        // This can't be zero because items_cursor is the count of an ItemStack and ItemStack is NonZeroU32.
        let items_per_slot =  (items_cursor / slots).max(1);
        self.move_items_into_slots(window, items_per_slot);
    }


    /// `items_per_slot` has to be NonZero.
    fn move_items_into_slots(&self, window: &mut Window, items_per_slot: u32) {
        debug_assert!(items_per_slot > 0);
        /* for slot_index in &self.slots {
            let cursor_item = match &mut window.cursor_item {
                Filled(stack) => stack,
                Empty => return,
            };
            if window.cursor_item().is_empty() {
                // We exit because we've exhausted cursor_item.
                break
            };
            match &mut *window.inner.item(*slot_index).unwrap() {
                Filled(slot) => {
                    if slot.item() == cursor_item.item() {
                        window.cursor_item = Filled((*cursor_item).drain_into_bounded(items_per_slot, slot).unwrap().unwrap());

                    }
                },
                Empty => {
                    let mut new_slot = window.cursor_item.take(NonZeroU32::new(items_per_slot.min(window.cursor_item.count())).unwrap());
                    match new_slot {
                        Filled(mut new_slot_stack) => {
                            new_slot_stack.set_count(1).unwrap();
                        // new_slot_stack will always increase by one or more even if cursor_item ends up becoming none.
                        window.cursor_item = Filled(cursor_item.drain_into_bounded(items_per_slot, &mut new_slot_stack).unwrap().unwrap());
                        new_slot_stack.remove(1).unwrap();
                        window.inner.set_item(*slot_index, Filled(new_slot_stack)).unwrap();
                        },
                        Empty => todo!(),
                    };
                },
            }
        } */
        todo!();
    }

    fn handle_right_drag(&self, window: &mut Window) {

    }
}

#[derive(Debug)]
enum Mouse {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use base::{Inventory, Item};

    use super::*;

    #[test]
    fn window_left_click_swap() {
        let mut window = window();

        window.left_click(0).unwrap();
        assert_eq!(window.cursor_item, Empty);

        let stack = ItemStack::new(Item::Diamond, 32).unwrap();
        window.set_item(0, Filled(stack.clone())).unwrap();
        window.left_click(0).unwrap();

        assert_eq!(window.cursor_item, Filled(stack.clone()));
        assert!(window.item(0).unwrap().is_empty());

        window.left_click(1).unwrap();
        assert_eq!(window.cursor_item, Empty);
        assert_eq!(*window.item(1).unwrap(), Filled(stack));
    }

    #[test]
    fn window_left_click_same_item() {
        let mut window = window();

        let item = ItemStack::new(Item::AcaciaSlab, 32).unwrap();
        window.set_item(0, Filled(item.clone())).unwrap();
        window.left_click(0).unwrap();

        window.set_item(1, Filled(item)).unwrap();
        window.left_click(1).unwrap();

        assert_eq!(window.cursor_item, Empty);
        assert_eq!(
            *window.item(1).unwrap(),
            Filled(ItemStack::new(Item::AcaciaSlab, 64).unwrap())
        );
    }

    #[test]
    fn window_right_click_pick_up_half() {
        let mut window = window();
        let stack = ItemStack::new(Item::GlassPane, 17).unwrap();
        window.set_item(0, Filled(stack)).unwrap();

        window.right_click(0).unwrap();
        assert_eq!(
            window.cursor_item,
            Filled(ItemStack::new(Item::GlassPane, 9).unwrap())
        );
        assert_eq!(
            *window.item(0).unwrap(),
            Filled(ItemStack::new(Item::GlassPane, 8).unwrap())
        );
    }

    #[test]
    fn window_right_click_drop_one_item() {
        let mut window = window();
        let stack = ItemStack::new(Item::GlassPane, 17).unwrap();
        window.cursor_item = Filled(stack);

        window.right_click(1).unwrap();
        assert_eq!(
            window.cursor_item,
            Filled(ItemStack::new(Item::GlassPane, 16).unwrap())
        );
        assert_eq!(
            *window.item(1).unwrap(),
            Filled(ItemStack::new(Item::GlassPane, 1).unwrap())
        );
    }

    #[test]
    fn window_right_click_swap() {
        let mut window = window();
        let stack1 = ItemStack::new(Item::GlassPane, 17).unwrap();
        let stack2 = ItemStack::new(Item::Diamond, 2).unwrap();
        window.cursor_item = Filled(stack1.clone());
        window.set_item(0, Filled(stack2.clone())).unwrap();

        window.right_click(0).unwrap();
        assert_eq!(window.cursor_item, Filled(stack2));
        assert_eq!(*window.item(0).unwrap(), Filled(stack1));
    }

    #[test]
    fn window_shift_click_full_hotbar() {
        let inventory = Inventory::player();
        for i in 0..9 {
            *inventory.item(Area::Hotbar, i).unwrap() =
                Filled(ItemStack::new(Item::EnderPearl, 1).unwrap());
        }
        *inventory.item(Area::Storage, 0).unwrap() =
            Filled(ItemStack::new(Item::AcaciaSign, 1).unwrap());
        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });
        let index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 0)
            .unwrap();
        window.shift_click(index).unwrap();
        assert_eq!(
            *window.item(index).unwrap(),
            Filled(ItemStack::new(Item::AcaciaSign, 1).unwrap())
        );
    }

    #[test]
    fn window_shift_click_available_item_in_hotbar() {
        let inventory = Inventory::player();
        *inventory.item(Area::Hotbar, 3).unwrap() = Filled(ItemStack::new(Item::Stone, 4).unwrap());
        *inventory.item(Area::Storage, 3).unwrap() = Filled(ItemStack::new(Item::Stone, 7).unwrap());
        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });

        let index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 3)
            .unwrap();
        window.shift_click(index).unwrap();

        let hotbar_index = window
            .inner()
            .slot_to_index(&inventory, Area::Hotbar, 3)
            .unwrap();
        assert_eq!(
            *window.item(hotbar_index).unwrap(),
            Filled(ItemStack::new(Item::Stone, 11).unwrap())
        );
        assert!(window.item(index).unwrap().is_empty());
    }

    #[test]
    fn window_shift_click_empty_hotbar() {
        let inventory = Inventory::player();
        *inventory.item(Area::Storage, 3).unwrap() = Filled(ItemStack::new(Item::Stone, 7).unwrap());
        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });

        let storage_index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 3)
            .unwrap();
        window.shift_click(storage_index).unwrap();
        let hotbar_index = window
            .inner()
            .slot_to_index(&inventory, Area::Hotbar, 0)
            .unwrap();
        assert_eq!(
            *window.item(hotbar_index).unwrap(),
            Filled(ItemStack::new(Item::Stone, 7).unwrap())
        );
        assert!(window.item(storage_index).unwrap().is_empty());
    }

    #[test]
    fn left_mouse_paint() {
        let mut window = window();
        window
            .set_item(0, Filled(ItemStack::new(Item::Stone, 64).unwrap()))
            .unwrap();
        window.left_click(0).unwrap();

        window.begin_left_mouse_paint();
        window.add_paint_slot(0).unwrap();
        window.add_paint_slot(1).unwrap();
        window.add_paint_slot(5).unwrap();
        window.end_paint().unwrap();

        for &slot in &[0, 1, 5] {
            assert_eq!(
                *window.item(slot).unwrap(),
                Filled(ItemStack::new(Item::Stone, 21).unwrap())
            );
        }
        assert_eq!(
            window.cursor_item,
            Filled(ItemStack::new(Item::Stone, 1).unwrap())
        );
    }

    #[test]
    fn right_mouse_paint() {
        let mut window = window();
        window
            .set_item(0, Filled(ItemStack::new(Item::Stone, 2).unwrap()))
            .unwrap();
        window
            .set_item(4, Filled(ItemStack::new(Item::Stone, 3).unwrap()))
            .unwrap();
        window.left_click(0).unwrap();

        window.begin_right_mouse_paint();
        window.add_paint_slot(4).unwrap();
        window.add_paint_slot(5).unwrap();
        window.end_paint().unwrap();

        assert_eq!(
            *window.item(4).unwrap(),
            Filled(ItemStack::new(Item::Stone, 4).unwrap())
        );
        assert_eq!(
            *window.item(5).unwrap(),
            Filled(ItemStack::new(Item::Stone, 1).unwrap())
        );
        assert_eq!(
            window.cursor_item,
            Filled(ItemStack::new(Item::Stone, 1).unwrap())
        );
    }

    fn window() -> Window {
        Window::new(BackingWindow::Player {
            player: Inventory::player(),
        })
    }
}
