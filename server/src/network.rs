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

/// System which polls for player disconnects.
#[system]
pub fn poll_player_disconnect(game: &mut Game, world: &mut World) {
    // For each player with a Network component,
    // check their channel for disconnects.
    let mut to_despawn = BumpVec::new_in(&game.bump);
    <Read<Network>>::query()
        .iter_entities(world.inner())
        .for_each(|(entity, network)| {
            while let Ok(msg) = network.rx.try_recv() {
                match msg {
                    WorkerToServerMessage::NotifyDisconnected { reason } => {
                        log::debug!("Server observed player disconnect: caused by {}", reason);
                        to_despawn.push(entity);
                    }
                }
            }
        });

    to_despawn.into_iter().for_each(|entity| {
        world.despawn(entity);
    });
}

/// System which polls for new clients from the listener task.
#[system]
pub fn poll_new_clients(game: &mut Game, world: &mut World) {
    while let Ok(msg) = game.io_handle.rx.try_recv() {
        match msg {
            ListenerToServerMessage::NewClient(info) => {
                game.spawn_player(info, world);
            }
            ListenerToServerMessage::RequestEntity => {
                let entity = world.spawn(iter::once(()))[0];
                let _ = game
                    .io_handle
                    .tx
                    .unbounded_send(ServerToListenerMessage::Entity(entity));
            }
        }
    }
}
