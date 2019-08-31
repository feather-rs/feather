use std::fs::File;

use crate::Position;
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
        };
        let pos = data.read_position();
        assert!(pos.is_none());
    }
}
