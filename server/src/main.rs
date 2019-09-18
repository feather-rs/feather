// Specs systems tend to have very long
// tuples as their SystemData, and Clippy
// doesn't seem to like this.
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate smallvec;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate feather_codegen;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate feather_core;

extern crate nalgebra_glm as glm;

use std::alloc::System;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use specs::{Builder, Dispatcher, DispatcherBuilder, Entity, LazyUpdate, World, WorldExt};

use feather_core::network::packet::implementation::DisconnectPlay;
use prelude::*;

use crate::chunk_logic::{ChunkHolders, ChunkWorkerHandle};
use crate::entity::{EntityDestroyEvent, NamedComponent};
use crate::network::send_packet_to_player;
use crate::player::PlayerDisconnectEvent;
use crate::systems::{BROADCASTER, ITEM_SPAWN, JOIN_HANDLER, NETWORK, PLAYER_INIT, SPAWNER};
use crate::util::Util;
use crate::worldgen::{
    ComposableGenerator, EmptyWorldGenerator, SuperflatWorldGenerator, WorldGenerator,
};
use backtrace::Backtrace;
use feather_core::level;
use feather_core::level::{deserialize_level_file, save_level_file, LevelData, LevelGeneratorType};
use rand::RngCore;
use shrev::EventChannel;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

#[global_allocator]
static ALLOC: System = System;

#[macro_use]
pub mod util;
pub mod blocks;
pub mod chunk_logic;
pub mod chunkworker;
pub mod config;
pub mod entity;
pub mod io;
pub mod joinhandler;
pub mod network;
pub mod physics;
pub mod player;
pub mod prelude;
pub mod systems;
#[cfg(test)]
pub mod testframework;
pub mod time;
pub mod worldgen;

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
        error!("The server panicked.");
        let location = info.location().unwrap();
        error!("Source: {}, line {}", location.file(), location.line());
        error!("Backtrace:\n{:?}", Backtrace::new());
        error!("An error occurred, and the server has shut down. Please report this at https://github.com/caelunshun/feather/issues");
    }));

    let server_icon = Arc::new(load_server_icon());

    let player_count = Arc::new(PlayerCount(AtomicUsize::new(0)));

    let io_manager = init_io_manager(
        Arc::clone(&config),
        Arc::clone(&player_count),
        Arc::clone(&server_icon),
    );

    let world_name = &config.world.name;
    let world_dir = Path::new(world_name.as_str());
    let level_file = &world_dir.join("level.dat");
    if !world_dir.is_dir() {
        // Create directory
        std::fs::create_dir(world_dir).unwrap();

        let level = create_level(&config);
        let root = level::Root { data: level };
        let mut level_file = File::create(level_file).unwrap();
        save_level_file(&root, &mut level_file).unwrap();
    }

    info!("Loading {}", level_file.to_str().unwrap());
    let level = load_level(level_file).unwrap_or_else(|e| {
        error!("Error occurred while loading level.dat: {}", e);
        error!("Please ensure that the world directory exists and is not corrupt.");
        exit(1)
    });

    let (mut world, mut dispatcher) = init_world(config, player_count, io_manager, level);

    info!("Initialized world");

    info!("Generating RSA keypair");
    io::init();

    info!("Queuing spawn chunks for loading");
    load_spawn_chunks(&mut world);

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
                file.write_all(config::DEFAULT_CONFIG_STR.as_bytes())
                    .unwrap();
                config
            }
            config::ConfigError::Parse(e) => {
                panic!("Failed to load configuration file: {}", e);
            }
        },
    }
}

fn create_level(config: &Arc<Config>) -> LevelData {
    let seed = get_seed(config);
    let world_name = &config.world.name;
    info!("Creating world '{}' with seed {}", world_name, seed);

    // TODO: Generate spawn position properly
    LevelData {
        allow_commands: false,
        border_center_x: 0.0,
        border_center_z: 0.0,
        border_damage_per_block: 0.0,
        border_safe_zone: 0.0,
        border_size: 0.0,
        clear_weather_time: 0,
        data_version: 0,
        day_time: 0,
        difficulty: 0,
        difficulty_locked: 0,
        game_type: 0,
        hardcore: false,
        initialized: false,
        last_played: 0,
        raining: false,
        rain_time: 0,
        seed,
        spawn_x: 0,
        spawn_y: 100,
        spawn_z: 0,
        thundering: false,
        thunder_time: 0,
        time: 0,
        version: Default::default(),
        generator_name: config.world.generator.to_string(),
        generator_options: None,
    }
}

fn get_seed(config: &Arc<Config>) -> i64 {
    let seed_raw = &config.world.seed;
    // Empty seed: random
    // Seed is valid i64: parse
    // Seed is something else: hash
    if seed_raw.is_empty() {
        rand::thread_rng().next_u64() as i64
    } else {
        match seed_raw.parse::<i64>() {
            Ok(seed_int) => seed_int,
            Err(_) => hash_seed(seed_raw.as_str()),
        }
    }
}

fn hash_seed(seed_raw: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    seed_raw.hash(&mut hasher);
    hasher.finish() as i64
}

/// Loads the level.dat file for the world.
fn load_level(path: &Path) -> Result<LevelData, failure::Error> {
    let file = File::open(path)?;
    let data = deserialize_level_file(file)?;
    Ok(data)
}

