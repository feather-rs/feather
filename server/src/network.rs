use std::sync::Mutex;

use mio_extras::channel::{Receiver, Sender};
use shrev::EventChannel;
use specs::{
    Component, DenseVecStorage, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System,
    WorldExt, Write, WriteStorage,
};

use feather_core::entitymeta::{EntityMetadata, MetaEntry};
use feather_core::network::packet::{implementation::*, Packet, PacketType};

use crate::entity::{EntityComponent, PlayerComponent};
use crate::io::{NetworkIoManager, NewClientInfo, ServerToListenerMessage, ServerToWorkerMessage};
use crate::joinhandler::JoinHandlerComponent;
use crate::prelude::*;
use crate::TickCount;

//const MAX_KEEP_ALIVE_TIME: u64 = 30;
//const HEAD_OFFSET: f64 = 1.62; // Offset from feet pos to head pos

/// A component which contains the received packets
/// for this tick.
pub struct PacketQueue {
    queue: Mutex<Vec<Vec<(Entity, Box<Packet>)>>>,
}

impl PacketQueue {
    /// Returns the packets queued for handling
    /// of the given type, draining the queue of this
    /// type of packet.
    pub fn for_packet(&self, ty: PacketType) -> Vec<(Entity, Box<Packet>)> {
        let mut queue = self.queue.lock().unwrap();

        let ordinal = ty.ordinal();
        if ordinal >= queue.len() {
            self.expand(&mut queue, ordinal);
        }

        let mut result = vec![];
        std::mem::swap(&mut result, queue.get_mut(ordinal).unwrap());

        result
    }

    /// Expands the internal vector to allow for additional packet types.
    fn expand(
        &self,
        queue: &mut std::sync::MutexGuard<Vec<Vec<(Entity, Box<Packet>)>>>,
        to: usize,
    ) {
        if to < queue.len() {
            return;
        }

        for _ in queue.len()..(to + 1) {
            queue.push(Vec::new());
        }
    }

    /// Adds a packet to the queue.
    pub fn add_for_packet(&self, player: Entity, packet: Box<Packet>) {
        let mut queue = self.queue.lock().unwrap();

        let ordinal = packet.ty().ordinal();
        if ordinal >= queue.len() {
            self.expand(&mut queue, ordinal);
        }

        queue[ordinal].push((player, packet));
    }
}

impl Default for PacketQueue {
    fn default() -> Self {
        Self {
            queue: Mutex::new(vec![]),
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
                        uuid: info.uuid.clone(),
                        profile_properties: info.profile.clone(),
                    };
                    join_events.single_write(event);
                }
                _ => panic!("Network system received invalid message from IO listener"),
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
                        entities.delete(player).unwrap();
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

/// Sends a packet to all (joined) players on the server, excluding
/// `neq`, if it exists.
pub fn send_packet_to_all_players<P: Packet + Clone + 'static>(
    net_comps: &ReadStorage<NetworkComponent>,
    player_comps: &ReadStorage<PlayerComponent>,
    entities: &Entities,
    packet: P,
    neq: Option<Entity>,
) {
    for (entity, net, _) in (entities, net_comps, player_comps).join() {
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
pub fn send_packet_boxed_to_player(comp: &NetworkComponent, packet: Box<Packet>) {
    let _ = comp.sender.send(ServerToWorkerMessage::SendPacket(packet));
}
