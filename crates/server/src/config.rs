use base::Gamemode;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

/// The server configuration file.
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

impl Config {
    /// Loads a config from the given string.
    pub fn load(s: &str) -> anyhow::Result<Config> {
        toml::from_str(s).map_err(Into::into)
    }

    /// Loads a config from the given file.
    pub fn load_from_file(f: &mut File) -> anyhow::Result<Config> {
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Self::load(&s)
    }

    /// Saves the configuration, writing its contents to the given string.
    pub fn save(&self) -> String {
        toml::to_string_pretty(self).expect("failed to serialize config")
    }

    /// Saves the configuration to the given file.
    pub fn save_to_file(&self, f: &mut File) -> anyhow::Result<()> {
        let string = self.save();

        f.write_all(string.as_bytes()).map_err(Into::into)
    }
}

pub const DEFAULT_CONFIG_STR: &str = include_str!("../config.toml");

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
    pub proxy_mode: ProxyMode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub online_mode: bool,
    pub motd: String,
    pub max_players: i32,
    pub view_distance: u8,
    pub address: String,
    pub port: u16,
    pub default_gamemode: Gamemode,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ProxyMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "bungee")]
    BungeeCord,
    #[serde(alias = "velocity")]
    Velocity,
}
