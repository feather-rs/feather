use super::initialhandler::InitialHandler;
use crate::io::ServerToWorkerMessage;
use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet};
use mio_extras::channel::{Receiver, Sender};

pub struct PlayerHandle {
    initial_handler: InitialHandler,

    packet_sender: Sender<ServerToWorkerMessage>,
    packet_receiver: Receiver<ServerToWorkerMessage>,

    entity_id: i32,

    pub should_remove: bool,
}

impl PlayerHandle {
    pub fn accept_player_connection(
        packet_sender: Sender<ServerToWorkerMessage>,
        packet_receiver: Receiver<ServerToWorkerMessage>,
        motd: String,
        player_count: u32,
        max_players: i32,
        rsa_key: openssl::rsa::Rsa<openssl::pkey::Private>,
    ) -> Self {
        Self {
            initial_handler: InitialHandler::new(motd, player_count, max_players, rsa_key),

            packet_sender,

            entity_id: 0,

            packet_receiver,
            should_remove: false,
        }
    }

    pub fn send_packet<P: Packet + Send + 'static>(&self, packet: P) {
        self.packet_sender
            .send(ServerToWorkerMessage::SendPacket(Box::new(packet)))
            .unwrap();
    }

    pub fn send_packet_boxed(&self, packet: Box<Packet>) {
        self.packet_sender
            .send(ServerToWorkerMessage::SendPacket(packet))
            .unwrap();
    }

    pub fn close_connection(&mut self) {
        self.packet_sender
            .send(ServerToWorkerMessage::Disconnect)
            .unwrap();
    }

    pub fn tick(&mut self, server: &Server) {
        while let Ok(msg) = self.packet_receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                    self.handle_packet(packet, server)
                }
                ServerToWorkerMessage::NotifyDisconnect => {
                    trace!("Server removing player");
                    self.should_remove = true;
                }
                _ => unreachable!(),
            }
        }
    }

    fn handle_packet(&mut self, packet: Box<Packet>, server: &Server) {
        trace!("Handling packet");
        if !self.initial_handler.finished {
            let r = self.initial_handler.handle_packet(packet);
            if let Some(threshold) = self.initial_handler.should_enable_compression() {
                self.packet_sender
                    .send(ServerToWorkerMessage::EnableCompression(threshold))
                    .unwrap();
            }
            if let Some(key) = self.initial_handler.should_enable_encryption() {
                trace!("Server: enabling encryption");
                self.packet_sender
                    .send(ServerToWorkerMessage::EnableEncryption(key))
                    .unwrap();
            }

            for pack in self.initial_handler.packets_to_send() {
                self.send_packet_boxed(pack);
            }

            if self.initial_handler.should_disconnect || r.is_err() {
                self.should_remove = true;
                self.close_connection();
            }

            if self.initial_handler.finished {
                // Run the play sequence to allow the player
                // to join
                self.run_play_sequence(server);
            }
        } else {
            // TODO
        }
    }

    /// Sends the join packets, such as Join Game, Chunk
    /// Data, etc.
    fn run_play_sequence(&mut self, server: &Server) {
        let entity_id = server.allocate_entity_id();
        self.entity_id = entity_id;

        let join_game = JoinGame::new(
            entity_id,
            Gamemode::Survival.get_id(),
            Dimension::Overwold.get_id(),
            Difficulty::Hard.get_id(),
            0, // Unused value - max players
            "default".to_string(),
            false,
        );

        self.send_packet(join_game);
    }
}
