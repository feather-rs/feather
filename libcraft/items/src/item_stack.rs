#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::{Enchantment, Item};
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

/// Represents an item stack.
///
/// An item stack includes an item type, an amount and a bunch of properties (enchantments, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemStack {
    /// The item type of this `ItemStack`.
    #[serde(rename = "id")]
    item: Item,

    /// The number of items in the `ItemStack`.
    #[serde(rename = "Count")]
    count: NonZeroU32,

    /// The `ItemStack` metadata, containing data such as damage,
    /// repair cost, enchantments...
    #[serde(rename = "tag")]
    meta: Option<ItemStackMeta>,
}

/// Represents the metadata of an `ItemStack`. Contains:
/// * Item title
/// * Item lore (Optional)
/// * Item damage (Optional)
/// * Item repair cost (Optional)
/// * Item enchantments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemStackMeta {
    /// The displayed title (name) of the associated `ItemStack`.
    title: String,

    /// The displayed lore of the associated `ItemStack`.
    lore: String,

    /// The damage taken by the `ItemStack`.
    damage: Option<u32>,

    /// The cost of repairing the `ItemStack`.
    repair_cost: Option<u32>,

    /// The enchantments applied to this `ItemStack`.
    enchantments: Vec<Enchantment>,
}

impl ItemStack {
    /// Creates a new `ItemStack` with the default name (title)
    /// no lore, no damage, no repair cost and no enchantments.
    pub fn new(item: Item, count: u32) -> Result<Self, ItemStackError> {
        let count = NonZeroU32::new(count);
        if count.is_none() {
            return Err(ItemStackError::EmptyStack);
        }
        Ok(Self {
            item,
            count: count.unwrap(),
            meta: Some(ItemStackMeta {
                title: String::from(item.name()),
                lore: "".to_string(),
                damage: None,
                repair_cost: None,
                enchantments: vec![],
            }),
        })
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
        if let (Some(self_meta), Some(other_meta)) = (self.meta.as_ref(), other.meta.as_ref()) {
            self_meta.damage == other_meta.damage
        } else {
            self.meta.is_none() && other.meta.is_none()
        }
    }

    /// Returns whether the given `ItemStack` has
    /// the same count as (but not necessarily the same
    /// type as) `self`.
    pub fn has_same_count(&self, other: &Self) -> bool {
        self.count == other.count
    }

    /// Returns whether the given `ItemStack` has the same
    /// type and count as (but not necessarily the same meta
    /// as) `self`.
    pub fn has_same_type_and_count(&self, other: &Self) -> bool {
        self.item == other.item && self.count == other.count
    }

    /// Returns whether the given `ItemStack` has
    /// the same type and damage as `self`.
    pub fn has_same_type_and_damage(&self, other: &Self) -> bool {
        self.item == other.item && self.has_same_damage(other)
    }

    /// Returns the item type for this `ItemStack`.
    pub fn item(&self) -> Item {
        self.item
    }

    /// Returns the number of items in this `ItemStack`.
    pub fn count(&self) -> u32 {
        self.count.get()
    }

    /// Adds more items to this `ItemStack`. Returns the new count.
    pub fn add(&mut self, count: u32) -> Result<u32, ItemStackError> {
        self.set_count(self.count.get() + count)
    }

    /// Adds more items to this `ItemStack`. Does not check if the
    /// addition will make the count to be greater than the
    /// stack size. Does not check count overflows. Returns the new count.
    pub fn unchecked_add(&mut self, count: u32) -> u32 {
        self.count = NonZeroU32::new(self.count.get() + count).unwrap();
        self.count.get()
    }

    /// Removes some items from this `ItemStack`.
    pub fn remove(&mut self, count: u32) -> Result<u32, ItemStackError> {
        if self.count.get() <= count {
            return Err(if self.count.get() == count {
                ItemStackError::EmptyStack
            } else {
                ItemStackError::NotEnoughAmount
            });
        }
        self.count = NonZeroU32::new(self.count.get() - count).unwrap();
        Ok(self.count.get())
    }

    /// Sets the item type for this `ItemStack`. Returns the new
    /// item type or fails if the current item count exceeds the
    /// new item type stack size.
    pub fn set_item(&mut self, item: Item) -> Result<Item, ItemStackError> {
        if self.count.get() > item.stack_size() {
            return Err(ItemStackError::ExceedsStackSize);
        }
        self.item = item;
        Ok(self.item)
    }

