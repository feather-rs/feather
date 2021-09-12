use crate::ItemStack;
use core::mem;
use std::num::NonZeroU32;
use serde::{Deserialize, Serialize};

/// Represents an Inventory slot. May be empty
/// or filled (contains an `ItemStack`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventorySlot {
    Filled(ItemStack),
    Empty,
}

impl Default for InventorySlot {
    fn default() -> Self {
        InventorySlot::Empty
    }
}

impl InventorySlot {
    /// Tries to take all items from the `InventorySlot`.
    /// If the `InventorySlot` is filled returns the `ItemStack`.
    /// If the `InventorySlot` is empty returns `None`.
    pub fn take_all(&mut self) -> Self {
        mem::take(self)
    }

    /// Tries to take (split) the specified amount from the associated
    /// `ItemStack` to this `InventorySlot`. If this `InventorySlot` is
    /// empty, the specified amount is zero, or the resulting stack is
    /// empty will return the `ItemStackError::EmptyStack` error. If the
    /// amount to take is bigger than the current amount it will return
    /// the `ItemStackError::NotEnoughAmount` error. On success returns
    /// the taken part of the `ItemStack` as a new one.
    pub fn take(&mut self, amount: NonZeroU32) -> Self {
        let (remaining, taken) = match mem::take(self) {
            Self::Empty => return Self::Empty,
            Self::Filled(stack) => stack.take(amount),
        };

        if let Some(stack) = remaining {
            *self = Self::Filled(stack)
        }
        Self::Filled(taken)
    }

    pub fn count(&self) -> u32 {
        match self {
            InventorySlot::Filled(stack) => stack.count(),
            InventorySlot::Empty => 0,
        }
    }

    /// Transfers up to `n` items to `other`.
    pub fn transfer_to(&mut self, n: NonZeroU32, other: &mut Self) {
        let taken = other.take(n);
        match (self, other) {
            (InventorySlot::Filled(us), InventorySlot::Filled(them)) if us.has_same_type(them) => {
                them.add(self.take(n).count());
            },
            _ => {
                
            }
        }
    }
}
