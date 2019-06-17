use super::super::mctypes::{McTypeRead, McTypeWrite};
use super::*;
use crate::bytebuf::BufMutAlloc;
use crate::prelude::*;
use bytes::{Buf, BufMut};

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

/*impl Packet for LoginStart {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        self.username = buf.read_string()?;

        Ok(())
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::LoginStart
    }
}*/

#[derive(Default, AsAny, new)]
pub struct EncryptionResponse {
    pub secret_length: i32,
    pub secret: Vec<u8>,
    pub verify_token_length: i32,
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

#[derive(Default, AsAny, new)]
pub struct Request {}

impl Packet for Request {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        Ok(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::Request
    }
}

#[derive(Default, AsAny, new)]
pub struct Ping {
    pub payload: u64,
}

impl Packet for Ping {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        self.payload = buf.get_u64_be();
        Ok(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::Ping
    }
}

// CLIENTBOUND
#[derive(Default, AsAny, new)]
pub struct DisconnectLogin {
    pub reason: String,
}

impl Packet for DisconnectLogin {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        self.reason = buf.read_string()?;
        Ok(())
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        buf.write_string(self.reason.as_str());
    }

    fn ty(&self) -> PacketType {
        PacketType::DisconnectLogin
    }
}

#[derive(Default, AsAny, new)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key_len: i32,
    pub public_key: Vec<u8>,
    pub verify_token_len: i32,
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

#[derive(Default, AsAny, new)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}

impl Packet for LoginSuccess {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        buf.write_string(self.uuid.as_str());
        buf.write_string(self.username.as_str());
    }

    fn ty(&self) -> PacketType {
        PacketType::LoginSuccess
    }
}

#[derive(Default, AsAny, new)]
pub struct SetCompression {
    pub threshold: i32,
}

impl Packet for SetCompression {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        buf.write_var_int(self.threshold);
    }

    fn ty(&self) -> PacketType {
        PacketType::SetCompression
    }
}

#[derive(Default, AsAny, new)]
pub struct Response {
    pub json_response: String,
}

impl Packet for Response {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        buf.write_string(self.json_response.as_str());
    }

    fn ty(&self) -> PacketType {
        PacketType::Response
    }
}

#[derive(Default, AsAny, new)]
pub struct Pong {
    pub payload: u64,
}

impl Packet for Pong {
    fn read_from(&mut self, mut buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut ByteBuf) {
        buf.write_u64_be(self.payload);
    }

    fn ty(&self) -> PacketType {
        PacketType::Pong
    }
}

// PLAY
#[derive(Default, AsAny, new)]
pub struct SpawnObject {
    pub entity_id: i32,
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

impl Packet for SpawnObject {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.entity_id);
        buf.write_uuid(&self.object_uuid);
        buf.write_i8(self.ty);
        buf.write_f64_be(self.x);
        buf.write_f64_be(self.y);
        buf.write_f64_be(self.z);
        buf.write_u8(self.pitch);
        buf.write_u8(self.yaw);
        buf.write_i32_be(self.data);
        buf.write_i16_be(self.velocity_x);
        buf.write_i16_be(self.velocity_y);
        buf.write_i16_be(self.velocity_z);
    }

    fn ty(&self) -> PacketType {
        PacketType::SpawnObject
    }
}

#[derive(Default, AsAny, new)]
pub struct SpawnExperienceOrb {
    pub entity_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

impl Packet for SpawnExperienceOrb {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.entity_id);
        buf.write_f64_be(self.x);
        buf.write_f64_be(self.y);
        buf.write_f64_be(self.z);
        buf.write_i16_be(self.count);
    }

    fn ty(&self) -> PacketType {
        PacketType::SpawnExperienceOrb
    }
}

#[derive(Default, AsAny, new)]
pub struct SpawnGlobalEntity {
    entity_id: i32,
    ty: u8,
    x: f64,
    y: f64,
    z: f64,
}

impl Packet for SpawnGlobalEntity {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.entity_id);
        buf.write_u8(self.ty);
        buf.write_f64_be(self.x);
        buf.write_f64_be(self.y);
        buf.write_f64_be(self.z);
    }

    fn ty(&self) -> PacketType {
        PacketType::SpawnGlobalEntity
    }
}

#[derive(Default, AsAny, new)]
pub struct SpawnMob {
    entity_id: i32,
    entity_uuid: Uuid,
    ty: i32,
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

impl Packet for SpawnMob {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.entity_id);
        buf.write_uuid(&self.entity_uuid);
        buf.write_var_int(self.ty);
        buf.write_f64_be(self.x);
        buf.write_f64_be(self.y);
        buf.write_f64_be(self.z);
        buf.write_u8(self.yaw);
        buf.write_u8(self.pitch);
        buf.write_u8(self.head_pitch);
        buf.write_i16_be(self.velocity_x);
        buf.write_i16_be(self.velocity_y);
        buf.write_i16_be(self.velocity_z);
        buf.write_u8(0xff); // TODO metadata
    }

    fn ty(&self) -> PacketType {
        PacketType::SpawnMob
    }
}

#[derive(Default, AsAny, new)]
pub struct SpawnPainting {}

#[derive(Default, AsAny, new)]
pub struct JoinGame {
    pub entity_id: i32,
    pub gamemode: u8,
    pub dimension: i32,
    pub difficulty: u8,
    pub max_players: u8,
    pub level_type: String,
    pub reduced_debug_info: bool,
}

impl Packet for JoinGame {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_i32_be(self.entity_id);
        buf.write_u8(self.gamemode);
        buf.write_i32_be(self.dimension);
        buf.write_u8(self.difficulty);
        buf.write_u8(self.max_players);
        buf.write_string(&self.level_type);
        buf.write_bool(self.reduced_debug_info);
    }

    fn ty(&self) -> PacketType {
        PacketType::JoinGame
    }
}
