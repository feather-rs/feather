use crate::item_stack::ItemStackError;
use crate::ItemStack;
use core::mem;

/// Represents an Inventory slot. May be empty
/// or filled (contains an `ItemStack`).
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
    pub fn take_all(&mut self) -> Option<ItemStack> {
        match mem::take(self) {
            Self::Filled(stack) => Some(stack),
            Self::Empty => None,
        }
    }

    /// Tries to take (split) the specified amount from the associated
    /// `ItemStack` to this `InventorySlot`. If this `InventorySlot` is
    /// empty, the specified amount is zero, or the resulting stack is
    /// empty will return the `ItemStackError::EmptyStack` error. If the
    /// amount to take is bigger than the current amount it will return
    /// the `ItemStackError::NotEnoughAmount` error. On success returns
    /// the taken part of the `ItemStack` as a new one.
    pub fn take(&mut self, amount: u32) -> Result<ItemStack, ItemStackError> {
        let split = match mem::take(self) {
            Self::Empty => return Err(ItemStackError::EmptyStack),
            Self::Filled(mut stack) => stack.split(amount),
        };
        let (stack, res) = match split {
            Ok((original, new)) => (original, Ok(new)),
            Err((original, error)) => (Some(original), Err(error)),
        };
        if let Some(stack) = stack {
            *self = Self::Filled(stack)
        }
        res
    }
}
