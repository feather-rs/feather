use crate::bytes_ext::{BytesExt, BytesMutExt};
use crate::mctypes::{EntityMetaRead, EntityMetaWrite, McTypeRead, McTypeWrite};
use crate::packet::{AsAny, PacketBuilder};
use crate::{Packet, PacketType};
use ahash::AHashMap;
use bytes::{Buf, BufMut, BytesMut};
use feather_blocks::{FacingCardinal, FacingCardinalAndDown, FacingCubic};
use feather_chunk::Chunk;
use feather_codegen::{AsAny, Packet};
use feather_entity_metadata::EntityMetadata;
use feather_items::ItemStack;
use feather_misc::ParticleData;
use feather_util::{BlockPosition, ClientboundAnimation, Gamemode, Hand};
use nbt::Blob;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::any::Any;
use std::convert::TryInto;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

type BlockFace = feather_blocks::Face;

type VarInt = i32;
type Slot = Option<ItemStack>;

macro_rules! insert_packet {
    ($map:ident, $ty:ident) => {
        $map.insert(
            PacketType::$ty,
            PacketBuilder::with(|| Box::new($ty::default())),
        );
    };
}

macro_rules! insert_packets {
    ($map:ident, $($ty:ident ,)+) => {
        $(insert_packet!($map, $ty));+
    }
}

