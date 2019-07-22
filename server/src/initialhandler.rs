use crate::entity::{EntityComponent, PlayerComponent};
use crate::network::{
    broadcast_player_join, enable_compression_for_player, enable_encryption_for_player,
    get_player_initialization_packets, send_packet_to_player, NetworkComponent,
    PacketQueueComponent,
};
use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use std::fmt::Formatter;
use std::str::FromStr;

use super::{PROTOCOL_VERSION, SERVER_VERSION};
use crate::chunkclient::{load_chunk, ChunkWorkerHandle};
use crate::{disconnect_player, PlayerCount};
use mojang_api::ServerAuthResponse;
use specs::{
    Component, Entities, Entity, HashMapStorage, Join, LazyUpdate, Read, ReadStorage, System,
    WorldExt, Write, WriteStorage,
};

const RSA_BITS: u32 = 1024; // Yes, very secure

const VERIFY_TOKEN_LEN: usize = 4;

#[derive(Debug, Eq, PartialEq)]
pub enum Stage {
    AwaitHandshake,

    AwaitRequest,
    AwaitPing,

    AwaitLoginStart,
    AwaitEncryptionResponse,
    AwaitChunkLoad,
}

pub struct InitialHandlerComponent {
    pub stage: Stage,
    rsa_key: Option<Rsa<Private>>,
    verify_token: [u8; VERIFY_TOKEN_LEN],
    /// Sent to server in Login Start
    username: Option<String>,
}

impl Component for InitialHandlerComponent {
    type Storage = HashMapStorage<Self>;
}

impl Default for InitialHandlerComponent {
    fn default() -> Self {
        Self {
            stage: Stage::AwaitHandshake,
            rsa_key: None,
            verify_token: rand::random(),
            username: None,
            should_finish: false,
        }
    }
}

/// System for handling the login sequence.
pub struct InitialHandlerSystem;

impl<'a> System for InitialHandlerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, IntitialHandlerComponent>,
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, PacketQueueComponent>,
        Read<'a, Config>,
        Write<'a, PlayerCount>,
        // LazyUpdate is used to add player + entity
        // components when a player joins. This prevents
        // other systems from being blocked by the initial
        // handler because of a dependency on WriteStorage<PlayerComponent>.
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut ih_comps, net_comps, packet_queue, config, mut player_count, lazy) =
            data;

        let mut packets = vec![];
        packets.append(packet_queue.for_packet(PacketType::Handshake));
        packets.append(packet_queue.for_packet(PacketType::LoginStart));
        packets.append(packet_queue.for_packet(PacketType::Request));
        packets.append(packet_queue.for_packet(PacketType::Ping));
        packets.append(packet_queue.for_packet(PacketType::EncryptionResponse));

        for (player, packet) in packets {
            if !player_check(player, &ih_comps, &lazy) {
                continue; // Bad player - kicked
            }

            if let Err(e) = handle_packet(
                player,
                packet,
                &entities,
                &mut ih_comps,
                &net_comps,
                &config,
                &mut player_count,
                &lazy,
            ) {
                disconnect_player(
                    player,
                    &format!("InitialHandler: error occurred: {}", e),
                    &lazy,
                );
                info!("InitialHandler: player was kicked: {}", e);
            }
        }

        // TODO send chunks
    }
}

/// Verifies that the given player is still in the login phase,
/// disconnecting them and returning `false` if they are not.
fn player_check(
    player: Entity,
    ih_comps: &WriteStorage<InitialHandlerComponent>,
    lazy: &LazyUpdate,
) -> bool {
    if !ih_comps.contains(player) {
        disconnect_player(player, "Sent invalid packet while in play", lazy);
        return false;
    }
    true
}

/// An eror which occurred during initial handling
/// for a player
enum Error {
    InvalidPacket(PacketType),
    MalformedData(String),
    InvalidProtocolVersion(u32, u32),
    AuthenticationFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::InvalidPacket(ty) => f
                .write_str(&format!("Sent invalid packet type {:?}", ty))
                .unwrap(),
            Error::MalformedData(details) => f
                .write_str(&format!("Sent invalid data: {}", details))
                .unwrap(),
            Error::InvalidProtocolVersion(server, client) => f
                .write_str(&format!(
                    "Protocol versions do not match: client is on {}; server is on {}",
                    client, server
                ))
                .unwrap(),
            Error::AuthenticationFailed => f.write_str("Authentication failed").unwrap(),
        };

        Ok(())
    }
}

