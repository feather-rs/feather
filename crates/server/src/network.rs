//! Connection handling and the interface between the server
//! thread and IO tasks.
//!
//! # Architecture
//! * A _listener_ task holds the server TcpListener and accepts connections.
//! When a connection is created, it spawns a worker task to handle it.
//! * There is one _worker_ task per connection. When created, a worker
//! task begins with _initial handling_: handling the first few packets
//! and the login sequence. Once initial handling completes, then the server
//! is notified that a new player has joined. At this point, the worker task
//! simply proxies packets between the server thread and the TCP stream.

use flume::{Receiver, Sender};
use protocol::{ClientPlayPacket, ServerPlayPacket};
use smartstring::{LazyCompact, SmartString};
use std::net::SocketAddr;
use uuid::Uuid;

mod initial_handling;
mod listener;
mod worker;

pub use listener::Listener;

/// A handle to send packets to a player.
pub struct Network {
    handle: WorkerHandle,
}

impl Network {
    pub fn new(handle: WorkerHandle) -> Self {
        Self { handle }
    }

    /// Sends a packet to this player.
    pub fn send(&self, packet: impl Into<ServerPlayPacket>) {
        self.handle.send(packet.into());
    }
}

/// A handle to the listener task.
pub struct ListenerHandle {
    /// Used to inform the listener that it should shut down.
    shutdown: Sender<()>,
    /// Receives newly joined players.
    new_players: Receiver<NewPlayer>,
}

impl ListenerHandle {
    /// Returns a non-blocking iterator over newly connected players.
    pub fn new_players<'a>(&'a self) -> impl Iterator<Item = NewPlayer> + 'a {
        self.new_players.try_iter()
    }

    /// Shuts down the listener.
    pub fn shut_down(&self) {
        let _ = self.shutdown.send(());
    }
}

/// Data associated with a newly joined player.
#[derive(Debug)]
pub struct NewPlayer {
    /// The player's IP address.
    pub addr: SocketAddr,
    /// The player's username.
    pub username: SmartString<LazyCompact>,
    /// The player's UUID.
    pub uuid: Uuid,

    /// Handle to the player's worker task.
    pub worker: WorkerHandle,
}

/// Message sent from the main thread to the worker.
#[derive(Debug)]
enum ToWorker {
    /// Sends a packet to the client.
    SendPacket(ServerPlayPacket),
    /// Terminates the connection and shuts down the worker task.
    Disconnect,
}

/// Message sent from the worker to the main thread.
#[derive(Debug)]
pub enum FromWorker {
    /// A packet was received from the client.
    PacketReceived(ClientPlayPacket),
    /// The connection failed, e.g. because it was disconnected.
    /// The worker automatically shut down.
    Failed { message: String },
}

/// A handle to the worker task for a player's connection.
#[derive(Debug)]
pub struct WorkerHandle {
    sender: Sender<ToWorker>,
    receiver: Receiver<FromWorker>,
}

impl WorkerHandle {
    /// Shuts down this worker, disconnecting it.
    pub fn shut_down(&self) {
        let _ = self.sender.send(ToWorker::Disconnect);
    }

    /// Sends a packet to the client.
    pub fn send(&self, packet: ServerPlayPacket) {
        let _ = self.sender.send(ToWorker::SendPacket(packet));
    }

    /// Returns an iterator over events received from the worker.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = FromWorker> + 'a {
        self.receiver.try_iter()
    }
}
