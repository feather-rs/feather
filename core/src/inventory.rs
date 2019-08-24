//! Module for creating and modifying inventories of any type.

use crate::item::Item;
use smallvec::SmallVec;
use std::cmp::min;

/// All possible slots in an inventory.
///
/// Some terms which are used here:
/// * "Hotbar" refers to the hotbar.
/// * "Main" refers to the 27 slots of the player
/// inventory not contained within the hotbar,
/// armor, and crafting slots.
/// * "Crafting" refers to the crafting area of a player's
/// inventory (but not a crafting table).
/// * "CraftingTable" refers to slots in a crafting table.
/// * "Chest" refers to the slots of a chest, either normal
/// or large.
/// TODO more of these
///
/// Variants come in the form `${Category}${Slot}`, where
/// `Slot` is the number of the slot. These slot numbers
/// are incremented consecutively from left to right and then
/// down to the next line. For example, the slot in the upper-left-hand
/// corner of the main inventory is called `Main0`, and the slot
/// directly below it `Main9`.
///
/// Internally, the `ToPrimitive` implementation is used
/// to convert slot indices to indices into arrays; this
/// is more efficient than utilizing a hash map.
#[derive(Debug, Clone, Copy, ToPrimitive, PartialEq, Eq, Hash)]
pub enum SlotIndex {
    Hotbar0,
    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    Hotbar6,
    Hotbar7,
    Hotbar8,
    ArmorHead,
    ArmorChest,
    ArmorLegs,
    ArmorFeet,
    ArmorElytra,
    Offhand,
    Main0,
    Main1,
    Main2,
    Main3,
    Main4,
    Main5,
    Main6,
    Main7,
    Main8,
    Main9,
    Main10,
    Main11,
    Main12,
    Main13,
    Main14,
    Main15,
    Main16,
    Main17,
    Main18,
    Main19,
    Main20,
    Main21,
    Main22,
    Main23,
    Main24,
    Main25,
    Main26,
    Crafting0,
    Crafting1,
    Crafting2,
    Crafting3,
    Crafting4,
    CraftingTable0,
    CraftingTable1,
    CraftingTable2,
    CraftingTable3,
    CraftingTable4,
    CraftingTable5,
    CraftingTable6,
    CraftingTable7,
    CraftingTable8,
    CraftingTable9,
    Chest0,
    Chest1,
    Chest2,
    Chest3,
    Chest4,
    Chest5,
    Chest6,
    Chest7,
    Chest8,
    Chest9,
    Chest10,
    Chest11,
    Chest12,
    Chest13,
    Chest14,
    Chest15,
    Chest16,
    Chest17,
    Chest18,
    Chest19,
    Chest20,
    Chest21,
    Chest22,
    Chest23,
    Chest24,
    Chest25,
    Chest26,
    Chest27,
    Chest28,
    Chest29,
    Chest30,
    Chest31,
    Chest32,
    Chest33,
    Chest34,
    Chest35,
    Chest36,
    Chest37,
    Chest38,
    Chest39,
    Chest40,
    Chest41,
    Chest42,
    Chest43,
    Chest44,
    Chest45,
    Chest46,
    Chest47,
    Chest48,
    Chest49,
    Chest50,
    Chest51,
    Chest52,
    Chest53,
}

// TODO implement this macro
slot_index_mappings! {
    struct IndexMappings(SlotIndex -> usize);

    InventoryType::Player => {
        Hotbar(i) => 36 + i,
        Main(i) => 9 + i,
        ArmorHead => 5,
        ArmorChest => 6,
        ArmorLegs => 7,
        ArmorFeet => 8,
        ArmorElytra => 45,
        Crafting(i) => i,
        Offhand => 45,
    }
    InventoryType::CraftingTable => {
        CraftingTable(i) => i,
        Main(i) => 10 + i,
        Hotbar(i) => 37 + i,
    }
    InventoryType::SmallChest => {
        Chest(i) => i,
        Main(i) => 27 + i,
        Hotbar(i) => 54 + i,
    }
    InventoryType::LargeChest => {
        Chest(i) => i,
        Main(i) => 54 + i,
        Hotbar(i) => 81 + i,
    }
}

