use std::fs::File;

use uuid::Uuid;

/// Represents the contents of a player data file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "playerGameType")]
    pub gamemode: Option<i32>,
}

pub fn load_player_data(uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    let file = File::open(format!("world/playerdata/{}.dat", uuid))?;
    let data = nbt::from_gzip_reader::<_, PlayerData>(file)?;
    Ok(data)
}