/// Loads the chunks around the spawn area and creates
/// a chunk hold on those chunks to prevent them from
/// being unloaded.
///
/// Note that these chunks are loaded asynchronously,
/// and this function will return before loading is complete.
fn load_spawn_chunks(world: &mut World) {
    let view_distance = i32::from(world.fetch::<Arc<Config>>().server.view_distance);

    // Create an entity for the server and
    // add chunk holders using it.
    let server_entity = world.create_entity().build();

    let mut chunk_holders = world.fetch_mut::<ChunkHolders>();
    let chunk_worker_handle = world.fetch::<ChunkWorkerHandle>();

    let level = world.fetch::<LevelData>();
    let offset_x = level.spawn_x / 16;
    let offset_z = level.spawn_z / 16;
    for x in -view_distance..=view_distance {
        for z in -view_distance..=view_distance {
            let chunk = ChunkPosition::new(x + offset_x, z + offset_z);

            chunk_logic::load_chunk(&chunk_worker_handle, chunk);
            chunk_holders.insert_holder(chunk, server_entity);
        }
    }
}

/// Runs the server loop, blocking until the server
/// is shut down.
fn run_loop(world: &mut World, dispatcher: &mut Dispatcher) {
    loop {
        let start_time = current_time_in_millis();

        dispatcher.dispatch(&world);
        world.maintain();

        world.fetch_mut::<Util>().reset();

        // Increment tick count
        let mut tick_count = world.write_resource::<TickCount>();
        tick_count.0 += 1;

        // Sleep correct amount
        let end_time = current_time_in_millis();
        let elapsed = end_time - start_time;
        if elapsed > TICK_TIME {
            debug!("Running behind! Starting next tick immediately");
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
fn init_io_manager(
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) -> io::NetworkIoManager {
    io::NetworkIoManager::start(
        format!("{}:{}", config.server.address, config.server.port)
            .parse()
            .unwrap(),
        config.io.io_worker_threads,
        config,
        player_count,
        server_icon,
    )
}

/// Initializes the Specs world and dispatchers.
fn init_world<'a, 'b>(
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    ioman: io::NetworkIoManager,
    level: LevelData,
) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();
    time::init_time(&mut world, &level);
    world.insert(config);
    world.insert(player_count);
    world.insert(ioman);
    world.insert(TickCount::default());

    let generator: Arc<dyn WorldGenerator> = match level.generator_type() {
        LevelGeneratorType::Flat => Arc::new(SuperflatWorldGenerator {
            options: level.clone().generator_options.unwrap_or_default(),
        }),
        LevelGeneratorType::Default => {
            Arc::new(ComposableGenerator::default_with_seed(level.seed as u64))
        }
        _ => Arc::new(EmptyWorldGenerator {}),
    };
    world.insert(level);
    world.insert(generator);

    let mut dispatcher = DispatcherBuilder::new();

    dispatcher.add(network::NetworkSystem, NETWORK, &[]);

    blocks::init_logic(&mut dispatcher);
    physics::init_logic(&mut dispatcher);
    entity::init_logic(&mut dispatcher);
    player::init_logic(&mut dispatcher);
    chunk_logic::init_logic(&mut dispatcher);
    time::init_logic(&mut dispatcher);

    dispatcher.add_barrier();

    blocks::init_handlers(&mut dispatcher);
    physics::init_handlers(&mut dispatcher);
    entity::init_handlers(&mut dispatcher);
    dispatcher.add(util::SpawnerSystem, SPAWNER, &[ITEM_SPAWN]);
    player::init_handlers(&mut dispatcher);
    chunk_logic::init_handlers(&mut dispatcher);

    // Player init dependency is so that player position is loaded
    // before the join handle runs.
    dispatcher.add(
        joinhandler::JoinHandlerSystem,
        JOIN_HANDLER,
        &[NETWORK, PLAYER_INIT],
    );

    dispatcher.add_barrier();

    player::init_broadcast(&mut dispatcher);
    entity::init_broadcast(&mut dispatcher);

    // Broadcast system needs to run last.
    dispatcher.add_barrier();
    dispatcher.add(util::BroadcasterSystem, BROADCASTER, &[]);

    let mut dispatcher = dispatcher.build();
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

/// Tries to load a server icon from the current directory.
fn load_server_icon() -> Option<String> {
    let icon_file: Option<File> = match File::open("server-icon.png") {
        Ok(file) => Some(file),
        Err(_) => None,
    };

    let mut icon_file = icon_file?;

    let mut data = Vec::new();
    if icon_file.read_to_end(&mut data).is_err() {
        warn!("Failed to load server icon.");
        return None;
    }

    let b64_icon = base64::encode(&data);
    Some(format!("data:image/png;base64,{}", b64_icon))
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
    let nameds = world.write_component::<NamedComponent>();
    let named = nameds.get(player).unwrap();

    info!("Disconnecting player {}: {}", named.display_name, reason);

    // Decrement player count
    let player_count = world.fetch_mut::<Arc<PlayerCount>>();
    player_count.0.fetch_sub(1, Ordering::SeqCst);

    // Trigger disconnect event
    let event = PlayerDisconnectEvent {
        player,
        uuid: named.uuid,
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
        let server_icon = Arc::new(Some(String::from("server_icon")));
        let ioman = init_io_manager(
            Arc::clone(&config),
            Arc::clone(&player_count),
            Arc::clone(&server_icon),
        );
        let level = LevelData::default();

        let (world, mut dispatcher) = init_world(config, player_count, ioman, level);
        dispatcher.dispatch(&world);
    }
}
