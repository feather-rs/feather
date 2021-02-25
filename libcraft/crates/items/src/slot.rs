use crate::item_stack::ItemStackError;
use crate::ItemStack;

/// Represents an `Inventory` slot. None if it's empty, Some if
/// it's not empty.
pub type Slot = Option<ItemStack>;

/// Trait of methods for the `Slot` type.
trait SlotMethods {
    fn take(&mut self, amount: u32) -> Result<(), ItemStackError>;
}

impl SlotMethods for Slot {
    /// Try to take a certain amount of items from a `Slot`. Returns an error
    /// if the slot is empty.
    fn take(&mut self, amount: u32) -> Result<(), ItemStackError> {
        if let Some(mut stack) = self.take() {
            *self = Some(stack.take(amount)?);
            Ok(())
        } else {
            Err(ItemStackError::EmptyStack)
        }
    }
}
