use super::super::mctypes::{McTypeRead, McTypeWrite};
use super::*;
use crate::bytebuf::{BufMutAlloc, BufResulted};
use crate::prelude::*;
use crate::world::chunk::Chunk;
use bytes::{Buf, BufMut};
use hashbrown::HashMap;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::Read;
use std::io::Write;

type VarInt = i32;
type VarLong = i64;

lazy_static! {
    pub static ref IMPL_MAP: HashMap<PacketType, PacketBuilder> = {
        let mut m = HashMap::new();

        // Serverbound
        m.insert(PacketType::Handshake, PacketBuilder::with(|| Box::new(Handshake::default())));
        m.insert(PacketType::LoginStart, PacketBuilder::with(|| Box::new(LoginStart::default())));
        m.insert(PacketType::EncryptionResponse, PacketBuilder::with(|| Box::new(EncryptionResponse::default())));

        m.insert(PacketType::Request, PacketBuilder::with(|| Box::new(Request::default())));
        m.insert(PacketType::Ping, PacketBuilder::with(|| Box::new(Ping::default())));

        // Play
        m.insert(PacketType::JoinGame, PacketBuilder::with(|| Box::new(JoinGame::default())));
        m.insert(PacketType::TeleportConfirm, PacketBuilder::with(|| Box::new(TeleportConfirm::default())));
        m.insert(PacketType::QueryBlockNBT, PacketBuilder::with(|| Box::new(QueryBlockNBT::default())));
        m.insert(PacketType::ChatMessageServerbound, PacketBuilder::with(|| Box::new(ChatMessageServerbound::default())));
        m.insert(PacketType::ClientStatus, PacketBuilder::with(|| Box::new(ClientStatus::default())));
        m.insert(PacketType::ClientSettings, PacketBuilder::with(|| Box::new(ClientSettings::default())));
        m.insert(PacketType::TabCompleteServerbound, PacketBuilder::with(|| Box::new(TabCompleteServerbound::default())));
        m.insert(PacketType::ConfirmTransactionServerbound, PacketBuilder::with(|| Box::new(ConfirmTransactionServerbound::default())));
        m.insert(PacketType::EnchantItem, PacketBuilder::with(|| Box::new(EnchantItem::default())));
        m.insert(PacketType::ClickWindow, PacketBuilder::with(|| Box::new(ClickWindow::default())));
        m.insert(PacketType::CloseWindowServerbound, PacketBuilder::with(|| Box::new(CloseWindowServerbound::default())));
        m.insert(PacketType::PluginMessageServerbound, PacketBuilder::with(|| Box::new(PluginMessageServerbound::default())));
        m.insert(PacketType::EditBook, PacketBuilder::with(|| Box::new(EditBook::default())));
        m.insert(PacketType::QueryEntityNBT, PacketBuilder::with(|| Box::new(QueryEntityNBT::default())));
        m.insert(PacketType::UseEntity, PacketBuilder::with(|| Box::new(UseEntity::default())));
        m.insert(PacketType::KeepAliveServerbound, PacketBuilder::with(|| Box::new(KeepAliveServerbound::default())));
        m.insert(PacketType::Player, PacketBuilder::with(|| Box::new(Player::default())));
        m.insert(PacketType::PlayerPosition, PacketBuilder::with(|| Box::new(PlayerPosition::default())));
        m.insert(PacketType::PlayerPositionAndLookServerbound, PacketBuilder::with(|| Box::new(PlayerPositionAndLookServerbound::default())));
        m.insert(PacketType::PlayerLook, PacketBuilder::with(|| Box::new(PlayerLook::default())));
        m.insert(PacketType::VehicleMoveServerbound, PacketBuilder::with(|| Box::new(VehicleMoveServerbound::default())));
        m.insert(PacketType::SteerBoat, PacketBuilder::with(|| Box::new(SteerBoat::default())));
        m.insert(PacketType::PickItem, PacketBuilder::with(|| Box::new(PickItem::default())));
        m.insert(PacketType::CraftRecipeRequest, PacketBuilder::with(|| Box::new(CraftRecipeRequest::default())));
        m.insert(PacketType::PlayerAbilitiesServerbound, PacketBuilder::with(|| Box::new(PlayerAbilitiesServerbound::default())));
        m.insert(PacketType::PlayerDigging, PacketBuilder::with(|| Box::new(PlayerDigging::default())));
        m.insert(PacketType::EntityAction, PacketBuilder::with(|| Box::new(EntityAction::default())));
        m.insert(PacketType::SteerVehicle, PacketBuilder::with(|| Box::new(SteerVehicle::default())));
        m.insert(PacketType::RecipeBookData, PacketBuilder::with(|| Box::new(RecipeBookData::default())));
        m.insert(PacketType::NameItem, PacketBuilder::with(|| Box::new(NameItem::default())));
        m.insert(PacketType::ResourcePackStatus, PacketBuilder::with(|| Box::new(ResourcePackStatus::default())));
        m.insert(PacketType::AdvancementTab, PacketBuilder::with(|| Box::new(AdvancementTab::default())));
        m.insert(PacketType::SelectTrade, PacketBuilder::with(|| Box::new(SelectTrade::default())));
        m.insert(PacketType::SetBeaconEffect, PacketBuilder::with(|| Box::new(SetBeaconEffect::default())));
        m.insert(PacketType::HeldItemChangeServerbound, PacketBuilder::with(|| Box::new(HeldItemChangeServerbound::default())));
        m.insert(PacketType::UpdateCommandBlock, PacketBuilder::with(|| Box::new(UpdateCommandBlock::default())));
        m.insert(PacketType::UpdateCommandBlockMinecart, PacketBuilder::with(|| Box::new(UpdateCommandBlockMinecart::default())));
        m.insert(PacketType::CreativeInventoryAction, PacketBuilder::with(|| Box::new(CreativeInventoryAction::default())));
        m.insert(PacketType::UpdateStructureBlock, PacketBuilder::with(|| Box::new(UpdateStructureBlock::default())));
        m.insert(PacketType::UpdateSign, PacketBuilder::with(|| Box::new(UpdateSign::default())));
        m.insert(PacketType::AnimationServerbound, PacketBuilder::with(|| Box::new(AnimationServerbound::default())));
        m.insert(PacketType::Spectate, PacketBuilder::with(|| Box::new(Spectate::default())));
        m.insert(PacketType::PlayerBlockPlacement, PacketBuilder::with(|| Box::new(PlayerBlockPlacement::default())));
        m.insert(PacketType::UseItem, PacketBuilder::with(|| Box::new(UseItem::default())));

        m
    };
}

