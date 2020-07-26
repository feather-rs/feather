use crate::entity::AnimalData;
use feather_inventory::player_constants::{
    HOTBAR_SIZE, INVENTORY_SIZE, SLOT_ARMOR_MAX, SLOT_ARMOR_MIN, SLOT_HOTBAR_OFFSET,
    SLOT_INVENTORY_OFFSET, SLOT_OFFHAND,
};
use feather_items::{Item, ItemStack};
use nbt::Value;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::io::AsyncWriteExt;
use tokio::prelude::{AsyncRead, AsyncWrite};
use uuid::Uuid;

/// Represents the contents of a player data file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    // Inherit base entity data
    #[serde(flatten)]
    pub animal: AnimalData,

    #[serde(rename = "playerGameType")]
    pub gamemode: i32,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<InventorySlot>,
    #[serde(rename = "SelectedItemSlot")]
    pub held_item: i32,
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
    #[serde(rename = "tag")]
    pub tags: Option<ItemTags>,
}

/// Represents NBT tags on an item.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemTags {
    #[serde(rename = "Damage")]
    pub damage: Option<i16>,
    // TODO enchantments, display name, ...
}

impl From<ItemStack> for ItemTags {
    fn from(stack: ItemStack) -> Self {
        Self {
            damage: stack.damage,
        }
    }
}

impl InventorySlot {
    /// Converts a slot to an ItemStack.
    pub fn to_stack(&self) -> ItemStack {
        ItemStack {
            ty: Item::from_identifier(self.item.as_str()).unwrap_or(Item::Air),
            amount: self.count as u8,
            damage: self.tags.as_ref().map(|t| t.damage).flatten(),
        }
    }

    /// Converts a network protocol index, item, and count
    /// to an `InventorySlot`.
    pub fn from_network_index(network: usize, stack: ItemStack) -> Option<Self> {
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

        let tags = stack.into();
        let tags = if tags == Default::default() {
            None
        } else {
            Some(tags)
        };
        Some(Self {
            count: stack.amount as i8,
            slot,
            item: stack.ty.identifier().to_string(),
            tags,
        })
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

    pub fn into_nbt_value(self) -> Value {
        let mut compound = HashMap::new();

        compound.insert(String::from("Count"), Value::Byte(self.count));
        compound.insert(String::from("id"), Value::String(self.item));
        compound.insert(String::from("Slot"), Value::Byte(self.slot));

        let mut tags_compound = HashMap::new();
        if let Some(tags) = self.tags {
            if let Some(damage) = tags.damage {
                tags_compound.insert(String::from("Damage"), Value::Short(damage));
            }
        }
        compound.insert(String::from("tag"), Value::Compound(tags_compound));
        Value::Compound(compound)
    }
}

async fn load_from_file<R: AsyncRead + Unpin>(mut reader: R) -> Result<PlayerData, nbt::Error> {
    let mut buf = vec![];
    tokio::io::copy(&mut reader, &mut buf).await?;
    nbt::from_gzip_reader(buf.as_slice())
}

pub async fn load_player_data(world_dir: &Path, uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    let file_path = file_path(world_dir, uuid);
    let file = tokio::fs::File::open(file_path).await?;
    let data = load_from_file(file).await?;
    Ok(data)
}

async fn save_to_file<W: AsyncWrite + Unpin>(
    mut writer: W,
    data: &PlayerData,
) -> Result<(), anyhow::Error> {
    let mut buf = vec![];
    nbt::to_gzip_writer(&mut buf, data, None)?;
    writer.write_all(&buf).await?;
    Ok(())
}

pub async fn save_player_data(
    world_dir: &Path,
    uuid: Uuid,
    data: &PlayerData,
) -> Result<(), anyhow::Error> {
    tokio::fs::create_dir_all(world_dir.join("playerdata")).await?;
    let file_path = file_path(world_dir, uuid);
    let file = tokio::fs::File::create(file_path).await?;
    save_to_file(file, data).await
}

fn file_path(world_dir: &Path, uuid: Uuid) -> PathBuf {
    world_dir.join("playerdata").join(format!("{}.dat", uuid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_inventory::player_constants::{
        SLOT_ARMOR_CHEST, SLOT_ARMOR_FEET, SLOT_ARMOR_HEAD, SLOT_ARMOR_LEGS,
    };
    use feather_util::Gamemode;
    use std::collections::HashMap;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_deserialize_player() {
        let cursor = Cursor::new(include_bytes!("player.dat").to_vec());

        let player = load_from_file(cursor).await.unwrap();
        assert_eq!(player.gamemode, i32::from(Gamemode::Creative.id()));
        assert_eq!(player.inventory[0].item, "minecraft:diamond_shovel");
        assert_eq!(player.inventory[0].tags, Some(ItemTags { damage: Some(3) }));
    }

    #[test]
    fn test_convert_item() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from(Item::Feather.identifier()),
            tags: None,
        };

        let item_stack = slot.to_stack();
        assert_eq!(item_stack.ty, Item::Feather);
        assert_eq!(item_stack.amount, 1);
    }

    #[test]
    fn test_convert_item_tags() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from(Item::DiamondAxe.identifier()),
            tags: Some(ItemTags { damage: Some(42) }),
        };

        let item_stack = slot.to_stack();
        assert_eq!(item_stack.ty, Item::DiamondAxe);
        assert_eq!(item_stack.amount, 1);
        assert_eq!(item_stack.damage, Some(42));
    }

    #[test]
    fn test_convert_item_unknown_type() {
        let slot = InventorySlot {
            count: 1,
            slot: 2,
            item: String::from("invalid:identifier"),
            tags: None,
        };

        let item_stack = slot.to_stack();
        assert_eq!(item_stack.ty, Item::Air);
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
                item: String::from(Item::Stone.identifier()),
                tags: None,
            };
            assert_eq!(slot.convert_index().unwrap(), expected);
            assert_eq!(
                InventorySlot::from_network_index(expected, ItemStack::new(Item::Stone, 1)),
                Some(slot),
            );
        }

        // Check that invalid slots error out
        for invalid_slot in [-1, -2, 104].iter() {
            let slot = InventorySlot {
                slot: *invalid_slot as i8,
                count: 1,
                item: String::from("invalid:identifier"),
                tags: None,
            };
            assert!(slot.convert_index().is_none());
        }
    }
}
