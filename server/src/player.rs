use super::initialhandler as ih;
use super::initialhandler::InitialHandler;
use crate::io::ServerToWorkerMessage;
use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use mio_extras::channel::{Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_KEEP_ALIVE_TIME: u64 = 30;

pub struct PlayerHandle {
    initial_handler: InitialHandler,

    packet_sender: Sender<ServerToWorkerMessage>,
    packet_receiver: Receiver<ServerToWorkerMessage>,

    entity_id: i32,

    pub should_remove: bool,

    /// The last time a keep alive packet
    /// was received from the client. If this
    /// value exceeds MAX_KEEP_ALIVE_TIME seconds,
    /// the client should be disconnected with a "time out"
    /// error as per the protocol specification.
    last_keep_alive_time: u64,
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

            last_keep_alive_time: current_time_in_secs(),
        }
    }

    pub fn send_packet<P: Packet + Send + 'static>(&self, packet: P) -> Result<(), ()> {
        self.packet_sender
            .send(ServerToWorkerMessage::SendPacket(Box::new(packet)))
            .map_err(|_| ())
    }

    pub fn send_packet_boxed(&self, packet: Box<Packet>) -> Result<(), ()> {
        self.packet_sender
            .send(ServerToWorkerMessage::SendPacket(packet))
            .map_err(|_| ())
    }

    pub fn close_connection(&self) {
        let _ = self.packet_sender.send(ServerToWorkerMessage::Disconnect);
    }

    pub fn disconnect(&mut self, reason: &str) {
        self.should_remove = true;
        // TODO send Disconnect packet
        self.close_connection();
    }

    pub fn tick(&mut self, server: &Server) -> Result<(), ()> {
        while let Ok(msg) = self.packet_receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                    self.handle_packet(packet, server)?;
                }
                ServerToWorkerMessage::NotifyDisconnect => {
                    trace!("Server removing player");
                    self.should_remove = true;
                    return Ok(());
                }
                _ => unreachable!(),
            }
        }

        if server.tick_count() % TPS == 0 && self.initial_handler.finished {
            // Send keep alive every second as per the protocol specification
            let keep_alive = KeepAliveClientbound::new(0);
            self.send_packet(keep_alive)?;

            if current_time_in_secs() - self.last_keep_alive_time >= MAX_KEEP_ALIVE_TIME {
                self.disconnect("Timed out");
            }
        }

        Ok(())
    }

    fn handle_packet(&mut self, packet: Box<Packet>, server: &Server) -> Result<(), ()> {
        trace!("Handling packet");
        if !self.initial_handler.finished {
            self.forward_packet_to_initial_handler(packet, server)?;
        } else {
            // TODO perhaps use HashMap instead of match here?
            match packet.ty() {
                PacketType::KeepAliveServerbound => self.handle_keep_alive(cast_packet::<KeepAliveServerbound>(&packet)),
                _ => (), // TODO
            }
        }

        Ok(())
    }

    fn forward_packet_to_initial_handler(
        &mut self,
        packet: Box<Packet>,
        server: &Server,
    ) -> Result<(), ()> {
        let r = self.initial_handler.handle_packet(packet);

        if r.is_err() {
            self.should_remove = true;
            self.close_connection();
        }

        for action in self.initial_handler.actions() {
            match action {
                ih::Action::SendPacket(packet) => self.send_packet_boxed(packet)?,
                ih::Action::EnableEncryption(key) => {
                    self.packet_sender
                        .send(ServerToWorkerMessage::EnableEncryption(key))
                        .map_err(|_| ())?;
                }
                ih::Action::EnableCompression(threshold) => {
                    self.packet_sender
                        .send(ServerToWorkerMessage::EnableCompression(threshold))
                        .map_err(|_| ())?;
                }
            }
        }

        if self.initial_handler.finished {
            // Run the play sequence to allow the player
            // to join
            self.run_play_sequence(server)?;
        }

        Ok(())
    }

    /// Sends the join packets, such as Join Game, Chunk
    /// Data, etc.
    fn run_play_sequence(&mut self, server: &Server) -> Result<(), ()> {
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

        self.send_packet(join_game)?;

        let spawn_position = SpawnPosition::new(BlockPosition::new(0, 64, 0));
        self.send_packet(spawn_position)?;

        let position_and_look =
            PlayerPositionAndLookClientbound::new(0.0, 64.0, 0.0, 0.0, 0.0, 0, 0);
        self.send_packet(position_and_look)?;

        Ok(())
    }

    fn handle_keep_alive(&mut self, _packet: &KeepAliveServerbound) {
        self.last_keep_alive_time = current_time_in_secs();
    }
}

fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
