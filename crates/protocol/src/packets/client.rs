//! Packets sent from client to server.

use super::*;

mod handshake;
mod login;
mod play;
mod status;

pub use handshake::*;
pub use login::*;
pub use play::*;
pub use status::*;

packet_enum!(ClientHandshakePacket {
    0x00 = Handshake,
});

packet_enum!(ClientStatusPacket {
    0x00 = Request,
    0x01 = Ping,
});

packet_enum!(ClientLoginPacket {
    0x00 = LoginStart,
    0x01 = EncryptionResponse,
    0x02 = LoginPluginResponse,
});

packet_enum!(ClientPlayPacket {
    0x00 = TeleportConfirm,
    0x01 = QueryBlockNbt,
    0x02 = QueryEntityNbt,
    0x03 = SetDifficulty,
    0x04 = ChatMessage,
    0x05 = ClientStatus,
    0x06 = ClientSettings,
    0x07 = TabComplete,
    0x08 = WindowConfirmation,
    0x09 = ClickWindowButton,
    0x0A = ClickWindow,
    0x0B = CloseWindow,
    0x0C = PluginMessage,
    0x0D = EditBook,
    0x0E = InteractEntity,
    0x0F = GenerateStructure,
    0x10 = KeepAlive,
    0x11 = LockDifficulty,
    0x12 = PlayerPosition,
    0x13 = PlayerPositionAndRotation,
    0x14 = PlayerRotation,
    0x15 = PlayerMovement,
    0x16 = VehicleMove,
    0x17 = SteerBoat,
    0x18 = PickItem,
    0x19 = CraftRecipeRequest,
    0x1A = PlayerAbilities,
    0x1B = PlayerDigging,
    0x1C = EntityAction,
    0x1D = SteerVehicle,
    0x1E = SetDisplayedRecipe,
    0x1F = SetRecipeBookState,
    0x20 = NameItem,
    0x21 = ResourcePackStatus,
    0x22 = AdvancementTab,
    0x23 = SelectTrade,
    0x24 = SetBeaconEffect,
    0x25 = HeldItemChange,
    0x26 = UpdateCommandBlock,
    0x27 = UpdateCommandBlockMinecart,
    0x28 = CreativeInventoryAction,
    0x29 = UpdateJigsawBlock,
    0x2A = UpdateStructureBlock,
    0x2B = UpdateSign,
    0x2C = Animation,
    0x2D = Spectate,
    0x2E = PlayerBlockPlacement,
    0x2F = UseItem,
});
