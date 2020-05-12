//! Window management. Used to map inventory slot indices
//! in the protocol to `(Area, usize)` indices into an `Inventory`.
//!
//! See https://wiki.vg/Inventory for more information.

use crate::{Area, Inventory, Slot};
use feather_items::ItemStack;
use fecs::{Entity, World};
use legion::borrow::Ref;
use smallvec::{smallvec, SmallVec};
use thiserror::Error;

/// Converted from of a protocol index, used
/// to access inventories directly.
#[derive(Copy, Clone, Debug)]
struct Index {
    /// The index into `Window.inventories`, used
    /// when the window consists of multiple inventories.
    inventory: usize,
    /// The area inside the inventory.
    area: Area,
    /// The index inside the area.
    slot: usize,
}

/// Error returned when a `Window` fails to create
/// a `WindowAccessor`
#[derive(Debug, Error)]
pub enum Error {
    #[error("no inventory component for entity")]
    MissingComponent,
}

/// A window represents the current context of a player's GUI.
/// It defines the functions to convert slot indices in the protocol
/// to those used for `Inventory`.
///
/// Critically, a window may wrap over multiple inventories. For example,
/// if a player is inside a chest, then their context is `Window::chest(player, chest)`.
/// The `Window` then delegates raw `usize`s from the protocol to either
/// `player` or `chest` depending on the value of said `usize`.
#[derive(Debug, Clone)]
pub struct Window {
    /// Mapping from `usize` in the protocol
    /// to indices into inventory.
    ///
    mapping: fn(usize) -> Option<Index>,
    /// Inventories wrapped over by this `Window`.
    ///
    /// Internally, we store the `Entity` handles.
    /// When accessing inventories, we retrieve the `Inventory`
    /// component.
    inventories: SmallVec<[Entity; 2]>,
}

impl Window {
    /// Creates a new `Window` for a normal player,
    /// i.e. a player's own inventory without any
    /// crafting tables, chests, furnaces, etc. open.
    ///
    /// https://wiki.vg/Inventory#Player_Inventory
    pub fn player(player: Entity) -> Self {
        Self {
            mapping: player_mapping,
            inventories: smallvec![player],
        }
    }

    /// Retrieves a `WindowAccessor` which may be used
    /// to access the underlying inventories.
    ///
    /// Returns an error if the `Entity`s stored in the `Window`
    /// do not exist in the `World` or have no inventory components.
    pub fn accessor<'a>(&'a self, world: &'a World) -> Result<WindowAccessor<'a>, Error> {
        let inventories = self
            .inventories
            .iter()
            .map(|entity| world.try_get::<Inventory>(*entity))
            .collect::<Option<_>>()
            .ok_or(Error::MissingComponent)?;

        Ok(WindowAccessor {
            window: self,
            inventories,
        })
    }

    // TODO: more mappings as the need arises
}

/// Accessor to a set of inventories returned by `Window::accessor`.
///
/// This stores references to the wrapped `Inventory`s
/// and allows for direct inventory access through protocol
/// indices.
pub struct WindowAccessor<'a> {
    window: &'a Window,
    inventories: SmallVec<[Ref<'a, Inventory>; 2]>,
}

impl<'a> WindowAccessor<'a> {
    /// Retrieves the item at the given protocol index.
    pub fn item_at(&self, index: usize) -> Result<Slot, crate::Error> {
        self.with_inv(index, |inv, idx| inv.item_at(idx.area, idx.slot))
    }

    /// Sets the item at the given protocol index.
    ///
    /// Returns the old item.
    pub fn set_item_at(&self, index: usize, stack: ItemStack) -> Result<Slot, crate::Error> {
        self.with_inv(index, |inv, idx| inv.set_item_at(idx.area, idx.slot, stack))
    }

    /// Removes the item at the given index.
    ///
    /// Returns the removed item.
    pub fn remove_item_at(&self, index: usize) -> Result<Slot, crate::Error> {
        self.with_inv(index, |inv, idx| inv.remove_item_at(idx.area, idx.slot))
    }

    /// Removes the item at the given index

    fn with_inv<T>(
        &self,
        index: usize,
        f: impl FnOnce(&Inventory, Index) -> Result<T, crate::Error>,
    ) -> Result<T, crate::Error> {
        let mapping = self.window.mapping;
        let index = mapping(index).ok_or(crate::Error::InvalidProtocolIndex(index))?;
        let inventory = &self.inventories[index.inventory];

        f(inventory, index)
    }
}

/// https://wiki.vg/Inventory#Player_Inventory
fn player_mapping(x: usize) -> Option<Index> {
    Some(match x {
        0 => index(0, Area::CraftingOutput, 0),
        1..=4 => index(0, Area::CraftingInput, x - 1),
        5 => index(0, Area::Head, 0),
        6 => index(0, Area::Torso, 0),
        7 => index(0, Area::Legs, 0),
        8 => index(0, Area::Feet, 0),
        9..=35 => index(0, Area::Main, x - 9),
        36..=44 => index(0, Area::Hotbar, x - 36),
        45 => index(0, Area::Offhand, 0),
        _ => return None,
    })
}

fn index(inventory: usize, area: Area, slot: usize) -> Index {
    Index {
        inventory,
        area,
        slot,
    }
}
