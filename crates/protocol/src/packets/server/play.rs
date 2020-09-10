use base::{EntityMetadata, ProfileProperty};

use super::*;
use crate::{Readable, Writeable};

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
pub struct AddPlayer {
    pub uuid: Uuid,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
    pub gamemode: Gamemode,
    pub ping: i32,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PlayerInfo {
    AddPlayers(Vec<AddPlayer>),
    UpdateGamemodes(Vec<(Uuid, Gamemode)>),
    UpdatePings(Vec<(Uuid, i32)>),
    UpdateDisplayNames(Vec<(Uuid, Option<String>)>),
    RemovePlayers(Vec<Uuid>),
}

impl Readable for PlayerInfo {
    fn read(
        buffer: &mut std::io::Cursor<&[u8]>,
        version: crate::ProtocolVersion,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let action = VarInt::read(buffer, version)?.0;
        let num_players = VarInt::read(buffer, version)?.0;

        match action {
            0 => {
                let mut vec = Vec::new();
                for _ in 0..num_players {
                    let uuid = Uuid::read(buffer, version)?;
                    let name = String::read(buffer, version)?;

                    let num_properties = VarInt::read(buffer, version)?;
                    let mut properties = Vec::new();
                    for _ in 0..num_properties.0 {
                        let name = String::read(buffer, version)?;
                        let value = String::read(buffer, version)?;
                        let signature = if bool::read(buffer, version)? {
                            String::read(buffer, version)?
                        } else {
                            String::new()
                        };
                        properties.push(ProfileProperty {
                            name,
                            value,
                            signature,
                        });
                    }

                    let gamemode = Gamemode::read(buffer, version)?;
                    let ping = VarInt::read(buffer, version)?.0;
                    let display_name = if bool::read(buffer, version)? {
                        Some(String::read(buffer, version)?)
                    } else {
                        None
                    };
                    vec.push(AddPlayer {
                        uuid,
                        name,
                        properties,
                        gamemode,
                        ping,
                        display_name,
                    })
                }
                Ok(PlayerInfo::AddPlayers(vec))
            }
            1 => {
                let mut vec = Vec::new();
                for _ in 0..num_players {
                    let uuid = Uuid::read(buffer, version)?;
                    let gamemode = Gamemode::read(buffer, version)?;
                    vec.push((uuid, gamemode));
                }
                Ok(PlayerInfo::UpdateGamemodes(vec))
            }
            2 => {
                let mut vec = Vec::new();
                for _ in 0..num_players {
                    let uuid = Uuid::read(buffer, version)?;
                    let ping = VarInt::read(buffer, version)?.0;
                    vec.push((uuid, ping));
                }
                Ok(PlayerInfo::UpdatePings(vec))
            }
            3 => {
                let mut vec = Vec::new();
                for _ in 0..num_players {
                    let uuid = Uuid::read(buffer, version)?;
                    let display_name = if bool::read(buffer, version)? {
                        Some(String::read(buffer, version)?)
                    } else {
                        None
                    };
                    vec.push((uuid, display_name));
                }
                Ok(PlayerInfo::UpdateDisplayNames(vec))
            }
            4 => {
                let mut vec = Vec::new();
                for _ in 0..num_players {
                    let uuid = Uuid::read(buffer, version)?;
                    vec.push(uuid);
                }
                Ok(PlayerInfo::RemovePlayers(vec))
            }
            x => Err(anyhow::anyhow!("invalid player info action '{}'", x)),
        }
    }
}

impl Writeable for PlayerInfo {
    fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) {
        let (action_id, num_players) = match self {
            PlayerInfo::AddPlayers(vec) => (0, vec.len()),
            PlayerInfo::UpdateGamemodes(vec) => (1, vec.len()),
            PlayerInfo::UpdatePings(vec) => (2, vec.len()),
            PlayerInfo::UpdateDisplayNames(vec) => (4, vec.len()),
            PlayerInfo::RemovePlayers(vec) => (5, vec.len()),
        };
        VarInt(action_id).write(buffer, version);
        VarInt(num_players as i32).write(buffer, version);

        match self {
            PlayerInfo::AddPlayers(vec) => {
                for action in vec {
                    action.uuid.write(buffer, version);
                    action.name.write(buffer, version);

                    VarInt(action.properties.len() as i32).write(buffer, version);
                    for prop in &action.properties {
                        prop.name.write(buffer, version);
                        prop.value.write(buffer, version);
                        true.write(buffer, version); // signature is present
                        prop.signature.write(buffer, version);
                    }

                    action.gamemode.write(buffer, version);
                    VarInt(action.ping).write(buffer, version);

                    action.display_name.is_some().write(buffer, version);
                    if let Some(display_name) = &action.display_name {
                        display_name.write(buffer, version);
                    }
                }
            }
            PlayerInfo::UpdateGamemodes(vec) => {
                for (uuid, gamemode) in vec {
                    uuid.write(buffer, version);
                    gamemode.write(buffer, version);
                }
            }
            PlayerInfo::UpdatePings(vec) => {
                for (uuid, ping) in vec {
                    uuid.write(buffer, version);
                    VarInt(*ping).write(buffer, version);
                }
            }
            PlayerInfo::UpdateDisplayNames(vec) => {
                for (uuid, display_name) in vec {
                    uuid.write(buffer, version);
                    display_name.is_some().write(buffer, version);
                    if let Some(display_name) = &display_name {
                        display_name.write(buffer, version);
                    }
                }
            }
            PlayerInfo::RemovePlayers(vec) => {
                for uuid in vec {
                    uuid.write(buffer, version)
                }
            }
        }
    }
}

