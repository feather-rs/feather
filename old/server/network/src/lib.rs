#![forbid(unsafe_code)]

//! The networking implementation for the server, based on async/await
//! and Tokio. Contains a listener task which accepts new connections
//! and a worker task for each client which reads and writes packets.

pub const PROTOCOL_VERSION: u32 = 404;
pub const SERVER_VERSION: &str = "Feather 1.13.2";

#[macro_use]
extern crate feather_core;

use derivative::Derivative;
use feather_core::anvil::player::PlayerData;
use feather_core::util::Position;
use feather_server_types::{
    Config, PacketBuffers, ServerToWorkerMessage, Uuid, WorkerToServerMessage, WrappedBanInfo,
};
use fecs::Entity;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tokio::net::TcpListener;

mod initial_handler;
mod listener;
mod worker;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ListenerToServerMessage {
    /// Notifies the server thread that a new client connected.
    ///
    /// This message is sent after initial handling completes.
    NewClient(NewClientInfo),
    /// Requests that the server create an empty `Entity` and send
    /// it to the listener. This entity will later be used as a player.
    RequestEntity,
    /// Tells the server that a requested entity is no longer needed
    /// and may be deleted.
    ///
    /// This typically happens when a connection comes in the form
    /// of a status ping, where the entity is no longer needed
    /// but has never served any purpose.
    DeleteEntity(Entity),
}

#[derive(Debug)]
pub enum ServerToListenerMessage {
    /// Sends an entity to the listener as a response
    /// to `ListenerToServerMessage::RequestEntity`.
    Entity(Entity),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct NewClientInfo {
    pub ip: SocketAddr,
    pub username: String,
    pub profile: Vec<mojang_api::ProfileProperty>,
    pub uuid: Uuid,
    pub data: PlayerData,
    pub position: Position,

    #[derivative(Debug = "ignore")]
    pub sender: flume::Sender<ServerToWorkerMessage>,
    #[derivative(Debug = "ignore")]
    pub receiver: flume::Receiver<WorkerToServerMessage>,

    pub entity: Entity,
}

pub struct NetworkIoManager {
    pub rx: Mutex<flume::Receiver<ListenerToServerMessage>>,
    pub tx: flume::Sender<ServerToListenerMessage>,
    /// Used for testing
    pub listener_tx: flume::Sender<ListenerToServerMessage>,
}

impl NetworkIoManager {
    /// Starts a new IO listener.
    pub fn start(
        listener: TcpListener,
        config: Arc<Config>,
        ban_info: WrappedBanInfo,
        player_count: Arc<AtomicU32>,
        server_icon: Arc<Option<String>>,
        packet_buffers: Arc<PacketBuffers>,
    ) -> Self {
        let (listener_tx, rx) = flume::bounded(16);
        let (tx, listener_rx) = flume::bounded(16);

        let future = run_listener(
            listener,
            listener_tx.clone(),
            listener_rx,
            (config, ban_info),
            player_count,
            server_icon,
            packet_buffers,
        );

        if cfg!(test) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.spawn(future);
        } else {
            tokio::spawn(future);
        }

        Self {
            rx: Mutex::new(rx),
            tx,
            listener_tx,
        }
    }
}

/// Initializes certain static variables.
pub fn init() {
    Lazy::force(&initial_handler::RSA_KEY);
}

async fn run_listener(
    listener: TcpListener,
    tx: flume::Sender<ListenerToServerMessage>,
    rx: flume::Receiver<ServerToListenerMessage>,
    config_bans: (Arc<Config>, WrappedBanInfo),
    player_count: Arc<AtomicU32>,
    server_icon: Arc<Option<String>>,
    packet_buffers: Arc<PacketBuffers>,
) {
    if let Err(e) = listener::run_listener(
        listener,
        tx,
        rx,
        config_bans,
        player_count,
        server_icon,
        packet_buffers,
    )
    .await
    {
        log::error!("An error occurred while binding to socket: {:?}", e);
        std::process::exit(1);
    }
}
