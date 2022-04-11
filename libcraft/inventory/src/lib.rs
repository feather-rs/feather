#![allow(unused_variables)]
#![allow(clippy::identity_op)]
mod inventory;

use std::{
    cell::{Ref, RefCell, RefMut},
    error::Error,
    fmt::Debug,
    rc::Rc,
};

pub use inventory::{Area, InventoryBacking, Window};

use libcraft_items::InventorySlot;

type Slot = RefCell<InventorySlot>;

/// A handle to an inventory.
///
/// An inventory is composed of one or more _areas_, each
/// if which contains one or more item stacks stored in an array. Areas are defined
/// by the `Area` enum; examples include `Storage`, `Hotbar`, `Helmet`, `Offhand`,
/// and `CraftingInput`.
///
/// Note that an `Inventory` is a _handle_; it's backed by an `Rc`. As such, cloning
/// it is cheap and creates a new handle to the same inventory. Interior mutability
/// is used to make this safe.
#[derive(Clone)]
pub struct Inventory {
    backing: Rc<InventoryBacking<Slot>>,
    slot_mutated_callback: Option<Rc<dyn Fn(Area, usize)>>,
}

impl Debug for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inventory")
            .field("backing", &self.backing)
            .finish()
    }
}

impl Inventory {
    /// Returns whether two `Inventory` handles point to the same
    /// backing inventory.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.backing, &other.backing)
    }

    /// Gets the item at the given index within an area in this inventory.
    pub fn item(&self, area: Area, slot: usize) -> Option<Ref<InventorySlot>> {
        self.item_cell(area, slot).map(RefCell::borrow)
    }

    /// Mutably gets the item at the given index within an area in this inventory.
    pub fn item_mut(&self, area: Area, slot: usize) -> Option<RefMut<InventorySlot>> {
        if let Some(callback) = &self.slot_mutated_callback {
            (&**callback)(area, slot);
        }
        self.item_cell(area, slot).map(RefCell::borrow_mut)
    }

    fn item_cell(&self, area: Area, slot: usize) -> Option<&RefCell<InventorySlot>> {
        let slice = self.backing.area_slice(area)?;
        slice.get(slot)
    }

    pub fn to_vec(&self) -> Vec<InventorySlot> {
        let mut vec = Vec::new();
        for area in self.backing.areas() {
            if let Some(items) = self.backing.area_slice(*area) {
                for item in items {
                    let i = item.borrow();
                    vec.push(i.clone());
                }
            }
        }
        vec
    }

    /// Creates a new handle to the same inventory.
    ///
    /// This operation is the same as calling `clone()`, but it's more explicit
    /// in its intent.
    pub fn new_handle(&self) -> Inventory {
        self.clone()
    }

    /// Sets a callback that will be invoked whenever
    /// a slot in the inventory may have changed.
    ///
    /// # Panics
    /// Panics if there is more than one handle to this `Inventory`.
    pub fn set_slot_mutated_callback(&mut self, callback: impl Fn(Area, usize) + 'static) {
        assert_eq!(Rc::strong_count(&self.backing), 1, "called Inventory::set_slot_mutated_callback when more than one Inventory handle is active");
        self.slot_mutated_callback = Some(Rc::new(callback));
    }
}

#[derive(Debug)]
pub enum WindowError {
    OutOfBounds(usize),
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds(value) => {
                f.write_fmt(format_args!("Slot index {} is out of bounds", value))
            }
        }
    }
}

impl Error for WindowError {}

impl Window {
    /// Gets the item at the provided protocol index.
    /// Returns an error if index is invalid.
    pub fn item(&self, index: usize) -> Result<Ref<InventorySlot>, WindowError> {
        let (inventory, area, slot) = self
            .index_to_slot(index)
            .ok_or(WindowError::OutOfBounds(index))?;
        inventory
            .item(area, slot)
            .ok_or(WindowError::OutOfBounds(index))
    }

    /// Mutably gets the item at the provided protocol index.
    /// Returns an error if the index is invalid.
    pub fn item_mut(&self, index: usize) -> Result<RefMut<InventorySlot>, WindowError> {
        let (inventory, area, slot) = self
            .index_to_slot(index)
            .ok_or(WindowError::OutOfBounds(index))?;
        inventory
            .item_mut(area, slot)
            .ok_or(WindowError::OutOfBounds(index))
    }

    /// Sets the item at the provided protocol index.
    /// Returns an error if the index is invalid.
    pub fn set_item(&self, index: usize, item: InventorySlot) -> Result<(), WindowError> {
        *self.item_mut(index)? = item;
        Ok(())
    }

    pub fn to_vec(&self) -> Vec<InventorySlot> {
        let mut i = 0;
        let mut vec = Vec::new();
        while let Ok(item) = self.item(i) {
            vec.push(item.clone());
            i += 1;
        }
        vec
    }
}
