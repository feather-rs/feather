use super::initialhandler::InitialHandler;
use crate::io::ServerToWorkerMessage;
use crate::prelude::*;
use feather_core::network::packet::Packet;
use mio_extras::channel::{Receiver, Sender};

pub struct PlayerHandle {
    initial_handler: InitialHandler,

    packet_sender: Sender<ServerToWorkerMessage>,
    packet_receiver: Receiver<ServerToWorkerMessage>,

    pub should_remove: bool,
}

impl PlayerHandle {
    pub fn accept_player_connection(
        packet_sender: Sender<ServerToWorkerMessage>,
        packet_receiver: Receiver<ServerToWorkerMessage>,
        motd: String,
        player_count: u32,
        max_players: i32,
    ) -> Self {
        Self {
            initial_handler: InitialHandler::new(motd, player_count, max_players),

            packet_sender,
            packet_receiver,
            should_remove: false,
        }
    }

    pub fn send_packet<P: Packet + Send + 'static>(&self, packet: P) {
        self.packet_sender
            .send(ServerToWorkerMessage::SendPacket(Box::new(packet)))
            .unwrap();
    }

    pub fn close_connection(&mut self) {
        self.packet_sender
            .send(ServerToWorkerMessage::Disconnect)
            .unwrap();
    }

    pub fn tick(&mut self, _server: &Server) {
        while let Ok(msg) = self.packet_receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => self.handle_packet(packet),
                _ => unimplemented!(),
            }
        }
    }

    fn handle_packet(&mut self, packet: Box<Packet>) {
        trace!("Handling packet");
        if !self.initial_handler.finished {
            let r = self.initial_handler.handle_packet(packet);
            if self.initial_handler.should_disconnect || r.is_err() {
                self.should_remove = true;
                self.close_connection();
            }
        } else {
            // TODO
        }
    }
}
