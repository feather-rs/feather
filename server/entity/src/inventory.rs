use feather_core::inventory::{slot, Area, SlotIndex};
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
        self.item_at(Area::Hotbar, held_item).unwrap()
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
    pub fn from_slot_index(index: SlotIndex, held_item: usize) -> Option<Self> {
        use feather_core::inventory::Area::*;
        match index.area {
            Offhand => Some(Equipment::OffHand),
            Feet => Some(Equipment::Boots),
            Legs => Some(Equipment::Leggings),
            Torso => Some(Equipment::Chestplate),
            Head => Some(Equipment::Helmet),
            Hotbar => {
                if index.slot == held_item {
                    Some(Equipment::MainHand)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn slot_index(self, held_item: usize) -> SlotIndex {
        use feather_core::inventory::Area::*;
        match self {
            Equipment::MainHand => slot(Hotbar, held_item),
            Equipment::OffHand => slot(Offhand, 0),
            Equipment::Boots => slot(Feet, 0),
            Equipment::Leggings => slot(Legs, 0),
            Equipment::Chestplate => slot(Torso, 0),
            Equipment::Helmet => slot(Head, 0),
        }
    }
}
