use crate::entity::{EntityComponent, PlayerComponent};
use crate::initialhandler::InitialHandlerComponent;
use crate::io::{NetworkIoManager, ServerToListenerMessage, ServerToWorkerMessage};
use crate::prelude::*;
use crate::TickCount;
use feather_core::entitymeta::{EntityMetadata, MetaEntry};
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use mio_extras::channel::{Receiver, Sender};
use specs::{
    Component, DenseVecStorage, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System,
    Write, WriteStorage,
};
use std::sync::Mutex;

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
    fn add_for_packet(&self, player: Entity, packet: Box<Packet>) {
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

impl<'a> System<'a> for NetworkSystem {
    type SystemData = (
        WriteStorage<'a, NetworkComponent>,
        WriteStorage<'a, InitialHandlerComponent>,
        ReadStorage<'a, PlayerComponent>,
        Write<'a, PacketQueue>,
        Read<'a, NetworkIoManager>,
        Entities<'a>,
        Read<'a, TickCount>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut netcomps, mut ihcomps, pcomps, packet_queue, ioman, entities, tick_count, _) =
            data;
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

                    // Create initial handler
                    let ih = InitialHandlerComponent::new();
                    ihcomps.insert(new_entity, ih).unwrap();
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

pub fn enable_compression_for_player(net: &NetworkComponent, threshold: usize) {
    let _ = net
        .sender
        .send(ServerToWorkerMessage::EnableCompression(threshold));
}

pub fn enable_encryption_for_player(net: &NetworkComponent, key: [u8; 16]) {
    let _ = net
        .sender
        .send(ServerToWorkerMessage::EnableEncryption(key));
}

/// Returns the player info and spawn player packets
/// for the given player.
pub fn get_player_initialization_packets(
    ecomp: &EntityComponent,
    pcomp: &PlayerComponent,
    player: Entity,
) -> (PlayerInfo, SpawnPlayer) {
    let display_name = json!({
        "text": ecomp.display_name
    })
    .to_string();

    let mut props = vec![];
    for prop in pcomp.profile_properties.iter() {
        props.push((
            prop.name.clone(),
            prop.value.clone(),
            prop.signature.clone(),
        ));
    }

    let action = PlayerInfoAction::AddPlayer(
        ecomp.display_name.clone(),
        props,
        Gamemode::Creative,
        50,
        display_name,
    );
    let player_info = PlayerInfo::new(action, ecomp.uuid.clone());

    let metadata = EntityMetadata::new().with(&[
        (0, MetaEntry::Byte(0)),
        (1, MetaEntry::VarInt(300)),
        (2, MetaEntry::OptChat(None)),
        (3, MetaEntry::Boolean(false)),
        (4, MetaEntry::Boolean(false)),
        (5, MetaEntry::Boolean(false)),
        (6, MetaEntry::Byte(0)),
        (7, MetaEntry::Float(1.0)),
        (8, MetaEntry::VarInt(0)),
        (9, MetaEntry::Boolean(false)),
        (10, MetaEntry::VarInt(0)),
        (11, MetaEntry::Float(0.0)),
        (12, MetaEntry::VarInt(0)),
        (13, MetaEntry::Byte(0)),
        (14, MetaEntry::Byte(1)),
        // TODO NBT
    ]);

    let spawn_player = SpawnPlayer::new(
        player.id() as i32,
        ecomp.uuid.clone(),
        ecomp.position.x,
        ecomp.position.y,
        ecomp.position.z,
        degrees_to_stops(ecomp.position.pitch),
        degrees_to_stops(ecomp.position.yaw),
        metadata,
    );

    (player_info, spawn_player)
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

pub fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.) as u8
}
