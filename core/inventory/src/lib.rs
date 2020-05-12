//! An implementation of inventory handling.
//!
//! # Key types
//! * `Area`: an area in an inventory, e.g. hotbar, storage, crafting
//! * `Inventory`: stores the items inside an inventory, associated
//! with their respective areas
//! * `Window`: handles mapping from protocol inventory indices
//! to internal indices used for `Inventory`.

use feather_items::ItemStack;
use maplit::btreemap;
use parking_lot::{RwLock, RwLockWriteGuard};
use std::collections::BTreeMap;
use std::iter::repeat_with;
use thiserror::Error;

mod window;

pub use window::{Error as WindowError, Window, WindowAccessor};

/// An area inside an inventory, used to differentiate between
/// different parts.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Area {
    /// A player's hotbar (9 slots total)
    Hotbar,
    /// The crafting output slot inside either a player's
    /// inventory or a crafting table. (1 slot total)
    CraftingOutput,
    /// Crafting input slots. (Total depends on window:
    /// 4 slots for `Player` and 9 slots for `Crafting`)
    CraftingInput,

    // armor
    Head,
    /// A player's chestplate
    Torso,
    Legs,
    Feet,
    Offhand,

    /// Main part of a player's inventory (27 slots total)
    Main,

    /// Chest storage (27 or 54 slots total, depending on whether
    /// chest is single or large)
    ///
    /// Note that this is not the chestplate slot; use `Torso` instead.
    Chest,
}

/// A slot in an inventory. This is an `Option<ItemStack>`;
/// if set to `None`, then there is no item in this slot.
pub type Slot = Option<ItemStack>;

/// An error emitted when accessing an item
/// fails.
#[derive(Debug, Error)]
pub enum Error {
    #[error("index {0} for area {1:?} out of bounds")]
    OutOfBounds(usize, Area),
    #[error("area {0:?} does not exist in this inventory")]
    NoSuchArea(Area),
    #[error("invalid protocol index {0}")]
    InvalidProtocolIndex(usize),
}

/// Stores items in some inventory.
///
/// Internally, items are each wrapped in a `RwLock`.
/// This allows for convenient access and shared
/// references to an `Inventory` without using
/// `RefCell`. The overhead from the use of locks
/// should be minimal.
///
/// # Structure
/// `Inventory` uses a composition-based design. It
/// stores a vector of items for each `Area` which it contains.
///
/// # Initialization
/// `Inventory` provides assorted functions to initialize inventories
/// of some kind. For example, `Inventory::large_chest()` creates an
/// empty inventory for a large chest.
///
/// # Indexing conventions
/// When an area consists of a rectangle
/// of slots on the client GUI, then indexing
/// starts at 0 with the top-left corner and proceeds
/// horizontally, then vertically.
#[derive(Default, Debug)]
pub struct Inventory {
    /// Associative array from `Area` => items
    /// contained within the `Area`.
    ///
    /// Might switch to another, more efficient map
    /// type at some point, but we have no profile
    /// results which indicate inventory handling
    /// is a bottleneck.
    slots: BTreeMap<Area, Vec<RwLock<Slot>>>,
}

impl Inventory {
    /// Creates an inventory for a player, i.e.
    /// one with hotbar, main storage, armor, offhand, survival
    /// crafting slots.
    pub fn player() -> Self {
        let slots = btreemap! {
            Area::Hotbar => empty(9),
            Area::Main => empty(27),

            Area::Head => empty(1),
            Area::Torso => empty(1),
            Area::Legs => empty(1),
            Area::Feet => empty(1),
            Area::Offhand => empty(1),

            Area::CraftingInput => empty(4),
            Area::CraftingOutput => empty(1),
        };

        Self { slots }
    }

    /// Creates an inventory for a crafting table.
    /// Contains `CraftingInput` and `CraftingOutput`
    /// areas.
    pub fn crafting_table() -> Self {
        let slots = btreemap! {
            Area::CraftingInput => empty(9),
            Area::CraftingOutput => empty(9),
        };

        Self { slots }
    }

    /// Returns the item at the given
    /// index inside some area.
    pub fn item_at(&self, area: Area, index: usize) -> Result<Slot, Error> {
        self.slot(area, index).map(RwLock::read).map(|guard| *guard)
    }

    /// Returns a mutable guard for an item
    /// at the given slot.
    pub fn item_at_mut(&self, area: Area, index: usize) -> Result<RwLockWriteGuard<Slot>, Error> {
        self.slot(area, index).map(RwLock::write)
    }

    /// Sets the item at the given index inside some area.
    ///
    /// Returns the old item in the slot.
    pub fn set_item_at(&self, area: Area, index: usize, stack: ItemStack) -> Result<Slot, Error> {
        let mut slot = self.item_at_mut(area, index)?;
        let old = *slot;
        *slot = if stack.amount == 0 {
            Slot::None
        } else {
            Slot::Some(stack)
        };
        Ok(old)
    }

    /// Removes the item at the given position. Returns
    /// the removed item.
    pub fn remove_item_at(&self, area: Area, index: usize) -> Result<Slot, Error> {
        let mut item = self.item_at_mut(area, index)?;

        Ok(item.take())
    }

    /// Returns an iterator over mutable references to all
    /// items in this inventory.
    pub fn iter_mut(&self) -> impl Iterator<Item = RwLockWriteGuard<Slot>> {
        self.slots
            .values()
            .flat_map(Vec::as_slice)
            .map(RwLock::write)
    }

    /// Returns an iterator over the areas in this inventory.
    pub fn areas<'a>(&'a self) -> impl Iterator<Item = Area> + 'a {
        self.slots.keys().copied()
    }

    fn slot(&self, area: Area, index: usize) -> Result<&RwLock<Slot>, Error> {
        let slots = self.slots(area)?;
        slots.get(index).ok_or(Error::OutOfBounds(index, area))
    }

    fn slots(&self, area: Area) -> Result<&[RwLock<Slot>], Error> {
        self.slots
            .get(&area)
            .ok_or(Error::NoSuchArea(area))
            .map(Vec::as_slice)
    }
}

fn empty(n: usize) -> Vec<RwLock<Slot>> {
    repeat_with(|| RwLock::new(None)).take(n).collect()
}
