use std::mem;

use anyhow::{anyhow, bail};
use base::{Area, Item, ItemStack};

use ecs::SysResult;
pub use generated::Window as BackingWindow;
use generated::WindowError;
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
    cursor_item: Option<ItemStack>,
    /// Current painting state (mouse drag)
    paint_state: Option<PaintState>,
}

impl Window {
    /// Creates a window from the backing window representation.
    pub fn new(inner: BackingWindow) -> Self {
        Self {
            inner,
            cursor_item: None,
            paint_state: None,
        }
    }

    /// Left-click a slot in the window.
    pub fn left_click(&mut self, slot: usize) -> SysResult {
        let mut slot_item = self.inner.item(slot)?;

        // Cases:
        // * Either the cursor slot or the clicked slot is empty; swap the two.
        // * Both slots are present but are of different types; swap the two.
        // * Both slots are present and have the same type; merge the two.
        match (slot_item.as_mut(), self.cursor_item.as_mut()) {
            (Some(slot_item), Some(cursor_item)) => {
                if cursor_item.has_same_type(slot_item) {
                    slot_item.merge_with(cursor_item);
                } else {
                    mem::swap(slot_item, cursor_item);
                }
            }
            (Some(_), None) => self.cursor_item = slot_item.take(),
            (None, Some(_)) => *slot_item = self.cursor_item.take(),
            (None, None) => (),
        }

        drop(slot_item);
        self.refresh();

        Ok(())
    }

    /// Right-clicks a slot in the window.
    pub fn right_click(&mut self, slot: usize) -> SysResult {
        let mut slot_item = self.inner.item(slot)?;

        // Cases:
        // * Cursor slot is present and clicked slot has the same item type; drop one item in the clicked slot.
        // * Clicked slot is present but cursor slot is not; move half the items into the cursor slot.
        // * Both slots are present but differ in type; swap the two.
        match (slot_item.as_mut(), self.cursor_item.as_mut()) {
            (Some(slot_item), Some(cursor_item)) => {
                if slot_item.has_same_type(cursor_item) {
                    cursor_item.transfer_to(1, slot_item);
                } else {
                    mem::swap(slot_item, cursor_item);
                }
            }
            (Some(slot_item), None) => self.cursor_item = Some(slot_item.take_half()),
            (None, Some(cursor_item)) => *slot_item = Some(cursor_item.take(1)),
            (None, None) => (),
        }

        drop(slot_item);
        self.refresh();

        Ok(())
    }