pub static IMPL_MAP: Lazy<AHashMap<PacketType, PacketBuilder>> = Lazy::new(|| {
    let mut m = AHashMap::new();

    // Serverbound
    m.insert(
        PacketType::Handshake,
        PacketBuilder::with(|| Box::new(Handshake::default())),
    );
    m.insert(
        PacketType::LoginStart,
        PacketBuilder::with(|| Box::new(LoginStart::default())),
    );
    m.insert(
        PacketType::EncryptionResponse,
        PacketBuilder::with(|| Box::new(EncryptionResponse::default())),
    );

    m.insert(
        PacketType::Request,
        PacketBuilder::with(|| Box::new(Request::default())),
    );
    m.insert(
        PacketType::Ping,
        PacketBuilder::with(|| Box::new(Ping::default())),
    );

    // Play
    m.insert(
        PacketType::JoinGame,
        PacketBuilder::with(|| Box::new(JoinGame::default())),
    );
    m.insert(
        PacketType::TeleportConfirm,
        PacketBuilder::with(|| Box::new(TeleportConfirm::default())),
    );
    m.insert(
        PacketType::QueryBlockNBT,
        PacketBuilder::with(|| Box::new(QueryBlockNBT::default())),
    );
    m.insert(
        PacketType::ChatMessageServerbound,
        PacketBuilder::with(|| Box::new(ChatMessageServerbound::default())),
    );
    m.insert(
        PacketType::ClientStatus,
        PacketBuilder::with(|| Box::new(ClientStatus::default())),
    );
    m.insert(
        PacketType::ClientSettings,
        PacketBuilder::with(|| Box::new(ClientSettings::default())),
    );
    m.insert(
        PacketType::TabCompleteServerbound,
        PacketBuilder::with(|| Box::new(TabCompleteServerbound::default())),
    );
    m.insert(
        PacketType::ConfirmTransactionServerbound,
        PacketBuilder::with(|| Box::new(ConfirmTransactionServerbound::default())),
    );
    m.insert(
        PacketType::EnchantItem,
        PacketBuilder::with(|| Box::new(EnchantItem::default())),
    );
    m.insert(
        PacketType::ClickWindow,
        PacketBuilder::with(|| Box::new(ClickWindow::default())),
    );
    m.insert(
        PacketType::OpenWindow,
        PacketBuilder::with(|| Box::new(OpenWindow::default())),
    );
    m.insert(
        PacketType::CloseWindowServerbound,
        PacketBuilder::with(|| Box::new(CloseWindowServerbound::default())),
    );
    m.insert(
        PacketType::PluginMessageServerbound,
        PacketBuilder::with(|| Box::new(PluginMessageServerbound::default())),
    );
    m.insert(
        PacketType::EditBook,
        PacketBuilder::with(|| Box::new(EditBook::default())),
    );
    m.insert(
        PacketType::QueryEntityNBT,
        PacketBuilder::with(|| Box::new(QueryEntityNBT::default())),
    );
    m.insert(
        PacketType::UseEntity,
        PacketBuilder::with(|| Box::new(UseEntity::default())),
    );
    m.insert(
        PacketType::KeepAliveServerbound,
        PacketBuilder::with(|| Box::new(KeepAliveServerbound::default())),
    );
    m.insert(
        PacketType::Player,
        PacketBuilder::with(|| Box::new(Player::default())),
    );
    m.insert(
        PacketType::PlayerPosition,
        PacketBuilder::with(|| Box::new(PlayerPosition::default())),
    );
    m.insert(
        PacketType::PlayerPositionAndLookServerbound,
        PacketBuilder::with(|| Box::new(PlayerPositionAndLookServerbound::default())),
    );
    m.insert(
        PacketType::PlayerLook,
        PacketBuilder::with(|| Box::new(PlayerLook::default())),
    );
    m.insert(
        PacketType::VehicleMoveServerbound,
        PacketBuilder::with(|| Box::new(VehicleMoveServerbound::default())),
    );
    m.insert(
        PacketType::SteerBoat,
        PacketBuilder::with(|| Box::new(SteerBoat::default())),
    );
    m.insert(
        PacketType::PickItem,
        PacketBuilder::with(|| Box::new(PickItem::default())),
    );
    m.insert(
        PacketType::CraftRecipeRequest,
        PacketBuilder::with(|| Box::new(CraftRecipeRequest::default())),
    );
    m.insert(
        PacketType::PlayerAbilitiesServerbound,
        PacketBuilder::with(|| Box::new(PlayerAbilitiesServerbound::default())),
    );
    m.insert(
        PacketType::PlayerDigging,
        PacketBuilder::with(|| Box::new(PlayerDigging::default())),
    );
    m.insert(
        PacketType::EntityAction,
        PacketBuilder::with(|| Box::new(EntityAction::default())),
    );
    m.insert(
        PacketType::SteerVehicle,
        PacketBuilder::with(|| Box::new(SteerVehicle::default())),
    );
    m.insert(
        PacketType::RecipeBookData,
        PacketBuilder::with(|| Box::new(RecipeBookData::default())),
    );
    m.insert(
        PacketType::NameItem,
        PacketBuilder::with(|| Box::new(NameItem::default())),
    );
    m.insert(
        PacketType::ResourcePackStatus,
        PacketBuilder::with(|| Box::new(ResourcePackStatus::default())),
    );
    m.insert(
        PacketType::AdvancementTab,
        PacketBuilder::with(|| Box::new(AdvancementTab::default())),
    );
    m.insert(
        PacketType::SelectTrade,
        PacketBuilder::with(|| Box::new(SelectTrade::default())),
    );
    m.insert(
        PacketType::SetBeaconEffect,
        PacketBuilder::with(|| Box::new(SetBeaconEffect::default())),
    );
    m.insert(
        PacketType::HeldItemChangeServerbound,
        PacketBuilder::with(|| Box::new(HeldItemChangeServerbound::default())),
    );
    m.insert(
        PacketType::UpdateCommandBlock,
        PacketBuilder::with(|| Box::new(UpdateCommandBlock::default())),
    );
    m.insert(
        PacketType::UpdateCommandBlockMinecart,
        PacketBuilder::with(|| Box::new(UpdateCommandBlockMinecart::default())),
    );
    m.insert(
        PacketType::CreativeInventoryAction,
        PacketBuilder::with(|| Box::new(CreativeInventoryAction::default())),
    );
    m.insert(
        PacketType::UpdateStructureBlock,
        PacketBuilder::with(|| Box::new(UpdateStructureBlock::default())),
    );
    m.insert(
        PacketType::UpdateSign,
        PacketBuilder::with(|| Box::new(UpdateSign::default())),
    );
    m.insert(
        PacketType::AnimationServerbound,
        PacketBuilder::with(|| Box::new(AnimationServerbound::default())),
    );
    m.insert(
        PacketType::Spectate,
        PacketBuilder::with(|| Box::new(Spectate::default())),
    );
    m.insert(
        PacketType::PlayerBlockPlacement,
        PacketBuilder::with(|| Box::new(PlayerBlockPlacement::default())),
    );
    m.insert(
        PacketType::UseItem,
        PacketBuilder::with(|| Box::new(UseItem::default())),
    );

    // Clientbound

    m.insert(
        PacketType::EntityMetadata,
        PacketBuilder::with(|| Box::new(PacketEntityMetadata::default())),
    );

    insert_packets!(
        m,
        DisconnectLogin,
        EncryptionRequest,
        LoginSuccess,
        SetCompression,
        SpawnObject,
        SpawnExperienceOrb,
        SpawnGlobalEntity,
        SpawnMob,
        SpawnPainting,
        SpawnPlayer,
        AnimationClientbound,
        Statistics,
        BlockBreakAnimation,
        UpdateBlockEntity,
        BlockAction,
        BlockChange,
        BossBar,
        ServerDifficulty,
        ChatMessageClientbound,
        OpenWindow,
        WindowItems,
        WindowProperty,
        SetSlot,
        SetCooldown,
        PluginMessageClientbound,
        NamedSoundEffect,
        DisconnectPlay,
        EntityStatus,
        NBTQueryResponse,
        Explosion,
        UnloadChunk,
        ChangeGameState,
        KeepAliveClientbound,
        ChunkData,
        Effect,
        Particle,
        JoinGame,
        EntityRelativeMove,
        EntityLookAndRelativeMove,
        EntityLook,
        VehicleMoveClientbound,
        OpenSignEditor,
        CraftRecipeResponse,
        CombatEvent,
        PlayerInfo,
        PlayerPositionAndLookClientbound,
        UseBed,
        DestroyEntities,
        RemoveEntityEffect,
        ResourcePackSend,
        Respawn,
        EntityHeadLook,
        EntityVelocity,
        EntityEquipment,
        HeldItemChangeClientbound,
        UpdateHealth,
        SpawnPosition,
        TimeUpdate,
        CollectItem,
        EntityTeleport,
        Tags,
        Response,
        Pong,
    );

    m
});

macro_rules! box_clone_impl {
    ($this:ident) => {
        return Box::new((*$this).clone());
    };
}

#[derive(Clone, Copy, Error, Debug)]
pub enum Error {
    #[error("invalid face value {0}")]
    InvalidFace(i32),
    #[error("invalid hand value {0}")]
    InvalidHand(i32),
    #[error("invalid entity action type {0}")]
    InvalidEntityAction(i32),
    #[error("invalid player digging status {0}")]
    InvalidPlayerDiggingStatus(i32),
    #[error("invalid use entity value {0}")]
    InvalidUseEntity(i32),
    #[error("insufficient array length")]
    InsufficientArrayLength,
    #[error("invalid handshake next state {0}")]
    InvalidHandshakeState(i32),
}

// SERVERBOUND

#[derive(Default, AsAny, Clone)]
pub struct Handshake {
    pub protocol_version: u32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

impl Packet for Handshake {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.protocol_version = buf.try_get_var_int()? as u32;
        self.server_address = buf.try_get_string()?;
        self.server_port = buf.try_get_u16()?;
        let state = buf.try_get_var_int()?;

