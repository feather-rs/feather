use lemio::ByteBuf;

pub trait Packet {
    fn get_type() -> PacketType;
    fn read_from(buf: ByteBuf) -> Self;
    fn write_to(&self, buf: ByteBuf);
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum PacketType {
    // Serverbound

    // Handshake
    Handshake,

    // Login
    LoginStart,
    EncryptionResponse,

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
    BlockAction,
    BlockChange,
    BossBar,
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
}

lazy_static! {
    static ref PACKET_ID_MAPPINGS: im::HashMap<PacketId, PacketType> = {

    };
}

fn insert_blas() {
    let mut m = im::HashMap::new();

    m.insert(PacketId(0x00, PacketDirection::Serverbound, PacketStage::Handshake), PacketType::Handshake);

    m.insert(PacketId(0x00, PacketDirection::Serverbound, PacketStage::Login), PacketType::LoginStart);
    m.insert(PacketId(0x01, PacketDirection::Serverbound, PacketStage::Login), PacketType::EncryptionResponse);

    m.insert(PacketId(0x00, PacketDirection::Serverbound, PacketStage::Play), PacketType::TeleportConfirm);
    m.insert(PacketId(0x01, PacketDirection::Serverbound, PacketStage::Play), PacketType::QueryBlockNBT);
    m.insert(PacketId(0x02, PacketDirection::Serverbound, PacketStage::Play), PacketType::ChatMessageServerbound);
    m.insert(PacketId(0x03, PacketDirection::Serverbound, PacketStage::Play), PacketType::ClientStatus);
    m.insert(PacketId(0x04, PacketDirection::Serverbound, PacketStage::Play), PacketType::ClientSettings);
    m.insert(PacketId(0x05, PacketDirection::Serverbound, PacketStage::Play), PacketType::TabCompleteServerbound);
}

/// Certain packets have the same ID as
/// another packet during a different login stage (blame Mojang),
/// so this struct is used to differentiate between packets like that.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct PacketId(u32, PacketDirection, PacketStage);

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
}