    /// Sets the item type for this `ItemStack`. Does not check if
    /// the new item type stack size will be lower than the current
    /// item count. Returns the new item type.
    pub fn unchecked_set_item(&mut self, item: Item) -> Item {
        self.item = item;
        self.item
    }

    /// Sets the count for this `ItemStack`. Returns the updated
    /// count or fails if the new count would exceed the stack
    /// size for that item type.
    pub fn set_count(&mut self, count: u32) -> Result<u32, ItemStackError> {
        let count = NonZeroU32::new(count);
        if count.is_none() {
            return Err(ItemStackError::EmptyStack);
        }
        let count = count.unwrap();
        if count.get() > self.item.stack_size() {
            return Err(ItemStackError::ExceedsStackSize);
        } else if count.get() > i32::MAX as u32 {
            return Err(ItemStackError::ClientOverflow);
        }
        self.count = count;
        Ok(self.count.get())
    }

    /// Sets the count for this `ItemStack`. It will not check if
    /// the desired count exceeds the current item type stack size.
    /// Does not check count overflows or if the parameter is zero.
    /// Returns the updated count.
    pub fn unchecked_set_count(&mut self, count: u32) -> u32 {
        self.count = NonZeroU32::new(count).unwrap();
        self.count.get()
    }

    /// Splits this `ItemStack` in half, returning the
    /// removed half. If the amount is odd, `self`
    /// will be left with the least items. Returns the taken
    /// half.
    pub fn split_half(&mut self) -> (Option<ItemStack>, ItemStack) {
        self.split((self.count.get() + 1) / 2).unwrap()
    }

    /// Splits this `ItemStack` by removing the
    /// specified amount. Returns the taken part.
    pub fn split(
        &mut self,
        amount: u32,
    ) -> Result<(Option<ItemStack>, ItemStack), (ItemStack, ItemStackError)> {
        let amount = NonZeroU32::new(amount);
        if amount.is_none() {
            return Err((self.clone(), ItemStackError::EmptyStack));
        }
        let amount = amount.unwrap();
        if self.count < amount {
            return Err((self.clone(), ItemStackError::NotEnoughAmount));
        }
        let count_left: u32 = self.count.get() - amount.get();
        let taken = ItemStack {
            count: amount,
            ..self.clone()
        };
        self.count = NonZeroU32::new(count_left).unwrap();
        Ok((
            if count_left == 0 {
                None
            } else {
                Some(self.clone())
            },
            taken,
        ))
    }

    /// Merges another `ItemStack` with this one.
    pub fn merge_with(&mut self, other: &mut Self) -> Result<(), ItemStackError> {
        if !self.has_same_type_and_damage(other) {
            return Err(ItemStackError::IncompatibleStacks);
        }
        let new_count = (self.count.get() + other.count.get()).min(self.item.stack_size());
        let amount_added = new_count - self.count.get();
        self.count = NonZeroU32::new(new_count).unwrap();
        other.count = NonZeroU32::new(other.count() - amount_added).unwrap();
        Ok(())
    }

    /// Transfers up to `n` items to `other`.
    pub fn transfer_to(&mut self, n: u32, other: &mut Self) -> Result<(), ItemStackError> {
        if self.count.get() <= n || n == 0 {
            return Err(if self.count.get() == n || n == 0 {
                ItemStackError::EmptyStack
            } else {
                ItemStackError::NotEnoughAmount
            });
        }
        let max_transfer = other.item.stack_size().saturating_sub(other.count.get());
        let transfer = max_transfer.min(self.count.get()).min(n);
        if other.count.get() + transfer > i32::MAX as u32 {
            return Err(ItemStackError::ClientOverflow);
        }
        self.count = NonZeroU32::new(transfer).unwrap();
        other.count = NonZeroU32::new(other.count.get() + transfer).unwrap();
        Ok(())
    }

    /// Damages the item by the specified amount.
    /// If this function returns `true`, then the item is broken.
    pub fn damage(&mut self, amount: u32) -> bool {
        if self.meta.is_none() {
            return false;
        }
        match &mut self.meta.clone().unwrap().damage {
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

/// An error type that may be returned when performing
/// operations over an `ItemStack`.
#[derive(Debug, Clone)]
pub enum ItemStackError {
    ClientOverflow,
    EmptyStack,
    ExceedsStackSize,
    IncompatibleStacks,
    NotEnoughAmount,
}

impl Display for ItemStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ItemStackError {}
