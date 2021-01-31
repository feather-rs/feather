use std::sync::Arc;

use client::{Client, ClientId, Clients};
use flume::Receiver;
use initial_handler::NewPlayer;
use listener::Listener;

mod client;
mod connection_worker;
mod favicon;
mod initial_handler;
mod listener;
mod options;

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
        })
    }
}

/// Low-level functions, mostly used internally.
/// You may find these useful for some custom functionality.
impl Server {
    /// Polls for newly connected players. Returns the IDs of the new clients.
    pub fn poll_new_players(&mut self) -> Vec<ClientId> {
        let mut clients = Vec::new();
        for player in self.new_players.clone().try_iter() {
            let id = self.create_client(player);
            clients.push(id);
        }
        clients
    }

    fn create_client(&mut self, player: NewPlayer) -> ClientId {
        let client = Client::new(player, Arc::clone(&self.options));
        self.clients.insert(client)
    }
}
