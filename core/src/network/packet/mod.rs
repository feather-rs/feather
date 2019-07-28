#[allow(unused)]
pub mod implementation;

use crate::bytebuf::ByteBuf;
use bytes::Buf;
use hashbrown::HashMap;
use std::any::Any;
use std::io::Read;

pub trait PacketBuf: Buf + Read {}
impl<T: Buf + Read> PacketBuf for T {}

pub trait AsAny {
    fn as_any(&self) -> &Any;
}

pub trait Packet: AsAny + Send {
    fn read_from(&mut self, buf: &mut PacketBuf) -> Result<(), ()>;
    fn write_to(&self, buf: &mut ByteBuf);
    fn ty(&self) -> PacketType;
}

#[derive(Clone, Debug)]
pub struct PacketBuilder {
    pub init_fn: fn() -> Box<Packet>,
}

impl PacketBuilder {
    pub fn build(&self) -> Box<Packet> {
        let f = self.init_fn;
        f()
    }

    pub fn with(f: fn() -> Box<Packet>) -> Self {
        Self { init_fn: f }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum PacketType {
    // Serverbound

    // Handshake
    Handshake = 1,

    // Login
    LoginStart = 2,
    EncryptionResponse = 3,
    LoginPluginResponse = 4,

    // Play
    TeleportConfirm = 5,
    QueryBlockNBT = 6,
    ChatMessageServerbound = 7,
    ClientStatus = 8,
    ClientSettings = 9,
    TabCompleteServerbound = 10,
    ConfirmTransactionServerbound = 11,
    EnchantItem = 12,
    ClickWindow = 13,
    CloseWindowServerbound = 14,
    PluginMessageServerbound = 15,
    EditBook = 16,
    QueryEntityNBT = 17,
    UseEntity = 18,
    KeepAliveServerbound = 19,
    Player = 20,
    PlayerPosition = 21,
    PlayerPositionAndLookServerbound = 22,
    PlayerLook = 23,
    VehicleMoveServerbound = 24,
    SteerBoat = 25,
    PickItem = 26,
    CraftRecipeRequest = 27,
    PlayerAbilitiesServerbound = 28,
    PlayerDigging = 29,
    EntityAction = 30,
    SteerVehicle = 31,
    RecipeBookData = 32,
    NameItem = 33,
    ResourcePackStatus = 34,
    AdvancementTab = 35,
    SelectTrade = 36,
    SetBeaconEffect = 37,
    HeldItemChangeServerbound = 38,
    UpdateCommandBlock = 39,
    UpdateCommandBlockMinecart = 40,
    CreativeInventoryAction = 41,
    UpdateStructureBlock = 42,
    UpdateSign = 43,
    AnimationServerbound = 44,
    Spectate = 45,
    PlayerBlockPlacement = 46,
    UseItem = 47,

    // Status
    Request = 48,
    Ping = 49,

    // Clientbound

    // Handshake
    // (none)

    // Login
    DisconnectLogin = 50,
    EncryptionRequest = 51,
    LoginSuccess = 52,
    SetCompression = 53,
    LoginPluginRequest = 54,

    // Play
    SpawnObject = 55,
    SpawnExperienceOrb = 56,
    SpawnGlobalOrb = 57,
    SpawnGlobalEntity = 58,
    SpawnMob = 59,
    SpawnPainting = 60,
    SpawnPlayer = 61,
    AnimationClientbound = 62,
    Statistics = 63,
    BlockBreakAnimation = 64,
    UpdateBlockEntity = 65,
    BlockAction = 66,
    BlockChange = 67,
    BossBar = 68,
    ServerDifficulty = 69,
    ChatMessageClientbound = 70,
    MultiBlockChange = 71,
    TabCompleteClientbound = 72,
    DeclareCommands = 73,
    ConfirmTransactionClientbound = 74,
    CloseWindowClientbound = 75,
    OpenWindow = 76,
    WindowItems = 77,
    WindowProperty = 78,
    SetSlot = 79,
    SetCooldown = 80,
    PluginMessageClientbound = 81,
    NamedSoundEffect = 82,
    DisconnectPlay = 83,
    EntityStatus = 84,
    NBTQueryResponse = 85,
    Explosion = 86,
    UnloadChunk = 87,
    ChangeGameState = 88,
    KeepAliveClientbound = 89,
    ChunkData = 90,
    Effect = 91,
    Particle = 92,
    JoinGame = 93,
    MapData = 94,
    Entity = 95,
    EntityRelativeMove = 96,
    EntityLookAndRelativeMove = 97,
    EntityLook = 98,
    VehicleMoveClientbound = 99,
    OpenSignEditor = 100,
    CraftRecipeResponse = 101,
    PlayerAbilitiesClientbound = 102,
    CombatEvent = 103,
    PlayerInfo = 104,
    FacePlayer = 105,
    PlayerPositionAndLookClientbound = 106,
    UseBed = 107,
    UnlockRecipes = 108,
    DestroyEntities = 109,
    RemoveEntityEffect = 110,
    ResourcePackSend = 111,
    Respawn = 112,
    EntityHeadLook = 113,
    SelectAdvancementTab = 114,
    WorldBorder = 115,
    Camera = 116,
    HeldItemChangeClientbound = 117,
    DisplayScoreboard = 118,
    EntityMetadata = 119,
    AttachEntity = 120,
    EntityVelocity = 121,
    EntityEquipment = 122,
    SetExperience = 123,
    UpdateHealth = 124,
    ScoreboardObjective = 125,
    SetPassengers = 126,
    Teams = 127,
    UpdateScore = 128,
    SpawnPosition = 129,
    TimeUpdate = 130,
    StopSound = 131,
    SoundEffect = 132,
    PlayerListHeaderAndFooter = 133,
    CollectItem = 134,
    EntityTeleport = 135,
    Advancements = 136,
    EntityProperties = 137,
    EntityEffect = 138,
    DeclareRecipes = 139,
    Tags = 140,

    // Status
    Response = 141,
    Pong = 142,
}

lazy_static! {
    static ref PACKET_ID_MAPPINGS: HashMap<PacketId, PacketType> = {
        let mut m = HashMap::new();

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
            PacketId(0x1B, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::DisconnectPlay,
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
            PacketId(0x0B, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::BlockChange,
        );

        m.insert(
            PacketId(0x22, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::ChunkData,
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
            PacketId(0x39, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::EntityHeadLook,
        );

        m.insert(
            PacketId(0x49, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::SpawnPosition,
        );

        m
    };
    static ref PACKET_TYPE_MAPPINGS: HashMap<PacketType, PacketId> = {
        let mut m = HashMap::new();

        for (key, val) in PACKET_ID_MAPPINGS.clone().into_iter() {
            m.insert(val, key);
        }

        m
    };
}

impl PacketType {
    pub fn get_from_id(id: PacketId) -> Result<PacketType, ()> {
        PACKET_ID_MAPPINGS.get(&id).map(|v| v.clone()).ok_or(())
    }

    pub fn get_id(&self) -> PacketId {
        PACKET_TYPE_MAPPINGS.get(self).unwrap().clone()
    }

    pub fn get_implementation(&self) -> Box<Packet> {
        implementation::IMPL_MAP.get(self).unwrap().build()
    }

    /// Returns a unique ID, allocated
    /// consecutively for each packet type.
    pub fn ordinal(&self) -> usize {
        *self as usize
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
