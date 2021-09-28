//! Loads an `Options` from a TOML config.

use std::{fs, net::IpAddr, path::Path, str::FromStr};

use anyhow::Context;
use base::Gamemode;
use serde::{Deserialize, Deserializer};

use crate::{favicon::Favicon, Options};

const DEFAULT_CONFIG: &str = include_str!("../config.toml");

/// Loads the config, creating a default config if needed.
pub fn load(path: &str) -> anyhow::Result<ConfigContainer> {
    let path = Path::new(path);
    let default_config = DEFAULT_CONFIG;
    let mut is_created = false;

    if !path.exists() {
        log::info!("Creating default config");
        fs::write(path, default_config)?;
        is_created = true;
    }

    let config_string = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_string).context("invalid config.toml file")?;

    Ok(ConfigContainer {
        config,
        was_config_created: is_created,
    })
}

/// A wrapper for the result returned by [load].
pub struct ConfigContainer {
    pub config: Config,
    pub was_config_created: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: Network,
    pub server: ServerConfig,
    pub log: Log,
    pub world: World,
    pub proxy: Proxy,
}

impl Config {
    pub fn to_options(&self) -> Options {
        Options {
            port: self.network.port,
            bind_address: self.network.address.to_string(),
            favicon: Favicon::load_default(),
            motd: self.server.motd.clone(),
            online_mode: if self.proxy.proxy_mode != ProxyMode::None {
                false
            } else {
                self.server.online_mode
            },
            compression_threshold: if self.network.compression_threshold <= 0 {
                None
            } else {
                Some(self.network.compression_threshold as usize)
            },
            view_distance: self.server.view_distance,
            max_players: self.server.max_players,
            default_gamemode: self.server.default_gamemode,
            proxy_mode: match self.proxy.proxy_mode {
                ProxyMode::None => None,
                ProxyMode::Bungee => Some(crate::options::ProxyMode::Bungeecord),
                ProxyMode::Velocity => Some(crate::options::ProxyMode::Velocity),
            },
            velocity_secret: self.proxy.velocity_secret.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub address: IpAddr,
    pub port: u16,
    pub compression_threshold: i32,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub online_mode: bool,
    pub motd: String,
    pub max_players: u32,
    pub default_gamemode: Gamemode,
    pub view_distance: u32,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    #[serde(deserialize_with = "deserialize_log_level")]
    pub level: log::LevelFilter,
}

#[derive(Debug, Deserialize)]
pub struct World {
    pub name: String,
    pub generator: String,
    pub seed: String,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub proxy_mode: ProxyMode,
    pub velocity_secret: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    None,
    Bungee,
    Velocity,
}

fn deserialize_log_level<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<log::LevelFilter, D::Error> {
    let string: String = String::deserialize(deserializer)?;
    let level = log::LevelFilter::from_str(&string).map_err(|_| {
        serde::de::Error::custom(
            "invalid log level: valid options are trace, debug, info, warn, error",
        )
    })?;
    Ok(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let _config: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
    }
}
