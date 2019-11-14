//! Feather, a Minecraft server implementation in Rust.
//!
//! This is the developer documenation, and anyone wishing to contribute
//! should read this first.
//!
//! The core of Feather is based on [`legion`](https://github.com/TomGillen/legion),
//! a fast ECS for Rust, and [`tonks`](https://github.com/feather-rs/tonks), a system
//! scheduler built on Legion. As a result, we use the ECS architecture: the
//! entire server consists of _entities_, simple IDs with no data; _components_,
//! arbitrary data, such as positions, which can be attached to an entity;
//! and _systems_, functions which can run logic over entities and components.
//!
//! The benefit of this design is the splitting between data and logic. With a traditional
//! object-oriented design, there would be an Entity class from which other entities
//! inherit and can override logic. However, this model does not work well with Rust's
//! borrow checker (as we found out in the early days of Feather, when this design was
//! used), and more importantly, it reduces flexibility. Say, for example, that a plugin
//! wants to modify the physics behavior of a cow by increasing gravity. With the object-oriented
//! design, it would have to somehow modify the `run_physics` method on `Cow`, which is not
//! possible in a native language (although it can be done in some languages using class rewriting).
//! On the other hand, using Feather, there is a `Physics` component which stores gravity,
//! drag, etc. for an entity, and all the plugin has to do is modify that component.
//!
//! Another benefit of the ECS architecture is performance. With the OO design, entities
//! would likely be stored in a `Vec<Box<dyn Entity>>`, which is horribly inefficient
//! with regards to cache locality and iteration performance. Legion, however, stores
//! entities in an efficient manner such that many of the same type of component
//! are stored contiguously, which is excellent for cache performance.
//!
//! Here is a more in-depth description of each concept in Feather.
//!
//! ## Entities
//! Entities, or `legion::entity::Entity`, are simple numerical IDs: they store no
//! data, but components can be attached to an entity. See the systems section
//! for information on how to access this data.
//!
//! ## Components
//! Components store data associated with an entity, such as `Position`.
//! Arbitrary amounts of components can be associated with any given entity.
//!
//! ## Resources
//! Resources are a branching off from the pure ECS concept. Like components, they
//! store arbitrary data in the form of structs; however, they are not associated
//! with any entity. An example of a resource might be the chunk map, which allows
//! access to blocks in the world.
//!
//! ## Systems
//! The systems concept is where things become more complex. `tonks`, the library
//! which runs systems, runs them _in parallel_, effectively multithreading the
//! entire server. This is still safe because the scheduler ensures that no
//! two systems which write to the same data run at the same time.
//!
//! A consequence of the above is that systems must explicitly state which
//! resources and components they might access. If a system needs access to positions,
//! it needs to state this upfront so the scheduler can ensure memory safety.
//!
//! Systems can be written by creating a function annotated with the `system` attribute.
//! `tonks` will automatically detect which resources are accessed based on the function
//! parameters, and it will register them to the system scheduler without you
//! having to do anything. (How does this work? Don't even bother.)
//!
//! ## Events
//! Events are another concept unique to Feather, at least in the way they are implemented
//! here. The name states what they are—BlockChangeEvent, for example, is triggered when
//! a block is updated.
//!
//! A system can trigger an event by specifying an `&mut Trigger<E>` resource.
//!
//! ## Event handlers
//! Event handlers are similar to systems, but they only run when the event they
//! handle is triggered. Use the `event_handler` attribute on a function to register
//! it as an event handler.
//!
//! # Networking model
//! Feather consists of two key parts: the server threads, which run systems
//! over entities, and the networking tasks, which run on [`tokio`](https://github.com/tokio-rs/tokio).
//! The networking tasks will accept connections and parse any packets received
//! from the player.
//!
//! When a connection is first made to the server's TCP listener, the networking
//! task will spawn another task to handle the connection. It's important to note that
//! at this time, _the server thread is totally unaware of the connection_—networking
//! runs entirely isolated from the rest of the program.
//!
//! At this point, the initial handler takes over, which runs on the networking task.
//! It will handle the login sequence or status pings, perform authentication, etc.
//! If successful, the server is notified of the new player through a channel.
//!
//! When the server is notified of a new player, it's essential to realize
//! that the player still hasn't been sent important data, such as
//! chunk packets, inventory, time, nearby entities, etc. `PlayerJoinEvent`
//! is used to send this data.

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
#[macro_use]
extern crate tonks;

extern crate nalgebra_glm as glm;

use crossbeam::Receiver;
use std::alloc::System;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use specs::{Builder, Dispatcher, DispatcherBuilder, Entity, LazyUpdate, World, WorldExt};

use feather_core::network::packet::implementation::DisconnectPlay;

use crate::chunk_logic::{ChunkHolders, ChunkWorkerHandle};
use crate::config::Config;
use crate::worldgen::{
    ComposableGenerator, EmptyWorldGenerator, SuperflatWorldGenerator, WorldGenerator,
};
use feather_core::level;
use feather_core::level::{deserialize_level_file, save_level_file, LevelData, LevelGeneratorType};
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

#[global_allocator]
static ALLOC: System = System;

pub mod chunk_logic;
pub mod chunkworker;
pub mod config;
pub mod io;
pub mod network;
pub mod physics;
pub mod shutdown;
#[cfg(test)]
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

pub fn main() {
    let config = Arc::new(load_config());
    init_log(&config);

    info!("Starting Feather; please wait...");

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
        info!(
            "World directory '{}' not found, creating it",
            world_dir.display()
        );
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

    // Channel used by the shutdown handler to notify the server thread.
    let (shutdown_tx, shutdown_rx) = crossbeam::unbounded();

    shutdown::init(shutdown_tx);

    info!("Initialized world");

    info!("Generating RSA keypair");
    io::init();

    info!("Queuing spawn chunks for loading");
    load_spawn_chunks(&mut world);

    info!("Server started");
    run_loop(&mut world, &mut dispatcher, shutdown_rx);

    info!("Shutting down");

    info!("Saving chunks");
    shutdown::save_chunks(&mut world);
    info!("Saving level.dat");
    shutdown::save_level(&world);
    info!("Saving player data");
    shutdown::save_player_data(&world);

    info!("Goodbye");
    exit(0);
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

fn create_level(config: &Config) -> LevelData {
    let seed = get_seed(config);
    let world_name = &config.world.name;
    debug!("Using seed {} for world '{}'", seed, world_name);

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

fn get_seed(config: &Config) -> i64 {
    let seed_raw = &config.world.seed;
    // Empty seed: random
    // Seed is valid i64: parse
    // Seed is something else: hash
    if seed_raw.is_empty() {
        rand::thread_rng().gen()
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
