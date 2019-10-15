use failure::_core::time::Duration;
use std::fs::read_to_string;

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "Badly formatted configuration file: {}", _0)]
    Parse(#[fail(cause)] toml::de::Error),
    #[fail(display = "Failed to read configuration file: {}", _0)]
    Io(#[fail(cause)] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub io: IO,
    pub proxy: Proxy,
    pub server: Server,
    pub gameplay: Gameplay,
    pub log: Log,
    pub resource_pack: ResourcePack,
    pub world: World,
}

pub const DEFAULT_CONFIG_STR: &str = include_str!("../config/feather.toml");

impl Default for Config {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG_STR).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IO {
    pub compression_threshold: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proxy {
    pub proxy_mode: ProxyMode
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub online_mode: bool,
    pub motd: String,
    pub max_players: i32,
    pub view_distance: u8,
    pub address: String,
    pub port: u16,
    pub default_gamemode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gameplay {
    pub monster_spawning: bool,
    pub animal_spawning: bool,
    pub pvp: bool,
    pub nerf_spawner_mobs: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourcePack {
    pub url: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub name: String,
    pub generator: String,
    pub seed: String,
    #[serde(with = "humantime_serde")]
    pub save_interval: Duration,
}

/// Loads the configuration from the given file/
pub fn load_from_file(path: &str) -> Result<Config, ConfigError> {
    let input = read_to_string(path).map_err(ConfigError::Io)?;
    load(input)
}

/// Loads the configuration from the given string.
pub fn load(input: String) -> Result<Config, ConfigError> {
    let config: Config = toml::from_str(&input).map_err(ConfigError::Parse)?;

    Ok(config)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ProxyMode {
    #[serde(alias = "none")]
    None,
    #[serde(alias = "bungeecord")]
    BungeeCord,
    #[serde(alias = "velocity")]
    Velocity,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_config() {
        let input = include_str!("../config/feather.toml");

        let config = load(input.to_string()).expect("Config load failed");
        let io = &config.io;
        assert_eq!(io.compression_threshold, 256);

        let server = &config.server;
        assert_eq!(server.online_mode, true);
        assert_eq!(server.motd, "A Feather server");
        assert_eq!(server.max_players, 16);
        assert_eq!(server.default_gamemode, "creative");
        assert_eq!(server.view_distance, 6);
        assert_eq!(server.address, "0.0.0.0");
        assert_eq!(server.port, 25565);

        let gameplay = &config.gameplay;
        assert_eq!(gameplay.animal_spawning, true);
        assert_eq!(gameplay.monster_spawning, true);
        assert_eq!(gameplay.pvp, true);
        assert_eq!(gameplay.nerf_spawner_mobs, false);

        let log = &config.log;
        assert_eq!(log.level, "debug");

        let resource_pack = &config.resource_pack;
        assert_eq!(resource_pack.url, "");
        assert_eq!(resource_pack.hash, "");

        let world = &config.world;
        assert_eq!(world.name, "world");
        assert_eq!(world.generator, "default");
        assert_eq!(world.seed, "");
        assert_eq!(world.save_interval.as_millis(), 1000 * 60);

        let proxy = &config.proxy;
        assert_eq!(proxy.proxy_mode, ProxyMode::None);
    }
}
