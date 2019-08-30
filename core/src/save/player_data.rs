use std::fs::File;

use std::io::Read;
use uuid::Uuid;

/// Represents the contents of a player data file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "playerGameType")]
    pub gamemode: Option<i32>,
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
        assert_eq!(
            player.gamemode.unwrap(),
            i32::from(Gamemode::Creative.get_id())
        );
    }
}
