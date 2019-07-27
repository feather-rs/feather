use crate::config::Config;
use crate::PlayerCount;
use feather_core::network::packet::Packet;
use mio_extras::channel::{channel, Receiver, Sender};
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

mod initialhandler;
mod listener;
mod worker;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Client(usize);

pub enum ServerToWorkerMessage {
    SendPacket(Box<Packet>),
    NotifyPacketReceived(Box<Packet>),
    NotifyDisconnect,
    Disconnect,
}

pub enum ServerToListenerMessage {
    ShutDown,
    NewClient(NewClientInfo),
}

pub enum ListenerToWorkerMessage {
    ShutDown,
    NewConnection(mio::net::TcpStream, SocketAddr),
    NewClient(NewClientInfo),
}

pub struct NewClientInfo {
    pub ip: SocketAddr,
    pub username: String,
    pub profile: Vec<mojang_api::ServerAuthProperty>,
    pub uuid: Uuid,

    pub sender: Sender<ServerToWorkerMessage>,
    pub receiver: Receiver<ServerToWorkerMessage>,
}

pub struct NetworkIoManager {
    pub sender: Sender<ServerToListenerMessage>,
    pub receiver: Receiver<ServerToListenerMessage>,
}

impl NetworkIoManager {
    /// Starts a new IO event loop with the specified number
    /// of worker threads.
    pub fn start(
        addr: SocketAddr,
        num_worker_threads: u16,
        config: Arc<Config>,
        player_count: Arc<PlayerCount>,
    ) -> Self {
        info!(
            "Starting IO event loop on {} with {} worker threads",
            addr, num_worker_threads
        );
        let mut workers = vec![];

        for _ in 0..num_worker_threads {
            let (send1, recv1) = channel();
            let (send2, recv2) = channel();
            let player_count = Arc::clone(&player_count);
            let config = Arc::clone(&config);

            thread::spawn(move || worker::start(recv1, send2, config, player_count));
            workers.push((send1, recv2));
        }

        let (sender1, receiver1) = channel();
        let (sender2, receiver2) = channel();
        thread::spawn(move || listener::start(addr.to_string(), sender1, receiver2, workers));

        Self {
            sender: sender2,
            receiver: receiver1,
        }
    }

    /// Gracefully shuts down all IO threads, consuming the object.
    pub fn stop(self) {
        let msg = ServerToListenerMessage::ShutDown;
        self.sender.send(msg).unwrap();

        info!("Shut down IO event loop");
    }
}

impl Default for NetworkIoManager {
    fn default() -> Self {
        panic!("Nope, don't call default() on the IO manager. That won't work.");
    }
}