fn handle_packet(
    player: Entity,
    packet: Box<Packet>,
    entities: &Entities,
    ihcomps: &mut WriteStorage<InitialHandlerComponent>,
    netcomps: &ReadStorage<NetworkComponent>,
    config: &Config,
    player_count: &mut Write<PlayerCount>,
    lazy: &LazyUpdate,
) -> Result<(), Error> {
    let ihcomp = ihcomps.get_mut(player).unwrap();
    let netcomp = netcomps.get(player).unwrap();

    match packet.ty() {
        PacketType::Handshake => handle_handshake(ihcomp, cast_packet::<Handshake>(&packet))?,
        PacketType::Request => handle_request(
            ihcomp,
            netcomp,
            config,
            player_count,
            cast_packet::<Request>(&packet),
        )?,
        PacketType::Ping => {
            handle_ping(ihcomp, netcomp, player, lazy, cast_packet::<Ping>(&packet))?
        }
        PacketType::LoginStart => handle_login_start(
            netcomp,
            config,
            player,
            ihcomps,
            ecomps,
            pcomps,
            netcomps,
            chunk_handle,
            chunk_map,
            lazy,
            cast_packet::<LoginStart>(&packet),
        ),
        PacketType::EncryptionResponse => handle_encryption_response(
            player,
            config,
            ihcomps,
            ecomps,
            pcomps,
            netcomps,
            chunk_handle,
            lazy,
            chunk_map,
            cast_packet::<EncryptionResponse>(&packet),
        ),
        _ => panic!("Invalid packet"),
    }

    Ok(())
}

fn handle_handshake(comp: &mut InitialHandlerComponent, packet: &Handshake) -> Result<(), Error> {
    if comp.stage != Stage::AwaitHandshake {
        return Err(Error::InvalidPacket(PacketType::Handshake));
    }

    if packet.protocol_version != PROTOCOL_VERSION && packet.next_state != HandshakeState::Status {
        return Err(Error::InvalidProtocolVersion(
            PROTOCOL_VERSION,
            packet.protocol_version,
        ));
    }

    match packet.next_state {
        HandshakeState::Login => comp.stage = Stage::AwaitLoginStart,
        HandshakeState::Status => comp.stage = Stage::AwaitRequest,
    }

    Ok(())
}

fn handle_request(
    comp: &mut InitialHandlerComponent,
    net: &NetworkComponent,
    config: &Config,
    player_count: &PlayerCount,
    packet: &Request,
) -> Result<(), Error> {
    if comp.stage != Stage::AwaitRequest {
        return Err(Error::InvalidPacket(PacketType::Request));
    }

    let payload = json!({
        "version": {
            "name": SERVER_VERSION,
            "protocol": PROTOCOL_VERSION,
        },
        "players": {
            "max": config.server.max_players,
            "online": joined_players.len(),
        },
        "description": config.server.motd,
    });

    let response = Response::new(payload.to_string());

    comp.stage = Stage::AwaitPing;

    send_packet_to_player(net, response);

    Ok(())
}

