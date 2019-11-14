pub mod codec;
pub mod mctypes;
pub mod packet;

pub fn cast_packet<P: packet::Packet + 'static + Send>(packet: Box<dyn packet::Packet>) -> P {
    packet.downcast().unwrap()
}
