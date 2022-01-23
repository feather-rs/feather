use crate::{Item, ItemStack};
use core::mem;
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
        Self::Empty
    }
}

impl From<Option<ItemStack>> for InventorySlot {
    fn from(it: Option<ItemStack>) -> Self {
        it.map(Self::Filled).unwrap_or_default()
    }
}

impl From<InventorySlot> for Option<ItemStack> {
    fn from(it: InventorySlot) -> Self {
        it.into_option()
    }
}

impl InventorySlot {
    /// Creates a new instance with the type `kind` and `count` items
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(kind: Item, count: u32) -> Self {
        ItemStack::new(kind, count)
            .map(Self::Filled)
            .unwrap_or_default()
    }

    /// If instace of `Self::Filled`, then it returns `Some(stack_size)`
    /// where `stack_size` is the biggest number of items allowable
    /// for the given item in a stack.
    /// If instance of `Self::Empty`, then we can't know the stack
    /// size and None is returned.
    #[must_use]
    pub fn stack_size(&self) -> Option<u32> {
        self.map_ref(ItemStack::stack_size)
    }

    /// Takes all items and makes self empty.
    pub fn take_all(&mut self) -> Self {
        mem::take(self)
    }

    /// Takes half (rounded down) of the items in self.
    pub fn take_half(&mut self) -> Self {
        let half = (self.count() + 1) / 2;
        self.try_take(half)
    }

