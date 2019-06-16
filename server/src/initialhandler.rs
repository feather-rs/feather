use crate::io::NewClientInfo;
use feather_core::network::packet::Packet;

pub struct InitialHandler {
    pub state: State,
    pub finished: bool,
}

impl InitialHandler {
    pub fn new() -> Self {
        Self {
            state: State::Handshake,
            finished: false,
        }
    }

    pub fn handle_packet(&mut self, packet: Box<Packet>) {
        if self.finished {
            return;
        }


    }
}

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
