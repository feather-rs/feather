use super::initialhandler::InitialHandler;
use crate::io::{NetworkIoManager, ServerToWorkerMessage};
use crate::prelude::*;
use feather_core::network::packet::{self, implementation::*, Packet};
use feather_core::prelude::*;
use mio_extras::channel::{Receiver, Sender};

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
        self.packet_sender
            .send(ServerToWorkerMessage::SendPacket(Box::new(packet)))
            .unwrap();
    }

    pub fn close_connection(self) {
        self.packet_sender.send(ServerToWorkerMessage::Disconnect);
    }
}
