use std::fs::File;

use crate::inventory::SlotIndex;
use crate::{ItemStack, Position};
use feather_items::Item;
use std::io::Read;
use uuid::Uuid;

/// Represents the contents of a player data file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "playerGameType")]
    pub gamemode: i32,
    #[serde(rename = "Pos")]
    pub position: Vec<f64>,
    #[serde(rename = "Rotation")]
    pub rotation: Vec<f32>,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<InventorySlot>,
}

/// Represents a single inventory slot (including position index).
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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

impl PlayerData {
    /// Reads the position and rotation fields. If the fields are invalid, None is returned.
    pub fn read_position(self: &PlayerData) -> Option<Position> {
        if self.position.len() == 3 && self.rotation.len() == 2 {
            Some(Position {
                x: self.position[0],
                y: self.position[1],
                z: self.position[2],
                yaw: self.rotation[0],
                pitch: self.rotation[1],
                on_ground: true,
            })
        } else {
            None
        }
    }
}

fn load_from_file<R: Read>(reader: R) -> Result<PlayerData, nbt::Error> {
    nbt::from_gzip_reader::<_, PlayerData>(reader)
}

pub fn load_player_data(uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    let file = File::open(format!("world/playerdata/{}.dat", uuid))?;
    let data = load_from_file(file)?;
    Ok(data)
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
    fn test_read_position() {
        let data = PlayerData {
            gamemode: 0,
            position: vec![1.0, 2.0, 3.0],
            rotation: vec![4.0, 5.0],
            inventory: vec![],
        };
        let pos = data.read_position().unwrap();

        assert!(pos.x - 1.0 < std::f64::EPSILON);
        assert!(pos.y - 2.0 < std::f64::EPSILON);
        assert!(pos.z - 3.0 < std::f64::EPSILON);
        assert!(pos.yaw - 4.0 < std::f32::EPSILON);
        assert!(pos.pitch - 5.0 < std::f32::EPSILON);
        assert!(pos.on_ground);
    }

    #[test]
    fn test_read_position_invalid() {
        let data = PlayerData {
            gamemode: 0,
            position: vec![1.0],
            rotation: vec![2.0, 3.0],
            inventory: vec![],
        };
        let pos = data.read_position();
        assert!(pos.is_none());
    }

    #[test]
    fn test_read_position_invalid_rot() {
        let data = PlayerData {
            gamemode: 0,
            position: vec![1.0, 2.0, 3.0],
            rotation: vec![4.0],
            inventory: vec![],
        };
        let pos = data.read_position();
        assert!(pos.is_none());
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

        // Check all valid slots
        for (src, expected) in map {
            let slot = InventorySlot {
                slot: src,
                count: 1,
                item: String::from("invalid:identifier"),
            };
            assert_eq!(slot.convert_index().unwrap(), expected);
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
