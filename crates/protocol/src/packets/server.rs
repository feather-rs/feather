//! Packets sent from server to client;

use super::*;

mod login;
mod play;
mod status;

pub use login::*;
pub use play::*;
pub use status::*;

packet_enum!(ServerStatusPacket {
    0x00 = Response,
    0x01 = Pong,
});

packet_enum!(ServerLoginPacket {
    0x00 = DisconnectLogin,
    0x01 = EncryptionRequest,
    0x02 = LoginSuccess,
    0x03 = SetCompression,
    0x04 = LoginPluginRequest,
});
