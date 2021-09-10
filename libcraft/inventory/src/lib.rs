mod inventory;

use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

pub use inventory::{Window, Area, InventoryBacking};

use libcraft_items::ItemStack;

type Slot = Mutex<Option<ItemStack>>;

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
    pub fn item(&self, area: Area, slot: usize) -> Option<MutexGuard<Option<ItemStack>>> {
        let slice = self.backing.area_slice(area)?;
        slice.get(slot).map(Mutex::lock)
    }

    /// Gets all the items in this inventory.
    pub fn to_vec(&self) -> Vec<Option<ItemStack>> {
        let mut vec = Vec::new();
        for area in self.backing.areas() {
            for item in self.backing.area_slice(*area).unwrap() {
                vec.push(item.lock().clone());
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