    /// Tries to take the specified amount from 'self'
    /// and put it into the output. If amount is bigger
    /// then what self can provide then this is the same
    /// as calling take.
    #[allow(clippy::missing_panics_doc)]
    pub fn try_take(&mut self, amount: u32) -> Self {
        if amount == 0 {
            return Self::Empty;
        }

        if let Self::Filled(stack) = self {
            if stack.count() <= amount {
                // We take all and set self to empty
                mem::take(self)
            } else {
                // We take some of self.
                let mut out = stack.clone();
                // `amount` != 0
                out.set_count(amount).unwrap();
                // `stack.count` > amount
                stack.remove(amount).unwrap();
                Self::Filled(out)
            }
        } else {
            Self::Empty
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
    #[must_use]
    pub fn count(&self) -> u32 {
        self.map_ref(ItemStack::count).unwrap_or(0)
    }

    /// Should only be called if the caller can guarantee that there is space
    /// such that the new could is not greater then `self.stack_size`().
    /// And that the slot actually contains an item.   
    fn add_count(&mut self, n: u32) {
        self.option_mut()
            .expect("add count called on empty inventory slot!")
            .add(n)
            .expect("new item count exceeds stack size");
    }

    /// Transfers up to `n` items from 'self' to `other`.
    #[allow(clippy::missing_panics_doc)]
    pub fn transfer_to(&mut self, n: u32, other: &mut Self) {
        if !self.is_mergable(other) {
            return;
        }

        match (self.is_filled(), other.is_filled()) {
            (true, true) => {
                // `other` is guaranteed to be `Filled`
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
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Checks if the `InventorySlot` is filled.
    #[must_use]
    pub const fn is_filled(&self) -> bool {
        !self.is_empty()
    }

    /// Returns the number of items moved from other to self.
    #[allow(clippy::missing_panics_doc)]
    pub fn merge(&mut self, other: &mut Self) -> u32 {
        if !self.is_mergable(other) {
            return 0;
        }
        match (self.is_filled(), other.is_filled()) {
            (true, true) => {
                // `self` is `Filled`
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
    /// contain the same `ItemStack` type. Does *not* consider
    /// if there is space enough for the move to happen.
    #[must_use]
    pub fn is_mergable(&self, other: &Self) -> bool {
        match (self, other) {
            (InventorySlot::Filled(a), InventorySlot::Filled(b)) => a.stackable_types(b),
            (InventorySlot::Empty, _) | (_, InventorySlot::Empty) => true,
        }
    }

    /// Returns the item kind of the inventory slot if it is filled,
    /// otherwise it returns None.
    #[must_use]
    pub fn item_kind(&self) -> Option<Item> {
        self.map_ref(ItemStack::item)
    }

    /// Convert `self` into an `Option<ItemStack>`
    #[must_use]
    pub fn into_option(self) -> Option<ItemStack> {
        match self {
            InventorySlot::Filled(f) => Some(f),
            InventorySlot::Empty => None,
        }
    }
    /// Convert a reference to `self` into an `Option<&ItemStack>`
    #[must_use]
    pub const fn option_ref(&self) -> Option<&ItemStack> {
        match self {
            InventorySlot::Filled(f) => Some(f),
            InventorySlot::Empty => None,
        }
    }
    /// Convert a mutable reference to `self` into an `Option<&mut ItemStack>`
    #[must_use]
    pub fn option_mut(&mut self) -> Option<&mut ItemStack> {
        match self {
            InventorySlot::Filled(f) => Some(f),
            InventorySlot::Empty => None,
        }
    }
    /// Map `f` over the inner item stack, optionally returning the resulting value.
    #[must_use]
    pub fn map<F: FnOnce(ItemStack) -> U, U>(self, f: F) -> Option<U> {
        self.into_option().map(f)
    }
    /// Map `f` over the inner item stack, optionally returning the resulting value.
    #[must_use]
    pub fn map_ref<F: FnOnce(&ItemStack) -> U, U>(&self, f: F) -> Option<U> {
        self.option_ref().map(f)
    }
    /// Map `f` over the inner item stack, optionally returning the resulting value.
    #[must_use]
    pub fn map_mut<F: FnOnce(&mut ItemStack) -> U, U>(&mut self, f: F) -> Option<U> {
        self.option_mut().map(f)
    }
}

impl IntoIterator for InventorySlot {
    type Item = ItemStack;

    type IntoIter = std::option::IntoIter<ItemStack>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_option().into_iter()
    }
}
impl<'a> IntoIterator for &'a InventorySlot {
    type Item = &'a ItemStack;

    type IntoIter = std::option::IntoIter<&'a ItemStack>;

    fn into_iter(self) -> Self::IntoIter {
        self.option_ref().into_iter()
    }
}
impl<'a> IntoIterator for &'a mut InventorySlot {
    type Item = &'a mut ItemStack;

    type IntoIter = std::option::IntoIter<&'a mut ItemStack>;

    fn into_iter(self) -> Self::IntoIter {
        self.option_mut().into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{InventorySlot, Item};

    #[test]
    fn test_merge() {
        let mut a = InventorySlot::new(Item::Stone, 5);
        let mut b = InventorySlot::new(Item::Stone, 30);
        a.merge(&mut b);
        println!("{:?}", a);
        assert!(a.is_mergable(&b));
        assert_eq!(a.count(), 35);
        assert!(b.is_empty());

        a = InventorySlot::new(Item::Stone, 60);
        b = InventorySlot::new(Item::Stone, 10);
        assert_eq!(a.merge(&mut b), 4);
        assert_eq!(b.count(), 6);

        a = InventorySlot::new(Item::AcaciaDoor, 1);
        b = InventorySlot::new(Item::AcaciaButton, 1);
        assert_eq!(a.merge(&mut b), 0);

        a = InventorySlot::new(Item::Stone, 1);
        b = InventorySlot::Empty;
        assert_eq!(a.merge(&mut b), 0);

        a = InventorySlot::Empty;
        b = InventorySlot::new(Item::Stone, 10);
        assert_eq!(a.merge(&mut b), 10);
        assert!(b.is_empty());
    }

    #[test]
    fn take_half() {
        let mut a = InventorySlot::new(Item::Stone, 5);
        let b = a.take_half();
        assert_eq!(a.count() + b.count(), 5);
        assert_eq!(a.count(), 2);

        a = InventorySlot::new(Item::Stone, 1);
        let b = a.take_half();
        assert_eq!(a.count(), 0);
        assert_eq!(b.count(), 1);

        a = InventorySlot::Empty;
        let b = a.take_half();
        assert!(a.is_empty() && b.is_empty());
    }

    #[test]
    fn transfer_to() {
        let mut a = InventorySlot::new(Item::Stone, 5);
        let mut b = InventorySlot::new(Item::Stone, 2);
        a.transfer_to(2, &mut b);
        assert_eq!(a.count(), 3);
        assert_eq!(b.count(), 4);

        a = InventorySlot::new(Item::AcaciaDoor, 3);
        b = InventorySlot::new(Item::AcaciaButton, 5);
        a.transfer_to(1, &mut b);
        assert_eq!(a.count(), 3);
        assert_eq!(b.count(), 5);

        a = InventorySlot::new(Item::Stone, 3);
        b = InventorySlot::new(Item::Stone, 5);
        a.transfer_to(10, &mut b);
        assert_eq!(a.count(), 0);
        assert_eq!(b.count(), 8);

        a = InventorySlot::new(Item::Stone, 10);
        b = InventorySlot::new(Item::Stone, 60);
        a.transfer_to(20, &mut b);
        assert_eq!(a.count(), 6);
        assert_eq!(b.count(), 64);

        a = InventorySlot::new(Item::Stone, 5);
        b = InventorySlot::Empty;
        a.transfer_to(2, &mut b);
        assert_eq!(a.count(), 3);
        assert_eq!(b.count(), 2);

        a = InventorySlot::Empty;
        b = InventorySlot::new(Item::Stone, 5);
        a.transfer_to(2, &mut b);
        assert_eq!(a.count(), 0);
        assert_eq!(b.count(), 5);
    }

    #[test]
    fn try_take() {
        let mut a = InventorySlot::new(Item::Stone, 64);
        assert_eq!(a.try_take(16), InventorySlot::new(Item::Stone, 16));
        assert_eq!(a.try_take(0), InventorySlot::Empty);
        assert_eq!(a.try_take(50), InventorySlot::new(Item::Stone, 48));
        assert_eq!(a.try_take(2), InventorySlot::Empty);
        a = InventorySlot::new(Item::Stone, 5);
        assert_eq!(a.try_take(u32::MAX), InventorySlot::new(Item::Stone, 5));
    }

    #[test]
    fn take() {
        let mut a = InventorySlot::new(Item::Stone, 64);
        assert_eq!(a.take(16), Some(InventorySlot::new(Item::Stone, 16)));
        assert_eq!(a.take(0), Some(InventorySlot::Empty));
        assert_eq!(a.take(50), None);
        assert_eq!(a.take(48), Some(InventorySlot::new(Item::Stone, 48)));
        assert_eq!(a.take(1), None);
    }
}
