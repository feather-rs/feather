//! Module for creating and modifying inventories of any type.

use crate::item::Item;
use smallvec::{Array, SmallVec};
use std::cmp::min;

pub type SlotIndex = usize;

// Constants representing various standard inventory slot indices

pub const SLOT_CRAFTING_OUTPUT: SlotIndex = 0;
pub const SLOT_CRAFTING_INPUT_X0_Y0: SlotIndex = 1;
pub const SLOT_CRAFTING_INPUT_X1_Y0: SlotIndex = 2;
pub const SLOT_CRAFTING_INPUT_X0_Y1: SlotIndex = 3;
pub const SLOT_CRAFTING_INPUT_X1_Y1: SlotIndex = 4;

pub const SLOT_ARMOR_MIN: SlotIndex = 5;
pub const SLOT_ARMOR_MAX: SlotIndex = 8;

pub const SLOT_ARMOR_HEAD: SlotIndex = 5;
pub const SLOT_ARMOR_CHEST: SlotIndex = 6;
pub const SLOT_ARMOR_LEGS: SlotIndex = 7;
pub const SLOT_ARMOR_FEET: SlotIndex = 8;

pub const SLOT_OFFHAND: SlotIndex = 45;

pub const SLOT_INVENTORY_OFFSET: SlotIndex = 9;
pub const SLOT_HOTBAR_OFFSET: SlotIndex = 36;

pub const HOTBAR_SIZE: SlotIndex = 9;
pub const INVENTORY_SIZE: SlotIndex = 27;

pub const SLOT_ENTITY_EQUIPMENT_MAIN_HAND: SlotIndex = 0;
pub const SLOT_ENTITY_EQUIPMENT_OFF_HAND: SlotIndex = 1;
pub const SLOT_ENTITY_EQUIPMENT_BOOTS: SlotIndex = 2;
pub const SLOT_ENTITY_EQUIPMENT_LEGGINGS: SlotIndex = 3;
pub const SLOT_ENTITY_EQUIPMENT_CHESTPLATE: SlotIndex = 4;
pub const SLOT_ENTITY_EQUIPMENT_HELMET: SlotIndex = 5;

pub type Slot = Option<ItemStack>;

lazy_static! {
    static ref COLLECT_SEARCH_ORDER: Vec<SlotIndex> = {
        let mut result = vec![];
        for x in SLOT_HOTBAR_OFFSET..SLOT_HOTBAR_OFFSET + HOTBAR_SIZE {
            result.push(x);
        }

        for x in SLOT_INVENTORY_OFFSET..SLOT_INVENTORY_OFFSET + INVENTORY_SIZE {
            result.push(x);
        }

        result
    };
}

pub fn armor_slot_to_entity_equipment(slot: SlotIndex) -> SlotIndex {
    assert!(slot >= 5 && slot <= 8);
    match slot {
        SLOT_ARMOR_HEAD => SLOT_ENTITY_EQUIPMENT_HELMET,
        SLOT_ARMOR_CHEST => SLOT_ENTITY_EQUIPMENT_CHESTPLATE,
        SLOT_ARMOR_LEGS => SLOT_ENTITY_EQUIPMENT_LEGGINGS,
        SLOT_ARMOR_FEET => SLOT_ENTITY_EQUIPMENT_BOOTS,
        _ => unreachable!(),
    }
}

