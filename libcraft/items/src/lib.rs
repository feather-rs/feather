#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! Libcraft crate for item manipulation.

mod enchantment;
mod inventory_slot;
mod item;
mod item_stack;

pub use enchantment::{Enchantment, EnchantmentKind};
pub use inventory_slot::InventorySlot;
pub use item::*;
pub use item_stack::{ItemStack, ItemStackBuilder, ItemStackError, ItemStackMeta};
