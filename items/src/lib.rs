#![forbid(unsafe_code, warnings)]

use num_traits::{FromPrimitive, ToPrimitive};

#[macro_use]
extern crate num_derive;

mod item;

pub use item::Item;

pub trait ItemExt {
    /// Retrieves the 1.13.2 protocol ID for this item.
    fn native_protocol_id(self) -> i32;
    /// Attempts to get an item by its 1.13.2 protocol ID.
    fn from_native_protocol_id(id: i32) -> Option<Self>
    where
        Self: Sized;
}

impl ItemExt for Item {
    fn native_protocol_id(self) -> i32 {
        // Conveniently, the item enum variants are listed
        // in the order of the protocol IDs, so we can
        // just use the `ToPrimitive` implementation.
        self.to_i32().unwrap()
    }

    fn from_native_protocol_id(id: i32) -> Option<Self>
    where
        Self: Sized,
    {
        Item::from_i32(id)
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
