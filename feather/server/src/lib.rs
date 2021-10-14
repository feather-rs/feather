#![allow(clippy::unnecessary_wraps)] // systems are required to return Results

use std::{sync::Arc, time::Instant};

use flume::Receiver;

use base::Position;
use chunk_subscriptions::ChunkSubscriptions;
pub use client::{Client, Clients};
use common::player_count::PlayerCount;
use common::Game;
use ecs::SystemExecutor;
use feather_commands::CommandDispatcher;
use initial_handler::NewPlayer;
pub use options::Options;
use quill_common::components::ClientId;
use systems::view::WaitingChunks;

use crate::listener::Listener;

mod chunk_subscriptions;
pub mod client;
pub mod config;
mod connection_worker;
pub mod console_input;
mod entities;
pub mod favicon;
mod initial_handler;
pub mod listener;
pub mod options;
mod packet_handlers;
mod systems;

/// A Minecraft server.
///
/// Call [`link_with_game`](Server::link_with_game) to register the server
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

    player_count: PlayerCount,
}

impl Server {
    /// Starts a server with the given `Options`.
    ///
    /// Must be called within the context of a Tokio runtime.
    pub async fn bind(options: Options) -> anyhow::Result<Server> {
        let options = Arc::new(options);
        let player_count = PlayerCount::new(options.max_players);

        let (new_players_tx, new_players) = flume::bounded(4);
        Listener::start(Arc::clone(&options), player_count.clone(), new_players_tx).await?;

        log::info!(
            "Server is listening on {}:{}",
            options.bind_address,
            options.port
        );

        Ok(Self {
            options,
            clients: Clients::new(),
            new_players,
            waiting_chunks: WaitingChunks::default(),
            chunk_subscriptions: ChunkSubscriptions::default(),
            last_keepalive_time: Instant::now(),
            player_count,
        })
    }

    /// Links this server with a `Game` so that players connecting
    /// to the server become part of this `Game`.
    pub fn link_with_game(self, game: &mut Game, systems: &mut SystemExecutor<Game>) {
        let mut dispatcher = CommandDispatcher::new();
        feather_commands::register_vanilla_commands(&mut dispatcher);
        systems::register(self, dispatcher, game, systems);
        game.add_entity_spawn_callback(entities::add_entity_components);
    }

    /// Gets the number of online players.
    pub fn player_count(&self) -> u32 {
        self.player_count.get()
    }
}

/// Low-level functions, mostly used internally.
/// You may find these useful for some custom functionality.
impl Server {
    /// Polls for newly connected players. Returns the IDs of the new clients.
    pub fn accept_new_players(&mut self) -> Vec<ClientId> {
        let mut clients = Vec::new();
        for player in self.new_players.clone().try_iter() {
            if let Some(old_client) = self.clients.iter().find(|x| x.uuid() == player.uuid) {
                old_client.disconnect("Logged in from another location!");
            }
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

    fn create_client(&mut self, player: NewPlayer) -> ClientId {
        log::debug!("Creating client for {}", player.username);
        let client = Client::new(player, Arc::clone(&self.options));
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