/// Returns the max size of a stack with the given
/// type.
pub fn max_size(item: Item) -> u8 {
    match item {
        Item::WoodenSword
        | Item::GoldenSword
        | Item::StoneSword
        | Item::IronSword
        | Item::DiamondSword
        | Item::WoodenAxe
        | Item::GoldenAxe
        | Item::StoneAxe
        | Item::IronAxe
        | Item::DiamondAxe
        | Item::WoodenHoe
        | Item::GoldenHoe
        | Item::StoneHoe
        | Item::IronHoe
        | Item::DiamondHoe
        | Item::WoodenPickaxe
        | Item::GoldenPickaxe
        | Item::StonePickaxe
        | Item::IronPickaxe
        | Item::DiamondPickaxe
        | Item::WoodenShovel
        | Item::GoldenShovel
        | Item::StoneShovel
        | Item::IronShovel
        | Item::DiamondShovel
        | Item::LeatherChestplate
        | Item::GoldenChestplate
        | Item::ChainmailChestplate
        | Item::IronChestplate
        | Item::DiamondChestplate
        | Item::LeatherLeggings
        | Item::GoldenLeggings
        | Item::ChainmailLeggings
        | Item::IronLeggings
        | Item::DiamondLeggings
        | Item::LeatherBoots
        | Item::GoldenBoots
        | Item::ChainmailBoots
        | Item::IronBoots
        | Item::DiamondBoots
        | Item::LeatherHelmet
        | Item::GoldenHelmet
        | Item::ChainmailHelmet
        | Item::IronHelmet
        | Item::DiamondHelmet
        | Item::Bow
        | Item::WritableBook
        | Item::FlintAndSteel
        | Item::WhiteBed
        | Item::OrangeBed
        | Item::MagentaBed
        | Item::LightBlueBed
        | Item::YellowBed
        | Item::LimeBed
        | Item::PinkBed
        | Item::GrayBed
        | Item::LightGrayBed
        | Item::CyanBed
        | Item::PurpleBed
        | Item::BlueBed
        | Item::BrownBed
        | Item::GreenBed
        | Item::RedBed
        | Item::BlackBed
        | Item::ShulkerBox
        | Item::TurtleEgg
        | Item::TurtleHelmet
        | Item::FishingRod
        | Item::EnchantedBook
        | Item::Potion
        | Item::LingeringPotion
        | Item::SplashPotion
        | Item::WaterBucket
        | Item::LavaBucket
        | Item::TropicalFishBucket
        | Item::CodBucket
        | Item::MilkBucket
        | Item::PufferfishBucket
        | Item::SalmonBucket
        | Item::CarrotOnAStick
        | Item::Elytra
        | Item::Shield
        | Item::Trident
        | Item::MusicDisc13
        | Item::MusicDiscCat
        | Item::MusicDiscBlocks
        | Item::MusicDiscChirp
        | Item::MusicDiscFar
        | Item::MusicDiscMall
        | Item::MusicDiscMellohi
        | Item::MusicDiscStal
        | Item::MusicDiscStrad
        | Item::MusicDiscWard
        | Item::MusicDisc11
        | Item::MusicDiscWait
        | Item::TotemOfUndying
        | Item::Shears
        | Item::AcaciaBoat
        | Item::DarkOakBoat
        | Item::OakBoat
        | Item::SpruceBoat
        | Item::BirchBoat
        | Item::JungleBoat
        | Item::MushroomStew
        | Item::BeetrootSoup
        | Item::RabbitStew
        | Item::Cake
        | Item::Minecart
        | Item::ChestMinecart
        | Item::CommandBlockMinecart
        | Item::FurnaceMinecart
        | Item::HopperMinecart
        | Item::TntMinecart
        | Item::DiamondHorseArmor
        | Item::GoldenHorseArmor
        | Item::Saddle
        | Item::KnowledgeBook
        | Item::DebugStick
        | Item::IronHorseArmor => 1,
        Item::EnderPearl
        | Item::Snowball
        | Item::WhiteBanner
        | Item::OrangeBanner
        | Item::MagentaBanner
        | Item::LightBlueBanner
        | Item::YellowBanner
        | Item::LimeBanner
        | Item::PinkBanner
        | Item::GrayBanner
        | Item::LightGrayBanner
        | Item::CyanBanner
        | Item::PurpleBanner
        | Item::BlueBanner
        | Item::BrownBanner
        | Item::GreenBanner
        | Item::RedBanner
        | Item::BlackBanner
        | Item::Sign
        | Item::ArmorStand
        | Item::Bucket
        | Item::WrittenBook
        | Item::Egg => 16,
        _ => 64,
    }
}

/// The various types of inventories ("windows").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InventoryType {
    Player,
    Container,
    Chest,
    CraftingTable,
    Furnace,
    Dispenser,
    EnchantingTable,
    BrewingStand,
    Villager,
    Beacon,
    Anvil,
    Hopper,
    Dropper,
    ShulkerBox,
    Horse,
}

/// An inventory, consisting of a vector
/// of `Slot`s and a type.
#[derive(Debug, Clone)]
pub struct Inventory {
    /// The item vector.
    ///
    /// The vector always contains an entry
    /// for each slot in the inventory, indexed
    /// by the slot IDs. When an entry is set to
    /// `None`, there is no item in the slot.
    items: Vec<Option<ItemStack>>,
    /// The type of this inventory.
    pub ty: InventoryType,
}

impl Inventory {
    /// Creates a new inventory of the given
    /// type and number of slots.
    pub fn new(ty: InventoryType, num_slots: u32) -> Self {
        Self {
            items: vec![None; num_slots as usize],
            ty,
        }
    }

