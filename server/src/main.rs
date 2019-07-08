#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::alloc::System;

#[global_allocator]
static ALLOC: System = System;

pub mod chunkworker;
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
use multimap::MultiMap;
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
    pub player_components: EntityMap<PlayerComponent>,

    /// Difference between players and joined_players:
    /// `players` includes all clients connected
    /// to the server, including those in the login
    /// or status phase, while `joined_players` only
    /// includes those who have joined the server (whose
    /// state is Play).
    pub players: Vec<Entity>,
    pub joined_players: Vec<Entity>,

    /// Each entity which requires its chunks and chunks
    /// around it to be loaded can insert an entry into
    /// this map. For example, a player will have
    /// an entry for every chunk within the view distance.
    /// When a loaded chunk has no more holders, it will
    /// be queued for unloading.
    pub chunk_holders: MultiMap<ChunkPosition, Entity>,

    pub chunk_worker_handle: ChunkWorkerHandle,

    pub running: bool,
    pub tick_count: u64,
}

/// A handle for interacting
/// with the chunk worker IO thread.
pub struct ChunkWorkerHandle {
    /// Channel used to send chunk requests
    sender: crossbeam::channel::Sender<chunkworker::Request>,
    /// Channel used to receive replies from the worker thread
    receiver: crossbeam::channel::Receiver<chunkworker::Reply>,
}

pub struct PlayerComponent {
    pub profile_properties: Vec<mojang_api::ServerAuthProperty>,
    pub gamemode: Gamemode,
}

pub struct EntityComponent {
    pub uuid: Uuid,
    pub display_name: String,
    pub position: Position,
    pub on_ground: bool,
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
    state
        .chunk_worker_handle
        .sender
        .send(chunkworker::Request::ShutDown)
        .unwrap();
}

fn run_loop(state: &mut State) {
    while state.running {
        // For optimal latency, the chunk worker system
        // should run before all other systems
        chunk_worker_system(state);
        network::network_system(state);

        state.tick_count += 1;
        sleep(Duration::from_millis(1000 / 20)); // TODO - proper game loop
    }
}

/// System for emptying queue of loaded chunks.
fn chunk_worker_system(state: &mut State) {
    // Receive all replies from chunk worker thread and load into world
    while let Ok((pos, result)) = state.chunk_worker_handle.receiver.try_recv() {
        match result {
            Ok(chunk) => {
                state.world.chunk_map.insert(pos, chunk);
                debug!("Loaded chunk at {:?}", pos);
            }
            Err(e) => {
                // TODO generate new chunk if it wasn't found
                warn!("Error occurred while loading chunk at {:?}: {:?}", pos, e);
            }
        }
    }
}

fn init_state(config: Config, io_manager: NetworkIoManager) -> State {
    // Initialize chunk worker thread
    let (tx, rx) = chunkworker::start("world");
    State {
        world: World::new(),
        config,

        allocator: GenerationalIndexAllocator::new(),
        io_manager,

        network_components: EntityMap::new(),
        ih_components: EntityMap::new(),
        entity_components: EntityMap::new(),
        player_components: EntityMap::new(),

        players: vec![],
        joined_players: vec![],

        chunk_holders: MultiMap::new(),

        chunk_worker_handle: ChunkWorkerHandle {
            sender: tx,
            receiver: rx,
        },

        running: true,
        tick_count: 0,
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
    // If player was joined, broadcast disconnect
    if state.joined_players.contains(&entity) {
        network::broadcast_player_leave(state, entity);
    }

    network::handle_player_remove(state, entity);

    remove_entity(state, entity);

    state.joined_players.retain(|e| *e != entity);
    state.players.retain(|e| *e != entity);
}

/// Asynchronously loads the chunk at the given position.
/// At some point in time after this function is called,
/// the chunk will appear in the chunk map.
///
/// In the event that the requested chunk does not exist
/// in the world save, it will be generated asynchronously.
pub fn load_chunk(state: &mut State, pos: ChunkPosition) {
    // Send request to chunk worker thread
    state
        .chunk_worker_handle
        .sender
        .send(chunkworker::Request::LoadChunk(pos))
        .unwrap();
}

pub fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
