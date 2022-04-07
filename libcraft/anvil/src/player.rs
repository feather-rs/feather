use std::collections::HashMap;
use std::convert::TryFrom;
use std::{
    fs,
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use libcraft_items::{Item, ItemStack};

use crate::inventory_consts::*;

use super::entity::{AnimalData, ItemNbt};

/// Represents the contents of a player data file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    // Inherit base entity data
    #[serde(flatten)]
    pub animal: AnimalData,

    #[serde(rename = "playerGameType")]
    pub gamemode: i32,
    #[serde(rename = "previousPlayerGameType")]
    pub previous_gamemode: i32,
    #[serde(rename = "Dimension")]
    pub dimension: String,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<InventorySlot>,
    #[serde(rename = "SelectedItemSlot")]
    pub held_item: i32,
    pub abilities: PlayerAbilities,
}

/// Represents player's abilities (flying, invulnerability, speed, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAbilities {
    #[serde(rename = "walkSpeed")]
    pub walk_speed: f32,
    #[serde(rename = "flySpeed")]
    pub fly_speed: f32,
    #[serde(rename = "mayfly")]
    pub may_fly: bool,
    #[serde(rename = "flying")]
    pub is_flying: bool,
    #[serde(rename = "mayBuild")]
    pub may_build: bool,
    #[serde(rename = "instabuild")]
    pub instabreak: bool,
    pub invulnerable: bool,
}

/// Represents a single inventory slot (including position index).
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventorySlot {
    #[serde(rename = "Count")]
    pub count: i8,
    #[serde(rename = "Slot")]
    #[serde(default)]
    pub slot: i8,
    #[serde(rename = "id")]
    pub item: String,
    #[serde(rename = "tag")]
    pub nbt: Option<ItemNbt>,
}

impl InventorySlot {
    /// Converts an [`ItemStack`] and network protocol index into an [`InventorySlot`].
    pub fn from_network_index(network: usize, stack: &ItemStack) -> Option<Self> {
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
            return None;
        };

