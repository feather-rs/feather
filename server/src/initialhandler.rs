use crate::network::{
    broadcast_player_join, enable_compression_for_player, enable_encryption_for_player,
    send_packet_to_player,
};
use crate::prelude::*;
use crate::{remove_player, Entity, EntityComponent, PlayerComponent, State};
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use std::fmt::Formatter;
use std::str::FromStr;

use super::{PROTOCOL_VERSION, SERVER_VERSION};
use mojang_api::ServerAuthResponse;

const RSA_BITS: u32 = 1024; // Yes, very secure

const VERIFY_TOKEN_LEN: usize = 4;

#[derive(Debug, Eq, PartialEq)]
enum Stage {
    AwaitHandshake,

    AwaitRequest,
    AwaitPing,

    AwaitLoginStart,
    AwaitEncryptionResponse,
}

pub struct InitialHandlerComponent {
    stage: Stage,
    rsa_key: Option<Rsa<Private>>,
    verify_token: [u8; VERIFY_TOKEN_LEN],
    /// Sent in Login Start
    username: Option<String>,
}

impl InitialHandlerComponent {
    pub fn new() -> Self {
        Self {
            stage: Stage::AwaitHandshake,
            rsa_key: None,
            verify_token: rand::random(),
            username: None,
        }
    }
}

