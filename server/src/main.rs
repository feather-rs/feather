#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::alloc::System;

#[global_allocator]
static ALLOC: System = System;

pub mod config;
pub mod genindex;
pub mod initialhandler;
pub mod io;
pub mod network;
pub mod prelude;

use crate::genindex::{GenerationalArray, GenerationalIndex, GenerationalIndexAllocator};
use crate::initialhandler::InitialHandlerComponent;
use crate::network::NetworkComponent;
use feather_core::world::GridChunkGenerator;
use prelude::*;
use std::time::Duration;

pub const TPS: u64 = 20;

type EntityMap<T> = GenerationalArray<T>;
type Entity = GenerationalIndex;

pub struct State {
    pub world: World,
    pub config: Config,

    pub allocator: GenerationalIndexAllocator,

    pub network_components: EntityMap<NetworkComponent>,
    pub ih_components: EntityMap<InitialHandlerComponent>,

    pub players: Vec<Entity>,

    pub running: bool,
}

impl State {
    pub fn remove_entity(&mut self, entity: Entity) {
        self.allocator.deallocate(entity);
    }
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

    let mut state = init_state(config);

    info!("Initialized server state");

    run_loop(&mut state);
}

fn run_loop(state: &mut State) {
    while state.running {}
}

fn init_state(config: Config) -> State {
    State {
        world: World::new(Box::new(GridChunkGenerator {})),
        config,

        allocator: GenerationalIndexAllocator::new(),

        network_components: EntityMap::new(),
        ih_components: EntityMap::new(),

        players: vec![],
        running: true,
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