impl SlotIndex {
    /// Returns the raw protocol index of this slot corresponding to the
    /// given inventory type.
    ///
    /// # Panics
    /// Panics if this slot index is not a valid slot
    /// in the given inventory type.
    pub fn raw_index(self, for_ty: InventoryType) -> usize {
        IndexMappings::to_index(self, for_ty)
    }

    pub fn from_raw_index(for_ty: InventoryType, index: usize) -> Option<Self> {
        IndexMappings::from_raw_index(for_ty, index)
    }

    /// Returns the slot index in the hotbar
    /// with the given offset. For example, `hotbar(5)` is equivalent
    /// to `SlotIndex::Hotbar5`.
    ///
    /// # Panics
    /// Panics if `index >= 9`.
    pub fn hotbar(index: usize) -> Self {
        assert!(index < 9);
        match index {
            0 => SlotIndex::Hotbar0,
            1 => SlotIndex::Hotbar1,
            2 => SlotIndex::Hotbar2,
            3 => SlotIndex::Hotbar3,
            4 => SlotIndex::Hotbar4,
            5 => SlotIndex::Hotbar5,
            6 => SlotIndex::Hotbar6,
            7 => SlotIndex::Hotbar7,
            8 => SlotIndex::Hotbar8,
            _ => unreachable!(),
        }
    }

    /// Returns the slot index in the main inventory
    /// with the given offset from the upper-left slot.
    ///
    /// # Panics
    /// Panics if `index >= 27`.
    pub fn main(index: usize) -> Self {
        assert!(index < 27);
        match index {
            0 => SlotIndex::Main0,
            1 => SlotIndex::Main1,
            2 => SlotIndex::Main2,
            3 => SlotIndex::Main3,
            4 => SlotIndex::Main4,
            5 => SlotIndex::Main5,
            6 => SlotIndex::Main6,
            7 => SlotIndex::Main7,
            8 => SlotIndex::Main8,
            9 => SlotIndex::Main9,
            10 => SlotIndex::Main10,
            11 => SlotIndex::Main11,
            12 => SlotIndex::Main12,
            13 => SlotIndex::Main13,
            14 => SlotIndex::Main14,
            15 => SlotIndex::Main15,
            16 => SlotIndex::Main16,
            17 => SlotIndex::Main17,
            18 => SlotIndex::Main18,
            19 => SlotIndex::Main19,
            20 => SlotIndex::Main20,
            21 => SlotIndex::Main21,
            22 => SlotIndex::Main22,
            23 => SlotIndex::Main23,
            24 => SlotIndex::Main24,
            25 => SlotIndex::Main25,
            26 => SlotIndex::Main26,
            _ => unreachable!(),
        }
    }

    /// Returns the slot index with the given offset
    /// into the player crafting slots.
    ///
    /// # Panics
    /// Panics if `index >= 5`.
    pub fn crafting(index: usize) -> Self {
        assert!(index < 5);
        match index {
            0 => SlotIndex::Crafting0,
            1 => SlotIndex::Crafting1,
            2 => SlotIndex::Crafting2,
            3 => SlotIndex::Crafting3,
            4 => SlotIndex::Crafting4,
            _ => unreachable!(),
        }
    }

    /// Returns the slot index with the given
    /// offset into the crafting table slots.
    ///
    /// # Panics
    /// Panics if `index >= 10`.
    pub fn crafting_table(index: usize) -> Self {
        assert!(index < 10);
        match index {
            0 => SlotIndex::CraftingTable0,
            1 => SlotIndex::CraftingTable1,
            2 => SlotIndex::CraftingTable2,
            3 => SlotIndex::CraftingTable3,
            4 => SlotIndex::CraftingTable4,
            5 => SlotIndex::CraftingTable5,
            6 => SlotIndex::CraftingTable6,
            7 => SlotIndex::CraftingTable7,
            8 => SlotIndex::CraftingTable8,
            9 => SlotIndex::CraftingTable9,
            _ => unreachable!(),
        }
    }

