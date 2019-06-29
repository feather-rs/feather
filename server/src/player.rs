use super::initialhandler as ih;
use super::initialhandler::InitialHandler;
use crate::io::ServerToWorkerMessage;
use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use feather_core::world::World;
use mio_extras::channel::{Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_KEEP_ALIVE_TIME: u64 = 30;

pub struct PlayerHandle {
    initial_handler: RefCell<InitialHandler>,

    packet_sender: Sender<ServerToWorkerMessage>,
    packet_receiver: Receiver<ServerToWorkerMessage>,

    entity_id: i32,

    pub should_remove: RefCell<bool>,

    /// The last time a keep alive packet
    /// was received from the client. If this
    /// value exceeds MAX_KEEP_ALIVE_TIME seconds,
    /// the client should be disconnected with a "time out"
    /// error as per the protocol specification.
    last_keep_alive_time: RefCell<u64>,
    server: Rc<Server>,
}

impl PlayerHandle {
    pub fn accept_player_connection(
        packet_sender: Sender<ServerToWorkerMessage>,
        packet_receiver: Receiver<ServerToWorkerMessage>,
        motd: String,
        player_count: u32,
        max_players: i32,
        rsa_key: openssl::rsa::Rsa<openssl::pkey::Private>,
        config: Rc<Config>,
        server: Rc<Server>,
    ) -> Self {
        Self {
            initial_handler: RefCell::new(InitialHandler::new(
                motd,
                player_count,
                max_players,
                Rc::clone(&config),
                rsa_key,
            )),

            packet_sender,

            entity_id: 0,

            packet_receiver,
            should_remove: RefCell::new(false),

            last_keep_alive_time: RefCell::new(current_time_in_secs()),
            server,
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

    pub fn disconnect(&mut self, _reason: &str) {
        *self.should_remove.borrow_mut() = true;
        // TODO send Disconnect packet
        self.close_connection();
    }

    pub fn tick(&mut self, world: &World) -> Result<(), ()> {
        while let Ok(msg) = self.packet_receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                    self.handle_packet(packet, world)?;
                }
                ServerToWorkerMessage::NotifyDisconnect => {
                    *self.should_remove.borrow_mut() = true;
                    return Ok(());
                }
                _ => unreachable!(),
            }
        }

        if self.server.tick_count() % TPS == 0 && self.initial_handler.borrow().finished {
            // Send keep alive every second as per the protocol specification
            let keep_alive = KeepAliveClientbound::new(0);
            self.send_packet(keep_alive)?;

            if current_time_in_secs() - *self.last_keep_alive_time.borrow() >= MAX_KEEP_ALIVE_TIME {
                self.disconnect("Timed out");
            }
        }

        Ok(())
    }

    fn handle_packet(&mut self, packet: Box<Packet>, world: &World) -> Result<(), ()> {
        trace!("Handling packet");
        if !self.initial_handler.borrow().finished {
            self.forward_packet_to_initial_handler(packet, world)?;
        } else {
            // TODO perhaps use HashMap instead of match here?
            match packet.ty() {
                PacketType::KeepAliveServerbound => {
                    self.handle_keep_alive(cast_packet::<KeepAliveServerbound>(&packet))
                }
                PacketType::ChatMessageServerbound => {
                    // TODO
                }
                _ => (), // TODO
            }
        }

        Ok(())
    }

    fn forward_packet_to_initial_handler(
        &mut self,
        packet: Box<Packet>,
        world: &World,
    ) -> Result<(), ()> {
        let r = self.initial_handler.borrow_mut().handle_packet(packet);

        for action in self.initial_handler.borrow_mut().actions() {
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

        if r.is_err() {
            *self.should_remove.borrow_mut() = true;
            self.close_connection();
        }

        if self.initial_handler.borrow().should_disconnect {
            *self.should_remove.borrow_mut() = true;
            self.close_connection();
        }

        if self.initial_handler.borrow().finished {
            // Run the play sequence to allow the player
            // to join
            self.run_play_sequence(world)?;
        }

        Ok(())
    }

    /// Sends the join packets, such as Join Game, Chunk
    /// Data, etc.
    fn run_play_sequence(&mut self, world: &World) -> Result<(), ()> {
        let entity_id = self.server.allocate_entity_id();
        self.entity_id = entity_id;

        let join_game = JoinGame::new(
            entity_id,
            Gamemode::Creative.get_id(),
            Dimension::Overwold.get_id(),
            Difficulty::Hard.get_id(),
            0, // Unused value - max players
            "default".to_string(),
            false,
        );

        self.send_packet(join_game)?;

        // Send chunk packets
        let view_distance = self.server.config.server.view_distance as i32;
        for x in -view_distance..view_distance + 1 {
            for y in -view_distance..view_distance + 1 {
                let chunk_data =
                    ChunkData::new(world.chunk_at(ChunkPosition::new(x, y)).borrow().clone());
                self.send_packet(chunk_data)?;
            }
        }

        // Spawn the player
        let spawn_pos = SpawnPosition::new(BlockPosition::new(0, 0, 0));
        self.send_packet(spawn_pos)?;
        let pos = PlayerPositionAndLookClientbound::new(0.0, 64.0, 0.0, 0.0, 0.0, 0, 1);
        self.send_packet(pos)?;

        Ok(())
    }

    fn handle_keep_alive(&mut self, _packet: &KeepAliveServerbound) {
        *self.last_keep_alive_time.borrow_mut() = current_time_in_secs();
    }
}

fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Calculates the relative move fields
/// as used in the EntityRelativeMove packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (u16, u16, u16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as u16;
    let y = ((current.y * 32.0 - old.x * 32.0) * 128.0) as u16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as u16;
    (x, y, z)
}
