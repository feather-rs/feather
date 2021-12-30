use crate::{Item, ItemStack};
use core::mem;
use serde::{Deserialize, Serialize};
use std::panic;

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

impl From<Option<ItemStack>> for InventorySlot {
    fn from(it: Option<ItemStack>) -> Self {
        match it {
            Some(item) => InventorySlot::Filled(item),
            None => InventorySlot::Empty,
        }
    }
}

impl InventorySlot {
    pub fn new(kind: Item, count: u32) -> Self {
        if count == 0 {
            Self::Empty
        } else {
            Self::Filled(ItemStack::new(kind, count).unwrap())
        }
    }

    /// If instace of Self::Filled, then it returns Some(stack_size)
    /// where stack_size is the biggest number of items allowable
    /// for the given item in a stack.
    /// If instance of Self::Empty, then we can't know the stack
    /// size and None is returned.
    pub fn stack_size(&self) -> Option<u32> {
        match self {
            InventorySlot::Filled(x) => Some(x.stack_size()),
            InventorySlot::Empty => None,
        }
    }

    /// Takes all items and makes self empty.
    pub fn take_all(&mut self) -> Self {
        mem::take(self)
    }

    /// Takes half (rounded down) of the items in self.
    pub fn take_half(&mut self) -> Self {
        if self.count() == 0 {
            Self::Empty
        } else {
            let half = (self.count() + 1) / 2;
            self.try_take(half)
        }
    }

    /// Tries to take the specified amount from 'self'
    /// and put it into the output. If amount is bigger
    /// then what self can provide then this is the same
    /// as calling take.
    pub fn try_take(&mut self, amount: u32) -> Self {
        if amount == 0 {
            return Self::Empty;
        }

        match self {
            Self::Filled(stack) => {
                if stack.count() <= amount {
                    // We take all and set self to empty
                    mem::take(self)
                } else {
                    // We take some of self.
                    let mut out = stack.clone();
                    out.set_count(amount).unwrap();
                    stack.set_count(stack.count() - amount).unwrap();
                    Self::Filled(out)
                }
            }
            Self::Empty => Self::Empty,
        }
    }

    /// Tries to take the exact specified amount from 'self',
    /// but if that is not possible then it returns None.
    pub fn take(&mut self, amount: u32) -> Option<Self> {
        if amount <= self.count() {
            Some(self.try_take(amount))
        } else {
            None
        }
    }

    /// Returns the number of items stored in the inventory slot.
    pub fn count(&self) -> u32 {
        match self {
            InventorySlot::Filled(stack) => stack.count(),
            InventorySlot::Empty => 0,
        }
    }

    /// Should only be called if the caller can guarantee that there is space
    /// such that the new could is not greater then self.stack_size().
    /// And that the slot actually contains an item.   
    fn add_count(&mut self, n: u32) {
        match self {
            InventorySlot::Filled(x) => x.add(n).unwrap(),
            InventorySlot::Empty => panic!("add count called on empty inventory slot!"),
        };
    }

    /// Transfers up to `n` items from 'self' to `other`.
    pub fn transfer_to(&mut self, n: u32, other: &mut Self) {
        if !self.is_mergable(other) {
            return;
        }

        match (self.is_filled(), other.is_filled()) {
            (true, true) => {
                let space_in_other = other.stack_size().unwrap() - other.count();
                let moving = n.min(space_in_other).min(self.count());
                let taken = self.try_take(moving);
                other.add_count(taken.count());
            }
            (true, false) => {
                let taken = self.try_take(n);
                *other = taken;
            }
            (false, _) => {} // No items to move
        }
    }

    /// Checks if the `InventorySlot` is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            InventorySlot::Filled(_) => false,
            InventorySlot::Empty => true,
        }
    }

    /// Checks if the `InventorySlot` is filled.
    pub fn is_filled(&self) -> bool {
        !self.is_empty()
    }

    /// Returns the number of items moved from other to self.
    pub fn merge(&mut self, other: &mut Self) -> u32 {
        if !self.is_mergable(other) {
            return 0;
        }
        match (self.is_filled(), other.is_filled()) {
            (true, true) => {
                let moving = (self.stack_size().unwrap() - self.count()).min(other.count());
                let taken = other.try_take(moving);
                self.add_count(taken.count());
                taken.count()
            }
            (_, false) => 0,
            (false, true) => {
                mem::swap(self, other);
                self.count()
            }
        }
    }

    /// Returns true if either one is empty, or they
    /// contain the same ItemStack type. Does *not* consider
    /// if there is space enough for the move to happen.
    pub fn is_mergable(&self, other: &Self) -> bool {
        match (self, other) {
            (InventorySlot::Filled(a), InventorySlot::Filled(b)) => a.stackable_types(b),
            (InventorySlot::Empty, _) | (_, InventorySlot::Empty) => true,
        }
    }

    /// Returns the item kind of the inventory slot if it is filled,
    /// otherwise it returns None.
    pub fn item_kind(&self) -> Option<Item> {
        match self {
            InventorySlot::Filled(x) => Some(x.item()),
            InventorySlot::Empty => None,
        }
    }

    pub fn item_stack(&self) -> Option<&ItemStack> {
        match self {
            InventorySlot::Filled(f) => Some(f),
            InventorySlot::Empty => None,
        }
    }

    pub fn item_stack_mut(&mut self) -> Option<&mut ItemStack> {
        match self {
            InventorySlot::Filled(f) => Some(f),
            InventorySlot::Empty => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{InventorySlot, Item};

    #[test]
    fn test_merge() {
        let a = &mut InventorySlot::new(Item::Stone, 5);
        let b = &mut InventorySlot::new(Item::Stone, 5);
        a.merge(b);
        println!("{:?}", a);
        assert!(a.is_mergable(b));
        assert!(a.count() == 10);
        assert!(b.is_empty());
    }

    #[test]
    fn take_half() {
        let a = &mut InventorySlot::new(Item::Stone, 5);
        let b = a.take_half();
        assert!(a.count() + b.count() == 5);
        assert!(a.count() == 2);

        let mut b = InventorySlot::Empty;
        let c = b.take_half();
        assert!(c.is_empty() && b.is_empty())
    }

    #[test]
    fn transfer_to() {
        let a = &mut InventorySlot::new(Item::Stone, 5);
        let b = &mut InventorySlot::new(Item::Stone, 2);
        a.transfer_to(2, b);
        assert!(a.count() == 3);
        assert!(b.count() == 4);

        let a = &mut InventorySlot::new(Item::Stone, 5);
        let b = &mut InventorySlot::Empty;
        a.transfer_to(2, b);
        assert!(a.count() == 3);
        assert!(b.count() == 2);

        let a = &mut InventorySlot::Empty;
        let b = &mut InventorySlot::new(Item::Stone, 5);
        a.transfer_to(2, b);
        assert!(a.count() == 0);
        assert!(b.count() == 5);
    }
}
