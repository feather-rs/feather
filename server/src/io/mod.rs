use feather_core::network::packet::{self, Packet};
use mio_extras::channel::{channel, Receiver, Sender};
use std::net::SocketAddr;
use uuid::Uuid;

mod listener;
mod worker;

#[derive(Debug, PartialEq, Eq, Hash)]
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
}

pub struct NewClientInfo {
    uuid: Uuid,
    name: String,
    networked_client_id: Client,
    ip: SocketAddr,

    sender: Sender<ServerToWorkerMessage>,
    receiver: Receiver<ServerToWorkerMessage>,
}

pub struct NetworkIoManager {}
