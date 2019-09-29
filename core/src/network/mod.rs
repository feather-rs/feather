pub mod codec;
pub mod mctypes;
pub mod packet;

pub fn cast_packet<P: packet::Packet + 'static + Send>(packet: &dyn packet::Packet) -> &P {
    packet.as_any().downcast_ref().unwrap()
}