    /// Returns the slot index with the given
    /// offset into a chest window.
    ///
    /// # Panics
    /// Panics if `index >= 54`.
    pub fn chest(index: usize) -> Self {
        assert!(index < 54);
        match index {
            0 => SlotIndex::Chest0,
            1 => SlotIndex::Chest1,
            2 => SlotIndex::Chest2,
            3 => SlotIndex::Chest3,
            4 => SlotIndex::Chest4,
            5 => SlotIndex::Chest5,
            6 => SlotIndex::Chest6,
            7 => SlotIndex::Chest7,
            8 => SlotIndex::Chest8,
            9 => SlotIndex::Chest9,
            10 => SlotIndex::Chest10,
            11 => SlotIndex::Chest11,
            12 => SlotIndex::Chest12,
            13 => SlotIndex::Chest13,
            14 => SlotIndex::Chest14,
            15 => SlotIndex::Chest15,
            16 => SlotIndex::Chest16,
            17 => SlotIndex::Chest17,
            18 => SlotIndex::Chest18,
            19 => SlotIndex::Chest19,
            20 => SlotIndex::Chest20,
            21 => SlotIndex::Chest21,
            22 => SlotIndex::Chest22,
            23 => SlotIndex::Chest23,
            24 => SlotIndex::Chest24,
            25 => SlotIndex::Chest25,
            26 => SlotIndex::Chest26,
            27 => SlotIndex::Chest27,
            28 => SlotIndex::Chest28,
            29 => SlotIndex::Chest29,
            30 => SlotIndex::Chest30,
            31 => SlotIndex::Chest31,
            32 => SlotIndex::Chest32,
            33 => SlotIndex::Chest33,
            34 => SlotIndex::Chest34,
            35 => SlotIndex::Chest35,
            36 => SlotIndex::Chest36,
            37 => SlotIndex::Chest37,
            38 => SlotIndex::Chest38,
            39 => SlotIndex::Chest39,
            40 => SlotIndex::Chest40,
            41 => SlotIndex::Chest41,
            42 => SlotIndex::Chest42,
            43 => SlotIndex::Chest43,
            44 => SlotIndex::Chest44,
            45 => SlotIndex::Chest45,
            46 => SlotIndex::Chest46,
            47 => SlotIndex::Chest47,
            48 => SlotIndex::Chest48,
            49 => SlotIndex::Chest49,
            50 => SlotIndex::Chest50,
            51 => SlotIndex::Chest51,
            52 => SlotIndex::Chest52,
            53 => SlotIndex::Chest53,
            _ => unreachable!(),
        }
    }
}

// TODO handle items with different stack sizes, e.g. ender pearls
pub const STACK_MAX_SIZE: u8 = 64;

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

/// The various types of inventories ("windows").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InventoryType {
    /// The normal player inventory.
    Player,
    /// An inventory with a crafting table open.
    CraftingTable,
    /// An inventory with a small chest open.
    SmallChest,
    /// An inventory with a large chest open.
    LargeChest,
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
    ///
    /// Indices into this vector are calculated using
    /// the `ToPrimtive` implementation of `SlotIndex`.
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
        self.items[index] = Some(item);
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
        for slot in COLLECT_SEARCH_ORDER.iter() {
            let slot_item = self.item_at(*slot);
            if slot_item.is_none() {
                self.set_item_at(*slot, item);
                return (smallvec![*slot], 0);
            }

            if let Some(slot_item) = slot_item.cloned() {
                if slot_item.ty == item.ty {
                    let added = min(item.amount, STACK_MAX_SIZE - slot_item.amount);
                    item.amount -= added;

                    self.set_item_at(
                        *slot,
                        ItemStack::new(slot_item.ty, slot_item.amount + added),
                    );
                    affected_slots.push(*slot);

                    if item.amount == 0 {
                        return (affected_slots, 0);
                    }
                }
            }
        }

        (affected_slots, item.amount)
    }

    /// Returns the number of slots in this inventory.
    pub fn slot_count(&self) -> u16 {
        self.items.len() as u16
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

        inv.set_item_at(0, ItemStack::new(Item::Air, 0));

        let item = inv.item_at(0).unwrap();
        assert_eq!(item.ty, Item::Air);
        assert_eq!(item.amount, 0);

        let item = inv.item_at_mut(0).unwrap();
        item.ty = Item::Sponge;
        assert_eq!(inv.item_at(0).unwrap().ty, Item::Sponge);

        inv.clear_item_at(0);
        assert!(inv.item_at(0).is_none());
    }
}
