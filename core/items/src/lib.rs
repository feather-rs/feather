#![forbid(unsafe_code, warnings)]

pub use feather_definitions::Item;

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

impl Default for ItemStack {
    fn default() -> Self {
        ItemStack::new(Item::Stone, 1)
    }
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
        assert_eq!(item.vanilla_id(), 0);
        assert_eq!(Item::from_vanilla_id(0), Some(item));
    }
}
