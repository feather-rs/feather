use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use openssl::pkey::Private;
use openssl::rsa::{self, Rsa};
use std::rc::Rc;
use crate::{State, Entity};
use std::fmt::Formatter;
use crate::io::ServerToWorkerMessage;
use crate::network::{send_packet_to_player, enable_compression_for_player, enable_encryption_for_player};

use super::{PROTOCOL_VERSION, SERVER_VERSION};

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
}

impl InitialHandlerComponent {
    pub fn new() -> Self {
        Self {
            stage: Stage::AwaitHandshake,
            rsa_key: None,
            verify_token: rand::random(),
        }
    }
}

pub enum Error {
    InvalidPacket(PacketType),
    MalformedData,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::InvalidPacket(ty) => f.write_str(&format!("Client sent invalid packet type {:?}", ty)),
            Error::MalformedData => f.write_str("Client sent invalid data"),
        };

        Ok(())
    }
}

pub fn handle_packet(state: &mut State, player: Entity, packet: Box<Packet>) -> Result<(), Error> {
    match packet.ty() {
        PacketType::Handshake => handle_handshake(state, player, cast_packet::<Handshake>(&packet))?,
        PacketType::Request => handle_request(state, player,  cast_packet::<Request>(&packet))?,
        PacketType::Ping => handle_ping(state, player, cast_packet::<Ping>(&packet))?,
        PacketType::LoginStart => handle_login_start(state, player, cast_packet::<LoginStart>(&packet))?,
        //PacketType::EncryptionResponse => handle_encryption_response(state, player, cast_packet::<EncryptionResponse>(&packet))?,
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
        disconnect(state, player, &format!("Bad protocol version {} - this server is on {}", packet.protocol_version, PROTOCOL_VERSION));
        return Ok(());
    }

    match packet.next_state {
        HandshakeState::Login => ih.stage = Stage::AwaitLoginStart,
        HandshakeState::Status => ih.stage = Stage::AwaitRequest,
    }

    Ok(())
}

fn handle_request(state: &mut State, player: Entity, packet: &Request) -> Result<(), Error> {
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
            "online": state.players.len(),
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

    state.remove_entity(player);
    debug!("Status handling success");

    Ok(())
}

fn handle_login_start(state: &mut State, player: Entity, packet: &LoginStart) -> Result<(), Error> {
    let ih = state.ih_components.get(player).unwrap();

    if ih.stage != Stage::AwaitLoginStart {
        return Err(Error::InvalidPacket(PacketType::Ping));
    }

    // Enable compression if needed
    let threshold = state.config.io.compression_threshold;
    if threshold > 0 {
        let set_compression = SetCompression::new(threshold);
        send_packet_to_player(state, player, set_compression);

        enable_compression_for_player(state, player, threshold as usize);
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
        // Login completed
        unimplemented!()
    }

    Ok(())
}

fn disconnect(state: &mut State, player: Entity, reason: &str) {
    let packet = DisconnectLogin::new(
        json!({
            "text": reason
        }).to_string()
    );

    send_packet_to_player(state, player, packet);

    state.remove_entity(player);
    info!("Player disconnected: {}", reason);
}