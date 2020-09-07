use super::*;
use crate::Readable;

packets! {
    SpawnEntity {
        entity_id VarInt;
        uuid String;
        kind VarInt;
        x f64;
        y f64;
        z f64;
        pitch Angle;
        yaw Angle;
        data i32;
        velocity_x i16;
        velocity_y i16;
        velocity_z i16;
    }

    SpawnExperienceOrb {
        entity_id VarInt;
        x f64;
        y f64;
        z f64;
        count u16;
    }

    SpawnWeatherEntity {
        entity_id VarInt;
        // (always 1=thunderbolt)
        kind u8;
        x f64;
        y f64;
        z f64;
    }

    SpawnLivingEntity {
        entity_id VarInt;
        entity_uuid Uuid;
        kind VarInt;
        x f64;
        y f64;
        z f64;
        yaw Angle;
        pitch Angle;
        head_pitch Angle;
        velocity_x i16;
        velocity_y i16;
        velocity_z i16;
    }

    SpawnPainting {
        entity_id VarInt;
        entity_uuid Uuid;
        motive VarInt;
        location BlockPosition;
        direction PaintingDirection;
    }
}

def_enum! {
    PaintingDirection (i8) {
        0 = South,
        1 = West,
        2 = North,
        3 = East,
    }
}

packets! {
    SpawnPlayer {
        entity_id VarInt;
        player_uuid Uuid;
        x f64;
        y f64;
        z f64;
        yaw Angle;
        pitch Angle;
    }

    EntityAnimation {
        entity_id VarInt;
        animation Animation;
    }
}

def_enum! {
    Animation (u8) {
        0 = SwingMainArm,
        1 = TakeDamage,
        2 = LeaveBed,
        3 = SwingOffhand,
        4 = CriticalEffect,
        5 = MagicCriticalEffect,
    }
}

packets! {
    Statistics {
        statistics LengthPrefixedVec<Statistic>;
    }

    Statistic {
        category_id VarInt;
        statistic_id VarInt;
        value VarInt;
    }

    AcknowledgePlayerDigging {
        position BlockPosition;
        block BlockId;
        status PlayerDiggingStatus;
        successful bool;
    }
}

def_enum! {
    PlayerDiggingStatus (VarInt) {
        0 = Started,
        1 = Cancelled,
        2 = Finished,
    }
}

packets! {
    BlockBreakAnimation {
        entity_id VarInt;
        position BlockPosition;
        destroy_stage u8;
    }

    BlockEntityData {
        position BlockPosition;
        action u8;
        data Nbt<Blob>;
    }

    BlockAction {
        position BlockPosition;
        action_id u8;
        action_param u8;
        block_type VarInt;
    }

    BlockChange {
        position BlockPosition;
        block BlockId;
    }

    BossBar {
        uuid Uuid;
        action BossBarAction;
    }
}

def_enum! {
    BossBarAction (VarInt) {
        0 = Add {
            title String;
            health f32;
            color BossBarColor;
            division BossBarDivision;
            flags u8;
        },
        1 = Remove,
        2 = UpdateHealth { health f32 },
        3 = UpdateTitle { title String },
        4 = UpdateStyle { color BossBarColor; division BossBarDivision; },
        5 = UpdateFlags { flags u8; }
    }
}

def_enum! {
    BossBarColor (VarInt) {
        0 = Pink,
        1 = Blue,
        2 = Red,
        3 = Green,
        4 = Yellow,
        5 = Purple,
        6 = White,
    }
}

def_enum! {
    BossBarDivision (VarInt) {
        0 = None,
        1 = Notch6,
        2 = Notch10,
        3 = Notch12,
        4 = Notch20,
    }
}

packets! {
    ServerDifficulty {
        difficulty u8;
        locked bool;
    }

    ChatMessage {
        message String;
        position ChatPosition;
    }
}

def_enum! {
    ChatPosition (i8) {
        0 = Chat,
        1 = SystemMessage,
        2 = Hotbar,
    }
}