    /// Shift-clicks the given slot. (Either right or left click.)
    pub fn shift_click(&mut self, slot: usize) -> SysResult {
        let mut slot_item_guard = self.inner.item(slot)?;
        let slot_item = match slot_item_guard.as_mut() {
            Some(item) => item,
            None => return Ok(()),
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
            if area == slot_area || !will_accept(area, slot_item) {
                continue;
            }

            // Find slot with same type first
            let mut i = 0;
            while let Some(mut stack) = inventory.item(area, i) {
                if let Some(stack) = stack.as_mut() {
                    if stack.has_same_type(slot_item) {
                        slot_item.transfer_to(u32::MAX, stack);
                    }
                }
                i += 1;
            }

            // If we still haven't moved all the items, transfer to any empty space
            i = 0;
            while let Some(mut stack) = inventory.item(area, i) {
                if stack.is_none() {
                    *stack = Some(slot_item.take(u32::MAX));
                    break;
                }
                i += 1;
            }

            if slot_item.count() == 0 {
                break;
            }
        }

        drop(slot_item_guard);
        self.refresh();

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
    pub fn cursor_item(&self) -> Option<ItemStack> {
        self.cursor_item.clone()
    }

    /// Refreshes items by fixing item stacks with count=0.
    fn refresh(&mut self) {
        Self::refresh_item(&mut self.cursor_item);
        let mut i = 0;
        while let Ok(mut slot) = self.inner.item(i) {
            Self::refresh_item(&mut *slot);
            i += 1;
        }
    }

    fn refresh_item(item: &mut Option<ItemStack>) {
        if let Some(inner) = item {
            if inner.count() == 0 {
                *item = None;
            }
        }
    }

    pub fn item(&self, index: usize) -> Result<MutexGuard<Option<ItemStack>>, WindowError> {
        self.inner.item(index)
    }

    pub fn set_item(&self, index: usize, item: Option<ItemStack>) -> Result<(), WindowError> {
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
        let cursor_item = match &mut window.cursor_item {
            Some(item) => item,
            None => bail!("cannot paint without cursor item"),
        };

        match self.mouse {
            Mouse::Left => {
                let amount = cursor_item.count / self.slots.len() as u32;
                let mut remainder = cursor_item.count % self.slots.len() as u32;

                for slot in self.slots {
                    if window.inner.item(slot)?.is_some() {
                        bail!("attempted to overwrite item");
                    }

                    let amount = if amount > 0 {
                        amount
                    } else {
                        let amount = 1.min(remainder);
                        remainder -= amount;
                        amount
                    };
                    window
                        .inner
                        .set_item(slot, Some(cursor_item.take(amount)))?;
                }
            }
            Mouse::Right => {
                for slot in self.slots {
                    let mut item = window.inner.item(slot)?;
                    let item = match item.as_mut() {
                        Some(item) => item,
                        None => {
                            *item = Some(cursor_item.take(0));
                            item.as_mut().unwrap()
                        }
                    };
                    cursor_item.transfer_to(1, &mut *item);
                }
            }
        }

        window.refresh();
        Ok(())
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
        assert_eq!(window.cursor_item, None);

        let stack = ItemStack::new(Item::Diamond, 32);
        window.set_item(0, Some(stack.clone())).unwrap();
        window.left_click(0).unwrap();

        assert_eq!(window.cursor_item, Some(stack.clone()));
        assert!(window.item(0).unwrap().is_none());

        window.left_click(1).unwrap();
        assert_eq!(window.cursor_item, None);
        assert_eq!(window.item(1).unwrap().as_ref(), Some(&stack));
    }

    #[test]
    fn window_left_click_same_item() {
        let mut window = window();

        let item = ItemStack::new(Item::AcaciaSlab, 32);
        window.set_item(0, Some(item.clone())).unwrap();
        window.left_click(0).unwrap();

        window.set_item(1, Some(item)).unwrap();
        window.left_click(1).unwrap();

        assert_eq!(window.cursor_item, None);
        assert_eq!(
            window.item(1).unwrap().as_ref(),
            Some(&ItemStack::new(Item::AcaciaSlab, 64))
        );
    }

    #[test]
    fn window_right_click_pick_up_half() {
        let mut window = window();
        let stack = ItemStack::new(Item::GlassPane, 17);
        window.set_item(0, Some(stack)).unwrap();

        window.right_click(0).unwrap();
        assert_eq!(window.cursor_item, Some(ItemStack::new(Item::GlassPane, 9)));
        assert_eq!(
            window.item(0).unwrap().as_ref(),
            Some(&ItemStack::new(Item::GlassPane, 8))
        );
    }

    #[test]
    fn window_right_click_drop_one_item() {
        let mut window = window();
        let stack = ItemStack::new(Item::GlassPane, 17);
        window.cursor_item = Some(stack);

        window.right_click(1).unwrap();
        assert_eq!(
            window.cursor_item,
            Some(ItemStack::new(Item::GlassPane, 16))
        );
        assert_eq!(
            window.item(1).unwrap().as_ref(),
            Some(&ItemStack::new(Item::GlassPane, 1))
        );
    }

    #[test]
    fn window_right_click_swap() {
        let mut window = window();
        let stack1 = ItemStack::new(Item::GlassPane, 17);
        let stack2 = ItemStack::new(Item::Diamond, 2);
        window.cursor_item = Some(stack1.clone());
        window.set_item(0, Some(stack2.clone())).unwrap();

        window.right_click(0).unwrap();
        assert_eq!(window.cursor_item, Some(stack2));
        assert_eq!(window.item(0).unwrap().as_ref(), Some(&stack1));
    }

    #[test]
    fn window_shift_click_full_hotbar() {
        let inventory = Inventory::player();
        for i in 0..9 {
            *inventory.item(Area::Hotbar, i).unwrap() = Some(ItemStack::new(Item::EnderPearl, 1));
        }
        *inventory.item(Area::Storage, 0).unwrap() = Some(ItemStack::new(Item::AcaciaSign, 1));
        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });
        let index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 0)
            .unwrap();
        window.shift_click(index).unwrap();
        assert_eq!(
            window.item(index).unwrap().as_ref(),
            Some(&ItemStack::new(Item::AcaciaSign, 1))
        );
    }

    #[test]
    fn window_shift_click_available_item_in_hotbar() {
        let inventory = Inventory::player();
        *inventory.item(Area::Hotbar, 3).unwrap() = Some(ItemStack::new(Item::Stone, 4));
        *inventory.item(Area::Storage, 3).unwrap() = Some(ItemStack::new(Item::Stone, 7));
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
            window.item(hotbar_index).unwrap().as_ref(),
            Some(&ItemStack::new(Item::Stone, 11))
        );
        assert!(window.item(index).unwrap().is_none());
    }

    #[test]
    fn window_shift_click_empty_hotbar() {
        let inventory = Inventory::player();
        *inventory.item(Area::Storage, 3).unwrap() = Some(ItemStack::new(Item::Stone, 7));
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
            window.item(hotbar_index).unwrap().as_ref(),
            Some(&ItemStack::new(Item::Stone, 7))
        );
        assert!(window.item(storage_index).unwrap().is_none());
    }

    #[test]
    fn left_mouse_paint() {
        let mut window = window();
        window
            .set_item(0, Some(ItemStack::new(Item::Stone, 64)))
            .unwrap();
        window.left_click(0).unwrap();

        window.begin_left_mouse_paint();
        window.add_paint_slot(0).unwrap();
        window.add_paint_slot(1).unwrap();
        window.add_paint_slot(5).unwrap();
        window.end_paint().unwrap();

        for &slot in &[0, 1, 5] {
            assert_eq!(
                window.item(slot).unwrap().as_ref(),
                Some(&ItemStack::new(Item::Stone, 21))
            );
        }
        assert_eq!(window.cursor_item, Some(ItemStack::new(Item::Stone, 1)));
    }

    #[test]
    fn right_mouse_paint() {
        let mut window = window();
        window
            .set_item(0, Some(ItemStack::new(Item::Stone, 64)))
            .unwrap();
        window
            .set_item(4, Some(ItemStack::new(Item::Stone, 3)))
            .unwrap();
        window.left_click(0).unwrap();

        window.begin_right_mouse_paint();
        window.add_paint_slot(4).unwrap();
        window.add_paint_slot(5).unwrap();
        window.end_paint().unwrap();

        assert_eq!(
            window.item(4).unwrap().as_ref(),
            Some(&ItemStack::new(Item::Stone, 4))
        );
        assert_eq!(
            window.item(5).unwrap().as_ref(),
            Some(&ItemStack::new(Item::Stone, 1))
        );
        assert_eq!(window.cursor_item, Some(ItemStack::new(Item::Stone, 62)));
    }

    fn window() -> Window {
        Window::new(BackingWindow::Player {
            player: Inventory::player(),
        })
    }
}
