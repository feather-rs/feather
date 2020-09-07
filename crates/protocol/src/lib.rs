use base::ItemStack;

pub mod codec;
mod io;
pub mod packets;

#[doc(inline)]
pub use codec::MinecraftCodec;
pub use io::{Readable, Writeable};
#[doc(inline)]
pub use packets::{
    client::{ClientHandshakePacket, ClientLoginPacket, ClientPlayPacket, ClientStatusPacket},
    server::{ServerLoginPacket, ServerPlayPacket, ServerStatusPacket},
    VariantOf,
};

pub type Slot = Option<ItemStack>;

/// A protocol version.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProtocolVersion {
    V1_16_2,
}

/// Denotes a type which may be treated as a packet.
///
/// If you want to store arbitrary packets (e.g. for sending
/// over a channel), use [`Packet`](crate::Packet) instead,
/// as it does not require boxing.
pub trait PacketTrait: Readable + Writeable {
    /// Returns the ID of this packet for the given protocol version.
    fn id(version: ProtocolVersion) -> u32
    where
        Self: Sized;
}

/// Current state of the connection.
/// This state is updated during the login
/// sequence. See wiki.vg.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stage {
    Handshake,
    Status,
    Login,
    Play,
}

/// Direction in which a packet is sent.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PacketDirection {
    Clientbound,
    Serverbound,
}