        self.next_state = match state {
            1 => HandshakeState::Status,
            2 => HandshakeState::Login,
            s => return Err(Error::InvalidHandshakeState(s).into()),
        };

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.protocol_version as i32);
        buf.push_string(&self.server_address);
        buf.push_u16(self.server_port);

        let state_id = match self.next_state {
            HandshakeState::Status => 1,
            HandshakeState::Login => 2,
        };
        buf.push_var_int(state_id);
    }

    fn ty(&self) -> PacketType {
        PacketType::Handshake
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::Handshake
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum HandshakeState {
    Status,
    Login,
}

impl Default for HandshakeState {
    fn default() -> Self {
        HandshakeState::Status
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct LoginStart {
    pub username: String,
}

#[derive(Default, AsAny, Clone)]
pub struct EncryptionResponse {
    pub secret_length: VarInt,
    pub secret: Vec<u8>,
    pub verify_token_length: VarInt,
    pub verify_token: Vec<u8>,
}

impl Packet for EncryptionResponse {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.secret_length = buf.try_get_var_int()?;

        let mut secret = vec![];
        for _ in 0..self.secret_length {
            secret.push(buf.get_u8());
        }
        self.secret = secret;

        self.verify_token_length = buf.try_get_var_int()?;

        let mut verify_token = vec![];
        for _ in 0..self.secret_length {
            verify_token.push(buf.get_u8());
        }

        self.verify_token = verify_token;
        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.secret.len() as i32);
        buf.put(self.secret.as_slice());
        buf.push_var_int(self.verify_token.len() as i32);
        buf.put(self.verify_token.as_slice());
    }

    fn ty(&self) -> PacketType {
        PacketType::EncryptionResponse
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::EncryptionResponse
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Request {}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Ping {
    pub payload: u64,
}

// PLAY
#[derive(Default, AsAny, Packet, Clone)]
pub struct TeleportConfirm {
    pub teleport_id: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct QueryBlockNBT {
    pub transaction_id: VarInt,
    pub location: BlockPosition,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ChatMessageServerbound {
    pub message: String, // Raw string, not a chat component
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ClientStatus {
    pub action_id: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ClientSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct TabCompleteServerbound {
    pub transaction_id: VarInt,
    pub text: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ConfirmTransactionServerbound {
    pub window_id: u8,
    pub action_number: u16,
    pub accepted: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EnchantItem {
    pub window_id: u8,
    pub enchantment: u8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ClickWindow {
    pub window_id: u8,
    pub slot: i16,
    pub button: u8,
    pub action_number: i16,
    pub mode: VarInt,
    pub clicked_item: Slot,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct CloseWindowServerbound {
    pub window_id: u8,
}

#[derive(Default, AsAny, Clone)]
pub struct PluginMessageServerbound {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for PluginMessageServerbound {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.channel = buf.try_get_string()?;

        let mut data = Vec::with_capacity(buf.remaining());
        buf.read(&mut data)
            .map_err(|_| Error::InsufficientArrayLength)?;
        self.data = data;

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_string(&self.channel);

        buf.put(self.data.as_slice());
    }

    fn ty(&self) -> PacketType {
        PacketType::PluginMessageServerbound
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::PluginMessageServerbound
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EditBook {
    pub new_book: Slot,
    pub is_signing: bool,
    pub hand: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct QueryEntityNBT {
    pub transaction_id: VarInt,
    pub entity_id: VarInt,
}

#[derive(Default, AsAny, Clone)]
pub struct UseEntity {
    pub target: VarInt,
    pub ty: UseEntityType,
}

impl Packet for UseEntity {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.target = buf.try_get_var_int()?;

        let ty_id = buf.try_get_var_int()?;
        self.ty = match ty_id {
            0 => UseEntityType::Interact,
            1 => UseEntityType::Attack,
            2 => {
                let x = buf.try_get_f32()?;
                let y = buf.try_get_f32()?;
                let z = buf.try_get_f32()?;
                let hand = buf.try_get_var_int()?;
                UseEntityType::InteractAt(x, y, z, hand)
            }
            i => return Err(Error::InvalidUseEntity(i).into()), // Invalid type
        };

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.target);

        let ty_id = match self.ty {
            UseEntityType::Interact => 0,
            UseEntityType::Attack => 1,
            UseEntityType::InteractAt(_, _, _, _) => 2,
        };
        buf.push_var_int(ty_id);

        if let UseEntityType::InteractAt(x, y, z, hand) = self.ty {
            buf.push_f32(x);
            buf.push_f32(y);
            buf.push_f32(z);
            buf.push_var_int(hand);
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::UseEntity
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::UseEntity
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(AsAny, Clone)]
pub enum UseEntityType {
    Interact,
    Attack,
    InteractAt(f32, f32, f32, VarInt),
}

impl Default for UseEntityType {
    fn default() -> Self {
        UseEntityType::Interact
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct KeepAliveServerbound {
    pub id: i64,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Player {
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct PlayerPosition {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct PlayerPositionAndLookServerbound {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct VehicleMoveServerbound {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SteerBoat {
    pub left_paddle_turning: bool,
    pub right_paddle_turning: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct PickItem {
    pub slot_to_use: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct CraftRecipeRequest {
    pub window_id: i8,
    pub recipe: String,
    pub make_all: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct PlayerAbilitiesServerbound {
    pub flags: u8,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

#[derive(AsAny, Clone, Default)]
pub struct PlayerDigging {
    pub status: PlayerDiggingStatus,
    pub location: BlockPosition,
    pub face: i8,
}

impl Packet for PlayerDigging {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.status = {
            let id = buf.try_get_var_int()?;
            match id {
                0 => PlayerDiggingStatus::StartedDigging,
                1 => PlayerDiggingStatus::CancelledDigging,
                2 => PlayerDiggingStatus::FinishedDigging,
                3 => PlayerDiggingStatus::DropItemStack,
                4 => PlayerDiggingStatus::DropItem,
                5 => PlayerDiggingStatus::ConsumeItem,
                6 => PlayerDiggingStatus::SwapItemInHand,
                i => return Err(Error::InvalidPlayerDiggingStatus(i).into()),
            }
        };

        self.location = buf.try_get_position()?;
        self.face = buf.try_get_i8()?;

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        let id = self.status as i32;
        buf.push_var_int(id);
        buf.push_position(&self.location);
        buf.push_i8(self.face);
    }

    fn ty(&self) -> PacketType {
        PacketType::PlayerDigging
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::PlayerDigging
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PlayerDiggingStatus {
    StartedDigging = 0,
    CancelledDigging = 1,
    FinishedDigging = 2,
    DropItemStack = 3,
    DropItem = 4,
    ConsumeItem = 5,
    SwapItemInHand = 6,
}

impl Default for PlayerDiggingStatus {
    fn default() -> Self {
        PlayerDiggingStatus::StartedDigging
    }
}

#[derive(Default, AsAny, Clone)]
pub struct EntityAction {
    pub entity_id: VarInt,
    pub action_id: EntityActionType,
    pub jump_boost: VarInt,
}

impl Packet for EntityAction {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.entity_id = buf.try_get_var_int()?;
        let action_id = buf.try_get_var_int()?;
        self.action_id =
            EntityActionType::from_i32(action_id).ok_or(Error::InvalidEntityAction(action_id))?;
        self.jump_boost = buf.try_get_var_int()?;

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.entity_id);
        let action_id = self.action_id.to_i32().unwrap();
        buf.push_var_int(action_id);
        buf.push_var_int(self.jump_boost);
    }

    fn ty(&self) -> PacketType {
        PacketType::EntityAction
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::EntityAction
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, FromPrimitive, ToPrimitive)]
pub enum EntityActionType {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    OpenHorseInventory,
    StartFlyingWithElytra,
}

impl Default for EntityActionType {
    fn default() -> Self {
        EntityActionType::StartSneaking
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SteerVehicle {
    pub sideways: f32,
    pub forward: f32,
    pub flags: u8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct RecipeBookData {
    pub ty: VarInt,
    // TODO
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct NameItem {
    pub item_name: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ResourcePackStatus {
    pub result: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct AdvancementTab {
    pub action: VarInt,
    pub tab_id: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SelectTrade {
    pub selected_slot: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SetBeaconEffect {
    pub primary_effect: VarInt,
    pub secondary_effect: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct HeldItemChangeServerbound {
    pub slot: i16,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct UpdateCommandBlock {
    pub location: BlockPosition,
    pub command: String,
    pub mode: VarInt,
    pub flags: u8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct UpdateCommandBlockMinecart {
    pub entity_id: VarInt,
    pub command: String,
    pub track_output: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct CreativeInventoryAction {
    pub slot: i16,
    pub clicked_item: Slot,
}

#[allow(clippy::too_many_arguments)]
#[derive(Default, AsAny, Packet, Clone)]
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

#[derive(Default, AsAny, Packet, Clone)]
pub struct UpdateSign {
    pub location: BlockPosition,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub line_4: String,
}

#[derive(Default, AsAny, Clone)]
pub struct AnimationServerbound {
    pub hand: Hand,
}

impl Packet for AnimationServerbound {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        let hand_id = buf.try_get_var_int()?;
        self.hand = match Hand::from_i32(hand_id) {
            Some(hand) => hand,
            None => return Err(Error::InvalidHand(hand_id).into()),
        };

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.hand.to_i32().unwrap());
    }

    fn ty(&self) -> PacketType {
        PacketType::AnimationServerbound
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::AnimationServerbound
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Spectate {
    pub target_player: Uuid,
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq)]
pub enum Face {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl Face {
    pub fn placement_offset(self) -> BlockPosition {
        match self {
            Face::Bottom => BlockPosition::new(0, -1, 0),
            Face::Top => BlockPosition::new(0, 1, 0),
            Face::North => BlockPosition::new(0, 0, -1),
            Face::South => BlockPosition::new(0, 0, 1),
            Face::West => BlockPosition::new(-1, 0, 0),
            Face::East => BlockPosition::new(1, 0, 0),
        }
    }
}

impl Face {
    pub fn face(self) -> BlockFace {
        match self {
            Face::Bottom => BlockFace::Ceiling,
            Face::Top => BlockFace::Floor,
            Face::North => BlockFace::Wall,
            Face::South => BlockFace::Wall,
            Face::West => BlockFace::Wall,
            Face::East => BlockFace::Wall,
        }
    }

    pub fn facing_cardinal(self) -> FacingCardinal {
        match self {
            Face::North => FacingCardinal::North,
            Face::South => FacingCardinal::South,
            Face::West => FacingCardinal::West,
            Face::East => FacingCardinal::East,
            Face::Top => panic!("Face::Top cannot be converted to FacingCardinal"),
            Face::Bottom => panic!("Face::Bottom cannot be converted to FacingCardinal"),
        }
    }

    pub fn facing_cardinal_and_down(self) -> FacingCardinalAndDown {
        match self {
            Face::North => FacingCardinalAndDown::North,
            Face::South => FacingCardinalAndDown::South,
            Face::West => FacingCardinalAndDown::West,
            Face::East => FacingCardinalAndDown::East,
            Face::Bottom => FacingCardinalAndDown::Down,
            Face::Top => panic!("Face::Top cannot be converted to FacingCardinalAndDown"),
        }
    }

    pub fn facing_cubic(self) -> FacingCubic {
        match self {
            Face::North => FacingCubic::North,
            Face::South => FacingCubic::South,
            Face::West => FacingCubic::West,
            Face::East => FacingCubic::East,
            Face::Top => FacingCubic::Up,
            Face::Bottom => FacingCubic::Down,
        }
    }
}

impl Default for Face {
    fn default() -> Self {
        Face::Bottom
    }
}

#[derive(Default, AsAny, Clone)]
pub struct PlayerBlockPlacement {
    pub location: BlockPosition,
    pub face: Face,
    pub hand: VarInt,
    pub cursor_position_x: f32,
    pub cursor_position_y: f32,
    pub cursor_position_z: f32,
}

impl Packet for PlayerBlockPlacement {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.location = buf.try_get_position()?;
        let face_id = buf.try_get_var_int()?;
        self.face = Face::from_i32(face_id).ok_or(Error::InvalidFace(face_id))?;
        self.hand = buf.try_get_var_int()?;
        self.cursor_position_x = buf.try_get_f32()?;
        self.cursor_position_y = buf.try_get_f32()?;
        self.cursor_position_z = buf.try_get_f32()?;
        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_position(&self.location);
        buf.push_var_int(self.face.to_i32().unwrap());
        buf.push_var_int(self.hand);
        buf.push_f32(self.cursor_position_x);
        buf.push_f32(self.cursor_position_y);
        buf.push_f32(self.cursor_position_z);
    }

    fn ty(&self) -> PacketType {
        PacketType::PlayerBlockPlacement
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::PlayerBlockPlacement
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct UseItem {
    pub hand: VarInt,
}

// CLIENTBOUND
#[derive(Default, AsAny, Packet, Clone)]
pub struct DisconnectLogin {
    pub reason: String,
}

#[derive(Default, AsAny, Clone)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl Packet for EncryptionRequest {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.server_id = buf.try_get_string()?;

        let pubkey_len = buf.try_get_var_int()?;
        for _ in 0..pubkey_len {
            self.public_key.push(buf.try_get_u8()?);
        }

        let token_len = buf.try_get_var_int()?;
        for _ in 0..token_len {
            self.verify_token.push(buf.try_get_u8()?);
        }

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_string(self.server_id.as_str());

        buf.push_var_int(self.public_key.len() as i32);
        buf.extend_from_slice(&self.public_key);

        buf.push_var_int(self.verify_token.len() as i32);
        buf.extend_from_slice(&self.verify_token);
    }

    fn ty(&self) -> PacketType {
        PacketType::EncryptionRequest
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::EncryptionRequest
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Response {
    pub json_response: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Pong {
    pub payload: u64,
}

// PLAY
#[allow(clippy::too_many_arguments)]
#[derive(Default, AsAny, Packet, Clone, Debug)]
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

#[derive(Default, AsAny, Packet, Clone)]
pub struct SpawnExperienceOrb {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SpawnGlobalEntity {
    pub entity_id: VarInt,
    pub ty: u8,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(clippy::too_many_arguments)]
#[derive(Default, AsAny, Packet, Clone)]
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
    pub meta: EntityMetadata,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SpawnPainting {
    pub entity_id: VarInt,
    pub entity_uuid: Uuid,
    pub motive: VarInt,
    pub location: BlockPosition,
    pub direction: u8,
}

#[allow(clippy::too_many_arguments)]
#[derive(AsAny, Clone, Default, Packet)]
pub struct SpawnPlayer {
    pub entity_id: VarInt,
    pub player_uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
    pub metadata: EntityMetadata,
}

#[derive(Default, AsAny, Clone)]
pub struct AnimationClientbound {
    pub entity_id: VarInt,
    pub animation: ClientboundAnimation,
}

impl Packet for AnimationClientbound {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.entity_id = buf.try_get_var_int()?;
        self.animation =
            ClientboundAnimation::from_u8(buf.try_get_u8()?).ok_or(Error::InvalidUseEntity(0))?;
        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.entity_id);
        buf.push_u8(self.animation as u8);
    }

    fn ty(&self) -> PacketType {
        PacketType::AnimationClientbound
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::AnimationClientbound
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Clone)]
pub struct Statistics {
    pub statistics: Vec<(VarInt, VarInt)>,
    pub value: VarInt,
}

impl Packet for Statistics {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        let num_statistics = buf.try_get_var_int()?;

        if num_statistics > 255 {
            return Err(Error::InsufficientArrayLength.into());
        }

        for _ in 0..num_statistics {
            self.statistics
                .push((buf.try_get_var_int()?, buf.try_get_var_int()?));
        }
        self.value = buf.try_get_var_int()?;
        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.statistics.len() as i32);
        for stat in &self.statistics {
            buf.push_var_int(stat.0);
            buf.push_var_int(stat.1);
        }
        buf.push_var_int(self.value);
    }

    fn ty(&self) -> PacketType {
        PacketType::Statistics
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::Statistics
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct BlockBreakAnimation {
    pub entity_id: VarInt,
    pub location: BlockPosition,
    pub destroy_stage: i8,
}

#[derive(AsAny, Packet, Clone)]
pub struct UpdateBlockEntity {
    pub location: BlockPosition,
    pub action: u8,
    pub data: Blob,
}

impl Default for UpdateBlockEntity {
    fn default() -> Self {
        Self {
            location: BlockPosition::default(),
            action: 0,
            data: Blob::new(),
        }
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct BlockAction {
    pub location: BlockPosition,
    pub action_id: u8,
    pub action_param: u8,
    pub block_type: VarInt, // NOTE: block type ID, not the block state ID
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct BlockChange {
    pub location: BlockPosition,
    pub block_id: VarInt,
}

#[derive(Default, AsAny, Clone)]
pub struct BossBar {
    pub uuid: Uuid,
    pub action: BossBarAction,
}

impl Packet for BossBar {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_uuid(&self.uuid);
        buf.push_var_int(self.action.id());

        match self.action.clone() {
            BossBarAction::Add(title, health, color, division, flags) => {
                buf.push_string(&title);
                buf.push_f32(health);
                buf.push_var_int(ToPrimitive::to_i32(&color).unwrap());
                buf.push_var_int(ToPrimitive::to_i32(&division).unwrap());
                buf.push_u8(flags);
            }
            BossBarAction::Remove => (),
            BossBarAction::UpdateHealth(health) => {
                buf.push_f32(health);
            }
            BossBarAction::UpdateTitle(title) => {
                buf.push_string(&title);
            }
            BossBarAction::UpdateStyle(color, division) => {
                buf.push_var_int(ToPrimitive::to_i32(&color).unwrap());
                buf.push_var_int(ToPrimitive::to_i32(&division).unwrap());
            }
            BossBarAction::UpdateFlags(flags) => {
                buf.push_u8(flags);
            }
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::BossBar
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::BossBar
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
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

#[derive(Default, AsAny, Packet, Clone)]
pub struct ServerDifficulty {
    pub difficulty: u8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ChatMessageClientbound {
    pub json_data: String,
    pub position: u8,
}

// TODO MultiBlockChange
// TODO TabCompleteClientbound
// TODO DeclareCommands

#[derive(Default, AsAny, Packet, Clone)]
pub struct ConfirmTransactionClientbound {
    pub window_id: i8,
    pub action_number: i16,
    pub accepted: bool,
}

#[derive(Default, AsAny, Clone)]
pub struct OpenWindow {
    pub window_id: u8,
    pub window_type: String,
    pub window_title: String, // Chat
    pub number_of_slots: u8,
    pub entity_id: Option<i32>,
}

impl Packet for OpenWindow {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.window_id = buf.try_get_u8()?;
        self.window_type = buf.try_get_string()?;
        self.window_title = buf.try_get_string()?;
        self.number_of_slots = buf.try_get_u8()?;

        self.entity_id = if self.window_type == "EntityHorse" {
            Some(buf.try_get_i32()?)
        } else {
            None
        };

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_u8(self.window_id);
        buf.push_string(&self.window_type);
        buf.push_string(&self.window_title);
        buf.push_u8(self.number_of_slots);

        if self.window_type == "EntityHorse" {
            buf.push_i32(self.entity_id.unwrap());
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::OpenWindow
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::OpenWindow
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Clone)]
pub struct WindowItems {
    pub window_id: u8,
    pub slots: Vec<Slot>,
}

impl Packet for WindowItems {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.window_id = buf.try_get_u8()?;
        let num_slots = buf.try_get_i16()?;

        for _ in 0..num_slots {
            self.slots.push(buf.try_get_slot()?);
        }

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_u8(self.window_id);
        buf.push_i16(self.slots.len() as i16);

        for slot in &self.slots {
            buf.push_slot(*slot);
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::WindowItems
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::WindowItems
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct WindowProperty {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SetSlot {
    pub window_id: i8,
    pub slot: i16,
    pub slot_data: Slot,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct SetCooldown {
    pub item_id: VarInt,
    pub cooldown_ticks: VarInt,
}

#[derive(Default, AsAny, Clone)]
pub struct PluginMessageClientbound {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for PluginMessageClientbound {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.channel = buf.try_get_string()?;

        self.data.extend_from_slice(*buf.get_ref());

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_string(&self.channel);
        buf.extend_from_slice(&self.data);
    }

    fn ty(&self) -> PacketType {
        PacketType::PluginMessageClientbound
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::PluginMessageClientbound
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct NamedSoundEffect {
    pub sound_name: String,
    pub sound_category: VarInt,
    pub effect_pos_x: i32,
    pub effect_pos_y: i32,
    pub effect_pos_z: i32,
    pub volume: f32,
    pub pitch: f32,
}

#[derive(Clone, Copy)]
pub enum SoundCategory {
    Master = 0,
    Music = 1,
    Records = 2,
    Weather = 3,
    Blocks = 4,
    Hostile = 5,
    Neutral = 6,
    Players = 7,
    Ambient = 8,
    Voice = 9,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct DisconnectPlay {
    pub reason: String, // Chat
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct NBTQueryResponse {
    pub transaction_id: VarInt,
    // TODO pub nbt: NbtTag,
}

#[allow(clippy::too_many_arguments)]
#[derive(Default, AsAny, Clone)]
pub struct Explosion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub records: Vec<(i8, i8, i8)>,
    pub player_motion_x: f32,
    pub player_motion_y: f32,
    pub player_motion_z: f32,
}

impl Packet for Explosion {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_f32(self.x);
        buf.push_f32(self.y);
        buf.push_f32(self.z);
        buf.push_f32(self.radius);

        buf.push_i32(self.records.len() as i32);

        for (x, y, z) in self.records.iter() {
            buf.push_i8(*x);
            buf.push_i8(*y);
            buf.push_i8(*z);
        }

        buf.push_f32(self.player_motion_x);
        buf.push_f32(self.player_motion_y);
        buf.push_f32(self.player_motion_z);
    }

    fn ty(&self) -> PacketType {
        PacketType::Explosion
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::Explosion
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct UnloadChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ChangeGameState {
    pub reason: u8,
    pub value: f32,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct KeepAliveClientbound {
    pub keep_alive_id: u64,
}

#[derive(Default, AsAny, Clone)]
pub struct ChunkData {
    pub chunk: ChunkHandle,
}

impl Packet for ChunkData {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        let chunk = self.chunk.read();
        buf.push_i32(chunk.position().x);
        buf.push_i32(chunk.position().z);
        buf.push_bool(true); // Full chunk - assume true

        // Produce primary bit mask
        let primary_mask = {
            let mut r = 0;
            for (i, section) in chunk.sections().iter().enumerate() {
                if section.is_some() {
                    r |= 1 << i;
                }
            }
            r
        };

        buf.push_var_int(primary_mask as i32);

        // TODO: approximate appropriate capacity
        let mut temp_buf = BytesMut::new();

        for section in chunk.sections() {
            if let Some(section) = section {
                temp_buf.push_u8(section.bits_per_block());

                let palette = section.palette();
                if let Some(palette) = palette {
                    let mut palette_buf = BytesMut::with_capacity(palette.len() + 4);
                    for block in palette {
                        palette_buf.push_var_int(i32::from(block.vanilla_id()));
                    }

                    temp_buf.push_var_int(palette.len() as i32);
                    temp_buf.extend_from_slice(&palette_buf);
                }

                let data = section.data();
                let data = data.inner();
                temp_buf.push_var_int(data.len() as i32);

                temp_buf.reserve(data.len());
                for val in data {
                    temp_buf.push_u64(*val);
                }

                // Light
                let sky_light_data = section.sky_light();
                let block_light_data = section.block_light();

                block_light_data
                    .inner()
                    .iter()
                    .chain(sky_light_data.inner().iter())
                    .for_each(|data| {
                        temp_buf.reserve(8);
                        temp_buf.put_u64_le(*data);
                    });
            }
        }

        // Biomes
        temp_buf.reserve(256 * 4);
        chunk
            .biomes()
            .iter()
            .map(|biome| biome.protocol_id())
            .for_each(|id| temp_buf.push_i32(id));

        buf.push_var_int(temp_buf.len() as i32);
        buf.extend_from_slice(&temp_buf);

        buf.push_var_int(0); // Block entities are sent separately
    }

    fn ty(&self) -> PacketType {
        PacketType::ChunkData
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::ChunkData
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Effect {
    pub effect_id: i32,
    pub location: BlockPosition,
    pub data: i32,
    pub disable_relative_volume: bool,
}

#[derive(Default, AsAny, Clone)]
pub struct Particle {
    pub long_distance: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub particle_data: f32,
    pub particle_count: i32,
    pub data: ParticleData,
}

impl Packet for Particle {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_i32(self.data.ordinal() as i32);
        buf.push_bool(self.long_distance);
        buf.push_f32(self.x);
        buf.push_f32(self.y);
        buf.push_f32(self.z);
        buf.push_f32(self.offset_x);
        buf.push_f32(self.offset_y);
        buf.push_f32(self.offset_z);
        buf.push_f32(self.particle_data);
        buf.push_i32(self.particle_count);
        match self.data {
            ParticleData::Block(id) => {
                buf.push_var_int(id.vanilla_id() as i32);
            }
            ParticleData::Dust {
                red,
                green,
                blue,
                scale,
            } => {
                buf.push_f32(red);
                buf.push_f32(green);
                buf.push_f32(blue);
                buf.push_f32(scale);
            }
            ParticleData::FallingDust(id) => {
                buf.push_var_int(id.vanilla_id() as i32);
            }
            ParticleData::Item(stack) => buf.push_slot(stack),
            _ => (),
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::Particle
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::Particle
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self)
    }
}

#[derive(Default, AsAny, Packet, Clone, Debug)]
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
// TODO EntityPacket

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityRelativeMove {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityLookAndRelativeMove {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityLook {
    pub entity_id: VarInt,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct VehicleMoveClientbound {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct OpenSignEditor {
    pub location: BlockPosition,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct CraftRecipeResponse {
    pub window_id: i8,
    pub recipe: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct PlayerAbilitiesClientbound {
    flags: u8,
    flying_speed: f32,
    field_of_view_modifier: f32,
}

#[derive(Default, AsAny, Clone)]
pub struct CombatEvent {
    pub event: CombatEventType,
}

impl Packet for CombatEvent {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        match &self.event {
            CombatEventType::EnterCombat => (),
            CombatEventType::EndCombat(duration, entity_id) => {
                buf.push_var_int(*duration);
                buf.push_i32(*entity_id);
            }
            CombatEventType::EntityDead(player_id, entity_id, message) => {
                buf.push_var_int(*player_id);
                buf.push_i32(*entity_id);
                buf.push_string(message);
            }
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::CombatEvent
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::CombatEvent
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Clone)]
pub enum CombatEventType {
    EnterCombat,
    EndCombat(VarInt, i32),
    EntityDead(VarInt, i32, String),
}

impl Default for CombatEventType {
    fn default() -> Self {
        CombatEventType::EnterCombat
    }
}

#[derive(AsAny, Clone, Default)]
pub struct PlayerInfo {
    pub action: PlayerInfoAction,
    pub uuid: Uuid,
}

impl Packet for PlayerInfo {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        let id = buf.try_get_var_int()?;
        let _ = buf.try_get_var_int()?;
        self.uuid = buf.try_get_uuid()?;

        self.action = match id {
            0 => {
                let name = buf.try_get_string()?;
                let num_props = buf.try_get_var_int()?;

                let mut props = vec![];
                for _ in 0..num_props {
                    let s0 = buf.try_get_string()?;
                    let s1 = buf.try_get_string()?;
                    let s2 = if buf.try_get_bool()? {
                        buf.try_get_string()?
                    } else {
                        String::default()
                    };
                    props.push((s0, s1, s2));
                }

                let gamemode = Gamemode::from_id(buf.try_get_var_int()? as u8);
                let ping = buf.try_get_var_int()?;
                let display_name = if buf.try_get_bool()? {
                    buf.try_get_string()?
                } else {
                    String::default()
                };

                PlayerInfoAction::AddPlayer(name, props, gamemode, ping, display_name)
            }
            1 => PlayerInfoAction::UpdateGamemode(Gamemode::from_id(buf.try_get_u8()?)),
            2 => PlayerInfoAction::UpdateLatency(buf.try_get_var_int()?),
            3 => {
                if buf.try_get_bool()? {
                    PlayerInfoAction::UpdateDisplayName(buf.try_get_string()?)
                } else {
                    PlayerInfoAction::UpdateDisplayName(String::default())
                }
            }
            _ => PlayerInfoAction::RemovePlayer,
        };

        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.action.id());
        buf.push_var_int(1); // Number of players - just use 1 for now

        buf.push_uuid(&self.uuid);

        match &self.action {
            PlayerInfoAction::AddPlayer(name, props, gamemode, ping, display_name) => {
                buf.push_string(name);
                buf.push_var_int(props.len() as i32);
                for prop in props {
                    buf.push_string(&prop.0);
                    buf.push_string(&prop.1);
                    buf.push_bool(true);
                    buf.push_string(&prop.2);
                }

                buf.push_var_int(i32::from(gamemode.id()));
                buf.push_var_int(*ping);
                buf.push_bool(true);
                buf.push_string(display_name);
            }
            PlayerInfoAction::UpdateGamemode(gamemode) => {
                buf.push_var_int(i32::from(gamemode.id()));
            }
            PlayerInfoAction::UpdateLatency(ping) => {
                buf.push_var_int(*ping);
            }
            PlayerInfoAction::UpdateDisplayName(display_name) => {
                buf.push_bool(true);
                buf.push_string(&display_name);
            }
            PlayerInfoAction::RemovePlayer => (),
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::PlayerInfo
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::PlayerInfo
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerInfoAction {
    AddPlayer(
        String,
        Vec<(String, String, String)>,
        Gamemode,
        VarInt,
        String,
    ),
    UpdateGamemode(Gamemode),
    UpdateLatency(VarInt),
    UpdateDisplayName(String),
    RemovePlayer,
}

impl PlayerInfoAction {
    pub fn id(&self) -> VarInt {
        match self {
            PlayerInfoAction::AddPlayer(_, _, _, _, _) => 0,
            PlayerInfoAction::UpdateGamemode(_) => 1,
            PlayerInfoAction::UpdateLatency(_) => 2,
            PlayerInfoAction::UpdateDisplayName(_) => 3,
            PlayerInfoAction::RemovePlayer => 4,
        }
    }
}

impl Default for PlayerInfoAction {
    fn default() -> Self {
        PlayerInfoAction::RemovePlayer
    }
}

// TODO Face Player

#[derive(Default, AsAny, Packet, Clone)]
pub struct PlayerPositionAndLookClientbound {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct UseBed {
    pub entity_id: VarInt,
    pub location: BlockPosition,
}

// TODO Unlock Recipes

#[derive(Default, AsAny, Clone)]
pub struct DestroyEntities {
    pub entity_ids: Vec<VarInt>,
}

impl Packet for DestroyEntities {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.entity_ids.len() as i32);

        for e in &self.entity_ids {
            buf.push_var_int(*e);
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::DestroyEntities
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::DestroyEntities
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct RemoveEntityEffect {
    pub entity_id: VarInt,
    pub effect_id: i8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct ResourcePackSend {
    pub url: String,
    pub hash: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct Respawn {
    pub dimension: i32,
    pub difficulty: u8,
    pub gamemode: u8,
    pub level_type: String,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityHeadLook {
    pub entity_id: VarInt,
    pub head_yaw: u8,
}

#[derive(Default, AsAny, Clone, Debug)]
pub struct PacketEntityMetadata {
    pub entity_id: VarInt,
    pub metadata: EntityMetadata,
}

impl Packet for PacketEntityMetadata {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        self.entity_id = buf.try_get_var_int()?;
        self.metadata = buf.try_get_metadata()?;
        Ok(())
    }

    fn write_to(&self, buf: &mut BytesMut) {
        buf.push_var_int(self.entity_id);
        buf.push_metadata(&self.metadata);
    }

    fn ty(&self) -> PacketType {
        PacketType::EntityMetadata
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::EntityMetadata
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityVelocity {
    pub entity_id: VarInt,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityEquipment {
    pub entity_id: VarInt,
    pub slot: VarInt,
    pub item: Slot,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct HeldItemChangeClientbound {
    pub slot: i8,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct UpdateHealth {
    pub health: f32,
    pub food: VarInt,
    pub saturation: f32,
}

// TODO Select Advancement Tab
// TODO World Border

#[derive(Default, AsAny, Packet, Clone)]
pub struct SpawnPosition {
    pub location: BlockPosition,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct TimeUpdate {
    pub world_age: i64,
    pub time_of_day: i64,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct CollectItem {
    pub collected: VarInt,
    pub collector: VarInt,
    pub count: VarInt,
}

#[derive(Default, AsAny, Packet, Clone)]
pub struct EntityTeleport {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}

#[derive(Default, AsAny, Clone)]
pub struct Tags {
    pub block_tags: Vec<(String, Vec<VarInt>)>,
    pub item_tags: Vec<(String, Vec<VarInt>)>,
    pub fluid_tags: Vec<(String, Vec<VarInt>)>,
}

impl Packet for Tags {
    fn read_from(&mut self, _buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut BytesMut) {
        for field in [&self.block_tags, &self.item_tags, &self.fluid_tags].iter_mut() {
            buf.push_var_int(field.len() as i32);
            for (identifier, entries) in field.iter() {
                buf.push_string(identifier.as_str());
                buf.push_var_int(entries.len().try_into().unwrap());
                for entry in entries {
                    buf.push_var_int(*entry);
                }
            }
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::Tags
    }

    fn ty_sized() -> PacketType
    where
        Self: Sized,
    {
        PacketType::Tags
    }

    fn box_clone(&self) -> Box<dyn Packet> {
        box_clone_impl!(self);
    }
}
