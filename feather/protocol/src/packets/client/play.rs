use base::ValidBlockPosition;

use super::*;
use crate::packets::server::Hand;

packets! {
    TeleportConfirm {
        teleport_id VarInt;
    }

    QueryBlockNbt {
        transaction_id VarInt;
        position ValidBlockPosition;
    }

    QueryEntityNbt {
        transaction_id VarInt;
        entity_id VarInt;
    }

    SetDifficulty {
        new_difficulty u8;
    }

    ChatMessage {
        message String;
    }
}

def_enum! {
    ClientStatus (VarInt) {
        0 = PerformRespawn,
        1 = RequestStats,
    }
}

packets! {
    ClientSettings {
        locale String;
        view_distance u8;
        chat_mode ChatMode;
        chat_colors bool;
        displayed_skin_parts u8;
        main_hand VarInt;
    }
}

def_enum! {
    ChatMode (VarInt) {
        0 = Enabled,
        1 = CommandsOnly,
        2 = Hidden,
    }
}

packets! {
    TabComplete {
        transaction_id VarInt;
        text String;
    }

    WindowConfirmation {
        window_id u8;
        action_number u16;
        accepted bool;
    }

    ClickWindowButton {
        window_id u8;
        button_id u8;
    }

    ClickWindow {
        window_id u8;
        slot i16;
        button i8;
        action_number u16;
        mode VarInt;
        clicked_item Slot;
    }

    CloseWindow {
        window_id u8;
    }

    PluginMessage {
        channel String;
        data LengthInferredVecU8;
    }

    EditBook {
        new_book Slot;
        is_signing bool;
        hand VarInt;
    }

    InteractEntity {
        entity_id VarInt;
        kind InteractEntityKind;
        sneaking bool;
    }
}

def_enum! {
    InteractEntityKind (VarInt) {
        0 = Interact,
        1 = Attack,
        2 = InteractAt {
            target_x f32;
            target_y f32;
            target_z f32;
            hand VarInt;
        },
    }
}

packets! {
    GenerateStructure {
        position ValidBlockPosition;
        levels VarInt;
        keep_jigsaws bool;
    }

    KeepAlive {
        id u64;
    }

    LockDifficulty {
        locked bool;
    }

    PlayerPosition {
        x f64;
        feet_y f64;
        z f64;
        on_ground bool;
    }

    PlayerPositionAndRotation {
        x f64;
        feet_y f64;
        z f64;
        yaw f32;
        pitch f32;
        on_ground bool;
    }

    PlayerRotation {
        yaw f32;
        pitch f32;
        on_ground bool;
    }

    PlayerMovement {
        on_ground bool;
    }

    VehicleMove {
        x f64;
        y f64;
        z f64;
        yaw f32;
        pitch f32;
    }

    SteerBoat {
        left_paddle_turning bool;
        right_paddle_turning bool;
    }

    PickItem {
        slot VarInt;
    }

    CraftRecipeRequest {
        window_id u8;
        recipe String;
        make_all bool;
    }

    PlayerAbilities {
        flags u8;
    }

    SetDisplayedRecipe {
        recipe_id String;
    }

    SetRecipeBookState {
        book_id VarInt;
        book_open bool;
        filter_active bool;
    }

    PlayerDigging {
        status PlayerDiggingStatus;
        position ValidBlockPosition;
        face BlockFace;
    }
}

def_enum! {
    PlayerDiggingStatus (VarInt) {
        0 = StartDigging,
        1 = CancelDigging,
        2 = FinishDigging,
        3 = DropItemStack,
        4 = DropItem,
        5 = ShootArrow,
        6 = SwapItemInHand,
    }
}

def_enum! {
    BlockFace (u8) {
        0 = Bottom,
        1 = Top,
        2 = North,
        3 = South,
        4 = West,
        5 = East,
    }
}

packets! {
    EntityAction {
        entity_id VarInt;
        action_id EntityActionKind;
        jump_boost VarInt;
    }
}

def_enum! {
    EntityActionKind (VarInt) {
        0 = StartSneaking,
        1 = StopSneaking,
        2 = LeaveBed,
        3 = StartSprinting,
        4 = StopSprinting,
        5 = StartHorseJump,
        6 = StopJorseJump,
        7 = OpenHorseInventory,
        8 = StartElytraFlight,
    }
}

packets! {
    SteerVehicle {
        sideways f32;
        forward f32;
        flags u8;
    }
}

packets! {
    NameItem {
        name String;
    }

    ResourcePackStatus {
        result VarInt;
    }

    AdvancementTab {
        tab_id Option<String>;
    }

    SelectTrade {
        selected_slot VarInt;
    }

    SetBeaconEffect {
        primary_effect VarInt;
        secondary_effect VarInt;
    }

    HeldItemChange {
        slot u16;
    }

    UpdateCommandBlock {
        position ValidBlockPosition;
        command String;
        mode VarInt;
        flags u8;
    }

    UpdateCommandBlockMinecart {
        entity_id VarInt;
        command String;
        track_output bool;
    }

    CreativeInventoryAction {
        slot i16;
        clicked_item Slot;
    }

    UpdateJigsawBlock {
        position ValidBlockPosition;
        name String;
        target String;
        pool String;
        final_state String;
        joint_type String;
    }

    UpdateStructureBlock {
        position ValidBlockPosition;
        action VarInt;
        mode VarInt;
        name String;
        offset_x i8;
        offset_y i8;
        offset_z i8;
        size_x i8;
        size_y i8;
        size_z i8;
        mirror VarInt;
        rotation VarInt;
        metadata String;
        integrity f32;
        seed u64;
        flags u8;
    }

    UpdateSign {
        position ValidBlockPosition;
        line_1 String;
        line_2 String;
        line_3 String;
        line_4 String;
    }

    Animation {
        hand Hand;
    }

    Spectate {
        target_player Uuid;
    }

    PlayerBlockPlacement {
        hand VarInt;
        position ValidBlockPosition;
        face BlockFace;
        cursor_position_x f32;
        cursor_position_y f32;
        cursor_position_z f32;
        inside_block bool;
    }

    UseItem {
        hand VarInt;
    }
}