packets! {
    FacePlayer {
        feet_or_eyes VarInt;
        target_x f64;
        target_y f64;
        target_z f64;
        entity Option<FacePlayerEntity>;
    }

    FacePlayerEntity {
        entity_id VarInt;
        feet_or_eyes VarInt;
    }

    PlayerPositionAndLook {
        x f64;
        y f64;
        z f64;
        yaw f32;
        pitch f32;
        flags u8;
        teleport_id VarInt;
    }

    // TODO: Unlock Recipes

    DestroyEntities {
        entity_ids LengthPrefixedVec<VarInt>;
    }

    RemoveEntityEffect {
        entity_id VarInt;
        effect_id u8;
    }

    ResourcePack {
        url String;
        hash String;
    }

    Respawn {
        dimension i32;
        hashed_seed u64;
        gamemode Gamemode;
        level_type String;
    }

    EntityHeadLook {
        entity_id VarInt;
        head_yaw Angle;
    }

    SelectAdvancementTab {
        identifier Option<String>;
    }
}

def_enum! {
    WorldBorder (VarInt) {
        0 = SetSize {
            diameter f64;
        },
        1 = LerpSize {
            old_diameter f64;
            new_diameter f64;
            speed u64;
        },
        2 = SetCenter {
            x f64;
            z f64;
        },
        3 = Initialize {
            x f64;
            z f64;
            old_diameter f64;
            new_diameter f64;
            speed u64;
            portal_teeport_boundary VarInt;
            warning_time VarInt;
            warning_blocks VarInt;
        },
        4 = SetWarningTime {
            warning_time VarInt;
        },
        5 = SetWarningBlocks {
            warning_blocks VarInt;
        },
    }
}

