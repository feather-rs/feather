use crate::initialhandler::InitialHandlerComponent;
use crate::io::{ServerToListenerMessage, ServerToWorkerMessage};
use crate::prelude::*;
use crate::{add_player, initialhandler as ih, remove_player, Entity, State};
use feather_blocks::Block;
use feather_core::entitymeta::{EntityMetadata, MetaEntry};
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use mio_extras::channel::{Receiver, Sender};

//const MAX_KEEP_ALIVE_TIME: u64 = 30;
//const HEAD_OFFSET: f64 = 1.62; // Offset from feet pos to head pos

pub struct NetworkComponent {
    sender: Sender<ServerToWorkerMessage>,
    receiver: Receiver<ServerToWorkerMessage>,
    //last_keep_alive_time: u64,
}

impl NetworkComponent {
    pub fn new(
        sender: Sender<ServerToWorkerMessage>,
        receiver: Receiver<ServerToWorkerMessage>,
    ) -> Self {
        Self { sender, receiver }
    }
}

pub fn network_system(state: &mut State) {
    handle_connections(state);

    send_keep_alives(state);

    poll_for_new_players(state);
}

fn handle_connections(state: &mut State) {
    let mut players_to_remove = vec![];

    for player in state.players.clone() {
        while let Ok(msg) = state.network_components[player].receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                    if let Some(_) = state.ih_components.get(player) {
                        if let Err(e) = ih::handle_packet(state, player, packet) {
                            info!("Disconnecting player: {}", e);
                            ih::disconnect_login(state, player, &e.to_string());
                            remove_player(state, player);
                        }
                    } else {
                        handle_player_packet(state, player, packet);
                    }
                }
                ServerToWorkerMessage::NotifyDisconnect => {
                    players_to_remove.push(player);
                }
                _ => panic!("Invalid message received from worker thread"),
            }
        }
    }

    for _player in players_to_remove {
        remove_player(state, _player);
    }
}

fn handle_player_packet(state: &mut State, player: Entity, packet: Box<Packet>) {
    match packet.ty() {
        PacketType::PlayerPositionAndLookServerbound => handle_player_pos_and_look(
            state,
            player,
            cast_packet::<PlayerPositionAndLookServerbound>(&packet),
        ),
        PacketType::PlayerPosition => {
            handle_player_pos(state, player, cast_packet::<PlayerPosition>(&packet));
        }
        PacketType::PlayerLook => {
            handle_player_look(state, player, cast_packet::<PlayerLook>(&packet));
        }
        PacketType::PlayerDigging => {
            handle_player_digging(state, player, cast_packet::<PlayerDigging>(&packet));
        }
        _ => (), // TODO
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
                .world
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
                    .world
                    .set_block_at(packet.location, Block::Air)
                    .is_err()
                {
                    // TODO kick player
                }
                broadcast_block_update(state, packet.location);
            }
        }
        _ => (), // TODO
    }
}

fn send_keep_alives(state: &mut State) {
    if state.tick_count % TPS != 0 {
        return; // Only run once per second
    }

    for player in state.joined_players.clone() {
        let keep_alive = KeepAliveClientbound::new(0);
        send_packet_to_player(state, player, keep_alive);
    }
}

fn poll_for_new_players(state: &mut State) {
    while let Ok(msg) = state.io_manager.receiver.try_recv() {
        match msg {
            ServerToListenerMessage::NewClient(info) => {
                debug!("Server registering player");
                let player = add_player(state);
                let ih = InitialHandlerComponent::new();
                state.ih_components.set(player, ih);

                let netc = NetworkComponent::new(info.sender, info.receiver);
                state.network_components.set(player, netc);
            }
            _ => panic!("Invalid message received from listener thread"),
        }
    }
}

pub fn send_packet_to_player<P: Packet + 'static>(state: &State, player: Entity, packet: P) {
    let comp = &state.network_components[player];
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::SendPacket(Box::new(packet)));
}

pub fn send_packet_boxed_to_player(state: &State, player: Entity, packet: Box<Packet>) {
    let comp = &state.network_components[player];
    let _ = comp.sender.send(ServerToWorkerMessage::SendPacket(packet));
}

pub fn enable_compression_for_player(state: &State, player: Entity, threshold: usize) {
    let comp = &state.network_components[player];
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::EnableCompression(threshold));
}

pub fn enable_encryption_for_player(state: &State, player: Entity, key: [u8; 16]) {
    let comp = &state.network_components[player];
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::EnableEncryption(key));
}

pub fn handle_player_remove(state: &mut State, player: Entity) {
    let comp = &state.network_components[player];
    let _ = comp.sender.send(ServerToWorkerMessage::Disconnect);
}

/// Broadcasts to all clients that the specified player
/// has joined the game. This should be called
/// whenever a player joins.
///
/// This function is currently called by the initial handler.
pub fn broadcast_player_join(state: &mut State, player: Entity) {
    let (player_info, spawn_player) = get_player_initialization_packets(state, player);

    for p in &state.joined_players {
        send_packet_to_player(state, *p, player_info.clone());
        if *p != player {
            send_packet_to_player(state, *p, spawn_player.clone());
        }
    }
}

/// Returns the player info and spawn player packets
/// for the given player.
pub fn get_player_initialization_packets(
    state: &State,
    player: Entity,
) -> (PlayerInfo, SpawnPlayer) {
    let entity_comp = state.entity_components.get(player).unwrap();
    let player_comp = state.player_components.get(player).unwrap();

    let display_name = json!({
        "text": entity_comp.display_name
    })
    .to_string();

    let mut props = vec![];
    for prop in player_comp.profile_properties.iter() {
        props.push((
            prop.name.clone(),
            prop.value.clone(),
            prop.signature.clone(),
        ));
    }

    let action = PlayerInfoAction::AddPlayer(
        entity_comp.display_name.clone(),
        props,
        Gamemode::Creative,
        50,
        display_name,
    );
    let player_info = PlayerInfo::new(action, entity_comp.uuid.clone());

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
        entity_comp.uuid.clone(),
        entity_comp.position.x,
        entity_comp.position.y,
        entity_comp.position.z,
        degrees_to_stops(entity_comp.position.pitch),
        degrees_to_stops(entity_comp.position.yaw),
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
    let block = state.world.block_at(pos).unwrap();
    let packet = BlockChange::new(pos, block.block_state_id() as i32);

    send_packet_to_all_players(state, packet, None);
}

fn send_packet_to_all_players<P: Packet + Clone + 'static>(
    state: &State,
    packet: P,
    neq: Option<Entity>,
) {
    for player in &state.joined_players {
        if neq.is_none() || *player != neq.unwrap() {
            send_packet_to_player(state, *player, packet.clone());
        }
    }
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
