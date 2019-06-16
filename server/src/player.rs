use crate::io::{ServerToWorkerMessage, NetworkIoManager};
use feather_core::network::{packet::{Packet, self, implementation::*}};
use feather_core::prelude::*;
use super::initialhandler::InitialHandler;
use mio_extras::channel::{Sender, Receiver};
use crate::prelude::*;

pub struct PlayerHandle {
    gamemode: Gamemode,
    initial_handler: InitialHandler,

    packet_sender: Sender<ServerToWorkerMessage>,
    packet_receiver: Receiver<ServerToWorkerMessage>,
}

impl PlayerHandle {
    pub fn accept_player_connection(
        server: &Server,
        packet_sender: Sender<ServerToWorkerMessage>,
        packet_receiver: Receiver<ServerToWorkerMessage>,
    ) {

    }

    pub fn send_packet<P: Packet + Send + 'static>(&self, packet: P) {
        self.packet_sender.send(packet).unwrap();
    }

    pub fn close_connection(self) {
        self.packet_sender.send(ServerToWorkerMessage::Disconnect);
    }
}