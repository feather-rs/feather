use bytes::BytesMut;
use std::any::Any;
use std::io::Cursor;

use crate::packets::IMPL_MAP;
use ahash::AHashMap;
use num_derive::{FromPrimitive, ToPrimitive};
use once_cell::sync::Lazy;
use strum_macros::*;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait IntoAny {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T> IntoAny for T
where
    T: Any,
{
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub trait Packet: AsAny + IntoAny + Send + Sync + Any {
    fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()>;
    fn write_to(&self, buf: &mut BytesMut);
    fn ty(&self) -> PacketType;
    fn ty_sized() -> PacketType
    where
        Self: Sized;

    /// Returns a clone of this packet in a dynamic box.
    fn box_clone(&self) -> Box<dyn Packet>;
}

#[derive(Clone, Debug)]
pub struct PacketBuilder {
    pub init_fn: fn() -> Box<dyn Packet>,
}

impl PacketBuilder {
    pub fn build(&self) -> Box<dyn Packet> {
        let f = self.init_fn;
        f()
    }

    pub fn with(f: fn() -> Box<dyn Packet>) -> Self {
        Self { init_fn: f }
    }
}

#[derive(
    Debug, Hash, PartialEq, Eq, Copy, Clone, EnumCount, EnumIter, ToPrimitive, FromPrimitive,
)]
pub enum PacketType {
    // Serverbound

    // Handshake
    Handshake,

    // Login
    LoginStart,
    EncryptionResponse,
    LoginPluginResponse,

    // Play
    TeleportConfirm,
    QueryBlockNBT,
    ChatMessageServerbound,
    ClientStatus,
    ClientSettings,
    TabCompleteServerbound,
    ConfirmTransactionServerbound,
    EnchantItem,
    ClickWindow,
    CloseWindowServerbound,
    PluginMessageServerbound,
    EditBook,
    QueryEntityNBT,
    UseEntity,
    KeepAliveServerbound,
    Player,
    PlayerPosition,
    PlayerPositionAndLookServerbound,
    PlayerLook,
    VehicleMoveServerbound,
    SteerBoat,
    PickItem,
    CraftRecipeRequest,
    PlayerAbilitiesServerbound,
    PlayerDigging,
    EntityAction,
    SteerVehicle,
    RecipeBookData,
    NameItem,
    ResourcePackStatus,
    AdvancementTab,
    SelectTrade,
    SetBeaconEffect,
    HeldItemChangeServerbound,
    UpdateCommandBlock,
    UpdateCommandBlockMinecart,
    CreativeInventoryAction,
    UpdateStructureBlock,
    UpdateSign,
    AnimationServerbound,
    Spectate,
    PlayerBlockPlacement,
    UseItem,

    // Status
    Request,
    Ping,

    // Clientbound

    // Handshake
    // (none)

    // Login
    DisconnectLogin,
    EncryptionRequest,
    LoginSuccess,
    SetCompression,
    LoginPluginRequest,

    // Play
    SpawnObject,
    SpawnExperienceOrb,
    SpawnGlobalOrb,
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
    MultiBlockChange,
    TabCompleteClientbound,
    DeclareCommands,
    ConfirmTransactionClientbound,
    CloseWindowClientbound,
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
    MapData,
    Entity,
    EntityRelativeMove,
    EntityLookAndRelativeMove,
    EntityLook,
    VehicleMoveClientbound,
    OpenSignEditor,
    CraftRecipeResponse,
    PlayerAbilitiesClientbound,
    CombatEvent,
    PlayerInfo,
    FacePlayer,
    PlayerPositionAndLookClientbound,
    UseBed,
    UnlockRecipes,
    DestroyEntities,
    RemoveEntityEffect,
    ResourcePackSend,
    Respawn,
    EntityHeadLook,
    SelectAdvancementTab,
    WorldBorder,
    Camera,
    HeldItemChangeClientbound,
    DisplayScoreboard,
    EntityMetadata,
    AttachEntity,
    EntityVelocity,
    EntityEquipment,
    SetExperience,
    UpdateHealth,
    ScoreboardObjective,
    SetPassengers,
    Teams,
    UpdateScore,
    SpawnPosition,
    TimeUpdate,
    StopSound,
    SoundEffect,
    PlayerListHeaderAndFooter,
    CollectItem,
    EntityTeleport,
    Advancements,
    EntityProperties,
    EntityEffect,
    DeclareRecipes,
    Tags,

    // Status
    Response,
    Pong,
}