fn bla() {}

// SERVERBOUND

#[derive(Default, AsAny, new)]
pub struct Handshake {
    pub protocol_version: u32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

impl Packet for Handshake {
    fn read_from(&mut self, mut buf: &mut PacketBuf) -> Result<(), ()> {
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
    fn read_from(&mut self, mut buf: &mut PacketBuf) -> Result<(), ()> {
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

// PLAY
#[derive(Default, AsAny, new, Packet)]
pub struct TeleportConfirm {
    pub teleport_id: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct QueryBlockNBT {
    pub transaction_id: VarInt,
    pub location: BlockPosition,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ChatMessageServerbound {
    pub message: String, // Raw string, not a chat component
}

#[derive(Default, AsAny, new, Packet)]
pub struct ClientStatus {
    pub action_id: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ClientSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct TabCompleteServerbound {
    pub transaction_id: VarInt,
    pub text: String,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ConfirmTransactionServerbound {
    pub window_id: u8,
    pub action_number: u16,
    pub accepted: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct EnchantItem {
    pub window_id: u8,
    pub enchantment: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ClickWindow {
    pub window_id: u8,
    pub slot: u16,
    pub button: u8,
    pub action_number: i16,
    pub mode: VarInt,
    // TODO clicked_item: Slot,
}

#[derive(Default, AsAny, new, Packet)]
pub struct CloseWindowServerbound {
    pub window_id: u8,
}

#[derive(Default, AsAny, new)]
pub struct PluginMessageServerbound {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for PluginMessageServerbound {
    fn read_from(&mut self, mut buf: &mut PacketBuf) -> Result<(), ()> {
        self.channel = buf.read_string()?;

        let mut data = Vec::with_capacity(buf.remaining());
        buf.read(&mut data).map_err(|_| ())?;
        self.data = data;

        Ok(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::PluginMessageServerbound
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct EditBook {
    // TODO new_book: Slot
    pub is_signing: bool,
    pub hand: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct QueryEntityNBT {
    pub transaction_id: VarInt,
    pub entity_id: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UseEntity {
    pub target: VarInt,
    pub ty: VarInt, // TODO "only if type is interact at"
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
    pub hand: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct KeepAliveServerbound {
    pub id: i64,
}

#[derive(Default, AsAny, new, Packet)]
pub struct Player {
    pub on_ground: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerPosition {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerPositionAndLookServerbound {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct VehicleMoveServerbound {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SteerBoat {
    pub left_paddle_turning: bool,
    pub right_paddle_turning: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PickItem {
    pub slot_to_use: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct CraftRecipeRequest {
    pub window_id: i8,
    pub recipe: String,
    pub make_all: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerAbilitiesServerbound {
    pub flags: u8,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerDigging {
    pub status: VarInt,
    pub location: BlockPosition,
    pub face: i8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct EntityAction {
    pub entity_id: VarInt,
    pub action_id: VarInt,
    pub jump_boost: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SteerVehicle {
    pub sideways: f32,
    pub forward: f32,
    pub flags: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct RecipeBookData {
    pub ty: VarInt,
    // TODO
}

#[derive(Default, AsAny, new, Packet)]
pub struct NameItem {
    pub item_name: String,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ResourcePackStatus {
    pub result: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct AdvancementTab {
    pub action: VarInt,
    pub tab_id: String,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SelectTrade {
    pub selected_slot: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SetBeaconEffect {
    pub primary_effect: VarInt,
    pub secondary_effect: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct HeldItemChangeServerbound {
    pub slot: i16,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UpdateCommandBlock {
    pub location: BlockPosition,
    pub command: String,
    pub mode: VarInt,
    pub flags: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UpdateCommandBlockMinecart {
    pub entity_id: VarInt,
    pub command: String,
    pub track_output: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct CreativeInventoryAction {
    pub slot: u16,
    // TODO clicked_item: Slot
}

#[derive(Default, AsAny, new, Packet)]
pub struct UpdateStructureBlock {
    pub location: BlockPosition,
    pub action: VarInt,
    pub mode: VarInt,
    pub name: String,
    pub offset_x: i8,
    pub offset_y: i8,
    pub offset_z: i8,
    pub size_x: i8,
    pub size_y: i8,
    pub size_z: i8,
    pub mirror: VarInt,
    pub rotation: VarInt,
    pub metadata: String,
    pub integrity: f32,
    // TODO seed: VarLong,
    pub flags: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UpdateSign {
    pub location: BlockPosition,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub line_4: String,
}

#[derive(Default, AsAny, new, Packet)]
pub struct AnimationServerbound {
    pub hand: VarInt,
}

#[derive(Default, AsAny, new, Packet)]
pub struct Spectate {
    pub target_player: Uuid,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerBlockPlacement {
    pub location: BlockPosition,
    pub face: VarInt,
    pub hand: VarInt,
    pub cursor_position_x: f32,
    pub cursor_position_y: f32,
    pub cursor_positiom_z: f32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UseItem {
    pub hand: VarInt,
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
    fn read_from(&mut self, mut buf: &mut PacketBuf) -> Result<(), ()> {
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
    pub entity_id: VarInt,
    pub ty: u8,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnMob {
    pub entity_id: VarInt,
    pub entity_uuid: Uuid,
    pub ty: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
    pub head_pitch: u8,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
    // TODO metadata
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnPainting {
    pub entity_id: VarInt,
    pub entity_uuid: Uuid,
    pub motive: VarInt,
    pub location: BlockPosition,
    pub direction: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnPlayer {
    pub entity_id: VarInt,
    pub player_uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
    // TODO metadata
}

#[derive(Default, AsAny, new, Packet)]
pub struct AnimationClientbound {
    pub entity_id: VarInt,
    pub animation: u8,
}

#[derive(Default, AsAny, new)]
pub struct Statistics {
    pub statistics: Vec<(VarInt, VarInt)>,
    pub value: VarInt,
}

impl Packet for Statistics {
    fn read_from(&mut self, buf: &mut PacketBuf) -> Result<(), ()> {
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
    pub entity_id: VarInt,
    pub location: BlockPosition,
    pub destroy_stage: i8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct UpdateBlockEntity {
    pub location: BlockPosition,
    pub action: u8,
    // TODO NBT
}

#[derive(Default, AsAny, new, Packet)]
pub struct BlockAction {
    pub location: BlockPosition,
    pub action_id: u8,
    pub action_param: u8,
    pub block_type: VarInt, // NOTE: block type ID, not the block state ID
}

#[derive(Default, AsAny, new, Packet)]
pub struct BlockChange {
    pub location: BlockPosition,
    pub block_id: VarInt,
}

#[derive(Default, AsAny, new)]
pub struct BossBar {
    pub uuid: Uuid,
    pub action: BossBarAction,
}

impl Packet for BossBar {
    fn read_from(&mut self, buf: &mut PacketBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_uuid(&self.uuid);
        buf.write_var_int(self.action.id());

        match self.action.clone() {
            BossBarAction::Add(title, health, color, division, flags) => {
                buf.write_string(&title);
                buf.write_f32_be(health);
                buf.write_var_int(ToPrimitive::to_i32(&color).unwrap());
                buf.write_var_int(ToPrimitive::to_i32(&division).unwrap());
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
                buf.write_var_int(ToPrimitive::to_i32(&color).unwrap());
                buf.write_var_int(ToPrimitive::to_i32(&division).unwrap());
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

impl BossBarAction {
    fn id(&self) -> i32 {
        match &self {
            BossBarAction::Add(_, _, _, _, _) => 0,
            BossBarAction::Remove => 1,
            BossBarAction::UpdateHealth(_) => 2,
            BossBarAction::UpdateTitle(_) => 3,
            BossBarAction::UpdateStyle(_, _) => 4,
            BossBarAction::UpdateFlags(_) => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
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
pub struct ServerDifficulty {
    pub difficulty: u8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ChatMessageClientbound {
    pub json_data: String,
    pub position: u8,
}

// TODO MultiBlockChange
// TODO TabCompleteClientbound
// TODO DeclareCommands

#[derive(Default, AsAny, new, Packet)]
pub struct ConfirmTransactionClientbound {
    pub window_id: i8,
    pub action_number: i16,
    pub accepted: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct OpenWindow {
    pub window_id: u8,
    pub window_type: String,
    pub window_title: String, // Chat
    pub number_of_slots: u8,
    pub entity_id: i32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct WindowItems {
    pub window_id: u8,
    pub count: i16,
    // TODO slot data
}

#[derive(Default, AsAny, new, Packet)]
pub struct WindowProperty {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}

#[derive(Default, AsAny, new, Packet)]
pub struct SetSlot {
    pub window_id: i8,
    pub slot: i16,
    // TOOD slot data
}

#[derive(Default, AsAny, new, Packet)]
pub struct SetCooldown {
    pub item_id: VarInt,
    pub cooldown_ticks: VarInt,
}

// TODO PluginMessageClientbound

#[derive(Default, AsAny, new, Packet)]
pub struct NamedSoundEffect {
    pub sound_name: String,
    pub sound_category: VarInt,
    pub effect_pos_x: i32,
    pub effect_pos_y: i32,
    pub effect_pos_z: i32,
    pub volume: f32,
    pub pitch: f32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct DisconnectPlay {
    pub reason: String, // Chat
}

#[derive(Default, AsAny, new, Packet)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}

#[derive(Default, AsAny, new, Packet)]
pub struct NBTQueryResponse {
    pub transaction_id: VarInt,
    // TODO nbt
}

// TODO Explosion

#[derive(Default, AsAny, new, Packet)]
pub struct UnloadChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct ChangeGameState {
    pub reason: u8,
    pub value: f32,
}

#[derive(Default, AsAny, new, Packet)]
pub struct KeepAliveClientbound {
    pub keep_alive_id: u64,
}

#[derive(Default, AsAny, new)]
pub struct ChunkData {
    pub chunk: Chunk,
}

impl Packet for ChunkData {
    fn read_from(&mut self, buf: &mut PacketBuf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_i32_be(self.chunk.position().x);
        buf.write_i32_be(self.chunk.position().z);
        buf.write_bool(true); // Full chunk - assume true

        // Produce primary bit mask
        let mut primary_mask = {
            let mut r = 0;
            for (i, section) in self.chunk.sections().iter().enumerate() {
                if !section.is_empty() {
                    r |= 1 << i;
                }
            }
            r
        };

        buf.write_var_int(primary_mask as i32);

        let mut temp_buf = ByteBuf::new();

        for section in self.chunk.sections() {
            if section.is_empty() {
                continue;
            }

            temp_buf.write_u8(section.bits_per_block());

            let palette = section.palette();
            if !palette.global() {
                let data = palette.data();

                let mut palette_buf = ByteBuf::with_capacity(data.len() + 4);
                for val in data {
                    palette_buf.write_var_int((*val) as i32);
                }

                temp_buf.write_var_int(palette_buf.len() as i32);
                temp_buf.write(palette_buf.inner());
            }

            let data = section.data();
            temp_buf.write_var_int(data.len() as i32);

            temp_buf.reserve(data.len());
            for val in data {
                temp_buf.write_u64_be(*val);
            }

            // Light — TODO
            for _ in 0..4096 {
                temp_buf.write_u8(0b11111111);
            }
        }

        // Biomes
        // Just plains for now - TODO proper biome support
        temp_buf.reserve(256 * 4);
        for _ in 0..256 {
            temp_buf.write_i32_be(1); // 1 = plains
        }

        buf.write_var_int(temp_buf.len() as i32);
        buf.write(temp_buf.inner());

        buf.write_var_int(0) // Block entities — TODO
    }

    fn ty(&self) -> PacketType {
        PacketType::ChunkData
    }
}

#[derive(Default, AsAny, new, Packet)]
pub struct Effect {
    pub effect_id: i32,
    pub location: BlockPosition,
    pub data: i32,
    pub disable_relative_volume: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct Particle {
    pub particle_id: i32,
    pub long_distance: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub offset_x: f32,
    pub offset_z: f32,
    pub particle_data: f32,
    pub particle_count: i32,
    // TODO data
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

// TODO MapData

// TODO entity

#[derive(Default, AsAny, new, Packet)]
pub struct EntityRelativeMove {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct EntityLookAndRelativeMove {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}

#[derive(Default, AsAny, new, Packet)]
pub struct EntityLook {
    pub entity_id: VarInt,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}

/// ...

#[derive(Default, AsAny, new, Packet)]
pub struct SpawnPosition {
    pub location: BlockPosition,
}

#[derive(Default, AsAny, new, Packet)]
pub struct PlayerPositionAndLookClientbound {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: VarInt,
}
