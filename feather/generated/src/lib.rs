use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

#[allow(clippy::all)]
mod biome;
#[allow(clippy::all)]
mod block;
#[allow(clippy::all)]
mod entity;
#[allow(clippy::all)]
mod inventory;
#[allow(clippy::all)]
mod item;
#[allow(clippy::all)]
mod particle;
#[allow(clippy::all)]
mod simplified_block;
#[allow(clippy::all)]
pub mod vanilla_tags;

pub use biome::Biome;
pub use block::BlockKind;
pub use entity::EntityKind;
pub use inventory::{Area, InventoryBacking, Window};
pub use item::Item;
pub use particle::Particle;
pub use simplified_block::SimplifiedBlockKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    pub item: Item,
    pub count: u32,

    /// Damage to the item, if it's damageable.
    pub damage: Option<u32>,
}

impl ItemStack {
    /// Creates a new `ItemStack`.
    pub fn new(item: Item, count: u32) -> Self {
        Self {
            item,
            count,
            damage: item.durability().map(|_| 0),
        }
    }

    /// Returns whether the given item stack has
    /// the same type as (but not necessarily the same
    /// amount as) `self`.
    pub fn has_same_type(&self, other: &ItemStack) -> bool {
        other.item == self.item && other.damage == self.damage
    }

    /// Returns the item type for this `ItemStack`.
    pub fn item(&self) -> Item {
        self.item
    }

    /// Returns the number of items in this `ItemStack`.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Adds more items to this ItemStack. Returns the new count.
    pub fn add(&mut self, count: u32) -> u32 {
        self.count += count;
        self.count
    }

    /// Removes some items from this ItemStack. Returns whether there
    /// were enough items to be removed.
    pub fn remove(&mut self, count: u32) -> bool {
        self.count = match self.count.checked_sub(count) {
            Some(count) => count,
            None => return false,
        };
        true
    }

    /// Sets the item for this `ItemStack`.
    pub fn set_item(&mut self, item: Item) {
        self.item = item;
    }

    /// Sets the count for this `ItemStack`.
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    /// Splits this `ItemStack` in half, returning the
    /// second item stack. If the amount is odd, `self`
    /// will be left with the least items.
    pub fn take_half(&mut self) -> ItemStack {
        let count_left = self.count / 2;
        let other_half = ItemStack {
            count: self.count - count_left,
            ..self.clone()
        };
        self.count = count_left;
        other_half
    }

    /// Splits this `ItemStack`.
    pub fn take(&mut self, amount: u32) -> ItemStack {
        let count_left = self.count.saturating_sub(amount);
        let count_lost = self.count - count_left;
        self.count = count_left;
        ItemStack {
            count: count_lost,
            ..self.clone()
        }
    }

    /// Merges another `ItemStack` with this one.
    pub fn merge_with(&mut self, other: &mut ItemStack) {
        if !self.has_same_type(other) {
            return;
        }
        let new_count = (self.count + other.count).min(self.item.stack_size());
        let amount_added = new_count - self.count;
        self.count = new_count;
        other.count -= amount_added;
    }

    /// Transfers up to `n` items to `other`.
    pub fn transfer_to(&mut self, n: u32, other: &mut ItemStack) {
        let max_transfer = other.item.stack_size().saturating_sub(other.count);
        let transfer = max_transfer.min(self.count).min(n);
        self.count -= transfer;
        other.count += transfer;
    }

    /// Damages the item by the specified amount.
    /// If this returns `true`, then the item is broken.
    pub fn damage(&mut self, amount: u32) -> bool {
        match &mut self.damage {
            Some(damage) => {
                *damage += amount;
                if let Some(durability) = self.item.durability() {
                    *damage >= durability
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

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

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("slot index {0} is out of bounds")]
    OutOfBounds(usize),
}

impl Window {
    /// Gets the item at the provided protocol index.
    /// Returns an error if index is invalid.
    pub fn item(&self, index: usize) -> Result<MutexGuard<Option<ItemStack>>, WindowError> {
        let (inventory, area, slot) = self
            .index_to_slot(index)
            .ok_or(WindowError::OutOfBounds(index))?;
        inventory
            .item(area, slot)
            .ok_or(WindowError::OutOfBounds(index))
    }

    /// Sets the item at the provided protocol index.
    /// Returns an error if the index is invalid.
    pub fn set_item(&self, index: usize, item: Option<ItemStack>) -> Result<(), WindowError> {
        *self.item(index)? = item;
        Ok(())
    }

    /// Gets a vector of all items in this window.
    pub fn to_vec(&self) -> Vec<Option<ItemStack>> {
        let mut i = 0;
        let mut vec = Vec::new();
        while let Ok(item) = self.item(i) {
            vec.push(item.clone());
            i += 1;
        }
        vec
    }
}
