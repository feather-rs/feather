use feather_core::inventory::{
    Inventory, InventoryType, SlotIndex, SLOT_ARMOR_CHEST, SLOT_ARMOR_FEET, SLOT_ARMOR_HEAD,
    SLOT_ARMOR_LEGS, SLOT_HOTBAR_OFFSET, SLOT_OFFHAND,
};
use feather_core::ItemStack;
use legion::entity::Entity;
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

/// Component for storing a player's inventory.
#[derive(Clone, Debug)]
pub struct EntityInventory {
    pub inventory: Inventory,
    /// The player's held item.
    /// This is stored as an index in the range 0..9.
    pub held_item: SlotIndex,
}

impl EntityInventory {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(InventoryType::Player, 46),
            held_item: 0,
        }
    }

    /// Returns the item in this inventory's
    /// main hand.
    pub fn item_in_main_hand(&self) -> Option<&ItemStack> {
        self.inventory.item_at(SLOT_HOTBAR_OFFSET + self.held_item)
    }

    /// Sets the item in this inventory's main hand.
    pub fn set_item_in_main_hand(&mut self, item: ItemStack) {
        self.inventory
            .set_item_at(SLOT_HOTBAR_OFFSET + self.held_item, item);
    }
}

impl Default for EntityInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for EntityInventory {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.inventory
    }
}

impl DerefMut for EntityInventory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inventory
    }
}

/// An equipment slot, with variants
/// listed in the order of the Entity Equipment
/// IDs to allow for easy conversion using `ToPrimitive`/`FromPrimitive`.
#[derive(Debug, Clone, Copy, ToPrimitive, FromPrimitive, PartialEq, Eq, Hash)]
pub enum Equipment {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}

impl Equipment {
    pub fn from_slot_index(index: SlotIndex) -> Option<Self> {
        match index {
            SLOT_OFFHAND => Some(Equipment::OffHand),
            SLOT_ARMOR_FEET => Some(Equipment::Boots),
            SLOT_ARMOR_LEGS => Some(Equipment::Leggings),
            SLOT_ARMOR_CHEST => Some(Equipment::Chestplate),
            SLOT_ARMOR_HEAD => Some(Equipment::Helmet),
            _ => None,
        }
    }

    pub fn slot_index(self, held_item: SlotIndex) -> SlotIndex {
        match self {
            Equipment::MainHand => held_item + SLOT_HOTBAR_OFFSET,
            Equipment::OffHand => SLOT_OFFHAND,
            Equipment::Boots => SLOT_ARMOR_FEET,
            Equipment::Leggings => SLOT_ARMOR_LEGS,
            Equipment::Chestplate => SLOT_ARMOR_CHEST,
            Equipment::Helmet => SLOT_ARMOR_HEAD,
        }
    }
}

/// Event which is triggered when a player
/// updates their inventory.
///
/// This event could also be triggered when the player
/// changes their held item.
#[derive(Debug, Clone)]
pub struct InventoryUpdateEvent {
    /// The slot(s) affected by the update.
    ///
    /// Multiple slots could be affected when, for
    /// example, a player uses the "drag" inventory interaction.
    pub slots: SmallVec<[SlotIndex; 2]>,
    /// The player owning the updated inventory.
    pub player: Entity,
}
