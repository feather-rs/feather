use std::sync::Arc;

use common::Game;
use ecs::SystemExecutor;
use flume::Receiver;
use initial_handler::NewPlayer;
use listener::Listener;

pub mod client;
mod connection_worker;
pub mod favicon;
mod initial_handler;
mod listener;
mod network_id_registry;
mod options;
mod packet_handlers;
mod systems;

use network_id_registry::NetworkIdAllocator;

pub use client::{Client, ClientId, Clients};
pub use network_id_registry::NetworkId;
pub use options::Options;

/// A Minecraft server.
///
/// Call [`register`] to register the server
/// with a [`Game`](common::Game). This will
/// cause the server to serve the game to players.
///
/// Uses asynchronous IO with Tokio.
pub struct Server {
    options: Arc<Options>,
    clients: Clients,
    new_players: Receiver<NewPlayer>,
    network_id_allocator: NetworkIdAllocator,
}

impl Server {
    /// Starts a server with the given `Options`.
    ///
    /// Must be called within the context of a Tokio runtime.
    pub async fn bind(options: Options) -> anyhow::Result<Self> {
        let options = Arc::new(options);

        let (new_players_tx, new_players) = flume::bounded(4);
        Listener::start(Arc::clone(&options), new_players_tx).await?;

        Ok(Self {
            options,
            clients: Clients::new(),
            new_players,
            network_id_allocator: NetworkIdAllocator::new(),
        })
    }

    /// Links this server with a `Game` so that players connecting
    /// to the server become part of this `Game`.
    pub fn link_with_game(self, game: &mut Game, systems: &mut SystemExecutor<Game>) {
        systems::register(self, game, systems);
    }
}

/// Low-level functions, mostly used internally.
/// You may find these useful for some custom functionality.
impl Server {
    /// Polls for newly connected players. Returns the IDs of the new clients.
    pub fn accept_new_players(&mut self) -> Vec<ClientId> {
        let mut clients = Vec::new();
        for player in self.new_players.clone().try_iter() {
            let id = self.create_client(player);
            clients.push(id);
        }
        clients
    }

    /// Allocates a `NetworkId` for an entity.
    pub fn create_network_id(&mut self) -> NetworkId {
        self.network_id_allocator.allocate_id()
    }

    fn create_client(&mut self, player: NewPlayer) -> ClientId {
        log::debug!("Creating client for {}", player.username);
        let network_id = self.create_network_id();
        let client = Client::new(player, Arc::clone(&self.options), network_id);
        self.clients.insert(client)
    }
}
