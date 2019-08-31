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
}
