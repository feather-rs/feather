#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::alloc::System;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use specs::{Dispatcher, DispatcherBuilder, Entity, LazyUpdate, World, WorldExt};

use feather_core::network::packet::implementation::DisconnectPlay;
use prelude::*;

use crate::entity::EntityComponent;
use crate::network::send_packet_to_player;

#[global_allocator]
static ALLOC: System = System;

pub mod chunkclient;
pub mod chunkworker;
pub mod config;
pub mod entity;
pub mod io;
pub mod joinhandler;
pub mod network;
pub mod player;
pub mod prelude;
#[cfg(test)]
pub mod testframework;
pub mod worldupdate;

pub const TPS: u64 = 20;
pub const PROTOCOL_VERSION: u32 = 404;
pub const SERVER_VERSION: &'static str = "Feather 1.13.2";
pub const TICK_TIME: u64 = 1000 / TPS;

#[derive(Default, Debug)]
pub struct PlayerCount(AtomicUsize);
#[derive(Default, Debug)]
pub struct TickCount(u64);

fn main() {
    let config = Arc::new(config::load_from_file("feather.toml")
        .expect("Failed to load configuration. Please ensure that the file feather.toml exists and is correct."));

    init_log(&config);

    info!("Starting Feather; please wait...");

    let player_count = Arc::new(PlayerCount(AtomicUsize::new(0)));

    let io_manager = init_io_manager(Arc::clone(&config), Arc::clone(&player_count));

    let (mut world, mut dispatcher) = init_world(config, player_count, io_manager);

    info!("Initialized world");

    info!("Server started");
    run_loop(&mut world, &mut dispatcher);
}

/// Runs the server loop, blocking until the server
/// is shut down.
fn run_loop(world: &mut World, dispatcher: &mut Dispatcher) {
    loop {
        let start_time = current_time_in_millis();

        dispatcher.dispatch(world);
        world.maintain();

        // Increment tick count
        let mut tick_count = world.write_resource::<TickCount>();
        tick_count.0 += 1;

        // Sleep correct amount
        let end_time = current_time_in_millis();
        let elapsed = end_time - start_time;
        if elapsed > TICK_TIME {
            continue; // Behind - start next tick immediately
        }

        // Sleep in 1ms increments until we've slept enough
        let mut sleep_time = (TICK_TIME - elapsed) as i64;
        let mut last_sleep_time = current_time_in_millis();
        while sleep_time > 0 {
            std::thread::sleep(Duration::from_millis(1));
            sleep_time -= (current_time_in_millis() - last_sleep_time) as i64;
            last_sleep_time = current_time_in_millis();
        }
    }
}

/// Starts the IO threads.
fn init_io_manager(config: Arc<Config>, player_count: Arc<PlayerCount>) -> io::NetworkIoManager {
    let ioman = io::NetworkIoManager::start(
        format!("127.0.0.1:{}", config.server.port).parse().unwrap(),
        config.io.io_worker_threads,
        config,
        player_count,
    );
    ioman
}

/// Initializes the Specs world.
fn init_world<'a, 'b>(
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    ioman: io::NetworkIoManager,
) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();
    world.insert(config);
    world.insert(player_count);
    world.insert(ioman);
    world.insert(TickCount::default());

    let mut dispatcher = DispatcherBuilder::new()
        .with(chunkclient::ChunkLoadSystem, "chunk_load", &[])
        .with(network::NetworkSystem, "network", &[])
        .with(
            worldupdate::PlayerDiggingSystem,
            "player_digging",
            &["network"],
        )
        .with(
            player::PlayerMovementSystem,
            "player_movement",
            &["network"],
        )
        .with(
            player::ChunkSendSystem::new(),
            "chunk_send",
            &["chunk_load"],
        )
        .with(
            joinhandler::JoinHandlerSystem,
            "join_handler",
            &["chunk_send"],
        )
        .with(player::PlayerInitSystem::new(), "player_init", &["network"])
        .with(
            player::JoinBroadcastSystem::new(),
            "join_broadcast",
            &["join_handler", "player_init"],
        )
        .build();

    dispatcher.setup(&mut world);

    (world, dispatcher)
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

/// Retrieves the current time in seconds
/// since the UNIX epoch.
pub fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Retrieves the current time in milleseconds
/// since the UNIX epoch.
pub fn current_time_in_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Disconnects the given player, removing them from the world.
/// This operation is performed lazily.
pub fn disconnect_player(player: Entity, reason: String, lazy: &LazyUpdate) {
    lazy.exec_mut(move |world| {
        let json = json!({
            "text": reason,
        });

        let packet = DisconnectPlay::new(json.to_string());
        send_packet_to_player(world.read_component().get(player).unwrap(), packet);

        disconnect_player_without_packet(player, world, reason);
    })
}

/// Disconnects a player without sending Disconnect Play.
/// This should be used when the client disconnects.
pub fn disconnect_player_without_packet(player: Entity, world: &mut World, reason: String) {
    if let Some(ecomp) = world.read_component::<EntityComponent>().get(player) {
        info!("Disconnected player {}: {}", ecomp.display_name, reason);
    }

    world.delete_entity(player).unwrap();

    // Decrement player count
    let player_count = world.fetch_mut::<Arc<PlayerCount>>();
    player_count.0.fetch_sub(1, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_world() {
        let config = Arc::new(Config::default());
        let player_count = Arc::new(PlayerCount(AtomicUsize::new(0)));
        let ioman = init_io_manager(Arc::clone(&config), Arc::clone(&player_count));

        let (world, mut dispatcher) = init_world(config, player_count, ioman);
        dispatcher.dispatch(&world);
    }
}
