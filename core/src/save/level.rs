//! Implements level.dat file loading.

use std::io::Read;

/// Root level tag
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Root {
    #[serde(rename = "Data")]
    data: LevelData,
}

/// Represents the contents of a level file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    #[serde(rename = "allowCommands")]
    pub allow_commands: bool,
    #[serde(default)]
    #[serde(rename = "BorderCenterX")]
    pub border_center_x: f64,
    #[serde(default)]
    #[serde(rename = "BorderCenterZ")]
    pub border_center_z: f64,
    #[serde(default)]
    #[serde(rename = "BorderDamagePerBlock")]
    pub border_damage_per_block: f64,
    #[serde(rename = "BorderSafeZone")]
    pub border_safe_zone: f64,
    #[serde(rename = "BorderSize")]
    pub border_size: f64,

    #[serde(rename = "clearWeatherTime")]
    pub clear_weather_time: i32,
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "DayTime")]
    pub day_time: i64,
    #[serde(rename = "Difficulty")]
    pub difficulty: i8,
    #[serde(rename = "DifficultyLocked")]
    pub difficulty_locked: i8,
    #[serde(rename = "GameType")]
    pub game_type: i32,

    pub hardcore: bool,

    pub initialized: bool,
    #[serde(rename = "LastPlayed")]
    pub last_played: i64,
    pub raining: bool,
    #[serde(rename = "rainTime")]
    pub rain_time: i32,
    #[serde(rename = "RandomSeed")]
    pub random_seed: i64,

    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,
    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,
    #[serde(rename = "SpawnZ")]
    pub spawn_z: i32,
    pub thundering: bool,
    #[serde(rename = "thunderTime")]
    pub thunder_time: i32,
    #[serde(rename = "Time")]
    pub time: i64,

    #[serde(rename = "Version")]
    pub version: LevelVersion,
}

/// Represents level version data.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LevelVersion {
    #[serde(rename = "Id")]
    id: i32,
    #[serde(rename = "Name")]
    name: String,
}

/// Deserializes a level.dat file from the given reader.
pub fn deserialize_level_file<R: Read>(reader: R) -> Result<LevelData, nbt::Error> {
    match nbt::from_gzip_reader::<_, Root>(reader) {
        Ok(root) => Ok(root.data),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_deserialize_level_file() {
        let cursor = Cursor::new(include_bytes!("level.dat").to_vec());

        let level = deserialize_level_file(cursor).unwrap();

        assert!(!level.allow_commands);
        assert_eq!(level.clear_weather_time, 0);
        assert_eq!(level.data_version, 1631);
        assert_eq!(level.day_time, 302_885);
        assert_eq!(level.difficulty, 1);
        assert_eq!(level.difficulty_locked, 0);
        assert_eq!(level.game_type, 0);
        assert!(!level.hardcore);
        assert!(level.initialized);
        assert_eq!(level.last_played, 1_560_968_104_655);
        assert_eq!(level.raining, false);
        assert_eq!(level.rain_time, 61872);
        assert_eq!(level.spawn_x, 0);
        assert_eq!(level.spawn_y, 70);
        assert_eq!(level.spawn_z, 0);
        assert!(level.thundering);
        assert_eq!(level.thunder_time, 5252);
    }
}
