#[allow(unused)]
pub mod implementation;

use crate::bytebuf::ByteBuf;
use std::any::Any;
use bytes::Buf;
use std::io::Read;

pub trait PacketBuf: Buf + Read {}
impl <T: Buf + Read> PacketBuf for T {}

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

lazy_static! {
    static ref PACKET_ID_MAPPINGS: im::HashMap<PacketId, PacketType> = {
        let mut m = im::HashMap::new();

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
            PacketId(0x25, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::JoinGame,
        );

        m.insert(
            PacketId(0x32, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::PlayerPositionAndLookClientbound,
        );

        m.insert(
            PacketId(0x49, PacketDirection::Clientbound, PacketStage::Play),
            PacketType::SpawnPosition,
        );

        m
    };
    static ref PACKET_TYPE_MAPPINGS: im::HashMap<PacketType, PacketId> = {
        let mut m = im::HashMap::new();

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
