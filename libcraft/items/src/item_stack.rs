#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::{Enchantment, Item};
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ItemStackMeta {
    /// The displayed title (name) of the associated `ItemStack`.
    title: String,

    /// The displayed lore of the associated `ItemStack`.
    lore: String,

    /// The damage taken by the `ItemStack`.
    damage: Option<i32>,

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

    /// Gets the `ItemStack` and returns it.
    pub fn get_item(&self) -> ItemStack {
        ItemStack {
            count: 1.try_into().unwrap(),
            ..self.clone()
        }
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
    pub fn take_half(self) -> (Option<ItemStack>, ItemStack) {
        let half = (self.count.get() + 1) / 2;
        self.take(NonZeroU32::new(half).unwrap())
    }

    /// Splits this `ItemStack` by removing the
    /// specified amount. Returns the taken part.
    pub fn take(mut self, amount: NonZeroU32) -> (Option<ItemStack>, ItemStack) {
        if self.count < amount {
            return (None, self);
        }
        let count_left: u32 = self.count.get() - amount.get();
        let taken = ItemStack {
            count: amount,
            ..self.clone()
        };
        self.count = NonZeroU32::new(count_left).unwrap();
        (Some(self), taken)
    }

    /// Merges another `ItemStack` with this one.
    pub fn merge_with(&mut self, other: Self) -> Result<(), ItemStackError> {
        if !self.has_same_type_and_damage(&other) {
            return Err(ItemStackError::IncompatibleStacks);
        }
        let new_count = (self.count.get() + other.count.get()).min(self.item.stack_size());
        self.count = NonZeroU32::new(new_count).unwrap();
        //other.count = NonZeroU32::new(other.count() - amount_added).unwrap();
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

        self.count = NonZeroU32::new(self.count.get() - transfer).unwrap();
        other.count = NonZeroU32::new(other.count.get() + transfer).unwrap();
        Ok(())
    }

    pub fn drain_into_bounded(
        mut self,
        n: u32,
        other: &mut Self,
    ) -> Result<Option<Self>, ItemStackError> {
        if !self.has_same_type(other) {
            return Err(ItemStackError::IncompatibleStacks);
        }

        // Stack size is the same for both self and other because they are the same type.
        let stack_size = self.item.stack_size();
        let space_in_other = stack_size - other.count();
        let items_in_self = self.count();
        let moving_items = space_in_other.min(n).min(items_in_self);

        other.set_count(moving_items + other.count()).unwrap();

        if self.count() - moving_items == 0 {
            Ok(None)
        } else {
            self.set_count(moving_items - items_in_self).unwrap();
            Ok(Some(self))
        }
    }

    /// Damages the item by the specified amount.
    /// If this function returns `true`, then the item is broken.
    pub fn damage(&mut self, amount: i32) -> bool {
        if self.meta.is_none() {
            return false;
        }
        match &mut self.meta.clone().unwrap().damage {
            Some(damage) => {
                *damage += amount;
                if let Some(durability) = self.item.durability() {
                    // This unwrap would only fail if our generated file contains an erroneous
                    // default damage value.
                    *damage >= durability.try_into().unwrap()
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Returns the amount of damage the items have taken.
    pub fn damage_taken(&self) -> Option<i32> {
        self.meta.as_ref().map_or(Some(0), |meta| meta.damage)
    }

    /// Returns true is the contents of other could be merged with the contents
    /// of self. This does not look at the item count, just the kind.
    /// Items can be merged when they have the same kind, damage, and enchantment.
    /// If a item has a stacksize of one then it can never be stacked.
    pub fn stackable_types(&self, other: &Self) -> bool {
        self.has_same_type(other) &&
        // Todo: make this function check that the items have same name
        // if you rename a item, then it does not stack with items that
        // dont share the rename. Someone need to explore this further.
        self.stack_size() > 1 &&
        other.stack_size() > 1
    }

    /// How many items could be stacked together
    pub fn stack_size(&self) -> u32 {
        self.item.stack_size()
    }

    pub fn metadata(&self) -> Option<&ItemStackMeta> {
        self.meta.as_ref()
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

impl ItemStackMeta {
    pub fn new(item: Item) -> Self {
        Self {
            title: item.name().to_owned(),
            lore: "".to_owned(),
            damage: None,
            repair_cost: None,
            enchantments: vec![],
        }
    }
    pub fn enchantments(&self) -> &[Enchantment] {
        &self.enchantments
    }
    pub fn enchantments_mut(&mut self) -> &mut Vec<Enchantment> {
        &mut self.enchantments
    }
}

pub struct ItemStackBuilder {
    item: Item,
    count: NonZeroU32,
    meta: Option<ItemStackMeta>,
}

impl Default for ItemStackBuilder {
    fn default() -> Self {
        Self {
            item: Item::Stone,
            count: 1.try_into().unwrap(),
            meta: Default::default(),
        }
    }
}

// Todo: implement all fields.
impl ItemStackBuilder {
    pub fn new() -> Self {
        Self {
            item: Item::Stone,
            count: 1.try_into().unwrap(),
            meta: None,
        }
    }

    pub fn with_item(item: Item) -> Self {
        Self {
            item,
            count: 1.try_into().unwrap(),
            meta: None,
        }
    }

    pub fn item(self, item: Item) -> Self {
        Self { item, ..self }
    }

    // panics if the count is zero
    pub fn count(self, count: u32) -> Self {
        Self {
            count: count.try_into().unwrap(),
            ..self
        }
    }

    pub fn title(self, title: impl AsRef<str>) -> Self {
        Self {
            meta: Some(ItemStackMeta {
                title: title.as_ref().to_owned(),
                lore: "".to_owned(),
                damage: None,
                repair_cost: None,
                enchantments: Vec::new(),
            }),
            ..self
        }
    }

    pub fn damage(mut self, damage: i32) -> Self {
        let mut meta = self.meta.unwrap_or_default();
        meta.damage = Some(damage);

        self.meta = Some(meta);
        self
    }

    /// If damage is some, then its value is applied, else this is a no-op.
    pub fn apply_damage(self, damage: Option<i32>) -> Self {
        match damage {
            Some(damage) => self.damage(damage),
            None => self,
        }
    }

    pub fn same_meta_as(mut self, other: &Self) -> Self {
        self.meta = other.meta.clone();
        self
    }
}

impl From<ItemStackBuilder> for ItemStack {
    fn from(it: ItemStackBuilder) -> Self {
        Self {
            item: it.item,
            count: it.count,
            meta: it.meta,
        }
    }
}
