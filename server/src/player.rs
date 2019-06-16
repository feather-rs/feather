use io::{ServerToWorkerMessage, NetworkIoManager};
use feather_core::network::{packet::{Packet, self, implementation::*}};
use feather_core::prelude::*;
use super::initialhandler::InitialHandler;
use mio_extras::channel::{Sender, Receiver};

pub struct PlayerHandle {
    gamemode: Gamemode,
    initial_handler: InitialHandler,

    packet_sender: Sender<ServerToWorkerMessage>,
    packet_receiver: Receiver<ServerToWorkerMessage>,
}

impl PlayerHandle {
    pub fn accept_player_connection(
        server: &server,
        packet_sender: Sender<ServerToWorkerMessage>,
        packet_receiver: Receiver<ServerToWorkerMessage>,
    ) {

    }
}