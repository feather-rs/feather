use crate::io::NewClientInfo;
use feather_core::network::packet::Packet;

pub struct InitialHandler {
    packets_to_send: Vec<Box<Packet>>,
    state: State,
    finished: bool,
}

impl InitialHandler {
    pub fn new() -> Self {
        Self {
            packets_to_send: vec![],
            state: State::Handshake,
            finished: false,
        }
    }
}

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
