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

pub const TPS: u64 = 20;

pub struct Server {
    config: Rc<Config>,
    player_count: u32,
    io_manager: io::NetworkIoManager,
    rsa_key: openssl::rsa::Rsa<openssl::pkey::Private>,

    entity_id_counter: RefCell<EntityId>,
    tick_counter: RefCell<u64>,

    players: Players, // Server reference
    world: Rc<World>,
}

#[derive(Clone)]
pub struct Players {
    players: Rc<RefCell<Vec<Rc<PlayerHandle>>>>,
}

impl Server {
    pub fn allocate_entity_id(&self) -> EntityId {
        let mut counter = self.entity_id_counter.borrow_mut();
        *counter += 1;
        *counter
    }

    pub fn tick_count(&self) -> u64 {
        *self.tick_counter.borrow()
    }

    pub fn set_block_at(&self, pos: BlockPosition, block: Block) {
        for player in self.players.players.borrow().iter() {
            if let Err(()) = player.notify_block_update(pos, block) {
                *player.should_remove.borrow_mut() = true;
            }
        }
        self.world.set_block_at(pos, block);
    }

    pub fn add_entity(&self, entity: Rc<Entity>) {
        self.world.add_entity(entity.id, entity);
    }

    pub fn register_player(&self, player: Rc<PlayerHandle>) {
        for player in self.players.players.borrow().iter() {
            player.notify_player_join(&player.entity());
        }
    }

    pub fn move_entity(&self, _entity: EntityId, _new_pos: Position) {
        unimplemented!()
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

    let mut players = Players {
        players: Rc::new(RefCell::new(vec![])),
    };

    let server = Rc::new(Server {
        config: Rc::new(config),
        player_count: 0,
        io_manager,
        rsa_key: openssl::rsa::Rsa::generate(1024).unwrap(),

        entity_id_counter: RefCell::new(0),
        tick_counter: RefCell::new(0),
        players: players.clone(),
        world: Rc::new(World::new()),
    });

    loop {
        while let Ok(msg) = server.io_manager.receiver.try_recv() {
            match msg {
                io::ServerToListenerMessage::NewClient(info) => {
                    debug!("Server registered connection");
                    let new_player = PlayerHandle::accept_player_connection(
                        info.sender,
                        info.receiver,
                        server.config.server.motd.clone(),
                        server.player_count,
                        server.config.server.max_players,
                        server.rsa_key.clone(),
                        Rc::clone(&server.config),
                        Rc::clone(&server),
                    );
                    players.players.borrow_mut().push(Rc::new(new_player));
                }
                _ => unreachable!(),
            }
        }

        tick(Rc::clone(&server), &mut players);

        std::thread::sleep(Duration::from_millis(50)); // TODO proper game loop
    }
}

fn tick(server: Rc<Server>, players: &mut Players) {
    let mut remove_indices = Vec::with_capacity(0);
    for (i, player) in players.players.borrow().iter().enumerate() {
        let ok = player.tick().is_ok();
        let should_keep = !*player.should_remove.borrow();
        if !(ok && should_keep) {
            remove_indices.push(i);
        }
    }

    {
        let mut count = 0;
        for i in remove_indices {
            players.players.borrow_mut().remove(i - count);
            count += 1;
        }
    }

    *server.tick_counter.borrow_mut() += 1;
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
