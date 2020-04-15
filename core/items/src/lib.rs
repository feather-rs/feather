#![forbid(unsafe_code, warnings)]

use num_traits::{FromPrimitive, ToPrimitive};

#[macro_use]
extern crate num_derive;

mod item;

pub use item::Item;

impl Item {
    /// Retrieves the 1.13.2 protocol ID for this item.
    pub fn native_protocol_id(self) -> i32 {
        // Conveniently, the item enum variants are listed
        // in the order of the protocol IDs, so we can
        // just use the `ToPrimitive` implementation.
        self.to_i32().unwrap()
    }

    /// Attempts to get an item by its 1.13.2 protocol ID.
    pub fn from_native_protocol_id(id: i32) -> Option<Self>
    where
        Self: Sized,
    {
        Item::from_i32(id)
    }
}

/// Represents an item stack.
///
/// An item stack includes a type, an amount, and a bunch of properties (enchantments, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemStack {
    /// The type of this item.
    pub ty: Item,
    /// The number of items in this stack.
    pub amount: u8,
    // TODO enchantments, more
}

impl ItemStack {
    pub const fn new(ty: Item, amount: u8) -> Self {
        Self { ty, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let item = Item::Air;
        assert_eq!(item.native_protocol_id(), 0);
        assert_eq!(Item::from_native_protocol_id(0), Some(item));
    }
}
