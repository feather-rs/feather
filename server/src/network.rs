use parking_lot::Mutex;

use mio_extras::channel::{Receiver, Sender};
use shrev::EventChannel;
use specs::{
    Component, DenseVecStorage, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System,
    WorldExt, Write, WriteStorage,
};

use feather_core::network::packet::{implementation::*, Packet, PacketType};

use crate::entity::PlayerComponent;
use crate::io::{NetworkIoManager, ServerToListenerMessage, ServerToWorkerMessage};
use crate::joinhandler::JoinHandlerComponent;
use crate::prelude::*;
use crate::{disconnect_player_without_packet, TickCount};
use strum::EnumCount;

//const MAX_KEEP_ALIVE_TIME: u64 = 30;
//const HEAD_OFFSET: f64 = 1.62; // Offset from feet pos to head pos

/// A packet received from a player.
pub type QueuedPacket = (Entity, Box<dyn Packet>);

/// Vector of `QueuedPacket`.
type QueuedPackets = Vec<QueuedPacket>;

/// A component which contains the received packets
/// for this tick.
pub struct PacketQueue {
    /// Vector of packet queues. For any given packet
    /// type, the queued packets of that type can
    /// be found by indexing into this vector with the ordinal
    /// of the packet type.
    ///
    /// A locked `Vec` is used rather than a `SegQueue` because
    /// there is typically no contention when accessing the queue
    /// for a single packet type (there is at most one system handling
    /// each packet type). As a result, there is no need for a lock-free
    /// data structure.
    queue: Vec<Mutex<QueuedPackets>>,
}

impl PacketQueue {
    /// Returns the packets queued for handling
    /// of the given type, draining the queue of this
    /// type of packet.
    pub fn for_packet(&self, ty: PacketType) -> Vec<QueuedPacket> {
        let ordinal = ty.ordinal();

        let mut queued_packets = self.queue[ordinal].lock();

        let mut new_queue = vec![];
        std::mem::swap(&mut new_queue, &mut queued_packets);

        new_queue
    }

    /// Adds a packet to the queue.
    pub fn add_for_packet(&self, player: Entity, packet: Box<dyn Packet>) {
        let ordinal = packet.ty().ordinal();

        let mut queued_packets = self.queue[ordinal].lock();
        queued_packets.push((player, packet));
    }
}

impl Default for PacketQueue {
    fn default() -> Self {
        // Initialize with an empty queue for each packet type.
        // Packet type ordinals start at 1 (who decided this? FIXME),
        // so we have to use an inclusive range.
        Self {
            queue: (0..=PacketType::count())
                .map(|_| Mutex::new(vec![]))
                .collect(),
        }
    }
}

pub struct NetworkComponent {
    sender: Sender<ServerToWorkerMessage>,
    receiver: Receiver<ServerToWorkerMessage>,
    /// A vector of all chunks that are currently
    /// being loaded and should be sent to the player
    /// once they have been loaded.
    pub chunks_to_send: Vec<ChunkPosition>,
    //last_keep_alive_time: u64,
}

impl NetworkComponent {
    pub fn new(
        sender: Sender<ServerToWorkerMessage>,
        receiver: Receiver<ServerToWorkerMessage>,
    ) -> Self {
        Self {
            sender,
            receiver,
            chunks_to_send: vec![],
        }
    }
}

impl Component for NetworkComponent {
    type Storage = DenseVecStorage<Self>;
}

/// The network system, responsible for
/// receiving and buffering packets received
/// from players. Received packets
/// are added to a queue (`PacketQueue`) so that
/// other systems can handle them.
pub struct NetworkSystem;

/// Event which is triggered when a player joins
/// but before the join handler is completed.
pub struct PlayerPreJoinEvent {
    pub player: Entity,
    pub username: String,
    pub uuid: Uuid,
    pub profile_properties: Vec<mojang_api::ServerAuthProperty>,
}

