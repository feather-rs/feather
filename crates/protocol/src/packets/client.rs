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
    0x0F = KeepAlive,
    0x10 = LockDifficulty,
    0x11 = PlayerPosition,
    0x12 = PlayerPositionAndRotation,
    0x13 = PlayerRotation,
    0x14 = PlayerMovement,
    0x15 = VehicleMove,
    0x16 = SteerBoat,
    0x17 = PickItem,
    0x18 = CraftRecipeRequest,
    0x19 = PlayerAbilities,
    0x1A = PlayerDigging,
    0x1B = EntityAction,
    0x1C = SteerVehicle,
    0x1D = RecipeBookData,
    0x1E = NameItem,
    0x1F = ResourcePackStatus,
    0x20 = AdvancementTab,
    0x21 = SelectTrade,
    0x22 = SetBeaconEffect,
    0x23 = HeldItemChange,
    0x24 = UpdateCommandBlock,
    0x25 = UpdateCommandBlockMinecart,
    0x26 = CreativeInventoryAction,
    0x27 = UpdateJigsawBlock,
    0x28 = UpdateStructureBlock,
    0x29 = UpdateSign,
    0x2A = Animation,
    0x2B = Spectate,
    0x2C = PlayerBlockPlacement,
    0x2D = UseItem,
});
