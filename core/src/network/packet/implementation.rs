use super::super::mctypes::{McTypeRead, McTypeWrite};
use super::*;
use crate::bytebuf::{BufMutAlloc, BufResulted};
use crate::prelude::*;
use bytes::{Buf, BufMut};

type VarInt = i32;
type VarLong = i64;

lazy_static! {
    pub static ref IMPL_MAP: im::HashMap<PacketType, PacketBuilder> = {
        let mut m = im::HashMap::new();

        // Serverbound
        m.insert(PacketType::Handshake, PacketBuilder::with(|| Box::new(Handshake::default())));
        m.insert(PacketType::LoginStart, PacketBuilder::with(|| Box::new(LoginStart::default())));
        m.insert(PacketType::EncryptionResponse, PacketBuilder::with(|| Box::new(EncryptionResponse::default())));

        m.insert(PacketType::Request, PacketBuilder::with(|| Box::new(Request::default())));
        m.insert(PacketType::Ping, PacketBuilder::with(|| Box::new(Ping::default())));

        // Play
        m.insert(PacketType::JoinGame, PacketBuilder::with(|| Box::new(JoinGame::default())));

        // Clientbound
        m.insert(PacketType::DisconnectLogin, PacketBuilder::with(|| Box::new(DisconnectLogin::default())));
        m.insert(PacketType::EncryptionRequest, PacketBuilder::with(|| Box::new(EncryptionRequest::default())));
        m.insert(PacketType::LoginSuccess, PacketBuilder::with(|| Box::new(LoginSuccess::default())));
        m.insert(PacketType::SetCompression, PacketBuilder::with(|| Box::new(SetCompression::default())));

        m.insert(PacketType::Response, PacketBuilder::with(|| Box::new(Response::default())));
        m.insert(PacketType::Pong, PacketBuilder::with(|| Box::new(Pong::default())));

        m
    };
}

// SERVERBOUND

#[derive(Default, AsAny, new)]
pub struct Handshake {
    pub protocol_version: u32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

impl Packet for Handshake {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        self.protocol_version = buf.read_var_int().unwrap() as u32;
        self.server_address = buf.read_string()?;
        self.server_port = buf.get_u16_be();
        let state = buf.read_var_int()?;

        self.next_state = match state {
            1 => HandshakeState::Status,
            2 => HandshakeState::Login,
            _ => return Err(()),
        };

        Ok(())
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::Handshake
    }
}

pub enum HandshakeState {
    Status,
    Login,
}

impl Default for HandshakeState {
    fn default() -> Self {
        HandshakeState::Status
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct LoginStart {
    pub username: String,
}

#[derive(Default, AsAny, new)]
pub struct EncryptionResponse {
    pub secret_length: VarInt,
    pub secret: Vec<u8>,
    pub verify_token_length: VarInt,
    pub verify_token: Vec<u8>,
}

impl Packet for EncryptionResponse {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        self.secret_length = buf.read_var_int()?;

        let mut secret = vec![];
        for _ in 0..self.secret_length {
            secret.push(buf.get_u8());
        }
        self.secret = secret;

        self.verify_token_length = buf.read_var_int()?;

        let mut verify_token = vec![];
        for _ in 0..self.secret_length {
            verify_token.push(buf.get_u8());
        }

        self.verify_token = verify_token;
        Ok(())
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::EncryptionResponse
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct Request {}

#[derive(Default, AsAny, new, Packet)]
pub struct Ping {
    pub payload: u64,
}

// CLIENTBOUND
#[derive(Default, AsAny, new, Packet)]
pub struct DisconnectLogin {
    pub reason: String,
}

#[derive(Default, AsAny, new)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key_len: VarInt,
    pub public_key: Vec<u8>,
    pub verify_token_len: VarInt,
    pub verify_token: Vec<u8>,
}

impl Packet for EncryptionRequest {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        buf.write_string(self.server_id.as_str());

        buf.write_var_int(self.public_key_len);
        buf.write(&self.public_key);

        buf.write_var_int(self.verify_token_len);
        buf.write(&self.verify_token);
    }

    fn ty(&self) -> PacketType {
        PacketType::EncryptionRequest
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct Response {
    pub json_response: String,
}

#[derive(Default, AsAny, new, Packet)]
pub struct Pong {
    pub payload: u64,
}

// PLAY
#[derive(Default, AsAny, new, Packet)]
pub struct SpawnObject {
    pub entity_id: VarInt,
    pub object_uuid: Uuid,
    pub ty: i8,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: u8,
    pub yaw: u8,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnExperienceOrb {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnGlobalEntity {
    entity_id: VarInt,
    ty: u8,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnMob {
    entity_id: VarInt,
    entity_uuid: Uuid,
    ty: VarInt,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
    head_pitch: u8,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
    // TODO metadata
}

#[derive(Default, AsAny, new, Packet)]
pub struct JoinGame {
    pub entity_id: i32,
    pub gamemode: u8,
    pub dimension: i32,
    pub difficulty: u8,
    pub max_players: u8,
    pub level_type: String,
    pub reduced_debug_info: bool,
}
