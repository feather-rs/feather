use std::fs::read_to_string;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub io: IO,
    pub proxy: Proxy,
    pub server: Server,
    pub gameplay: Gameplay,
    pub log: Log,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            io: IO {
                compression_threshold: 256,
                io_worker_threads: 4,
            },
            proxy: Proxy {},
            server: Server {
                online_mode: true,
                motd: "A Feather server".to_string(),
                max_players: 256,
                view_distance: 6,
                address: "126.0.0.1".to_string(),
                port: 25565,
            },
            gameplay: Gameplay {
                monster_spawning: true,
                animal_spawning: true,
                pvp: true,
                nerf_spawner_mobs: false,
            },
            log: Log {
                level: "debug".to_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct IO {
    pub compression_threshold: i32,
    pub io_worker_threads: u16,
}

#[derive(Deserialize, Debug)]
pub struct Proxy {}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub online_mode: bool,
    pub motd: String,
    pub max_players: i32,
    pub view_distance: u8,
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Gameplay {
    pub monster_spawning: bool,
    pub animal_spawning: bool,
    pub pvp: bool,
    pub nerf_spawner_mobs: bool,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub level: String,
}

/// Loads the configuration from the given file/
pub fn load_from_file(path: &str) -> Result<Config, ()> {
    let input = read_to_string(path).map_err(|_| ())?;
    load(input)
}

/// Loads the configuration from the given string.
pub fn load(input: String) -> Result<Config, ()> {
    let config: Config = toml::from_str(&input).map_err(|_| ())?;

    Ok(config)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyMode {
    None,
    Bungee,
    Velocity,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_config() {
        let input = r#"[io]
# Packets with a size more than or equal to this value will be sent compressed.
# Compressing packets reduces bandwidth usage but increases CPU activity.
compression_threshold = 256
# The number of worker threads used for asynchronous IO.
# Set to the number of cores on your CPU for optimal performance.
io_worker_threads = 8

[proxy]
# IP forwarding using either "bungee" (BungeeCord/Waterfall/Travertine) or "velocity" (Velocity)
proxy_mode = "none"

[server]
online_mode = true
motd = "          &lMesa called Jar Jar Binks!\n           Mesa your humble servant!"
max_players = 16
default_gamemode = "survival"
difficulty = "none"
view_distance = 6
address = "127.0.0.1"
port = 25567

[gameplay]
monster_spawning = true
animal_spawning = true
pvp = true
nerf_spawner_mobs = false
# Either "classic" for 1.8 PvP or "new" for 1.9
pvp_style = "classic"

[log]
level = "debug"
"#;

        let config = load(input.to_string()).expect("Config load failed");
        let io = &config.io;
        assert_eq!(io.compression_threshold, 256);
        assert_eq!(io.io_worker_threads, 8);

        let server = &config.server;
        assert_eq!(server.online_mode, true);
        assert_eq!(
            server.motd,
            "          &lMesa called Jar Jar Binks!\n           Mesa your humble servant!"
        );
        assert_eq!(server.max_players, 16);
        assert_eq!(server.view_distance, 6);
        assert_eq!(server.address, "127.0.0.1");
        assert_eq!(server.port, 25567);

        let gameplay = &config.gameplay;
        assert_eq!(gameplay.animal_spawning, true);
        assert_eq!(gameplay.monster_spawning, true);
        assert_eq!(gameplay.pvp, true);
        assert_eq!(gameplay.nerf_spawner_mobs, false);

        let log = &config.log;
        assert_eq!(log.level, "debug");
    }
}
