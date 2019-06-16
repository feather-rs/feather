use crate::io::NewClientInfo;
use feather_core::network::packet::{Packet, PacketType, implementation::*};
use crate::prelude::*;
use std::rc::Rc;
use openssl::rsa::Rsa;
use openssl::pkey::{Private, Public};

const PROTOCOL_VERSION: u32 = 404;

pub struct InitialHandler {
    server: Rc<Server>,
    sent_encryption_request: bool,
    pub state: State,
    pub finished: bool,
    pub should_disconnect: bool,
    pub handle: Rc<PlayerHandle>,
    pub name: Option<String>,
    key: Rsa,
    verify_token: [u8; 4],
}

impl InitialHandler {
    pub fn new(server: Rc<Server>, handle: Rc<PlayerHandle>) -> Self {
        let (public)
        Self {
            server,
            sent_encryption_request: false,
            state: State::Handshake,
            finished: false,
            should_disconnect: false,
            handle,
            name: None,
            key: PKey::from_rsa(rsa::generate(1024).unwrap()).unwrap(),
            verify_token: rand::random(),
        }
    }

    pub fn handle_packet(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        if self.finished {
            Err(());
        }

        if self.state == State::Handshake {
            return self.handle_handshake(packet);
        } else if self.state == State::Status {
            return self.handle_status(packet);
        } else if self.state = State::Login {
            return self.handle_login(packe);
        }

        Ok(())
    }

    fn handle_handshake(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        if packet.ty() != PacketType::Handshake {
            self.disconnect_login("Client sent incorrect packet at stage HANDSHAKE");
            return Err(());
        }

        let handshake = cast_packet::<Handshake>(&packet);

        match handshake.next_state {
            HandshakeState::Login => self.state = State::Login,
            HandshakeState::Status => self.state = State::Status,
        }

        if handshake.protocol_version != PROTOCOL_VERSION {
            self.disconnect_login(&format!("Protocol version does not match: client is on {}, server on {}", handshake.protocol_version, PROTOCOL_VERSION));
            return Err(());
        }

        Ok(())
    }

    fn handle_status(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        match packet.ty() {
            PacketType::Request => {
                let response = Response::new(self.get_status_response());
                self.handle.send_packet(response);
            },
            PacketType::Ping => {
                let ping = cast_packet::<Ping>(&packet);
                let pong = Pong::new(ping.payload);
                self.handle.send_packet(pong);
                self.should_disconnect = true;
            },
            _ => {
                self.disconnect_login("Client sent incorrect packet at stage STATUS");
                return Err(());
            }
        }

        Ok(())
    }

    fn get_status_response(&self) -> String {
        let json = json!({
            "version": {
                "name": "1.13.2",
                "protocol": PROTOCOL_VERSION,
            },
           "players": {
                "max": self.server.config.server.max_players,
                "online": self.server.player_count,
           },
           "description": {
                "text": self.server.config.server.motd,
           }
        });

        json.to_string()
    }

    fn handle_login(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        match packet.ty() {
            PacketType::LoginStart => {
                if self.name.is_some() {
                    // Client already sent LoginStart - disconnect!
                    self.disconnect_login("LoginStart sent twice");
                    return Err(());
                }

                let login_start = cast_packet::<LoginStart>(&packet);
                self.name = Some(login_start.username.clone());
                self.send_encryption_request();
            },
            PacketType::EncryptionResponse => {

            }
        }

        Ok(())
    }

    fn send_encryption_request(&self) {
        let key_bytes = self.key.public_keu_to_der().unwrap();

        let mut verify_token = Vec::with_capacity(4);
        verify_token.extend_from_slice(&self.verify_token);

        let packet = EncryptionRequest::new(
            "", // Server ID - empty
            key_bytes.len(),
            key_bytes,
            verify_token.len(),
            verify_token
        );

        self.handle.send_packet(packet);
    }

    pub fn disconnect_login(&mut self, reason: &str) {
        let packet = DisconnectLogin::new(reason.to_string());
        self.handle.send_packet(packet);
        self.should_disconnect = true;
    }
}

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
