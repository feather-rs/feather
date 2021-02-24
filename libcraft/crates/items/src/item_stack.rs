#![forbid(unsafe_code, warnings)]

use crate::Enchantment;

// TODO Remove when all codegen has been migrated and generated. DUMMY STRUCT
#[derive(Copy, Clone)]
pub struct Item {}
impl Item {
    pub fn durability(&self) -> Option<u32> { None }
    pub fn stack_size(&self) -> u32         { 64 }
}

/// Represents an item stack.
///
/// An item stack includes an item type, an amount and a bunch o properties (enchantments, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {

    // TODO Migrate all the codegen boilerplate.
    /// The item type of this `ItemStack`.
    pub item: Item,

    /// The number of items in the `ItemStack`.
    pub count: u32,

    /// Damage to the item, if it's damageable.
    pub damage: Option<u32>,

    /// Repair cost of the item, if it's repairable.
    pub repair_cost: Option<u32>,

    /// Enchantments applied to the item.
    pub enchantments: Vec<Enchantment>,

    // TODO Add other properties like title and lore.
}

impl ItemStack {

    /// Creates a new `ItemStack`
    pub fn new(item: Item, count: u32) -> Self {
        Self {
            item,
            count,
            damage: item.durability().map(|_| 0)
        }
    }

    /// Returns whether the given item stack has
    /// the same type as (but not necessarily the same
    /// amount as) `self`.
    pub fn has_same_type(&self, other: &Self) -> bool {
        self.item == other.item
    }

    /// Returns whether the given item stack has the same damage
    /// as `self`.
    pub fn has_same_damage(&self, other: &Self) -> bool {
        self.damage == other.damage
    }

    /// Returns whether the given `ItemStack` has
    /// the same count as (but not necessarily the same
    /// type as) `self`.
    pub fn has_same_count(&self, other: &Self) -> bool {
        self.count == self.count
    }

    /// Returns whether the given `ItemStack` has the same
    /// type and count as `self`.
    pub fn has_same_type_and_count(&self, other: &Self) -> bool {
        self.item == other.item && self.count == other.count
    }

    /// Returns whether the given `ItemStack` has
    /// the same type and damage as `self`.
    pub fn has_same_type_and_damage(&self, other: &Self) -> bool {
        self.item == other.item && self.damage == other.damage
    }

    /// Returns the item type for this `ItemStack`.
    pub fn item(&self) -> Item {
        self.item
    }

    /// Returns the number of items in this `ItemStack`.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Adds more items to this `ItemStack`. Returns the new count.
    pub fn add(&mut self, count: u32) -> u32 {
        self.count += count;
        self.cout
    }

    /// Removes some items from this `ItemStack`.
    pub fn remove(&mut self, count: u32) -> Result<u32, ()> {
        self.count = match self.count.checked_sub(count) {
            Some(count) => count,
            None => return Err(())
        };
        Ok(self.count)
    }

    /// Sets the item type for this `ItemStack`.
    pub fn set_item(&mut self, item: Item) {
        self.item = item
    }

    /// Sets the count for this `ItemStack`.
    pub fn set_count(&mut self, count: u32) {
        self.count = count
    }

    /// Splits this `ItemStack` in half, returning the
    /// removed half. If the amount is odd, `self`
    /// will be left with the least items.
    pub fn take_half(&mut self) -> ItemStack {
        self.take((self.count as f64 / 2 as f64).ceil() as u32).unwrap()
    }

    /// Splits this `ItemStack` by removing the
    /// specified amount.
    pub fn take(&mut self, amount: u32) -> Result<ItemStack, ()> {
        let count_left = self.count - amount;
        if count_left < 0 { return Err(()) }
        let other_half = ItemStack {
            count: self.count - count_left,
            ..self.clone()
        };
        self.count = count_left;
        Ok(other_half)
    }

    /// Merges another `ItemStack` with this one.
    pub fn merge_with(&mut self, other: &mut Self) -> bool {
        if !self.has_same_type_and_damage(other) {
            return false
        }
        let new_count = (self.count + other.count).min(self.item.stack_size());
        let amount_added = new_count - self.count;
        self.count = new_count;
        other.count -= amount_added;
        true
    }

    /// Transfers up to `n` items to `other`.
    pub fn transfer_to(&mut self, n: u32, other: &mut Self) {
        let max_transfer = other.item.stack_size().saturating_sub(other.count);
        let transfer = max_transfer.min(self.count).min(n);
        self.count -= transfer;
        other.count += transfer;
    }

    /// Damages the item by the specified amount.
    /// If this function returns `true`, then the item is broken.
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
            None => false
        }
    }
}