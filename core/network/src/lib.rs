mod bytes_ext;
mod codec;
mod mctypes;
mod packet;
pub mod packets;

pub use codec::{Error, MinecraftCodec};
pub use packet::{Packet, PacketBuilder, PacketDirection, PacketId, PacketStage, PacketType};

pub fn cast_packet<P: packet::Packet + 'static + Send>(packet: Box<dyn Packet>) -> P {
    *packet.into_any().downcast().unwrap()
}
