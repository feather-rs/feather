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

use base::{Setup, State};
use common::Name;
use ecs::{Entity, EntityBuilder, Stage, SysResult};
use flume::{Receiver, Sender};
use protocol::{ClientPlayPacket, ServerPlayPacket};
use smartstring::{LazyCompact, SmartString};
use std::{mem::take, net::SocketAddr};
use uuid::Uuid;

mod initial_handling;
mod listener;
mod worker;

pub use listener::Listener;

use crate::{entity::build_player, packet_handler::PACKET_HANDLERS};

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

pub fn setup(setup: &mut Setup) {
    setup
        .system_in_stage(poll_new_connections, Stage::Pre)
        .system_in_stage(poll_clients, Stage::Pre)
        .resource(EventBuffer::default());
}

/// System to poll for new connections and create new players.
fn poll_new_connections(s: &mut State) -> SysResult {
    let listener = s.resources.get::<ListenerHandle>()?;

    for player in listener.new_players() {
        log::info!("{} has joined", player.username);
        let mut builder = EntityBuilder::new();
        build_player(s, &mut builder, player)?;
        s.ecs.spawn(builder.build());
    }

    Ok(())
}

#[derive(Default)]
struct EventBuffer(Vec<(Entity, FromWorker)>);

/// System to poll for packets and disconnect events received from clients.
fn poll_clients(s: &mut State) -> SysResult {
    let mut buffer_borrow = s.resource_mut::<EventBuffer>()?;

    for (player, network) in s.ecs.query::<(Entity, &Network)>().iter() {
        for event in network.handle.iter() {
            buffer_borrow.0.push((player, event));
        }
    }

    let mut buffer = take(&mut *buffer_borrow);
    drop(buffer_borrow);
    for (player, event) in buffer.0.drain(..) {
        match event {
            FromWorker::PacketReceived(packet) => handle_packet(s, player, packet)?,
            FromWorker::Failed { message } => {
                log::info!("{} disconnected: {}", s.ecs.get::<Name>(player)?.0, message);
                s.despawn(player)?;
            }
        }
    }

    *s.resource_mut::<EventBuffer>()? = buffer;

    Ok(())
}

fn handle_packet(s: &mut State, player: Entity, packet: ClientPlayPacket) -> SysResult {
    log::trace!(
        "Handling packet from {}: {:?}",
        s.ecs.get::<Name>(player)?.0,
        packet
    );
    if let Err(e) = PACKET_HANDLERS.handle(s, player, packet) {
        log::debug!(
            "Failed to handle packet received from '{}': {:?}",
            s.ecs.get::<Name>(player)?.0,
            e
        );
    }
    Ok(())
}
