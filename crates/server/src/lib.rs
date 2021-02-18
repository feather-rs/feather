use std::{sync::Arc, time::Instant};

use base::Position;
use chunk_subscriptions::ChunkSubscriptions;
use common::Game;
use ecs::SystemExecutor;
use flume::Receiver;
use initial_handler::NewPlayer;
use listener::Listener;

mod chunk_subscriptions;
pub mod client;
mod connection_worker;
mod entities;
pub mod favicon;
mod initial_handler;
mod listener;
mod network_id_registry;
mod options;
mod packet_handlers;
mod systems;

pub use client::{Client, ClientId, Clients};
pub use network_id_registry::NetworkId;
pub use options::Options;
use systems::view::WaitingChunks;

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

    waiting_chunks: WaitingChunks,
    chunk_subscriptions: ChunkSubscriptions,

    last_keepalive_time: Instant,
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
            waiting_chunks: WaitingChunks::default(),
            chunk_subscriptions: ChunkSubscriptions::default(),
            last_keepalive_time: Instant::now(),
        })
    }

    /// Links this server with a `Game` so that players connecting
    /// to the server become part of this `Game`.
    pub fn link_with_game(self, game: &mut Game, systems: &mut SystemExecutor<Game>) {
        systems::register(self, game, systems);
        game.add_entity_spawn_callback(entities::add_entity_components);
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

    /// Removes a client.
    pub fn remove_client(&mut self, id: ClientId) {
        let client = self.clients.remove(id);
        if let Some(client) = client {
            log::debug!("Removed client for {}", client.username());
        }
    }

    /// Allocates a `NetworkId` for an entity.
    pub fn create_network_id(&mut self) -> NetworkId {
        NetworkId::new()
    }

    fn create_client(&mut self, player: NewPlayer) -> ClientId {
        log::debug!("Creating client for {}", player.username);
        let network_id = self.create_network_id();
        let client = Client::new(player, Arc::clone(&self.options), network_id);
        self.clients.insert(client)
    }

    /// Invokes a callback on all clients.
    pub fn broadcast_with(&self, mut callback: impl FnMut(&Client)) {
        for client in self.clients.iter() {
            callback(client);
        }
    }

    /// Sends a packet to all clients currently subscribed
    /// to the given position. This function should be
    /// used for entity updates, block updates, etc—
    /// any packets that need to be sent only to nearby players.
    pub fn broadcast_nearby_with(&self, position: Position, mut callback: impl FnMut(&Client)) {
        for &client_id in self.chunk_subscriptions.subscriptions_for(position.chunk()) {
            if let Some(client) = self.clients.get(client_id) {
                callback(client);
            }
        }
    }

    pub fn broadcast_keepalive(&mut self) {
        self.broadcast_with(|client| client.send_keepalive());
        self.last_keepalive_time = Instant::now();
    }
}
