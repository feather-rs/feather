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
pub mod initialhandler;
pub mod io;
pub mod player;
pub mod prelude;

use prelude::*;
use std::time::Duration;

pub type EntityId = i32;

pub const TPS: u64 = 20;

pub struct Server {
    config: Rc<Config>,
    player_count: u32,
    io_manager: io::NetworkIoManager,
    rsa_key: openssl::rsa::Rsa<openssl::pkey::Private>,

    entity_id_counter: RefCell<EntityId>,
    tick_counter: u64,

    action_queue: RefCell<Vec<Action>>,
}

pub struct Players {
    players: Vec<RefCell<PlayerHandle>>,
}

impl Server {
    pub fn allocate_entity_id(&self) -> EntityId {
        let mut counter = self.entity_id_counter.borrow_mut();
        *counter += 1;
        *counter
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_counter
    }

    pub fn set_block_at(&self, pos: BlockPosition, block: Block) {
        self.action_queue.borrow_mut().push(Action::SetBlock(pos, block));
    }

    pub fn move_entity(&self, entity: EntityId, new_pos: Position) {
        self.action_queue.borrow_mut().push(Action::MoveEntity(entity, new_pos));
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

    let mut server = Server {
        config: Rc::new(config),
        player_count: 0,
        io_manager,
        rsa_key: openssl::rsa::Rsa::generate(1024).unwrap(),

        entity_id_counter: RefCell::new(0),
        tick_counter: 0,
        action_queue: RefCell::new(vec![]),
    };

    let mut players = Players { players: vec![] };

    let mut world = World::new();

    loop {
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
                        Rc::clone(&server.config),
                    );
                    players.players.push(RefCell::new(new_player));
                }
                _ => unreachable!(),
            }
        }

        tick(&mut server, &mut players, &mut world);

        std::thread::sleep(Duration::from_millis(50)); // TODO proper game loop
    }
}

fn tick(server: &mut Server, players: &mut Players, world: &World) {
    players.players.retain(|player| {
        let ok = player.borrow_mut().tick(server, world).is_ok();
        let should_keep = !player.borrow().should_remove;
        ok && should_keep
    });

    for action in server.action_queue.borrow_mut().drain(..) {
        match action {
            Action::SetBlock(pos, block) => world.set_block_at(pos, block),
            Action::MoveEntity(entity, pos) => {
                // Notify all players of mvoement
                // TODO check for only nearby players
                players.players.iter().for_each(|player| {
                    // TODO
                })
            }
        }
    }

    server.tick_counter += 1;
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

#[derive(Clone, Copy, Debug)]
enum Action {
    SetBlock(BlockPosition, Block),
    MoveEntity(EntityId, Position),
}