pub enum Error {
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

pub fn handle_packet(state: &mut State, player: Entity, packet: Box<Packet>) -> Result<(), Error> {
    match packet.ty() {
        PacketType::Handshake => {
            handle_handshake(state, player, cast_packet::<Handshake>(&packet))?
        }
        PacketType::Request => handle_request(state, player, cast_packet::<Request>(&packet))?,
        PacketType::Ping => handle_ping(state, player, cast_packet::<Ping>(&packet))?,
        PacketType::LoginStart => {
            handle_login_start(state, player, cast_packet::<LoginStart>(&packet))?
        }
        PacketType::EncryptionResponse => {
            handle_encryption_response(state, player, cast_packet::<EncryptionResponse>(&packet))?
        }
        _ => return Err(Error::InvalidPacket(packet.ty())),
    }
    Ok(())
}

fn handle_handshake(state: &mut State, player: Entity, packet: &Handshake) -> Result<(), Error> {
    let ih = state.ih_components.get_mut(player).unwrap();

    if ih.stage != Stage::AwaitHandshake {
        return Err(Error::InvalidPacket(PacketType::Handshake));
    }

    if packet.protocol_version != PROTOCOL_VERSION && packet.next_state != HandshakeState::Status {
        return Err(Error::InvalidProtocolVersion(
            PROTOCOL_VERSION,
            packet.protocol_version,
        ));
    }

    match packet.next_state {
        HandshakeState::Login => ih.stage = Stage::AwaitLoginStart,
        HandshakeState::Status => ih.stage = Stage::AwaitRequest,
    }

    Ok(())
}

fn handle_request(state: &mut State, player: Entity, _packet: &Request) -> Result<(), Error> {
    let ih = state.ih_components.get_mut(player).unwrap();

    if ih.stage != Stage::AwaitRequest {
        return Err(Error::InvalidPacket(PacketType::Request));
    }

    let payload = json!({
        "version": {
            "name": SERVER_VERSION,
            "protocol": PROTOCOL_VERSION,
        },
        "players": {
            "max": state.config.server.max_players,
            "online": state.joined_players.len(),
        },
        "description": state.config.server.motd,
    });

    let response = Response::new(payload.to_string());

    ih.stage = Stage::AwaitPing;

    send_packet_to_player(state, player, response);

    Ok(())
}

fn handle_ping(state: &mut State, player: Entity, packet: &Ping) -> Result<(), Error> {
    let ih = state.ih_components.get_mut(player).unwrap();

    if ih.stage != Stage::AwaitPing {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    let pong = Pong::new(packet.payload);
    send_packet_to_player(state, player, pong);

    remove_player(state, player);
    debug!("Status handling success");

    Ok(())
}

fn handle_login_start(state: &mut State, player: Entity, packet: &LoginStart) -> Result<(), Error> {
    state.ih_components[player].username = Some(packet.username.clone());

    let ih = state.ih_components.get(player).unwrap();

    if ih.stage != Stage::AwaitLoginStart {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    // If in online mode, enable encryption
    if state.config.server.online_mode {
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

        state.ih_components[player].rsa_key = Some(rsa_key);
        state.ih_components[player].stage = Stage::AwaitEncryptionResponse;

        send_packet_to_player(state, player, encryption_request);
    } else {
        // Login completed - initialize entity and player components
        // This would otherwise be done in `handle_encryption_response`
        let player_comp = PlayerComponent {
            profile_properties: vec![],
        };
        state.player_components.set(player, player_comp);

        let entity_comp = EntityComponent {
            position: Position::new(0.0, 0.0, 0.0, 0.0, 0.0),
            uuid: Uuid::new_v4(),
            display_name: ih.username.as_ref().unwrap().clone(),
        };
        state.entity_components.set(player, entity_comp);

        finish(state, player);
    }

    Ok(())
}

fn handle_encryption_response(
    state: &mut State,
    player: Entity,
    packet: &EncryptionResponse,
) -> Result<(), Error> {
    let ih = state.ih_components.get(player).unwrap();

    if ih.stage != Stage::AwaitEncryptionResponse {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    let rsa = ih.rsa_key.as_ref().unwrap();

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
        &ih.rsa_key.as_ref().unwrap().public_key_to_der().unwrap(),
        ih.username.as_ref().unwrap(),
    );
    if let Ok(res) = auth_res {
        let player_comp = PlayerComponent {
            profile_properties: res.properties,
        };
        state.player_components.set(player, player_comp);

        let entity_comp = EntityComponent {
            position: Position::new(0.0, 64.0, 0.0, 0.0, 0.0),
            uuid: Uuid::from_str(&res.id).unwrap(),
            display_name: ih.username.as_ref().unwrap().clone(),
        };
        state.entity_components.set(player, entity_comp);
        debug!("Authentication successful");
    } else {
        return Err(Error::AuthenticationFailed);
    }

    enable_encryption_for_player(state, player, secret);

    finish(state, player);

    Ok(())
}

fn finish(state: &mut State, player: Entity) {
    // Enable compression if needed
    let threshold = state.config.io.compression_threshold;
    if threshold > 0 {
        let set_compression = SetCompression::new(threshold);
        send_packet_to_player(state, player, set_compression);

        enable_compression_for_player(state, player, threshold as usize);
    }

    let entity_comp = state.entity_components.get(player).unwrap();

    let login_success = LoginSuccess::new(
        entity_comp.uuid.to_hyphenated_ref().to_string(),
        entity_comp.display_name.to_string(),
    );
    send_packet_to_player(state, player, login_success);

    state.ih_components.remove(player);
    join_game(state, player);
    debug!("InitialHandler finished");
}

fn join_game(state: &mut State, player: Entity) {
    let join_game = JoinGame::new(
        player.index() as i32,
        Gamemode::Creative.get_id(),
        Dimension::Overwold.get_id(),
        Difficulty::Medium.get_id(),
        0,
        "default".to_string(),
        false,
    );
    send_packet_to_player(state, player, join_game);

    // Send chunk data
    let view_distance = state.config.server.view_distance as i32;
    for x in -view_distance..view_distance + 1 {
        for y in -view_distance..view_distance + 1 {
            // TODO proper chunk polling
            let pos = ChunkPosition::new(x, y);
            state.world.load_chunk(pos);
            let chunk = state.world.chunk_at(pos).unwrap().clone(); // hmmm... really efficient

            let chunk_data = ChunkData::new(chunk);
            send_packet_to_player(state, player, chunk_data);
        }
    }

    // Send spawn position + player position
    // TODO proper persistence
    let spawn_position = SpawnPosition::new(BlockPosition::new(0, 64, 0));
    send_packet_to_player(state, player, spawn_position);

    let pos_and_look = PlayerPositionAndLookClientbound::new(0.0, 64.0, 0.0, 0.0, 0.0, 0, 0);
    send_packet_to_player(state, player, pos_and_look);

    state.joined_players.push(player);

    broadcast_player_join(state, player);

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

pub fn disconnect_login(state: &mut State, player: Entity, reason: &str) {
    let packet = DisconnectLogin::new(json!({ "text": reason }).to_string());

    send_packet_to_player(state, player, packet);
}
