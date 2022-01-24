use crate::{Enchantment, EnchantmentKind, Item};
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
    /// # Errors
    /// Return `ItemStackError::EmptyStack` when `count` is zero.
    pub fn new(item: Item, count: u32) -> Result<Self, ItemStackError> {
        let count = NonZeroU32::new(count).ok_or(ItemStackError::EmptyStack)?;
        Ok(Self {
            item,
            count,
            meta: Some(ItemStackMeta {
                title: String::from(item.name()),
                lore: "".to_owned(),
                damage: None,
                repair_cost: None,
                enchantments: vec![],
            }),
        })
    }

    /// Returns whether the given item stack has
    /// the same type as (but not necessarily the same
    /// amount as) `self`.
    #[must_use]
    pub fn has_same_type(&self, other: &Self) -> bool {
        self.item == other.item
    }

    /// Returns whether the given item stack has the same damage
    /// as `self`.
    #[must_use]
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
    #[must_use]
    pub fn has_same_count(&self, other: &Self) -> bool {
        self.count == other.count
    }

    /// Returns whether the given `ItemStack` has the same
    /// type and count as (but not necessarily the same meta
    /// as) `self`.
    #[must_use]
    pub fn has_same_type_and_count(&self, other: &Self) -> bool {
        self.item == other.item && self.count == other.count
    }

    /// Returns whether the given `ItemStack` has
    /// the same type and damage as `self`.
    #[must_use]
    pub fn has_same_type_and_damage(&self, other: &Self) -> bool {
        self.item == other.item && self.has_same_damage(other)
    }

    /// Returns the item type for this `ItemStack`.
    #[must_use]
    pub const fn item(&self) -> Item {
        self.item
    }

    /// Returns the number of items in this `ItemStack`.
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.count.get()
    }

    /// Adds more items to this `ItemStack`. Returns the new count.
    /// # Errors
    /// Returns `ExceedsStackSize` when the combined amount of items is greater than the stack size.
    /// If the new item count cannot be represented as `i32`, returns `ClientOverflow`.
    pub fn add(&mut self, count: u32) -> Result<u32, ItemStackError> {
        self.set_count(self.count.get() + count)
    }

    /// Adds more items to this `ItemStack`. Does not check if the
    /// addition will make the count to be greater than the
    /// stack size. Does not check count overflows. Returns the new count.
    /// # Panics
    /// Panics if the new item count is greater than the stack size.
    pub fn unchecked_add(&mut self, count: u32) -> u32 {
        self.count = NonZeroU32::new(self.count.get() + count).unwrap();
        self.count.get()
    }

    /// Removes some items from this `ItemStack` and return the new item count.
    /// # Errors
    /// Returns `NotEnoughItems` if the new stack would be empty.
    #[allow(clippy::missing_panics_doc)]
    pub fn remove(&mut self, count: u32) -> Result<u32, ItemStackError> {
        if self.count.get() <= count {
            return Err(ItemStackError::NotEnoughItems);
        }
        // Cannot panic because `self.count` > `count` at this point
        self.count = NonZeroU32::new(self.count.get() - count).unwrap();
        Ok(self.count.get())
    }

    /// Change the item type for this `ItemStack` and return the new item.
    /// # Errors
    /// Returns `ExceedsStackSize` when the new item's stack size is lower than the current amount of items.
    pub fn set_item(&mut self, item: Item) -> Result<Item, ItemStackError> {
        if self.count.get() > item.stack_size() {
            return Err(ItemStackError::ExceedsStackSize);
        }
        self.item = item;
        Ok(self.item)
    }

    /// Gets the `ItemStack` and returns it.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn get_item(&self) -> Self {
        Self {
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

    /// Sets the count for this `ItemStack` and returns the updated count.
    /// # Errors
    /// Returns `EmptyStack` when `count` is zero and `ExceedsStackSize` when `count` is greater than this item's stack size.
    /// If the new item count cannot be represented as `i32`, returns `ClientOverflow`.
    pub fn set_count(&mut self, count: u32) -> Result<u32, ItemStackError> {
        let count = NonZeroU32::new(count).ok_or(ItemStackError::EmptyStack)?;
        if count.get() > self.item.stack_size() {
            Err(ItemStackError::ExceedsStackSize)
        } else if count.get() > i32::MAX as u32 {
            Err(ItemStackError::ClientOverflow)
        } else {
            self.count = count;
            Ok(self.count.get())
        }
    }

    /// Sets the count for this `ItemStack`. Does not check if
    /// the desired count exceeds the current item type stack size, nor whether it overflows.
    /// Returns the updated count.
    /// # Panics
    /// Panics if `count` is zero.
    pub fn unchecked_set_count(&mut self, count: u32) -> u32 {
        self.count = NonZeroU32::new(count).unwrap();
        self.count.get()
    }

    /// Splits this `ItemStack` in half, returning the
    /// removed half. If the amount is odd, `self`
    /// will be left with the least items. Returns the taken
    /// half.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn take_half(self) -> (Option<Self>, Self) {
        let half = (self.count.get() + 1) / 2;
        // Cannot panic because `half` is always > 0; `self.count` >= 1 -> At minimum (1 + 1) / 2 = 1
        self.take(NonZeroU32::new(half).unwrap())
    }

    /// Splits this `ItemStack` by removing the
    /// specified amount. Returns the taken part.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn take(mut self, amount: NonZeroU32) -> (Option<Self>, Self) {
        if self.count <= amount {
            return (None, self);
        }
        let count_left: u32 = self.count.get() - amount.get();
        let taken = Self {
            count: amount,
            ..self.clone()
        };
        // Cannot panic because `self.count` > `amount` -> `self.count` - `amount` > 0
        self.count = NonZeroU32::new(count_left).unwrap();
        (Some(self), taken)
    }

    /// Merges another `ItemStack` with this one.
    /// # Errors
    /// Returns `IncompatibleStacks` when the two stacks have different item types.
    #[allow(clippy::missing_panics_doc)]
    pub fn merge_with(&mut self, other: &Self) -> Result<(), ItemStackError> {
        if !self.has_same_type_and_damage(other) {
            return Err(ItemStackError::IncompatibleStacks);
        }
        let new_count = (self.count.get() + other.count.get()).min(self.item.stack_size());
        // Cannot panic because `self.count` > 0 and `other.count` > 0 -> `self.count` + `other.count` > 0
        self.count = NonZeroU32::new(new_count).unwrap();
        //other.count = NonZeroU32::new(other.count() - amount_added).unwrap();
        Ok(())
    }

    /// Transfers up to `n` items to `other`.
    /// # Errors
    /// Returns `NotEnoughItems` when there aren't enough items to complete the transfer.
    /// If the new item count in `other` cannot be represented by `i32`, returns `ClientOverflow`.
    #[allow(clippy::missing_panics_doc)]
    pub fn transfer_to(&mut self, n: u32, other: &mut Self) -> Result<(), ItemStackError> {
        if self.count.get() <= n || n == 0 {
            return Err(ItemStackError::NotEnoughItems);
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

    /// Move up to `n` items from `self` to `other`.
    /// # Errors
    /// Returns `IncompatibleStacks` when the two item stacks have different items.
    #[allow(clippy::missing_panics_doc)]
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

        // Guaranteed to be <`self.stack_size` because of `space_in_other` being one of the minimums
        other.set_count(moving_items + other.count()).unwrap();

        if items_in_self - moving_items == 0 {
            Ok(None)
        } else {
            self.set_count(items_in_self - moving_items).unwrap();
            Ok(Some(self))
        }
    }

    /// Damages the item by the specified amount.
    /// If this function returns `true`, then the item is broken.
    #[allow(clippy::missing_panics_doc)]
    pub fn damage(&mut self, amount: i32) -> bool {
        if self.meta.is_none() {
            return false;
        }
        match &mut self.meta.clone().unwrap().damage {
            Some(damage) => {
                *damage += amount;
                if let Some(durability) = self.item.durability() {
                    // Convert to a larger type for a safe conversion
                    i64::from(*damage) >= i64::from(durability)
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Returns the amount of damage the items have taken.
    #[must_use]
    pub fn damage_taken(&self) -> Option<i32> {
        self.meta.as_ref().map_or(Some(0), |meta| meta.damage)
    }

    /// Returns true is the contents of other could be merged with the contents
    /// of self. This does not look at the item count, just the kind.
    /// Items can be merged when they have the same kind, damage, and enchantment.
    /// If a item has a stacksize of one then it can never be stacked.
    #[must_use]
    pub fn stackable_types(&self, other: &Self) -> bool {
        self.has_same_type(other) &&
        // Todo: make this function check that the items have same name
        // if you rename a item, then it does not stack with items that
        // dont share the rename. Someone need to explore this further.
        self.stack_size() > 1 &&
        other.stack_size() > 1
    }

    /// How many items could be stacked together
    #[must_use]
    pub fn stack_size(&self) -> u32 {
        self.item.stack_size()
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
    NotEnoughItems,
}

impl Display for ItemStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ItemStackError {}

impl ItemStackMeta {
    #[must_use]
    pub fn new(item: Item) -> Self {
        Self {
            title: item.display_name().to_owned(),
            lore: "".to_owned(),
            damage: None,
            repair_cost: None,
            enchantments: vec![],
        }
    }

    /// Get the level of the given enchantment.
    pub fn get_enchantment_level(&self, ench: EnchantmentKind) -> Option<u32> {
        self.enchantments
            .iter()
            .find(|e| e.kind() == ench)
            .map(Enchantment::level)
    }
    /// Change the level of the given enchantment in-place or add it at the end of the list.
    pub fn set_enchantment_level(&mut self, ench: EnchantmentKind, level: u32) {
        if let Some(enchant) = self.enchantments.iter_mut().find(|e| e.kind() == ench) {
            enchant.set_level(level);
        } else {
            self.enchantments.push(Enchantment::new(ench, level));
        }
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
            meta: None,
        }
    }
}

impl ItemStackBuilder {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new() -> Self {
        Self {
            item: Item::Stone,
            count: 1.try_into().unwrap(),
            meta: None,
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_item(item: Item) -> Self {
        Self {
            item,
            count: 1.try_into().unwrap(),
            meta: None,
        }
    }

    #[must_use]
    pub fn item(self, item: Item) -> Self {
        Self { item, ..self }
    }

    /// Set the item `count`.
    /// # Panics
    /// Panics if `count` is zero.
    #[must_use]
    pub fn count(self, count: u32) -> Self {
        Self {
            count: count.try_into().expect("`count` cannot be zero"),
            ..self
        }
    }

    /// Set the item `title`.
    pub fn title(mut self, title: impl AsRef<str>) -> Self {
        self.get_or_init_meta().title = title.as_ref().to_owned();
        self
    }

    /// Set the item `lore`.
    pub fn lore(mut self, lore: impl AsRef<str>) -> Self {
        self.get_or_init_meta().lore = lore.as_ref().to_owned();
        self
    }

    /// Set the item's repair cost metadata to `repair_cost`.
    #[must_use]
    pub fn repair_cost(mut self, cost: u32) -> Self {
        self.get_or_init_meta().repair_cost = Some(cost);
        self
    }

    /// Set the item's damage metadata to `damage`.
    #[must_use]
    pub fn damage(mut self, damage: i32) -> Self {
        self.get_or_init_meta().damage = Some(damage);
        self
    }

    /// Set the item's enchantment metadata to `enchantments`
    #[must_use]
    pub fn enchantments(mut self, enchantments: Vec<Enchantment>) -> Self {
        self.get_or_init_meta().enchantments = enchantments;
        self
    }

    /// If `damage` is some, then its value is applied, else this is a no-op.
    #[must_use]
    pub fn apply_damage(self, damage: Option<i32>) -> Self {
        match damage {
            Some(damage) => self.damage(damage),
            None => self,
        }
    }

    /// Copy metadate from another item.
    #[must_use]
    pub fn copy_meta(mut self, other: &Self) -> Self {
        self.meta = other.meta.clone();
        self
    }

    /// Placeholder for the `Option::get_or_insert_default` funcion, setting the default item name.
    fn get_or_init_meta(&mut self) -> &mut ItemStackMeta {
        if let Some(ref mut s) = self.meta {
            s
        } else {
            self.meta = Some(ItemStackMeta {
                title: self.item.display_name().to_owned(),
                ..ItemStackMeta::default()
            });
            self.meta.as_mut().unwrap()
        }
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
