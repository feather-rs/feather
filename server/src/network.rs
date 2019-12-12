//! Network logic. This module includes:
//! * `Network`, a component assigned to entities
//! which can send and receive packets. (This is only
//! added for players, obviously.)
//! * `PacketQueue`, which stores packets received
//! from players and allows systems to poll for packets
//! received of a given type.

use crate::io::{ListenerToServerMessage, NetworkIoManager, ServerToWorkerMessage};
use crate::lazy::Lazy;
use crate::player;
use crate::player::Player;
use crate::state::State;
use crossbeam::Receiver;
use feather_core::network::cast_packet;
use feather_core::{Packet, PacketType};
use futures::channel::mpsc::UnboundedSender;
use legion::entity::Entity;
use legion::query::Read;
use parking_lot::{Mutex, MutexGuard};
use std::iter;
use std::vec::Drain;
use tonks::{PreparedQuery, PreparedWorld, Query};

type QueuedPackets = Vec<(Entity, Box<dyn Packet>)>;

/// The packet queue. This type allows systems to poll for
/// received packets of a given type.
///
/// A system should never require mutable access to this type.
pub struct PacketQueue {
    /// Vector of queued packets. This vector is indexed
    /// by the ordinal of the packet type, and each
    /// queue contains only packets of its type.
    queue: Vec<Mutex<QueuedPackets>>,
}

impl Default for PacketQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketQueue {
    /// Creates a new, empty `PacketQueue`.
    pub fn new() -> Self {
        Self {
            queue: iter::repeat_with(|| Mutex::new(vec![]))
                .take(PacketType::count() + 1)
                .collect(),
        }
    }

    /// Returns an iterator over packets of a given type.
    pub fn received<P: Packet>(&self) -> MutexGuard<impl IntoIterator<Item = (Entity, P)>> {
        let queue = self.queue[P::ty().ordinal()].lock();

        MutexGuard::map(queue, |queue| {
            queue
                .drain(..)
                .map(|(entity, packet)| (entity, cast_packet::<P>(packet)))
        })
    }

    /// Adds a packet to the queue.
    pub fn push(&self, packet: Box<dyn Packet>, entity: Entity) {
        let ordinal = packet.ty().ordinal();

        self.queue[ordinal].lock().push((packet, entity));
    }
}

/// Network component containing channels to send and receive packets.
///
/// Systems should call `Self::send` to send a packet to this entity (player).
pub struct Network {
    pub sender: UnboundedSender<ServerToWorkerMessage>,
    pub receiver: Receiver<ServerToWorkerMessage>,
}

impl Network {
    /// Sends a packet to this player.
    pub fn send<P>(&self, packet: P) {
        self.send_boxed(Box::new(packet));
    }

    /// Sends a boxed packet to this player.
    pub fn send_boxed(&self, packet: Box<dyn Packet>) {
        // Discard error in case the channel was disconnected
        // (e.g. if the player disconnected and its worker task
        // shut down, and the disconnect was not yet registered
        // by the server)
        let _ = self
            .sender
            .unbounded_send(ServerToWorkerMessage::SendPacket(packet));
    }
}

/// The network system. This system is responsible for:
/// * Handling player disconnects.
/// * Pushing received packets to the packet queue.
/// * Accepting new clients and creating entities for them.
#[system]
pub fn network(
    state: &State,
    io: &NetworkIoManager,
    packet_queue: &PacketQueue,
    mut query: PreparedQuery<Read<Network>>,
    world: PreparedWorld,
) {
    // For each `Network`, handle any disconnects and received packets.
    query.par_entities_for_each(world, |(entity, network): (Entity, Network)| {
        while let Ok(msg) = network.receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyDisconnect(reason) => {
                    state.exec(move |world| {
                        debug_assert!(world.delete(entity), "player already deleted");
                    });
                }
                ServerToWorkerMessage::SendPacket(packet) => {
                    packet_queue.push(packet, entity);
                }
                _ => unreachable!(),
            }
        }
    });

    // Handle new clients.
    while let Ok(msg) = io.receiver.try_recv() {
        match msg {
            ListenerToServerMessage::NewClient(info) => {
                player::create(state, info);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_core::network::packet::implementation::Handshake;
    use feather_core::network::packet::PacketType::SpawnObject;
    use legion::world::World;

    #[test]
    fn packet_queue() {
        let queue = PacketQueue::new();

        let mut world = World::new();
        let entities = world.insert((), vec![(), ()]);

        queue.push(Box::new(Handshake::default()), entities[0]);
        queue.push(Box::new(SpawnObject::default()), entities[1]);
        queue.push(Handshake::default(), entities[1]);

        let mut handshakes = queue.received::<Handshake>();
        assert_eq!(handshakes.next().unwrap().0, entities[0]);
        assert_eq!(handshakes.next().unwrap().0, entities[1]);
        assert!(handshakes.next().is_none());

        let mut spawn_objects = queue.received::<SpawnObject>();
        assert_eq!(spawn_objects.next().unwrap().0, entities[1]);
        assert!(spawn_objects.next().is_none());
    }
}
