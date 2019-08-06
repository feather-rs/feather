// Specs systems tend to have very long
// tuples as their SystemData, and Clippy
// doesn't seem to like this.
#![allow(clippy::type_complexity)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;

use std::alloc::System;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use specs::{Dispatcher, DispatcherBuilder, Entity, LazyUpdate, World, WorldExt};

use feather_core::network::packet::implementation::DisconnectPlay;
use prelude::*;

use crate::entity::{EntityComponent, EntityDestroyEvent};
use crate::network::send_packet_to_player;
use crate::player::PlayerDisconnectEvent;
use backtrace::Backtrace;
use feather_core::level;
use feather_core::level::LevelData;
use shrev::EventChannel;
use std::fs::File;
use std::io::Write;
use std::process::abort;

#[global_allocator]
static ALLOC: System = System;

pub mod chunk_logic;
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
pub const SERVER_VERSION: &str = "Feather 1.13.2";
pub const TICK_TIME: u64 = 1000 / TPS;

#[derive(Default, Debug)]
pub struct PlayerCount(AtomicUsize);
#[derive(Default, Debug)]
pub struct TickCount(u64);

fn main() {
    let config = Arc::new(load_config());
    init_log(&config);

    info!("Starting Feather; please wait...");

    std::panic::set_hook(Box::new(|info| {
        error!(
            "The server panicked: {:?}",
            info.payload().downcast_ref::<&str>().unwrap()
        );
        let location = info.location().unwrap();
        error!("Source: {}, line {}", location.file(), location.line());
        error!("Backtrace:\n{:?}", Backtrace::new());
        error!("An error occurred, and the server has shut down. Please report this at https://github.com/caelunshun/feather/issues");
    }));

    let player_count = Arc::new(PlayerCount(AtomicUsize::new(0)));

    let io_manager = init_io_manager(Arc::clone(&config), Arc::clone(&player_count));

    info!("Loading level.dat");
    let level = load_level().unwrap_or_else(|e| {
        error!("Error occurred while loading level.dat: {}", e);
        error!("Please ensure that the world directory exists and is not corrupt.");
        abort()
    });

    let (mut world, mut dispatcher) = init_world(config, player_count, io_manager, level);

    info!("Initialized world");

    info!("Server started");
    run_loop(&mut world, &mut dispatcher);
}

/// Loads the configuration file, creating a default
/// one if it does not exist.
fn load_config() -> Config {
    match config::load_from_file("feather.toml") {
        Ok(config) => config,
        Err(e) => match e {
            config::ConfigError::Io(_) => {
                // Use default config
                println!("Config not found - creating it");
                let config = Config::default();
                let mut file = File::create("feather.toml").unwrap();
                file.write_all(toml::to_string(&config).unwrap().as_bytes())
                    .unwrap();
                config
            }
            config::ConfigError::Parse(e) => {
                panic!("Failed to load configuration file: {}", e);
            }
        },
    }
}

/// Loads the level.dat file for the world.
fn load_level() -> Result<LevelData, failure::Error> {
    let file = File::open("world/level.dat")?;
    let data = level::deserialize_level_file(file)?;
    Ok(data)
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
    io::NetworkIoManager::start(
        format!("127.0.0.1:{}", config.server.port).parse().unwrap(),
        config.io.io_worker_threads,
        config,
        player_count,
    )
}

/// Initializes the Specs world.
fn init_world<'a, 'b>(
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    ioman: io::NetworkIoManager,
    level: LevelData,
) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();
    world.insert(config);
    world.insert(player_count);
    world.insert(ioman);
    world.insert(TickCount::default());
    world.insert(level);

    let mut dispatcher = DispatcherBuilder::new()
        .with(chunk_logic::ChunkLoadSystem, "chunk_load", &[])
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
        .with(
            player::DisconnectBroadcastSystem::new(),
            "disconnect_broadcast",
            &[],
        )
        .with(
            entity::EntityDestroyBroadcastSystem::new(),
            "entity_destroy_broadcast",
            &[],
        )
        .with(chunk_logic::ChunkOptimizeSystem, "chunk_optimize", &[])
        .with(chunk_logic::ChunkUnloadSystem::new(), "chunk_unload", &[])
        .with(
            chunk_logic::ChunkHoldRemoveSystem::new(),
            "chunk_hold_remove",
            &[],
        )
        .with(
            entity::EntityDestroySystem::new(),
            "entity_destroy",
            &["chunk_hold_remove"],
        )
        .with(
            player::ChunkCrossSystem::default(),
            "chunk_cross",
            &["player_movement"],
        )
        .with(
            player::ClientChunkUnloadSystem,
            "client_chunk_unload",
            &["chunk_cross"],
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
    let ecomps = world.write_component::<EntityComponent>();
    let ecomp = ecomps.get(player).unwrap();

    info!("Disconnecting player {}: {}", ecomp.display_name, reason);

    // Decrement player count
    let player_count = world.fetch_mut::<Arc<PlayerCount>>();
    player_count.0.fetch_sub(1, Ordering::SeqCst);

    // Trigger disconnect event
    let event = PlayerDisconnectEvent {
        player,
        uuid: ecomp.uuid,
        reason,
    };
    world
        .fetch_mut::<EventChannel<PlayerDisconnectEvent>>()
        .single_write(event);

    // Trigger entity destroy event
    let event = EntityDestroyEvent { entity: player };
    world
        .fetch_mut::<EventChannel<EntityDestroyEvent>>()
        .single_write(event);

    // The entity is removed from the world by `entity::EntityDestroySystem`.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_world() {
        let config = Arc::new(Config::default());
        let player_count = Arc::new(PlayerCount(AtomicUsize::new(0)));
        let ioman = init_io_manager(Arc::clone(&config), Arc::clone(&player_count));
        let level = LevelData::default();

        let (world, mut dispatcher) = init_world(config, player_count, ioman, level);
        dispatcher.dispatch(&world);
    }
}
