#![allow(clippy::match_single_binding)] // https://github.com/mcarton/rust-derivative/issues/58

use crate::config::Config;
use crate::packet_buffer::PacketBuffers;
use derivative::Derivative;
use feather_core::network::packet::Packet;
use feather_core::player_data::PlayerData;
use feather_core::Position;
use fecs::Entity;
use futures::channel::mpsc;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use uuid::Uuid;

mod initial_handler;
mod listener;
mod worker;

pub enum ServerToWorkerMessage {
    /// Requests that a packet be sent to the client.
    SendPacket(Box<dyn Packet>),
    /// Requests that the client be disconnected.
    Disconnect,
}

#[derive(Debug)]
pub enum WorkerToServerMessage {
    /// Notifies the server thread that the player disconnected.
    NotifyDisconnected { reason: String },
}

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
    pub sender: mpsc::UnboundedSender<ServerToWorkerMessage>,
    #[derivative(Debug = "ignore")]
    pub receiver: crossbeam::Receiver<WorkerToServerMessage>,

    pub entity: Entity,
}

pub struct NetworkIoManager {
    pub rx: crossbeam::Receiver<ListenerToServerMessage>,
    pub tx: mpsc::UnboundedSender<ServerToListenerMessage>,
    /// Used for testing
    pub listener_tx: crossbeam::Sender<ListenerToServerMessage>,
}

impl NetworkIoManager {
    /// Starts a new IO listener.
    pub fn start(
        addr: SocketAddr,
        config: Arc<Config>,
        player_count: Arc<AtomicU32>,
        server_icon: Arc<Option<String>>,
        packet_buffers: Arc<PacketBuffers>,
    ) -> Self {
        info!("Starting IO listener on {}", addr,);

        let (listener_tx, rx) = crossbeam::unbounded();
        let (tx, listener_rx) = mpsc::unbounded();

        let future = run_listener(
            addr,
            listener_tx.clone(),
            listener_rx,
            config,
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
            rx,
            tx,
            listener_tx,
        }
    }
}

/// Initializes certain static variables.
pub fn init() {
    lazy_static::initialize(&initial_handler::RSA_KEY);
}

async fn run_listener(
    addr: SocketAddr,
    tx: crossbeam::Sender<ListenerToServerMessage>,
    rx: mpsc::UnboundedReceiver<ServerToListenerMessage>,
    config: Arc<Config>,
    player_count: Arc<AtomicU32>,
    server_icon: Arc<Option<String>>,
    packet_buffers: Arc<PacketBuffers>,
) {
    if let Err(e) = listener::run_listener(
        addr,
        tx,
        rx,
        config,
        player_count,
        server_icon,
        packet_buffers,
    )
    .await
    {
        error!("An error occurred while binding to socket: {:?}", e);
        std::process::exit(1);
    }
}
