use crate::config::Config;
use crate::PlayerCount;
use feather_core::network::packet::Packet;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

mod initial_handler;
mod listener;
mod worker;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Client(usize);

pub enum ServerToWorkerMessage {
    SendPacket(Box<dyn Packet>),
    NotifyPacketReceived(Box<dyn Packet>),
    NotifyDisconnect(String),
    Disconnect,
}

pub enum ListenerToServerMessage {
    NewClient(NewClientInfo),
}

pub struct NewClientInfo {
    pub ip: SocketAddr,
    pub username: String,
    pub profile: Vec<mojang_api::ProfileProperty>,
    pub uuid: Uuid,

    pub sender: futures::channel::mpsc::UnboundedSender<ServerToWorkerMessage>,
    pub receiver: crossbeam::Receiver<ServerToWorkerMessage>,
}

pub struct NetworkIoManager {
    pub receiver: crossbeam::Receiver<ListenerToServerMessage>,
    /// Used for testing
    pub listener_sender: crossbeam::Sender<ListenerToServerMessage>,
}

impl NetworkIoManager {
    /// Starts a new IO listener.
    pub fn start(
        addr: SocketAddr,
        config: Arc<Config>,
        player_count: Arc<PlayerCount>,
        server_icon: Arc<Option<String>>,
    ) -> Self {
        info!("Starting IO listener on {}", addr,);

        let (sender, receiver) = crossbeam::unbounded();

        let future = run_listener(addr, sender.clone(), config, player_count, server_icon);

        if cfg!(test) {
            let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
            rt.spawn(future);
        } else {
            tokio::spawn(future);
        }

        Self {
            receiver,
            listener_sender: sender,
        }
    }
}

impl Default for NetworkIoManager {
    fn default() -> Self {
        panic!("Don't try this");
    }
}

/// Initializes certain static variables.
pub fn init() {
    lazy_static::initialize(&initial_handler::RSA_KEY);
}

async fn run_listener(
    addr: SocketAddr,
    sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) {
    if let Err(e) = listener::run_listener(addr, sender, config, player_count, server_icon).await {
        error!("An error occurred while binding to socket: {:?}", e);
        std::process::exit(1);
    }
}