impl<'a> System<'a> for NetworkSystem {
    type SystemData = (
        WriteStorage<'a, NetworkComponent>,
        ReadStorage<'a, PlayerComponent>,
        Write<'a, EventChannel<PlayerPreJoinEvent>>,
        Write<'a, PacketQueue>,
        Read<'a, NetworkIoManager>,
        Entities<'a>,
        Read<'a, TickCount>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut netcomps,
            pcomps,
            mut join_events,
            packet_queue,
            ioman,
            entities,
            tick_count,
            lazy,
        ) = data;
        // Poll for new connections
        while let Ok(msg) = ioman.receiver.try_recv() {
            match msg {
                ServerToListenerMessage::NewClient(info) => {
                    // New connection - handle it
                    info!("Accepting connection from {}", info.ip);
                    let netcomp = NetworkComponent::new(info.sender, info.receiver);

                    // Create entity
                    let new_entity = entities.create();
                    netcomps.insert(new_entity, netcomp).unwrap();

                    // Create join handler
                    let join_handler = JoinHandlerComponent::new();
                    lazy.exec_mut(move |world| {
                        world
                            .write_component::<JoinHandlerComponent>()
                            .insert(new_entity, join_handler)
                            .unwrap();
                    });

                    // Queue event
                    let event = PlayerPreJoinEvent {
                        player: new_entity,
                        username: info.username.clone(),
                        uuid: info.uuid,
                        profile_properties: info.profile.clone(),
                    };
                    join_events.single_write(event);
                }
                _ => unreachable!(),
            }
        }

        // Receive packets + disconnects from players
        for (player, netcomp) in (&entities, &netcomps).join() {
            while let Ok(msg) = netcomp.receiver.try_recv() {
                match msg {
                    ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                        packet_queue.add_for_packet(player, packet);
                    }
                    ServerToWorkerMessage::NotifyDisconnect => {
                        // TODO broadcast disconnect
                        lazy.exec_mut(move |world| {
                            disconnect_player_without_packet(
                                player,
                                world,
                                "Client disconnected".to_string(),
                            )
                        });
                    }
                    _ => panic!("Network system received invalid message from IO worker}"),
                }
            }
        }

        // Send keepalives every second. The dependency on the player
        // component is required because keepalives should
        // only be sent to players who have joined (completed
        // the login process).
        // TODO check that player hasn't timed out
        if tick_count.0 % TPS == 0 {
            for (netcomp, _) in (&netcomps, &pcomps).join() {
                send_packet_to_player(netcomp, KeepAliveClientbound::new(0));
            }
        }
    }
}

/// Sends a packet to all players on the server, excluding
/// `neq`, if it exists.
pub fn send_packet_to_all_players<P: Packet + Clone + 'static>(
    net_comps: &ReadStorage<NetworkComponent>,
    entities: &Entities,
    packet: P,
    neq: Option<Entity>,
) {
    for (entity, net) in (entities, net_comps).join() {
        if let Some(e) = neq.as_ref() {
            if *e == entity {
                continue; // Exclude this entity
            }
        }

        send_packet_to_player(net, packet.clone());
    }
}

/// Sends a packet to the given player.
pub fn send_packet_to_player<P: Packet + 'static>(comp: &NetworkComponent, packet: P) {
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::SendPacket(Box::new(packet)));
}

