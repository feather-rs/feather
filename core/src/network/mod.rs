pub mod mctypes;
pub mod packet;
pub mod serialize;
pub mod world;

pub fn cast_packet<'a, P: packet::Packet + 'static + Send>(
    packet: &'a Box<packet::Packet>,
) -> &'a P {
    packet.as_any().downcast_ref().unwrap()
}