        Some(Self::from_inventory_index(slot, stack))
    }

    /// Converts an [`ItemStack`] and inventory position index into an [`InventorySlot`].
    pub fn from_inventory_index(slot: i8, stack: &ItemStack) -> Self {
        let nbt = stack.clone().into();
        let nbt = if nbt == Default::default() {
            None
        } else {
            Some(nbt)
        };
        Self {
            count: stack.count() as i8,
            slot,
            item: stack.item().name().to_owned(),
            nbt,
        }
    }

    /// Converts an NBT inventory index to a network protocol index.
    /// Returns None if the index is invalid.
    pub fn convert_index(&self) -> Option<usize> {
        if 0 <= self.slot && self.slot <= 8 {
            // Hotbar
            Some(SLOT_HOTBAR_OFFSET + (self.slot as usize))
        } else if self.slot == -106 {
            // Offhand
            Some(SLOT_OFFHAND as usize)
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

    pub fn into_nbt_value(self) -> nbt::Value {
        let mut compound = HashMap::new();

        compound.insert(String::from("Count"), nbt::Value::Byte(self.count));
        compound.insert(String::from("id"), nbt::Value::String(self.item));
        compound.insert(String::from("Slot"), nbt::Value::Byte(self.slot));

        let mut tags_compound = HashMap::new();
        if let Some(nbt) = self.nbt {
            if let Some(damage) = nbt.damage {
                tags_compound.insert(String::from("Damage"), nbt::Value::Int(damage));
            }
        }
        compound.insert(String::from("tag"), nbt::Value::Compound(tags_compound));
        nbt::Value::Compound(compound)
    }
}

#[derive(Debug)]
pub struct NoSuckItemError;

impl TryFrom<InventorySlot> for ItemStack {
    type Error = NoSuckItemError;

    fn try_from(slot: InventorySlot) -> Result<Self, Self::Error> {
        ItemStack::try_from(&slot)
    }
}

// Can't do proper Borrow trait impl because of orphan rule
impl TryFrom<&InventorySlot> for ItemStack {
    type Error = NoSuckItemError;

    fn try_from(slot: &InventorySlot) -> Result<Self, Self::Error> {
        Ok(ItemNbt::item_stack(
            &slot.nbt,
            Item::from_name(slot.item.as_str()).ok_or(NoSuckItemError)?,
            slot.count as u8,
        ))
    }
}

pub fn load_player_data(world_dir: &Path, uuid: Uuid) -> Result<PlayerData, anyhow::Error> {
    let file_path = file_path(world_dir, uuid);
    let file = File::open(file_path)?;
    let data = nbt::from_gzip_reader(&file)?;
    Ok(data)
}

pub fn save_player_data(
    world_dir: &Path,
    uuid: Uuid,
    data: &PlayerData,
) -> Result<(), anyhow::Error> {
    fs::create_dir_all(world_dir.join("playerdata"))?;
    let file_path = file_path(world_dir, uuid);
    let mut file = File::create(file_path)?;
    nbt::to_gzip_writer(&mut file, data, None).map_err(anyhow::Error::from)
}

fn file_path(world_dir: &Path, uuid: Uuid) -> PathBuf {
    world_dir.join("playerdata").join(format!("{}.dat", uuid))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::io::Cursor;

    use libcraft_core::Gamemode;
    use num_traits::ToPrimitive;

    use super::*;

    #[test]
    fn test_deserialize_player() {
        let mut cursor = Cursor::new(include_bytes!("player.dat").to_vec());

        let player: PlayerData = nbt::from_gzip_reader(&mut cursor).unwrap();
        assert_eq!(player.gamemode, Gamemode::Creative.to_i32().unwrap());
        assert_eq!(
            player.previous_gamemode,
            Gamemode::Spectator.to_i32().unwrap()
        );
        assert_eq!(player.inventory[0].item, "minecraft:diamond_shovel");
        assert_eq!(player.inventory[0].nbt, Some(ItemNbt { damage: Some(3) }));
    }

    #[test]
    fn test_convert_item() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from(Item::Feather.name()),
            nbt: None,
        };

        let item_stack: ItemStack = slot.try_into().unwrap();
        assert_eq!(item_stack.item(), Item::Feather);
        assert_eq!(item_stack.count(), 1);
    }

    #[test]
    fn test_convert_item_tags() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from(Item::DiamondAxe.name()),
            nbt: Some(ItemNbt { damage: Some(42) }),
        };

        let item_stack: ItemStack = slot.try_into().unwrap();
        assert_eq!(item_stack.item(), Item::DiamondAxe);
        assert_eq!(item_stack.count(), 1);
        assert_eq!(item_stack.damage_taken(), Some(42));
    }

    #[test]
    fn test_convert_item_unknown_type() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from("invalid:identifier"),
            nbt: None,
        };

        let item_stack: Result<ItemStack, NoSuckItemError> = slot.try_into();
        assert!(item_stack.is_err());
    }

    #[test]
    fn test_convert_slot_index() {
        let mut map: HashMap<i8, usize> = HashMap::new();

        // Equipment
        map.insert(103, SLOT_ARMOR_HEAD);
        map.insert(102, SLOT_ARMOR_CHEST);
        map.insert(101, SLOT_ARMOR_LEGS);
        map.insert(100, SLOT_ARMOR_FEET);
        map.insert(-106, SLOT_OFFHAND);

        // Hotbar
        for x in 0..9 {
            map.insert(x, SLOT_HOTBAR_OFFSET + (x as usize));
        }

        // Rest of inventory
        for x in 9..36 {
            map.insert(x, x as usize);
        }

        // Check all valid slots
        for (src, expected) in map {
            let slot = InventorySlot {
                slot: src,
                count: 1,
                item: String::from(Item::Stone.name()),
                nbt: None,
            };
            assert_eq!(slot.convert_index().unwrap(), expected);
            assert_eq!(
                InventorySlot::from_network_index(
                    expected,
                    &ItemStack::new(Item::Stone, 1).unwrap()
                ),
                Some(slot),
            );
        }

        // Check that invalid slots error out
        for invalid_slot in [-1, -2, 104].iter() {
            let slot = InventorySlot {
                slot: *invalid_slot as i8,
                count: 1,
                item: String::from("invalid:identifier"),
                nbt: None,
            };
            assert!(slot.convert_index().is_none());
        }
    }
}
