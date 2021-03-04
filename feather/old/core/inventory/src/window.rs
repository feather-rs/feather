//! Window management. Used to map inventory slot indices
//! in the protocol to `(Area, usize)` indices into an `Inventory`.
//!
//! See https://wiki.vg/Inventory for more information.

use crate::{Area, Inventory, Slot, SlotIndex};
use feather_items::ItemStack;
use fecs::{Entity, World};
use legion::borrow::Ref;
use smallvec::{smallvec, SmallVec};
use thiserror::Error;

pub mod constants {

}

/// Converted from of a protocol index, used
/// to access inventories directly.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Index {
    /// The index into `Window.inventories`, used
    /// when the window consists of multiple inventories.
    pub inventory: usize,
    /// The area inside the inventory.
    pub area: Area,
    /// The index inside the area.
    pub slot: usize,
}

impl From<Index> for SlotIndex {
    fn from(idx: Index) -> Self {
        SlotIndex {
            area: idx.area,
            slot: idx.slot,
        }
    }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    /// Mapping from `usize` in the protocol
    /// to indices into the inventory.
    protocol_to_slot: fn(usize) -> Option<Index>,
    /// Inverse of `protocol_to_slot`.
    slot_to_protocol: fn(Index) -> usize,
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
            protocol_to_slot: player_to_slot,
            slot_to_protocol: player_from_slot,
            inventories: smallvec![player],
        }
    }

    /// Creates a new `Window` for an opened chest.
    pub fn chest(player: Entity, chest: Entity) -> Self {
        Self {
            protocol_to_slot: chest_to_slot,
            slot_to_protocol: chest_from_slot,
            inventories: smallvec![player, chest],
        }
    }

    /// Creates a new `Window` for a large opened chest.
    ///
    /// `left_chest` is the northern or western chest, while
    //// `right_chest` is the southern or eastern one.
    pub fn large_chest(player: Entity, left_chest: Entity, right_chest: Entity) -> Self {
        Self {
            protocol_to_slot: large_chest_to_slot,
            slot_to_protocol: large_chest_from_slot,
            inventories: smallvec![player, left_chest, right_chest],
        }
    }

    /// Returns the entities other than the player
    /// which this window wraps over. For example,
    /// for `Window::chest(),` this will return the chest.
    /// For `Window::player()`, this is the empty slice.
    pub fn wrapped_entities(&self) -> &[Entity] {
        // convention: index 0 is the player
        &self.inventories[1..]
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

    /// Converts a network index to a `SlotIndex`.
    pub fn convert_network(&self, network: usize) -> Option<Index> {
        let protocol_to_slot = self.protocol_to_slot;
        protocol_to_slot(network)
    }

    /// Converts a `SlotIndex` and the entity whose
    /// inventory the `SlotIndex` belongs to to a network index.
    pub fn convert_slot(&self, slot: SlotIndex, entity: Entity) -> Option<usize> {
        let slot_to_protocol = self.slot_to_protocol;
        let inventory = self.inventories.iter().position(|e| *e == entity)?;
        let index = Index {
            area: slot.area,
            inventory,
            slot: slot.slot,
        };
        Some(slot_to_protocol(index))
    }

    /// Returns which entity has the inventory corresponding to the given
    /// protocol index.
    pub fn corresponding_entity(&self, network: usize) -> Option<Entity> {
        self.convert_network(network)
            .map(|index| self.inventories[index.inventory])
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

    /// Sets the slot (`Option<ItemStack>`) at the given protocol
    /// index.
    ///
    /// Returns the old slot.
    pub fn set_slot_at(&self, index: usize, slot: Slot) -> Result<Slot, crate::Error> {
        self.with_inv(index, |inv, idx| {
            inv.item_at_mut(idx.area, idx.slot).map(|mut guard| {
                let old = *guard;
                *guard = slot;
                old
            })
        })
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

    fn with_inv<T>(
        &self,
        index: usize,
        f: impl FnOnce(&Inventory, Index) -> Result<T, crate::Error>,
    ) -> Result<T, crate::Error> {
        let protocol_to_slot = self.window.protocol_to_slot;
        let index = protocol_to_slot(index).ok_or(crate::Error::InvalidProtocolIndex(index))?;
        let inventory = &self.inventories[index.inventory];

        f(inventory, index)
    }
}

/// https://wiki.vg/Inventory#Player_Inventory
fn player_to_slot(x: usize) -> Option<Index> {
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

fn player_from_slot(slot: Index) -> usize {
    use Area::*;
    match slot.area {
        CraftingOutput => 0,
        CraftingInput => slot.slot + 1,
        Head => 5,
        Torso => 6,
        Legs => 7,
        Feet => 8,
        Main => 9 + slot.slot,
        Hotbar => 36 + slot.slot,
        Offhand => 45,
        x => panic!("unreachable area {:?} for player window", x),
    }
}

fn chest_to_slot(x: usize) -> Option<Index> {
    Some(match x {
        0..=26 => index(1, Area::Chest, x),
        27..=53 => index(0, Area::Main, x - 27),
        54..=62 => index(0, Area::Hotbar, x - 54),
        _ => return None,
    })
}

fn chest_from_slot(slot: Index) -> usize {
    use Area::*;
    match slot.area {
        Chest => slot.slot,
        Main => slot.slot + 27,
        Hotbar => slot.slot + 54,
        x => panic!("unreachable area {:?} for chest window", x),
    }
}

fn large_chest_to_slot(x: usize) -> Option<Index> {
    Some(match x {
        0..=26 => index(1, Area::Chest, x),       // top half of chest
        27..=53 => index(2, Area::Chest, x - 27), // bottom half of chest
        54..=80 => index(0, Area::Main, x - 54),
        81..=89 => index(0, Area::Hotbar, x - 81),
        _ => return None,
    })
}

fn large_chest_from_slot(slot: Index) -> usize {
    use Area::*;
    match slot.area {
        Chest => {
            let offset = if slot.inventory == 2 { 27 } else { 0 };
            slot.slot + offset
        }
        Main => slot.slot + 54,
        Hotbar => slot.slot + 81,
        x => panic!("unreachable area {:?} for chest window", x),
    }
}

fn index(inventory: usize, area: Area, slot: usize) -> Index {
    Index {
        inventory,
        area,
        slot,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_roundtrip() {
        (0..=45).for_each(|i| assert_eq!(i, player_from_slot(player_to_slot(i).unwrap())));
    }

    #[test]
    fn chest_roundtrip() {
        (0..62).for_each(|i| assert_eq!(i, chest_from_slot(chest_to_slot(i).unwrap())));
    }

    #[test]
    fn large_chest_roundtrip() {
        (0..89).for_each(|i| assert_eq!(i, large_chest_from_slot(large_chest_to_slot(i).unwrap())));
    }
}