packets! {
    MultiBlockChange {
        chunk_x i32;
        chunk_z i32;
        records LengthPrefixedVec<BlockChangeRecord>;
    }

    BlockChangeRecord {
        horizontal_position u8;
        y_coordinate u8;
        block BlockId;
    }

    TabComplete {
        id VarInt;
        start VarInt;
        length VarInt;
        matches LengthPrefixedVec<TabCompleteMatch>;
    }

    TabCompleteMatch {
        value String;
        has_tooltip bool;
        tooltip Option<String>;
    }

    DeclareCommands {
        nodes LengthPrefixedVec<CommandNode>;
        root_index VarInt;
    }

    CommandNode {
        flags u8;
        children LengthPrefixedVec<VarInt>;
        redirect_node Option<VarInt>;
        name Option<String>;
        parser Option<String>;
        // TODO: handle properties, which vary depending on the value of `parser`.
        // This can be handled with an enum.
        __todo__ LengthInferredVecU8;
    }

    WindowConfirmation {
        window_id u8;
        action_number i16;
        is_accepted bool;
    }

    CloseWindow {
        window_id u8;
    }

    WindowItems {
        window_id u8;
        items LengthPrefixedVec<Slot>;
    }

    WindowProperty {
        window_id u8;
        property i16;
        value i16;
    }

    SetSlot {
        window_id u8;
        slot i16;
        slot_data Slot;
    }

    SetCooldown {
        item_id VarInt;
        cooldown_ticks VarInt;
    }

    PluginMessage {
        channel String;
        data LengthInferredVecU8;
    }

    NamedSoundEffect {
        name String;
        category VarInt;
        position_x i32;
        position_y i32;
        position_z i32;
        volume f32;
        pitch f32;
    }

    Disconnect {
        reason String;
    }

    EntityStatus {
        entity_id i32;
        status i8;
    }

    Explosion {
        x f32;
        y f32;
        z f32;
        strength f32;
        records LengthPrefixedVec<ExplosionRecord>;
        player_motion_x f32;
        player_motion_y f32;
        player_motion_z f32;
    }

    ExplosionRecord {
        x_offset i8;
        y_offset i8;
        z_offset i8;
    }

    UnloadChunk {
        chunk_x i32;
        chunk_z i32;
    }

    ChangeGameState {
        reason u8;
        value f32;
    }

    OpenHorseWindow {
        window_id u8;
        slot_count VarInt;
        entity_id i32;
    }

    KeepAlive {
        id i64;
    }

    // TODO: ChunkData

    Effect {
        effect_id i32;
        position BlockPosition;
        data i32;
        disable_relative_volume bool;
    }

    Particle {
        particle_id i32;
        long_distance bool;
        x f64;
        y f64;
        z f64;
        offset_x f32;
        offset_y f32;
        offset_z f32;
        particle_data f32;
        particle_count i32;
        // TODO: remaining data varies depending on the particle
        __todo__ LengthInferredVecU8;
    }

    // TODO: UpdateLight

    JoinGame {
        entity_id i32;
        gamemode Gamemode;
        dimension i32;
        hashed_seed u64;
        max_players u8;
        level_type String;
        view_distance VarInt;
        reduced_debug_info bool;
        enable_respawn_screen bool;
    }
}

def_enum! {
    Gamemode (u8) {
        0 = Survival,
        1 = Creative,
        2 = Adventure,
        3 = Specator,
    }
}

packets! {
    MapData {
        map_id VarInt;
        scale i8;
        show_tracking_position bool;
        is_locked bool;
        icons LengthPrefixedVec<Icon>;
        // TODO: a bunch of fields only if a Columns is set to 0
        __todo__ LengthInferredVecU8;
    }

    Icon {
        kind VarInt;
        x i8;
        z i8;
        direction i8;
        display_name Option<String>;
    }

    // TODO: TradeList

    EntityPosition {
        entity_id VarInt;
        delta_x i16;
        delta_y i16;
        delta_z i16;
        on_ground bool;
    }

    EntityPositionAndRotation {
        entity_id VarInt;
        delta_x i16;
        delta_y i16;
        delta_z i16;
        yaw Angle;
        pitch Angle;
        on_ground bool;
    }

    EntityRotation {
        entity_id VarInt;
        yaw Angle;
        pitch Angle;
        on_ground bool;
    }

    EntityMovement {
        entity_id VarInt;
    }

    VehicleMove {
        x f64;
        y f64;
        z f64;
        yaw f32;
        pitch f32;
    }

    OpenBook {
        hand Hand;
    }
}

def_enum! {
    Hand (VarInt) {
        0 = Main,
        1 = Off,
    }
}

packets! {
    OpenWindow {
        window_id VarInt;
        window_kind VarInt;
        window_title String;
    }

    OpenSignEditor {
        position BlockPosition;
    }

    CraftRecipeResponse {
        window_id i8;
        recipe String;
    }

    PlayerAbilities {
        flags u8;
        flying_speed f32;
        fov_modifier f32;
    }

    CombatEvent {
        event CombatEventKind;
    }
}

def_enum! {
    CombatEventKind (VarInt) {
        0 = EnterCombat,
        1 = EndCombat {
            duration VarInt;
            entity_id i32;
        },
        2 = EntityDead {
            player_id VarInt;
            entity_id i32;
            message String;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

pub enum PlayerInfoAction {
    AddPlayer {
        name: String,
        properties: Vec<ProfileProperty>,
        gamemode: Gamemode,
        ping: i32,
        display_name: Option<String>,
    },
    UpdateGamemode(Gamemode),
    UpdateLatency(i32),
    UpdateDisplayName(Option<String>),
    RemovePlayer,
}

pub struct PlayerInfo {}

impl Readable for PlayerInfo {
    fn read(
        buffer: &mut std::io::Cursor<&[u8]>,
        version: crate::ProtocolVersion,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let _action = VarInt::read(buffer, version)?.0;
        let _player_count = VarInt::read(buffer, version)?.0;

        todo!()
    }
}
