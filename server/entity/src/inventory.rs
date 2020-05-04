use feather_core::inventory::{
    SlotIndex, SLOT_ARMOR_CHEST, SLOT_ARMOR_FEET, SLOT_ARMOR_HEAD, SLOT_ARMOR_LEGS,
    SLOT_HOTBAR_OFFSET, SLOT_OFFHAND,
};
use feather_core::items::ItemStack;
use feather_server_types::{HeldItem, Inventory};
use fecs::{Entity, World};
use num_derive::{FromPrimitive, ToPrimitive};

pub trait InventoryExt {
    /// Returns the item in the main hand of this entity.
    fn item_in_main_hand(&self, entity: Entity, world: &World) -> Option<ItemStack>;
}

impl InventoryExt for Inventory {
    fn item_in_main_hand(&self, entity: Entity, world: &World) -> Option<ItemStack> {
        let held_item = world.get::<HeldItem>(entity).0;
        self.item_at(SLOT_HOTBAR_OFFSET + held_item).copied()
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
