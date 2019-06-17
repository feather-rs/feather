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
use std::time::Duration;

pub struct Server {
    config: Config,
    player_count: u32,
    players: Vec<RefCell<PlayerHandle>>,
    io_manager: io::NetworkIoManager,
    rsa_key: openssl::rsa::Rsa<openssl::pkey::Private>,
}

fn main() {
    let config = config::load()
        .expect("Failed to load configuration. Please ensure that the file feather.toml exists and is correct.");

    init_log(&config);

    info!("Starting Feather; please wait...");

    let io_manager = io::NetworkIoManager::start(
        format!("127.0.0.1:{}", config.server.port).parse().unwrap(),
        config.io.io_worker_threads,
    );

    let mut server = Server {
        config,
        player_count: 0,
        players: vec![],
        io_manager,
        rsa_key: openssl::rsa::Rsa::generate(1024).unwrap(),
    };

    loop {
        tick(&mut server);

        while let Ok(msg) = server.io_manager.receiver.try_recv() {
            match msg {
                io::ServerToListenerMessage::NewClient(info) => {
                    trace!("Server registered connection");
                    let new_player = PlayerHandle::accept_player_connection(
                        info.sender,
                        info.receiver,
                        server.config.server.motd.clone(),
                        server.player_count,
                        server.config.server.max_players,
                        server.rsa_key.clone(),
                    );
                    server.players.push(RefCell::new(new_player));
                }
                _ => unreachable!(),
            }
        }

        std::thread::sleep(Duration::from_millis(50)); // TODO proper game loop
    }
}

fn tick(server: &mut Server) {
    for player in server.players.iter() {
        player.borrow_mut().tick(server);
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
