use feather_core::network::Packet;
use parking_lot::Mutex;

/// Network component containing channels to send and receive packets.
///
/// Systems should call `Network::send` to send a packet to this entity (player).
pub struct Network {
    pub tx: flume::Sender<ServerToWorkerMessage>,
    pub rx: Mutex<flume::Receiver<WorkerToServerMessage>>,
}

impl Network {
    /// Sends a packet to this player.
    pub fn send(&self, packet: impl Packet) {
        self.send_boxed(Box::new(packet));
    }

    /// Sends a boxed packet to this player.
    pub fn send_boxed(&self, packet: Box<dyn Packet>) {
        // Discard error in case the channel was disconnected
        // (e.g. if the player disconnected and its worker task
        // shut down, and the disconnect was not yet registered
        // by the server)
        let _ = self.tx.try_send(ServerToWorkerMessage::SendPacket(packet));
    }
}

/// Message sent from the server threads to a player's
/// IO task.
pub enum ServerToWorkerMessage {
    /// Requests that a packet be sent to the client.
    SendPacket(Box<dyn Packet>),
    /// Requests that the client be disconnected.
    Disconnect,
}

/// Message sent from a player's IO task to the server threads.
#[derive(Debug)]
pub enum WorkerToServerMessage {
    /// Notifies the server thread that the player disconnected.
    NotifyDisconnected { reason: String },
}