fn handle_ping(
    comp: &mut InitialHandlerComponent,
    net: &NetworkComponent,
    player: Entity,
    lazy: &LazyUpdate,
    packet: &Ping,
) -> Result<(), Error> {
    if comp.stage != Stage::AwaitPing {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    let pong = Pong::new(packet.payload);
    send_packet_to_player(net, pong);

    lazy.remove(player);
    debug!("Status handling success");

    Ok(())
}

fn handle_login_start<'a>(
    net: &NetworkComponent,
    config: &Config,
    player: Entity,
    ihcomps: &mut WriteStorage<InitialHandlerComponent>,
    ecomps: &ReadStorage<EntityComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    netcomps: &ReadStorage<NetworkComponent>,
    chunk_handle: &ChunkWorkerHandle,
    chunk_map: &ChunkMap,
    lazy: &LazyUpdate,
    packet: &LoginStart,
) -> Result<(), Error> {
    let comp = ihcomps.get_mut(player).unwrap();
    comp.username = Some(packet.username.clone());

    if comp.stage != Stage::AwaitLoginStart {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    // If in online mode, enable encryption
    if config.server.online_mode {
        let rsa_key = Rsa::generate(RSA_BITS).unwrap();

        let key_bytes = rsa_key.public_key_to_der().unwrap();

        let mut verify_token = vec![];
        verify_token.extend_from_slice(&ih.verify_token);

        let encryption_request = EncryptionRequest::new(
            "".to_string(), // Server ID - always empty nowadays
            key_bytes.len() as i32,
            key_bytes,
            VERIFY_TOKEN_LEN as i32,
            verify_token,
        );

        comp.rsa_key = Some(rsa_key);
        comp.stage = Stage::AwaitEncryptionResponse;

        send_packet_to_player(net, encryption_request);
    } else {
        // Login completed - initialize entity and player components
        // This would otherwise be done in `handle_encryption_response`
        let player_comp = PlayerComponent {
            profile_properties: vec![],
            gamemode: Gamemode::Creative,
        };
        lazy.insert(player, player_comp);

        let entity_comp = EntityComponent {
            position: Position::new(0.0, 0.0, 0.0, 0.0, 0.0),
            uuid: Uuid::new_v4(),
            display_name: ih.username.as_ref().unwrap().clone(),
            on_ground: true,
        };
        lazy.insert(player, entity_comp);

        finish(
            player,
            ihcomps,
            ecomps,
            pcomps,
            netcomps,
            chunk_handle,
            lazy,
            chunk_map,
            config,
        );
    }

    Ok(())
}

fn handle_encryption_response(
    player: Entity,
    config: &Config,
    ihcomps: &mut WriteStorage<InitialHandlerComponent>,
    ecomps: &ReadStorage<EntityComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    netcomps: &ReadStorage<NetworkComponent>,
    chunk_handle: &ChunkWorkerHandle,
    lazy: &LazyUpdate,
    chunk_map: &ChunkMap,
    packet: &EncryptionResponse,
) -> Result<(), Error> {
    let comp = ihcomps.get_mut(player).unwrap();
    if comp.stage != Stage::AwaitEncryptionResponse {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    let rsa = comp.rsa_key.as_ref().unwrap();

    let secret: [u8; 16] = {
        let mut buf = vec![0u8; rsa.size() as usize];

        if let Ok(amnt) = rsa.private_decrypt(&packet.secret, &mut buf, Padding::PKCS1) {
            if amnt != 16 {
                return Err(Error::MalformedData(format!(
                    "Invalid shared secret length {}",
                    amnt
                )));
            }

            let mut res = [0u8; 16];
            for (i, val) in buf[..16].iter().enumerate() {
                res[i] = *val;
            }
            res
        } else {
            return Err(Error::MalformedData(
                "Error decrypting shared secret".to_string(),
            ));
        }
    };

    let verify_token: [u8; VERIFY_TOKEN_LEN] = {
        let mut buf = vec![0u8; rsa.size() as usize];

        if let Ok(amnt) = rsa.private_decrypt(&packet.verify_token, &mut buf, Padding::PKCS1) {
            if amnt != VERIFY_TOKEN_LEN {
                return Err(Error::MalformedData(format!(
                    "Invalid verify token length {}",
                    amnt
                )));
            }

            let mut res = [0u8; VERIFY_TOKEN_LEN];
            for (i, val) in buf[..VERIFY_TOKEN_LEN].iter().enumerate() {
                res[i] = *val;
            }
            res
        } else {
            return Err(Error::MalformedData(
                "Error decrypting verify token".to_string(),
            ));
        }
    };

    if !compare_verify_tokens(ih.verify_token.clone(), verify_token) {
        return Err(Error::MalformedData(
            "Verify tokens do not match".to_string(),
        ));
    }

    // Authenticate
    let auth_res = authenticate(
        secret.clone(),
        &comp.rsa_key.as_ref().unwrap().public_key_to_der().unwrap(),
        comp.username.as_ref().unwrap(),
    );
    if let Ok(res) = auth_res {
        let player_comp = PlayerComponent {
            profile_properties: res.properties,
            gamemode: Gamemode::Creative,
        };
        lazy.insert(player, player_comp);

        let entity_comp = EntityComponent {
            position: Position::new(0.0, 64.0, 0.0, 0.0, 0.0),
            uuid: Uuid::from_str(&res.id).unwrap(),
            display_name: ih.username.as_ref().unwrap().clone(),
            on_ground: true,
        };
        lazy.insert(player, entity_comp);
        debug!("Authentication successful");
    } else {
        return Err(Error::AuthenticationFailed);
    }

    enable_encryption_for_player(state, player, secret);

    finish(
        player,
        ihcomps,
        ecomps,
        pcomps,
        netcomps,
        chunk_handle,
        lazy,
        chunk_map,
        config,
    );

    Ok(())
}

fn finish(
    player: Entity,
    ihcomps: &mut WriteStorage<InitialHandlerComponent>,
    ecomps: &ReadStorage<EntityComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    netcomps: &ReadStorage<NetworkComponent>,
    chunk_handle: &ChunkWorkerHandle,
    lazy: &LazyUpdate,
    chunk_map: &ChunkMap,
    config: &Config,
) {
    let net = ncomps.get(player).unwrap();
    // Enable compression if needed
    let threshold = config.io.compression_threshold;
    if threshold > 0 {
        let set_compression = SetCompression::new(threshold);
        send_packet_to_player(net, set_compression);

        enable_compression_for_player(state, player, threshold as usize);
    }

    let entity_comp = state.entity_components.get(player).unwrap();

    let login_success = LoginSuccess::new(
        entity_comp.uuid.to_hyphenated_ref().to_string(),
        entity_comp.display_name.to_string(),
    );
    send_packet_to_player(net, login_success);

    join_game(
        player,
        ihcomps,
        ecomps,
        pcomps,
        netcomps,
        chunk_handle,
        lazy,
        chunk_map,
    );
}

fn join_game(
    player: Entity,
    ihcomps: &mut WriteStorage<InitialHandlerComponent>,
    ecomps: &ReadStorage<EntityComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    netcomps: &ReadStorage<NetworkComponent>,
    chunk_handle: &ChunkWorkerHandle,
    lazy: &LazyUpdate,
    chunk_map: &ChunkMap,
) {
    let join_game = JoinGame::new(
        player.index() as i32,
        is: open,
        Gamemode::Creative.get_id(),
        Dimension::Overwold.get_id(),
        Difficulty::Medium.get_id(),
        0,
        "default".to_string(),
        false,
    );

    let net = netcomps.get(player).unwrap();

    send_packet_to_player(net, join_game);

    // Send chunk data. If a chunk in the view distance
    // hasn't been loaded, a load request will be queued.
    let view_distance = state.config.server.view_distance as i32;
    for x in -view_distance..view_distance + 1 {
        for y in -view_distance..view_distance + 1 {
            let pos = ChunkPosition::new(x, y);

            if let Some(chunk) = state.chunk_map.chunk_at(pos) {
                let chunk_data = ChunkData::new(chunk.clone());
                send_packet_to_player(net, chunk_data);
            } else {
                // Queue chunk for loading.
                load_chunk(handle, pos);
                // Make sure that the chunk is sent once loaded
                lazy.exec_mut(|world| {
                    world
                        .write_component::<NetworkComponent>()
                        .get_mut(player)
                        .chunks_to_send
                        .push(pos);
                });
            }
        }
    }

    if net.chunks_to_send.is_empty() {
        complete_join_game(player, ihcomps, ecomps, pcomps, netcomps);
    } else {
        // If not all chunks have been sent, we need to wait
        // until they're loaded and sent before spawning the player.
        // The network system will call `complete_join_game` upon
        // sending all chunks.
        ihcomps.get_mut(player).unwrap().stage = Stage::AwaitChunkLoad;
    }
}

pub fn complete_join_game(
    player: Entity,
    ihcomps: &mut WriteStorage<InitialHandlerComponent>,
    ecomps: &ReadStorage<EntityComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
    netcomps: &ReadStorage<NetworkComponent>,
) {
    // Send spawn position + player position
    // TODO proper persistence

    let spawn_position = SpawnPosition::new(BlockPosition::new(0, 64, 0));
    send_packet_to_player(net, spawn_position);

    let pos_and_look = PlayerPositionAndLookClientbound::new(0.0, 64.0, 0.0, 0.0, 0.0, 0, 0);
    send_packet_to_player(net, pos_and_look);

    // Send other players on the server
    for (ecomp, pcomp, net) in (ecomps, pcomps, netcomps).join() {
        let (player_info, spawn_player) =
            get_player_initialization_packets(ecomp, pcomp, *other_player);
        send_packet_to_player(net, player_info);
        send_packet_to_player(net, spawn_player);
    }

    ihcomps.remove(player);

    broadcast_player_join(player, netcomps, pcomps, ecomps);

    info!("A player joined the game");
}

/// Authenticates a client.
fn authenticate(secret: [u8; 16], pubkey: &[u8], username: &str) -> Result<ServerAuthResponse, ()> {
    let server_hash = mojang_api::server_hash("", secret, pubkey);
    let res = mojang_api::server_auth(username, &server_hash);

    res.map_err(|_| ())
}

fn compare_verify_tokens(x: [u8; VERIFY_TOKEN_LEN], y: [u8; VERIFY_TOKEN_LEN]) -> bool {
    for i in 0..VERIFY_TOKEN_LEN {
        if x[i] != y[i] {
            return false;
        }
    }

    true
}

pub fn disconnect_login(net: &NetworkComponent, reason: &str) {
    let packet = DisconnectLogin::new(json!({ "text": reason }).to_string());

    send_packet_to_player(net, packet);
}
