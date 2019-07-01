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
use crate::io::NetworkIoManager;
use crate::network::NetworkComponent;
use feather_core::world::GridChunkGenerator;
use prelude::*;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const TPS: u64 = 20;
pub const PROTOCOL_VERSION: u32 = 404;
pub const SERVER_VERSION: &'static str = "Feather 1.13.2";

type EntityMap<T> = GenerationalArray<T>;
type Entity = GenerationalIndex;

pub struct State {
    pub world: World,
    pub config: Config,

    pub allocator: GenerationalIndexAllocator,
    pub io_manager: NetworkIoManager,

    pub network_components: EntityMap<NetworkComponent>,
    pub ih_components: EntityMap<InitialHandlerComponent>,
    pub entity_components: EntityMap<EntityComponent>,

    pub players: Vec<Entity>,

    pub running: bool,
}

pub struct EntityComponent {
    pub uuid: Uuid,
    pub display_name: String,
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

    let mut state = init_state(config, io_manager);

    info!("Initialized server state");

    run_loop(&mut state);

    state.io_manager.stop();
}

fn run_loop(state: &mut State) {
    while state.running {
        network::network_system(state);

        sleep(Duration::from_millis(1000 / 20)); // TODO - proper game loop
    }
}

fn init_state(config: Config, io_manager: NetworkIoManager) -> State {
    State {
        world: World::new(Box::new(GridChunkGenerator {})),
        config,

        allocator: GenerationalIndexAllocator::new(),
        io_manager,

        network_components: EntityMap::new(),
        ih_components: EntityMap::new(),
        entity_components: EntityMap::new(),

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

pub fn add_entity(state: &mut State) -> Entity {
    let e = state.allocator.allocate();
    e
}

pub fn add_player(state: &mut State) -> Entity {
    let e = add_entity(state);
    state.players.push(e);
    e
}

pub fn remove_entity(state: &mut State, entity: Entity) {
    state.allocator.deallocate(entity);
}

pub fn remove_player(state: &mut State, entity: Entity) {
    network::handle_player_remove(state, entity);

    remove_entity(state, entity);
    state.players.retain(|e| *e != entity);
}

pub fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
