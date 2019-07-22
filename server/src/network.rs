use crate::entity::{EntityComponent, PlayerComponent};
use crate::initialhandler::InitialHandlerComponent;
use crate::io::{NetworkIoManager, ServerToListenerMessage, ServerToWorkerMessage};
use crate::prelude::*;
use crate::{initialhandler as ih, TickCount};
use feather_blocks::Block;
use feather_core::entitymeta::{EntityMetadata, MetaEntry};
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use mio_extras::channel::{Receiver, Sender};
use specs::{Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, Write, WriteStorage};
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

        self.queue[ordinal].push((player, packet));
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
        let (mut netcomps, mut ihcomps, pcomps, packet_queue, ioman, entities, tick_count, lazy) =
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
                    netcomps.insert(new_entity, netcomp);

                    // Create initial handler
                    let ih = InitialHandlerComponent::new();
                    ihcomps.insert(new_entity, ih);
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
                        entities.delete(player);
                    }
                    msg => panic!(
                        "Network system received invalid message from IO worker: {:?}",
                        msg
                    ),
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

// TODO proper validation of new position
fn handle_player_pos_and_look(
    state: &mut State,
    player: Entity,
    packet: &PlayerPositionAndLookServerbound,
) {
    let ecomp = &state.entity_components[player];
    let old_pos = ecomp.position;

    let new_pos = Position::new(packet.x, packet.feet_y, packet.z, packet.pitch, packet.yaw);

    broadcast_entity_movement(state, player, old_pos, new_pos, true, true);

    state.entity_components[player].position = new_pos;
}

fn handle_player_pos(state: &mut State, player: Entity, packet: &PlayerPosition) {
    let ecomp = &state.entity_components[player];
    let old_pos = ecomp.position;

    let new_pos = Position::new(
        packet.x,
        packet.feet_y,
        packet.z,
        old_pos.pitch,
        old_pos.yaw,
    );

    broadcast_entity_movement(state, player, old_pos, new_pos, true, false);

    state.entity_components[player].position = new_pos;
}

fn handle_player_look(state: &mut State, player: Entity, packet: &PlayerLook) {
    let ecomp = &state.entity_components[player];
    let old_pos = ecomp.position;

    let new_pos = Position::new(old_pos.x, old_pos.y, old_pos.z, packet.pitch, packet.yaw);

    broadcast_entity_movement(state, player, old_pos, new_pos, false, true);

    state.entity_components[player].position = new_pos;
}

