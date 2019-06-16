use crate::io::NewClientInfo;
use feather_core::network::packet::Packet;

pub struct InitialHandler {
    state: State,
    finished: bool,
}

impl InitialHandler {
    pub fn new() -> Self {
        Self {
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
