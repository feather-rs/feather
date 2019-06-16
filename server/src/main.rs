#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate mockers_derive;

pub mod config;
pub mod initialhandler;
pub mod io;
pub mod player;
pub mod prelude;

use prelude::*;
use core::borrow::BorrowMut;

pub struct Server {
    config: Config,
    player_count: u32,
    players: Vec<PlayerHandle>,
}

fn main() {
    let config = config::load()
        .expect("Failed to load configuration. Please ensure that the file feather.toml exists and is correct.");

    init_log(&config);

    info!("Starting Feather; please wait...");

    let mut server = Server {
        config,
        player_count: 0,
        players: vec![],
    };

    tick(&mut server);
}

fn tick(server: &mut Server) {
    for player in server.players.iter_mut() {
        player.tick(server);
    }
}

fn init_log(config: &Config) {
    let level = match config.log.level.as_str() {
        "trace" => log::Level::Trace,
        "debug" => log::Level::Debug,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        _ => panic!("Unknown log level {}", config.log.level),
    };

    simple_logger::init_with_level(level).unwrap();
}