fn handle_player_digging(state: &mut State, player: Entity, packet: &PlayerDigging) {
    match packet.status {
        PlayerDiggingStatus::FinishedDigging => {
            if state
                .chunk_map
                .set_block_at(packet.location, Block::Air)
                .is_err()
            {
                // TODO kick player
            }
            broadcast_block_update(state, packet.location);
        }
        PlayerDiggingStatus::StartedDigging => {
            let pcomp = &state.player_components[player];
            if pcomp.gamemode == Gamemode::Creative {
                // Break block instantly - TODO not with sword in hand
                if state
                    .chunk_map
                    .set_block_at(packet.location, Block::Air)
                    .is_err()
                {
                    // TODO kick player
                    warn!("Client sent invalid Player Digging packet");
                    return;
                }
                broadcast_block_update(state, packet.location);
            }
        }
        _ => (), // TODO
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

/// Broadcasts to all clients that the specified player
/// has joined the game. This should be called
/// whenever a player joins.
///
/// This function is currently called by the initial handler.
pub fn broadcast_player_join(
    player: Entity,
    netcomps: &ReadStorage<NetworkComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    ecomps: &ReadStorage<EntityComponent>,
) {
    let ecomp = ecomps.get(player).unwrap();
    let pcomp = pcomps.get(player).unwrap();
    let (player_info, spawn_player) = get_player_initialization_packets(ecomp, pcomp, player);

    for (net, _) in (netcomps, pcomps).join() {
        send_packet_to_player(net, player_info.clone());
        if *p != player {
            send_packet_to_player(net, spawn_player.clone());
        }
    }
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
        player.index() as i32,
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

/// Broadcasts to all joined players that the
/// given player has left the server. This should
/// remove the player from the tablist.
pub fn broadcast_player_leave(state: &mut State, player: Entity) {
    let ecomp = &state.entity_components[player];

    let player_info = PlayerInfo::new(PlayerInfoAction::RemovePlayer, ecomp.uuid.clone());
    send_packet_to_all_players(state, player_info, Some(player));

    let destroy_entities = DestroyEntities::new(vec![player.index() as i32]);
    send_packet_to_all_players(state, destroy_entities, Some(player));
}

/// Notifies all players within range
/// that an entity has moved. This
/// entity can be a player.
///
/// The `has_moved` and `has_moved` fields indicate
/// whether the entity has moved its position
/// or changed its pitch/yaw. These values
/// are used to determine which packet to send:
/// for example, if an entity has only moved and not looked,
/// an Entity Relative Move packet is sent
/// rather than an Entity Look and Relative Move.
///
/// In the event that the entity has moved
/// more than 8 blocks, an Entity Teleport packet
/// is sent instead.
pub fn broadcast_entity_movement(
    state: &mut State,
    entity: Entity,
    old_pos: Position,
    new_pos: Position,
    has_moved: bool,
    has_looked: bool,
) {
    let ecomp = &state.entity_components[entity];

    let dist = new_pos.distance(old_pos).abs();

    if dist <= 8.0 {
        if has_moved && has_looked {
            // Entity Look and Relative Move
            let (dx, dy, dz) = calculate_relative_move(old_pos, new_pos);
            let packet = EntityLookAndRelativeMove::new(
                entity.index() as i32,
                dx,
                dy,
                dz,
                degrees_to_stops(new_pos.yaw),
                degrees_to_stops(new_pos.pitch),
                ecomp.on_ground,
            );
            send_packet_to_all_players(state, packet, Some(entity));
        } else if has_moved {
            // Entity Relative Move
            let (dx, dy, dz) = calculate_relative_move(old_pos, new_pos);
            let packet =
                EntityRelativeMove::new(entity.index() as i32, dx, dy, dz, ecomp.on_ground);
            send_packet_to_all_players(state, packet, Some(entity));
        } else if has_looked {
            // Entity Look
            let packet = EntityLook::new(
                entity.index() as i32,
                degrees_to_stops(new_pos.yaw),
                degrees_to_stops(new_pos.pitch),
                ecomp.on_ground,
            );
            send_packet_to_all_players(state, packet, Some(entity));
        }
    } else {
        // TODO
    }

    // Send Entity Head Look for head yaw
    if has_looked {
        let packet = EntityHeadLook::new(entity.index() as i32, degrees_to_stops(new_pos.yaw));
        send_packet_to_all_players(state, packet, Some(entity));
    }
}

pub fn broadcast_block_update(state: &mut State, pos: BlockPosition) {
    // TODO only send for players in range
    let block = state.chunk_map.block_at(pos).unwrap();
    let packet = BlockChange::new(pos, block.block_state_id() as i32);

    send_packet_to_all_players(state, packet, None);
}

/// Sends a packet to all (joined) players on the server, excluding
/// `neq`, if it exists.
fn send_packet_to_all_players<P: Packet + Clone + 'static>(
    net_comps: &ReadStorage<NetworkComponent>,
    player_comps: &ReadStorage<PlayerComponent>,
    entities: &Entities,
    packet: P,
    neq: Option<Entity>,
) {
    for (entity, net, player) in (entities, net_comps, player_comps).join() {
        if let Some(e) = neq.as_ref() {
            if e == entity {
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

pub fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.) as u8
}

/// Calculates the relative move fields
/// as used in the Entity Relative Move packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (i16, i16, i16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as i16;
    let y = ((current.y * 32.0 - old.y * 32.0) * 128.0) as i16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as i16;
    (x, y, z)
}
