use std::fs::read_to_string;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub io: IO,
    pub proxy: Proxy,
    pub server: Server,
    pub gameplay: Gameplay,
    pub log: Log,
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

pub fn load() -> Result<Config, ()> {
    let config_content = read_to_string("feather.toml").expect("Could not load configuration file");

    let config: Config = toml::from_str(&config_content).expect("Invalid configuration file");

    Ok(config)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyMode {
    None,
    Bungee,
    Velocity,
}