    /// Retrieves a reference to the item at the given slot index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn item_at(&self, index: SlotIndex) -> Option<&ItemStack> {
        self.items[index].as_ref()
    }

    pub fn item_at_mut(&mut self, index: SlotIndex) -> Option<&mut ItemStack> {
        self.items[index].as_mut()
    }

    /// Sets the item at the given slot index.
    pub fn set_item_at(&mut self, index: SlotIndex, item: ItemStack) {
        if item.amount == 0 {
            self.items[index] = None;
        } else {
            self.items[index] = Some(item);
        }
    }

    /// Clears the item at the given slot index, returning
    /// the old item.
    pub fn clear_item_at(&mut self, index: SlotIndex) -> Option<ItemStack> {
        self.items[index].take()
    }

    /// Attempts to insert the given item into a player
    /// inventory.
    ///
    /// Returns the affected slots and the number of remaining
    /// items which were not added to the inventory.
    pub fn collect_item(&mut self, mut item: ItemStack) -> (SmallVec<[SlotIndex; 2]>, u8) {
        let mut affected_slots = smallvec![];

        // First, look for slots already having the type.
        for slot in COLLECT_SEARCH_ORDER.iter() {
            if let Some(slot_item) = self.item_at(*slot).cloned() {
                if slot_item.ty == item.ty {
                    self.add_to_stack(&mut item, &slot_item, *slot, &mut affected_slots);

                    if item.amount == 0 {
                        return (affected_slots, 0);
                    }
                }
            }
        }

        for slot in COLLECT_SEARCH_ORDER.iter() {
            let slot_item = self.item_at(*slot).cloned();
            if slot_item.is_none() {
                let fake = ItemStack::new(item.ty, 0);
                self.add_to_stack(&mut item, &fake, *slot, &mut affected_slots);
                if item.amount == 0 {
                    return (affected_slots, 0);
                }
            }

            if let Some(slot_item) = slot_item {
                if slot_item.ty == item.ty {
                    self.add_to_stack(&mut item, &slot_item, *slot, &mut affected_slots);

                    if item.amount == 0 {
                        return (affected_slots, 0);
                    }
                }
            }
        }

        (affected_slots, item.amount)
    }

    /// Adds an item to a stack.
    fn add_to_stack<A: Array<Item = SlotIndex>>(
        &mut self,
        item: &mut ItemStack,
        slot_item: &ItemStack,
        slot: SlotIndex,
        affected_slots: &mut SmallVec<A>,
    ) {
        let added = min(item.amount, max_size(item.ty) - slot_item.amount);
        item.amount -= added;

        self.set_item_at(slot, ItemStack::new(slot_item.ty, slot_item.amount + added));
        affected_slots.push(slot);
    }

    /// Returns the number of slots in this inventory.
    pub fn slot_count(&self) -> u16 {
        self.items.len() as u16
    }

    /// Returns a reference to this inventory's items.
    pub fn items(&self) -> &[Option<ItemStack>] {
        &self.items
    }
}

/// Represents an item stack.
///
/// An item stack includes a type, an amount, and a bunch of properties (enchantments, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    /// The type of this item.
    pub ty: Item,
    /// The number of items in this stack.
    pub amount: u8,
    // TODO enchantments, more
}

impl ItemStack {
    pub fn new(ty: Item, amount: u8) -> Self {
        Self { ty, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory() {
        let mut inv = Inventory::new(InventoryType::Chest, 36);
        assert_eq!(inv.slot_count(), 36);

        inv.set_item_at(0, ItemStack::new(Item::Air, 1));

        let item = inv.item_at(0).unwrap();
        assert_eq!(item.ty, Item::Air);
        assert_eq!(item.amount, 1);

        let item = inv.item_at_mut(0).unwrap();
        item.ty = Item::Sponge;
        assert_eq!(inv.item_at(0).unwrap().ty, Item::Sponge);

        inv.clear_item_at(0);
        assert!(inv.item_at(0).is_none());
    }

    #[test]
    fn test_collect_item_basic() {
        let mut inv = Inventory::new(InventoryType::Player, 46);
        let item = ItemStack::new(Item::Cobblestone, 32);
        inv.collect_item(item.clone());
        assert_eq!(inv.item_at(SLOT_HOTBAR_OFFSET).unwrap(), &item);
    }

    #[test]
    fn test_collect_item_full() {
        let mut inv = Inventory::new(InventoryType::Player, 46);
        let item = ItemStack::new(Item::DriedKelpBlock, 64);

        for i in 0..46 {
            inv.set_item_at(i, item.clone());
        }

        inv.collect_item(ItemStack::new(Item::Cobblestone, 16));
        for i in 0..46 {
            assert_eq!(inv.item_at(i).unwrap(), &item);
        }
    }

    #[test]
    fn test_collect_item_type_already_in() {
        let mut inv = Inventory::new(InventoryType::Player, 46);
        let item = ItemStack::new(Item::Cobblestone, 33);
        inv.set_item_at(31, item.clone());

        inv.collect_item(item.clone());
        assert_eq!(inv.item_at(31).unwrap(), &ItemStack::new(item.ty, 64));
        assert_eq!(
            inv.item_at(SLOT_HOTBAR_OFFSET).unwrap(),
            &ItemStack::new(item.ty, 2)
        );
    }

    #[test]
    fn test_collect_item_overstack() {
        let mut inv = Inventory::new(InventoryType::Player, 46);
        let item = ItemStack::new(Item::DiamondSword, 1);
        inv.set_item_at(SLOT_HOTBAR_OFFSET, item.clone());

        inv.collect_item(item.clone());
        assert_eq!(inv.item_at(SLOT_HOTBAR_OFFSET).unwrap(), &item);
        assert_eq!(inv.item_at(SLOT_HOTBAR_OFFSET + 1).unwrap(), &item);
    }
}
