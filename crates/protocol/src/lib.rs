use anyhow::anyhow;
use base::ItemStack;

pub mod codec;
mod io;
pub mod packets;

#[doc(inline)]
pub use codec::MinecraftCodec;
pub use io::Nbt;
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

/// A protocol state.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Play,
}

/// Reads an arbitrary packet sent by a client based on a dynamically-updated
/// protocol state. As opposed to `MinecraftCodec`, this struct does not type-encode
/// the current protocol state using generics.
///
/// This is a wrapper around a `MinecraftCodec` but more useful in certain sitations
/// (e.g. when writing a proxy.)
pub struct ClientPacketCodec {
    state: ProtocolState,
    codec: MinecraftCodec,
}

impl ClientPacketCodec {
    pub fn new() -> Self {
        Self {
            state: ProtocolState::Handshake,
            codec: MinecraftCodec::new(),
        }
    }

    pub fn set_state(&mut self, state: ProtocolState) {
        self.state = state
    }

    /// Decodes a `ClientPacket` using the provided data.
    pub fn decode(&mut self, data: &[u8]) -> anyhow::Result<Option<ClientPacket>> {
        match self.state {
            ProtocolState::Handshake => self
                .codec
                .decode::<ClientHandshakePacket>(data)
                .map(|opt| opt.map(ClientPacket::from)),
            ProtocolState::Status => self
                .codec
                .decode::<ClientStatusPacket>(data)
                .map(|opt| opt.map(ClientPacket::from)),
            ProtocolState::Login => self
                .codec
                .decode::<ClientLoginPacket>(data)
                .map(|opt| opt.map(ClientPacket::from)),
            ProtocolState::Play => self
                .codec
                .decode::<ClientPlayPacket>(data)
                .map(|opt| opt.map(ClientPacket::from)),
        }
    }

    /// Encodes a `ClientPacket` into a buffer.
    pub fn encode(&mut self, packet: &ClientPacket, buffer: &mut Vec<u8>) {
        match packet {
            ClientPacket::Handshake(packet) => self.codec.encode(packet, buffer),
            ClientPacket::Status(packet) => self.codec.encode(packet, buffer),
            ClientPacket::Login(packet) => self.codec.encode(packet, buffer),
            ClientPacket::Play(packet) => self.codec.encode(packet, buffer),
        }
    }
}

/// Similar to `ClientPacketCodec` but for server-sent packets.
pub struct ServerPacketCodec {
    state: ProtocolState,
    codec: MinecraftCodec,
}

impl ServerPacketCodec {
    pub fn new() -> Self {
        Self {
            state: ProtocolState::Handshake,
            codec: MinecraftCodec::new(),
        }
    }

    pub fn set_state(&mut self, state: ProtocolState) {
        self.state = state
    }

    /// Decodes a `ServerPacket` using the provided data.
    pub fn decode(&mut self, data: &[u8]) -> anyhow::Result<Option<ServerPacket>> {
        match self.state {
            ProtocolState::Handshake => Err(anyhow!("server sent data during handshake state")),
            ProtocolState::Status => self
                .codec
                .decode::<ServerStatusPacket>(data)
                .map(|opt| opt.map(ServerPacket::from)),
            ProtocolState::Login => self
                .codec
                .decode::<ServerLoginPacket>(data)
                .map(|opt| opt.map(ServerPacket::from)),
            ProtocolState::Play => self
                .codec
                .decode::<ServerPlayPacket>(data)
                .map(|opt| opt.map(ServerPacket::from)),
        }
    }

    /// Encodes a `ServerPacket` into a buffer.
    pub fn encode(&mut self, packet: &ServerPacket, buffer: &mut Vec<u8>) {
        match packet {
            ServerPacket::Status(packet) => self.codec.encode(packet, buffer),
            ServerPacket::Login(packet) => self.codec.encode(packet, buffer),
            ServerPacket::Play(packet) => self.codec.encode(packet, buffer),
        }
    }
}

/// A packet sent by the client from any one of the packet stages.
#[derive(Debug, Clone)]
pub enum ClientPacket {
    Handshake(ClientHandshakePacket),
    Status(ClientStatusPacket),
    Login(ClientLoginPacket),
    Play(ClientPlayPacket),
}

impl From<ClientHandshakePacket> for ClientPacket {
    fn from(packet: ClientHandshakePacket) -> Self {
        ClientPacket::Handshake(packet)
    }
}

impl From<ClientStatusPacket> for ClientPacket {
    fn from(packet: ClientStatusPacket) -> Self {
        ClientPacket::Status(packet)
    }
}

impl From<ClientLoginPacket> for ClientPacket {
    fn from(packet: ClientLoginPacket) -> Self {
        ClientPacket::Login(packet)
    }
}

impl From<ClientPlayPacket> for ClientPacket {
    fn from(packet: ClientPlayPacket) -> Self {
        ClientPacket::Play(packet)
    }
}

/// A packet sent by the server from any one of the packet stages.
#[derive(Debug, Clone)]
pub enum ServerPacket {
    Status(ServerStatusPacket),
    Login(ServerLoginPacket),
    Play(ServerPlayPacket),
}

impl From<ServerStatusPacket> for ServerPacket {
    fn from(packet: ServerStatusPacket) -> Self {
        ServerPacket::Status(packet)
    }
}

impl From<ServerLoginPacket> for ServerPacket {
    fn from(packet: ServerLoginPacket) -> Self {
        ServerPacket::Login(packet)
    }
}

impl From<ServerPlayPacket> for ServerPacket {
    fn from(packet: ServerPlayPacket) -> Self {
        ServerPacket::Play(packet)
    }
}
