//! Network logic. This module includes:
//! * `Network`, a component assigned to entities
//! which can send and receive packets. (This is only
//! added for players, obviously.)
//! * `PacketQueue`, which stores packets received
//! from players and allows systems to poll for packets
//! received of a given type.

use crate::game::Game;
use crate::io::{
    ListenerToServerMessage, ServerToListenerMessage, ServerToWorkerMessage, WorkerToServerMessage,
};
use crate::BumpVec;
use crossbeam::Receiver;
use feather_core::Packet;
use fecs::{IntoQuery, Read, World};
use futures::channel::mpsc::UnboundedSender;
use std::iter;

/// Network component containing channels to send and receive packets.
///
/// Systems should call `Network::send` to send a packet to this entity (player).
pub struct Network {
    pub tx: UnboundedSender<ServerToWorkerMessage>,
    pub rx: Receiver<WorkerToServerMessage>,
}

impl Network {
    /// Sends a packet to this player.
    pub fn send<P>(&self, packet: P)
    where
        P: Packet,
    {
        self.send_boxed(Box::new(packet));
    }

    /// Sends a boxed packet to this player.
    pub fn send_boxed(&self, packet: Box<dyn Packet>) {
        // Discard error in case the channel was disconnected
        // (e.g. if the player disconnected and its worker task
        // shut down, and the disconnect was not yet registered
        // by the server)
        let _ = self
            .tx
            .unbounded_send(ServerToWorkerMessage::SendPacket(packet));
    }
}