packets! {
    Camera {
        camera_id VarInt;
    }

    HeldItemChange {
        slot u8; // 0-8
    }

    UpdateViewPosition {
        chunk_x VarInt;
        chunk_z VarInt;
    }

    UpdateViewDistance {
        view_distance VarInt;
    }

    DisplayScoreboard {
        position u8;
        score_name String;
    }

    SendEntityMetadata {
        entity_id VarInt;
        metadata EntityMetadata;
    }

    AttachEntity {
        attached_entity_id i32;
        holding_entity_id i32;
    }

    EntityVelocity {
        entity_id VarInt;
        velocity_x i16;
        velocity_y i16;
        velocity_z i16;
    }

    EntityEquipment {
        entity_id VarInt;
        slot VarInt;
        item Slot;
    }

    SetExperience {
        experience_bar f32;
        level VarInt;
        total_experience VarInt;
    }

    UpdateHealth {
        health f32;
        food VarInt;
        food_saturation f32;
    }

    ScoreboardObjective {
        objective_name String;
        mode i8;
        objective_value Option<String>;
        kind Option<VarInt>;
    }

    SetPassengers {
        entity_id VarInt;
        passengers LengthPrefixedVec<VarInt>;
    }

    Teams {
        team_name String;
        mode TeamsMode;
    }
}

def_enum! {
    TeamsMode (i8) {
        0 = CreateTeam {
            display_name String;
            friendly_flags u8;
            name_tag_visibility String;
            collision_rule String;
            team_color VarInt;
            team_prefix String;
            team_suffix String;
            entities LengthPrefixedVec<String>; // usernames or UUIDs
        },
        1 = RemoveTeam,
        2 = UpdateTeamInfo {
            display_name String;
            friendly_flags u8;
            name_tag_visibility String;
            collision_rule String;
            team_color VarInt;
            team_prefix String;
            team_suffix String;
        },
        3 = AddEntitiesToTeam {
            entities LengthPrefixedVec<String>;
        },
        4 = RemoveEntitiesFromTeam {
            entities LengthPrefixedVec<String>;
        },
    }
}

packets! {
    UpdateScore {
        entity_name String;
        action u8;
        objective_name String;
        value Option<VarInt>;
    }

    SpawnPosition {
        position BlockPosition;
    }

    TimeUpdate {
        world_age u64;
        time_of_day u64;
    }
}

def_enum! {
    Title (VarInt) {
        0 = SetTitle {
            text String;
        },
        1 = SetSubtitle {
            text String;
        },
        2 = SetActionBar {
            text String;
        },
        3 = SetTimesAndDisplay {
            fade_in i32;
            stay i32;
            fade_out i32;
        },
        4 = Hide,
        5 = Reset,
    }
}

packets! {
    EntitySoundEffect {
        sound_id VarInt;
        sound_category VarInt;
        entity_id VarInt;
        volume f32;
        pitch f32;
    }

    SoundEffect {
        sound_id VarInt;
        sound_category VarInt;
        position_x i32;
        position_y i32;
        position_z i32;
        volume f32;
        pitch f32;
    }

    StopSound {
        flags u8;
        source Option<VarInt>;
        sound Option<String>;
    }

    PlayerListHeaderAndFooter {
        header String;
        footer String;
    }

    NbtQueryResponse {
        transaction_id VarInt;
        nbt Nbt<Blob>;
    }

    CollectItem {
        collected_entity_id VarInt;
        collector_entity_id VarInt;
        item_count VarInt;
    }

    EntityTeleport {
        entity_id VarInt;
        x f64;
        y f64;
        z f64;
        yaw Angle;
        pitch Angle;
        on_ground bool;
    }

    // TODO: Advancements, Entity Properties

    EntityEffect {
        entity_id VarInt;
        effect_id u8;
        amplifier i8;
        duration VarInt;
        flags u8;
    }

    DeclareRecipes {
        recipes LengthPrefixedVec<Recipe>;
    }

    Recipe {
        kind String;
        recipe_id String;
        // TODO: there's some data field?
    }

    AllTags {
        block_tags Tags;
        item_tags Tags;
        fluid_tags Tags;
        entity_tags Tags;
    }

    Tags {
        tag_name String;
        entries LengthPrefixedVec<VarInt>;
    }
}