/// Sends a packet to the given player.
pub fn send_packet_boxed_to_player(comp: &NetworkComponent, packet: Box<dyn Packet>) {
    let _ = comp.sender.send(ServerToWorkerMessage::SendPacket(packet));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::NewClientInfo;
    use crate::player::PlayerDisconnectEvent;
    use crate::testframework as t;
    use std::net::SocketAddr;

    #[test]
    fn test_packet_queue() {
        let queue = PacketQueue::default();

        let (mut w, _) = t::init_world();
        let player = t::add_player(&mut w);

        let packet = LoginStart::new("test".to_string());
        queue.add_for_packet(player.entity, Box::new(packet));

        let packets = queue.for_packet(PacketType::LoginStart);
        assert_eq!(packets.len(), 1);

        let (entity, packet) = packets.first().unwrap();
        assert_eq!(*entity, player.entity);
        assert_eq!(packet.ty(), PacketType::LoginStart);
    }

    #[test]
    fn test_new_client() {
        let (mut w, mut d) = t::init_world();

        let ioman = w.fetch_mut::<NetworkIoManager>();

        let (send1, _recv1) = mio_extras::channel::channel();
        let (_send2, recv2) = mio_extras::channel::channel();

        let new_client = NewClientInfo {
            ip: SocketAddr::new("127.0.0.1".parse().unwrap(), 25565),
            username: "".to_string(),
            profile: vec![],
            uuid: Uuid::new_v4(),
            sender: send1,
            receiver: recv2,
        };

        let msg = ServerToListenerMessage::NewClient(new_client);
        ioman.listener_sender.send(msg).unwrap();

        let mut event_reader = t::reader(&w);

        drop(ioman);

        // Call the network system
        d.dispatch(&w);

        w.maintain();

        // Confirm that an entity was created
        let mut count = 0;
        let mut entity = None;
        for (e, _, _) in (
            &*w.entities(),
            &w.read_component::<NetworkComponent>(),
            &w.read_component::<JoinHandlerComponent>(),
        )
            .join()
        {
            entity = Some(e);
            count += 1;
        }
        assert_eq!(count, 1);

        // Confirm that playerprejoinevent was queued
        let channel = w.fetch_mut::<EventChannel<PlayerPreJoinEvent>>();
        let join_events: Vec<&PlayerPreJoinEvent> = channel.read(&mut event_reader).collect();
        assert_eq!(join_events.len(), 1);
        let event = join_events.first().unwrap();

        assert_eq!(event.player, entity.unwrap());
    }

    #[test]
    fn test_packet_receive() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        // Send a packet
        let packet = LoginStart::new("".to_string());
        t::send_packet(&player, packet);

        // Run system
        d.dispatch(&w);

        w.maintain();

        // Confirm that packet was received properly
        let queue = w.fetch::<PacketQueue>();
        let packets = queue.for_packet(PacketType::LoginStart);

        assert_eq!(packets.len(), 1);

        let (entity, _packet) = packets.first().unwrap();
        assert_eq!(*entity, player.entity);
    }

    #[test]
    fn test_disconnect() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let mut event_reader = t::reader(&w);

        player
            .network_sender
            .send(ServerToWorkerMessage::NotifyDisconnect)
            .unwrap();

        d.dispatch(&w);

        w.maintain();

        let channel = w.fetch::<EventChannel<PlayerDisconnectEvent>>();
        let events = channel.read(&mut event_reader).collect::<Vec<_>>();

        assert_eq!(events.len(), 1);
        let first = events.first().unwrap();
        assert_eq!(first.player, player.entity);
    }

    #[test]
    fn test_keep_alives() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        d.dispatch(&w);

        t::assert_packet_received(&player, PacketType::KeepAliveClientbound);
        t::assert_packet_received(&player2, PacketType::KeepAliveClientbound);
    }

    #[test]
    fn test_send_packet_to_all_players() {
        let (mut w, _) = t::init_world();

        let player1 = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);
        let player3 = t::add_player(&mut w);

        let packet = LoginStart::new("test".to_string());

        send_packet_to_all_players(
            &w.read_component(),
            &w.entities(),
            packet,
            Some(player1.entity),
        );

        t::assert_packet_received(&player2, PacketType::LoginStart);
        t::assert_packet_received(&player3, PacketType::LoginStart);

        // Check that exclusion was not sent
        let sent = t::received_packets(&player1, None);
        assert!(sent.is_empty());
    }
}
