mod inventory;

use parking_lot::{Mutex, MutexGuard};
use std::{error::Error, sync::Arc};

pub use inventory::{Area, InventoryBacking, Window};

use libcraft_items::InventorySlot;

type Slot = Mutex<InventorySlot>;

/// A handle to an inventory.
///
/// An inventory is composed of one or more _areas_, each
/// if which contains one or more item stacks stored in an array. Areas are defined
/// by the `Area` enum; examples include `Storage`, `Hotbar`, `Helmet`, `Offhand`,
/// and `CraftingInput`.
///
/// Note that an `Inventory` is a _handle_; it's backed by an `Arc`. As such, cloning
/// it is cheap and creates a new handle to the same inventory. Interior mutability
/// is used to make this safe.
#[derive(Debug, Clone)]
pub struct Inventory {
    backing: Arc<InventoryBacking<Slot>>,
}

impl Inventory {
    /// Returns whether two `Inventory` handles point to the same
    /// backing inventory.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.backing, &other.backing)
    }

    /// Gets the item at the given index within an area in this inventory.
    ///
    /// The returned value is a `MutexGuard` and can be mutated.
    ///
    /// # Note
    /// _Never_ keep two returned `MutexGuard`s for the same inventory alive
    /// at once. Deadlocks are not fun.
    pub fn item(&self, area: Area, slot: usize) -> Option<MutexGuard<InventorySlot>> {
        let slice = self.backing.area_slice(area)?;
        slice.get(slot).map(Mutex::lock)
    }

    pub fn to_vec(&self) -> Vec<InventorySlot> {
        let mut vec = Vec::new();
        for area in self.backing.areas() {
            if let Some(items) = self.backing.area_slice(*area) {
                for item in items {
                    let i = item.lock();
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
    pub fn item(&self, index: usize) -> Result<MutexGuard<InventorySlot>, WindowError> {
        let (inventory, area, slot) = self
            .index_to_slot(index)
            .ok_or(WindowError::OutOfBounds(index))?;
        inventory
            .item(area, slot)
            .ok_or(WindowError::OutOfBounds(index))
    }

    /// Sets the item at the provided protocol index.
    /// Returns an error if the index is invalid.
    pub fn set_item(&self, index: usize, item: InventorySlot) -> Result<(), WindowError> {
        *self.item(index)? = item;
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
