use crate::Enchantment;

use serde::{Deserialize, Serialize};

/// Contains all data related to enchantments and the
/// `ItemStack`:
/// * Enchantments applied.
/// * Enchantments stored in the case of enchanted books.
/// * Repair cost of repairing the item. (Number of XP levels to add)
///   to the base level cost when repairing, combining or renaming.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnchantmentCompound {
    /// The applied enchantments to the `ItemStack`.
    enchantments: Option<Vec<Enchantment>>,

    /// The stored enchantments if this tag is assigned to
    /// enchanted books.
    stored_enchantments: Option<Vec<Enchantment>>,

    /// Repair cost of this item, if applicable.
    repair_cost: Option<u32>,
}
