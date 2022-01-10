//! Packets sent from client to server.
use crate::VarInt;
use crate::VariantOf;
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
    0x02 = SetDifficulty,
    0x03 = ChatMessage,
    0x04 = ClientStatus,
    0x05 = ClientSettings,
    0x06 = TabComplete,
    0x07 = ClickWindowButton,
    0x08 = ClickWindow,
    0x09 = CloseWindow,
    0x0A = PluginMessage,
    0x0B = EditBook,
    0x0C = QueryEntityNbt,
    0x0D = InteractEntity,
    0x0E = GenerateStructure,
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
    0x1D = Pong,
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
