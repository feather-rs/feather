use feather_core::network::packet::{self, Packet};
use uuid::Uuid;
use mio_extras::channel::{channel, Sender, Receiver};

pub struct Client(usize);

pub enum ServerIoThreadMessage {
    SendPacket(Client, Box<Packet>),
    NotifyPacketReceived(Client, Box<Packet>),
    NewClient(NewClientInfo),
    NotifyDisconnect(Client),
    Disconnect(Client),
}

pub struct NewClientInfo {
    uuid: Uuid,
    name: String,
    networked_client_id: Client,
    sender: Sender<ServerIoThreadMessage>,
    receiver: Receiver<ServerIoThreadMessage>,
}

pub struct NetworkIoManager {}
