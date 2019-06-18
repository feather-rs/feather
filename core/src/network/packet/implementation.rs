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
pub struct SpawnPainting {
    entity_id: VarInt,
    entity_uuid: Uuid,
    motive: VarInt,
    location: BlockPosition,
    direction: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnPlayer {
    entity_id: VarInt,
    palyer_uuid: Uuid,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
    // TODO metadata
}

#[derive(Default, AsAny, new, Packet)]
pub struct AnimationClientbound {
    entity_id: VarInt,
    animation: u8,
}

#[derive(Default, AsAny, new)]
pub struct Statistics {
    statistics: Vec<(VarInt, VarInt)>,
    value: VarInt,
}

impl Packet for Statistics {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.statistics.len() as i32);
        for stat in &self.statistics {
            buf.write_var_int(stat.0);
            buf.write_var_int(stat.1);
        }
        buf.write_var_int(self.value);
    }

    fn ty(&self) -> PacketType {
        PacketType::Statistics
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct BlockBreakAnimation {
    entity_id: VarInt,
    location: BlockPosition,
    destroy_stage: i8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UpdateBlockEntity {
    location: BlockPosition,
    action: u8,
    // TODO NBT
}

#[derive(Default, AsAny, new, Packet)]
pub struct BlockAction {
    location: BlockPosition,
    action_id: u8,
    action_param: u8,
    block_type: VarInt, // NOTE: block type ID, not the block state ID
}

#[derive(Default, AsAny, new, Packet)]
pub struct BlockChange {
    location: BlockPosition,
    block_id: VarInt,
}

#[derive(Default, AsAny, new)]
pub struct BossBar {
    uuid: Uuid,
    action: BossBarAction,
}

impl Packet for BossBar {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_uuid(&self.uuid);
        buf.write_var_int(self.action as i32);

        match self.action.clone() {
            BossBarAction::Add(title, health, color, division, flags) => {
                buf.write_string(&title);
                buf.write_f32_be(health);
                buf.write_var_int(color as i32);
                buf.write_var_int(division as i32);
                buf.write_u8(flags);
            }
            BossBarAction::Remove => (),
            BossBarAction::UpdateHealth(health) => {
                buf.write_f32_be(health);
            }
            BossBarAction::UpdateTitle(title) => {
                buf.write_string(&title);
            }
            BossBarAction::UpdateStyle(color, division) => {
                buf.write_var_int(color as i32);
                buf.write_var_int(division as i32);
            }
            BossBarAction::UpdateFlags(flags) => {
                buf.write_u8(flags);
            }
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::BossBar
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct ServerDifficulty {
    difficulty: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ChatMessageClientbound {
    json_data: String,
    position: u8,
}

#[derive(Clone, Debug)]
pub enum BossBarAction {
    Add(String, f32, BossBarColor, BossBarDivision, u8),
    Remove,
    UpdateHealth(f32),
    UpdateTitle(String),
    UpdateStyle(BossBarColor, BossBarDivision),
    UpdateFlags(u8),
}

impl Default for BossBarAction {
    fn default() -> Self {
        BossBarAction::Remove
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

impl Default for BossBarColor {
    fn default() -> Self {
        BossBarColor::Purple
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BossBarDivision {
    NoDivision,
    SixNotches,
    TenNotches,
    TwelveNotches,
    TwentyNotches,
}

impl Default for BossBarDivision {
    fn default() -> Self {
        BossBarDivision::NoDivision
    }
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