static PACKET_ID_MAPPINGS: Lazy<AHashMap<PacketId, PacketType>> = Lazy::new(|| {
    let mut m = AHashMap::new();

    m.insert(
        PacketId(0x00, PacketDirection::Serverbound, PacketStage::Handshake),
        PacketType::Handshake,
    );

    m.insert(
        PacketId(0x00, PacketDirection::Serverbound, PacketStage::Login),
        PacketType::LoginStart,
    );
    m.insert(
        PacketId(0x01, PacketDirection::Serverbound, PacketStage::Login),
        PacketType::EncryptionResponse,
    );
    m.insert(
        PacketId(0x02, PacketDirection::Serverbound, PacketStage::Login),
        PacketType::LoginPluginResponse,
    );

    m.insert(
        PacketId(0x00, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::TeleportConfirm,
    );
    m.insert(
        PacketId(0x01, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::QueryBlockNBT,
    );
    m.insert(
        PacketId(0x02, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::ChatMessageServerbound,
    );
    m.insert(
        PacketId(0x03, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::ClientStatus,
    );
    m.insert(
        PacketId(0x04, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::ClientSettings,
    );
    m.insert(
        PacketId(0x05, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::TabCompleteServerbound,
    );
    m.insert(
        PacketId(0x06, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::ConfirmTransactionServerbound,
    );
    m.insert(
        PacketId(0x07, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::EnchantItem,
    );
    m.insert(
        PacketId(0x08, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::ClickWindow,
    );
    m.insert(
        PacketId(0x09, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::CloseWindowServerbound,
    );
    m.insert(
        PacketId(0x0A, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PluginMessageServerbound,
    );
    m.insert(
        PacketId(0x0B, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::EditBook,
    );
    m.insert(
        PacketId(0x0C, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::QueryEntityNBT,
    );
    m.insert(
        PacketId(0x0D, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::UseEntity,
    );
    m.insert(
        PacketId(0x0E, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::KeepAliveServerbound,
    );
    m.insert(
        PacketId(0x0F, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::Player,
    );
    m.insert(
        PacketId(0x10, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PlayerPosition,
    );
    m.insert(
        PacketId(0x11, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PlayerPositionAndLookServerbound,
    );
    m.insert(
        PacketId(0x12, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PlayerLook,
    );
    m.insert(
        PacketId(0x13, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::VehicleMoveServerbound,
    );
    m.insert(
        PacketId(0x14, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::SteerBoat,
    );
    m.insert(
        PacketId(0x15, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PickItem,
    );
    m.insert(
        PacketId(0x16, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::CraftRecipeRequest,
    );
    m.insert(
        PacketId(0x17, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PlayerAbilitiesServerbound,
    );
    m.insert(
        PacketId(0x18, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PlayerDigging,
    );
    m.insert(
        PacketId(0x19, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::EntityAction,
    );
    m.insert(
        PacketId(0x1A, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::SteerVehicle,
    );
    m.insert(
        PacketId(0x1B, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::RecipeBookData,
    );
    m.insert(
        PacketId(0x1C, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::NameItem,
    );
    m.insert(
        PacketId(0x1D, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::ResourcePackStatus,
    );
    m.insert(
        PacketId(0x1E, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::AdvancementTab,
    );
    m.insert(
        PacketId(0x1F, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::SelectTrade,
    );
    m.insert(
        PacketId(0x20, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::SetBeaconEffect,
    );
    m.insert(
        PacketId(0x21, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::HeldItemChangeServerbound,
    );
    m.insert(
        PacketId(0x22, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::UpdateCommandBlock,
    );
    m.insert(
        PacketId(0x23, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::UpdateCommandBlockMinecart,
    );
    m.insert(
        PacketId(0x24, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::CreativeInventoryAction,
    );
    m.insert(
        PacketId(0x25, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::UpdateStructureBlock,
    );
    m.insert(
        PacketId(0x26, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::UpdateSign,
    );
    m.insert(
        PacketId(0x27, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::AnimationServerbound,
    );
    m.insert(
        PacketId(0x28, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::Spectate,
    );
    m.insert(
        PacketId(0x29, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::PlayerBlockPlacement,
    );
    m.insert(
        PacketId(0x2A, PacketDirection::Serverbound, PacketStage::Play),
        PacketType::UseItem,
    );

    m.insert(
        PacketId(0x00, PacketDirection::Serverbound, PacketStage::Status),
        PacketType::Request,
    );
    m.insert(
        PacketId(0x01, PacketDirection::Serverbound, PacketStage::Status),
        PacketType::Ping,
    );

    m.insert(
        PacketId(0x00, PacketDirection::Clientbound, PacketStage::Login),
        PacketType::DisconnectLogin,
    );
    m.insert(
        PacketId(0x01, PacketDirection::Clientbound, PacketStage::Login),
        PacketType::EncryptionRequest,
    );
    m.insert(
        PacketId(0x02, PacketDirection::Clientbound, PacketStage::Login),
        PacketType::LoginSuccess,
    );
    m.insert(
        PacketId(0x03, PacketDirection::Clientbound, PacketStage::Login),
        PacketType::SetCompression,
    );
    m.insert(
        PacketId(0x04, PacketDirection::Clientbound, PacketStage::Login),
        PacketType::LoginPluginRequest,
    );

    m.insert(
        PacketId(0x00, PacketDirection::Clientbound, PacketStage::Status),
        PacketType::Response,
    );

    m.insert(
        PacketId(0x01, PacketDirection::Clientbound, PacketStage::Status),
        PacketType::Pong,
    );

    m.insert(
        PacketId(0x00, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::SpawnObject,
    );

    m.insert(
        PacketId(0x02, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::SpawnGlobalEntity,
    );

    m.insert(
        PacketId(0x03, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::SpawnMob,
    );

    m.insert(
        PacketId(0x06, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::AnimationClientbound,
    );

    m.insert(
        PacketId(0x09, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::UpdateBlockEntity,
    );

    m.insert(
        PacketId(0x0A, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::BlockAction,
    );

    m.insert(
        PacketId(0x12, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::ConfirmTransactionClientbound,
    );
    m.insert(
        PacketId(0x14, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::OpenWindow,
    );
    m.insert(
        PacketId(0x15, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::WindowItems,
    );
    m.insert(
        PacketId(0x0E, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::ChatMessageClientbound,
    );

    m.insert(
        PacketId(0x17, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::SetSlot,
    );

    m.insert(
        PacketId(0x1A, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::NamedSoundEffect,
    );

    m.insert(
        PacketId(0x1B, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::DisconnectPlay,
    );

    m.insert(
        PacketId(0x1C, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityStatus,
    );

    m.insert(
        PacketId(0x1F, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::UnloadChunk,
    );

    m.insert(
        PacketId(0x21, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::KeepAliveClientbound,
    );

    m.insert(
        PacketId(0x05, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::SpawnPlayer,
    );

    m.insert(
        PacketId(0x08, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::BlockBreakAnimation,
    );

    m.insert(
        PacketId(0x0B, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::BlockChange,
    );

    m.insert(
        PacketId(0x20, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::ChangeGameState,
    );

    m.insert(
        PacketId(0x22, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::ChunkData,
    );

    m.insert(
        PacketId(0x23, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::Effect,
    );

    m.insert(
        PacketId(0x24, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::Particle,
    );

    m.insert(
        PacketId(0x25, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::JoinGame,
    );

    m.insert(
        PacketId(0x28, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityRelativeMove,
    );

    m.insert(
        PacketId(0x29, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityLookAndRelativeMove,
    );

    m.insert(
        PacketId(0x2A, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityLook,
    );

    m.insert(
        PacketId(0x30, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::PlayerInfo,
    );

    m.insert(
        PacketId(0x32, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::PlayerPositionAndLookClientbound,
    );

    m.insert(
        PacketId(0x35, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::DestroyEntities,
    );

    m.insert(
        PacketId(0x37, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::ResourcePackSend,
    );

    m.insert(
        PacketId(0x38, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::Respawn,
    );

    m.insert(
        PacketId(0x39, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityHeadLook,
    );

    m.insert(
        PacketId(0x3D, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::HeldItemChangeClientbound,
    );

    m.insert(
        PacketId(0x3F, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityMetadata,
    );

    m.insert(
        PacketId(0x41, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityVelocity,
    );

    m.insert(
        PacketId(0x42, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityEquipment,
    );

    m.insert(
        PacketId(0x44, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::UpdateHealth,
    );

    m.insert(
        PacketId(0x49, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::SpawnPosition,
    );

    m.insert(
        PacketId(0x4A, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::TimeUpdate,
    );

    m.insert(
        PacketId(0x4F, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::CollectItem,
    );

    m.insert(
        PacketId(0x50, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::EntityTeleport,
    );

    m.insert(
        PacketId(0x55, PacketDirection::Clientbound, PacketStage::Play),
        PacketType::Tags,
    );

    m
});

static PACKET_TYPE_MAPPINGS: Lazy<AHashMap<PacketType, PacketId>> = Lazy::new(|| {
    let mut m = AHashMap::new();

    for (key, val) in PACKET_ID_MAPPINGS.clone().into_iter() {
        m.insert(val, key);
    }

    m
});

impl PacketType {
    pub fn get_from_id(id: PacketId) -> Result<PacketType, ()> {
        PACKET_ID_MAPPINGS.get(&id).copied().ok_or(())
    }

    pub fn get_id(self) -> PacketId {
        *PACKET_TYPE_MAPPINGS.get(&self).unwrap_or_else(|| panic!("failed to find packet ID for packet type {:?} (try inserting it into the ID map in core/network/packet.rs)", self))
    }

    pub fn get_implementation(self) -> Box<dyn Packet> {
        IMPL_MAP.get(&self).unwrap().build()
    }

    /// Returns a unique ID, allocated
    /// consecutively for each packet type.
    pub fn ordinal(self) -> usize {
        self as usize
    }
}

/// Certain packets have the same ID as
/// another packet during a different login stage (blame Mojang),
/// so this struct is used to differentiate between packets like that.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct PacketId(pub u32, pub PacketDirection, pub PacketStage);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum PacketStage {
    Handshake,
    Status,
    Login,
    Play,
}
