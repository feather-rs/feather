mod bytes_ext;
mod codec;
mod mctypes;
mod packet;

pub use codec::{Error, MinecraftCodec};
pub use packet::implementation as packets;
pub use packet::{Packet, PacketBuilder, PacketDirection, PacketId, PacketStage, PacketType};

pub fn cast_packet<P: packet::Packet + 'static + Send>(packet: Box<dyn Packet>) -> P {
    *packet.into_any().downcast().unwrap()
}
