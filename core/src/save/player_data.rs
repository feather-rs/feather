use std::fs::File;

use crate::entity::BaseEntityData;
use crate::inventory::{
    SlotIndex, HOTBAR_SIZE, INVENTORY_SIZE, SLOT_ARMOR_MAX, SLOT_ARMOR_MIN, SLOT_HOTBAR_OFFSET,
    SLOT_INVENTORY_OFFSET, SLOT_OFFHAND,
};
use crate::ItemStack;
use feather_items::Item;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Represents the contents of a player data file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    // Inherit base entity data
    #[serde(flatten)]
    pub entity: BaseEntityData,

    #[serde(rename = "playerGameType")]
    pub gamemode: i32,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<InventorySlot>,
}

/// Represents a single inventory slot (including position index).
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventorySlot {
    #[serde(rename = "Count")]
    pub count: i8,
    #[serde(rename = "Slot")]
    pub slot: i8,
    #[serde(rename = "id")]
    pub item: String,
}

impl InventorySlot {
    /// Converts a slot to an ItemStack.
    pub fn to_stack(&self) -> ItemStack {
        ItemStack {
            ty: Item::from_identifier(self.item.as_str()).unwrap_or(Item::Air),
            amount: self.count as u8,
        }
    }

    /// Converts a network protocol index, item, and count
    /// to an `InventorySlot`.
    pub fn from_network_index(network: SlotIndex, stack: ItemStack) -> Self {
        let slot = if SLOT_HOTBAR_OFFSET <= network && network < SLOT_HOTBAR_OFFSET + HOTBAR_SIZE {
            // Hotbar
            (network - SLOT_HOTBAR_OFFSET) as i8
        } else if network == SLOT_OFFHAND {
            -106
        } else if SLOT_ARMOR_MIN <= network && network <= SLOT_ARMOR_MAX {
            ((SLOT_ARMOR_MAX - network) + 100) as i8
        } else if SLOT_INVENTORY_OFFSET <= network
            && network < SLOT_INVENTORY_OFFSET + INVENTORY_SIZE
        {
            network as i8
        } else {
            panic!("Invalid slot index {} on server", network);
        };

        Self {
            count: stack.amount as i8,
            slot,
            item: stack.ty.identifier().to_string(),
        }
    }

    /// Converts an NBT inventory index to a network protocol index.
    /// Returns None if the index is invalid.
    pub fn convert_index(&self) -> Option<SlotIndex> {
        if 0 <= self.slot && self.slot <= 8 {
            // Hotbar
            Some(crate::inventory::SLOT_HOTBAR_OFFSET + (self.slot as usize))
        } else if self.slot == -106 {
            // Offhand
            Some(crate::inventory::SLOT_OFFHAND as usize)
        } else if 100 <= self.slot && self.slot <= 103 {
            // Equipment
            Some((108 - self.slot) as usize)
        } else if 9 <= self.slot && self.slot <= 35 {
            // Rest of inventory
            Some(self.slot as usize)
        } else {
            // Unknown index
            None
        }
    }
}

fn load_from_file<R: Read>(reader: R) -> Result<PlayerData, nbt::Error> {
    nbt::from_gzip_reader::<_, PlayerData>(reader)
}

pub fn load_player_data(world_dir: &Path, uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    let file_path = file_path(world_dir, uuid);
    let file = File::open(file_path)?;
    let data = load_from_file(file)?;
    Ok(data)
}

fn save_to_file<W: Write>(mut writer: W, data: PlayerData) -> Result<(), nbt::Error> {
    nbt::to_gzip_writer(&mut writer, &data, None)
}

pub fn save_player_data(world_dir: &Path, uuid: Uuid, data: PlayerData) -> Result<(), nbt::Error> {
    fs::create_dir_all(world_dir.join("playerdata"))?;
    let file_path = file_path(world_dir, uuid);
    let file = File::create(file_path)?;
    save_to_file(file, data)
}

fn file_path(world_dir: &Path, uuid: Uuid) -> PathBuf {
    world_dir.join("playerdata").join(format!("{}.dat", uuid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Gamemode;
    use hashbrown::HashMap;
    use std::io::Cursor;

    #[test]
    fn test_deserialize_player() {
        let cursor = Cursor::new(include_bytes!("player.dat").to_vec());

        let player = load_from_file(cursor).unwrap();
        assert_eq!(player.gamemode, i32::from(Gamemode::Creative.get_id()));
    }

    #[test]
    fn test_convert_item() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from(Item::Feather.identifier()),
        };

        let item_stack = slot.to_stack();
        assert_eq!(item_stack.ty, Item::Feather);
        assert_eq!(item_stack.amount, 1);
    }

    #[test]
    fn test_convert_item_unknown_type() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from("invalid:identifier"),
        };

        let item_stack = slot.to_stack();
        assert_eq!(item_stack.ty, Item::Air);
    }

    #[test]
    fn test_convert_slot_index() {
        let mut map: HashMap<i8, usize> = HashMap::new();

        // Equipment
        map.insert(103, crate::inventory::SLOT_ARMOR_HEAD);
        map.insert(102, crate::inventory::SLOT_ARMOR_CHEST);
        map.insert(101, crate::inventory::SLOT_ARMOR_LEGS);
        map.insert(100, crate::inventory::SLOT_ARMOR_FEET);
        map.insert(-106, crate::inventory::SLOT_OFFHAND);

        // Hotbar
        for x in 0..9 {
            map.insert(x, crate::inventory::SLOT_HOTBAR_OFFSET + (x as usize));
        }

        // Rest of inventory
        for x in 9..36 {
            map.insert(x, x as usize);
        }

        dbg!(map.clone());

        // Check all valid slots
        for (src, expected) in map {
            let slot = InventorySlot {
                slot: src,
                count: 1,
                item: String::from(Item::Stone.identifier()),
            };
            assert_eq!(slot.convert_index().unwrap(), expected);
            assert_eq!(
                InventorySlot::from_network_index(expected, ItemStack::new(Item::Stone, 1)),
                slot
            );
        }

        // Check that invalid slots error out
        for invalid_slot in [-1, -2, 104].iter() {
            let slot = InventorySlot {
                slot: *invalid_slot as i8,
                count: 1,
                item: String::from("invalid:identifier"),
            };
            assert!(slot.convert_index().is_none());
        }
    }
}
