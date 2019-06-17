use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use openssl::pkey::Private;
use openssl::rsa::{self, Rsa};
use std::rc::Rc;

const PROTOCOL_VERSION: u32 = 404;

pub struct InitialHandler {
    sent_encryption_request: bool,
    pub state: State,
    pub finished: bool,
    pub should_disconnect: bool,
    pub handle: Option<Rc<PlayerHandle>>,
    pub name: Option<String>,
    key: Rsa<Private>,
    verify_token: [u8; 4],

    motd: String,
    player_count: u32,
    max_players: i32,

    packets_to_send: RefCell<Vec<Box<Packet>>>,

    encryption_key: Option<[u8; 16]>,
    compression_threshold: Option<usize>,

    enable_encryption: bool,
    enable_compression: bool,

    uuid: Uuid,
}

impl InitialHandler {
    pub fn new(motd: String, player_count: u32, max_players: i32, rsa_key: Rsa<Private>) -> Self {
        Self {
            sent_encryption_request: false,
            state: State::Handshake,
            finished: false,
            should_disconnect: false,
            handle: None,
            name: None,
            key: rsa_key,
            verify_token: [100, 128, 255, 0],
            motd,
            player_count,
            max_players,

            packets_to_send: RefCell::new(vec![]),

            encryption_key: None,
            compression_threshold: None,

            enable_encryption: false,
            enable_compression: false,

            uuid: Uuid::new_v4(),
        }
    }

    pub fn handle_packet(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        if self.finished {
            return Err(());
        }

        if self.state == State::Handshake {
            return self.handle_handshake(packet);
        } else if self.state == State::Status {
            return self.handle_status(packet);
        } else if self.state == State::Login {
            return self.handle_login(packet);
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

        if handshake.protocol_version != PROTOCOL_VERSION && self.state != State::Status {
            self.disconnect_login(&format!(
                "Protocol version does not match: client is on {}, server on {}",
                handshake.protocol_version, PROTOCOL_VERSION
            ));
            return Err(());
        }

        Ok(())
    }

    fn handle_status(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        match packet.ty() {
            PacketType::Request => {
                let response = Response::new(self.get_status_response());
                self.send_packet(response);
            }
            PacketType::Ping => {
                let ping = cast_packet::<Ping>(&packet);
                let pong = Pong::new(ping.payload);
                self.send_packet(pong);
                self.should_disconnect = true;
            }
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
                "max": self.max_players,
                "online": self.player_count,
           },
           "description": {
                "text": self.motd,
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
            }
            PacketType::EncryptionResponse => {
                return self.handle_encryption_response(packet);
            }
            _ => {
                self.disconnect_login("Client sent incorrect packet at stage LOGIN");
                return Err(());
            }
        }

        Ok(())
    }

    fn send_encryption_request(&mut self) {
        let key_bytes = self.key.public_key_to_der().unwrap();

        let mut verify_token = Vec::with_capacity(4);
        verify_token.extend_from_slice(&self.verify_token);

        let packet = EncryptionRequest::new(
            "".to_string(), // Server ID - empty
            key_bytes.len() as i32,
            key_bytes,
            4,
            verify_token,
        );

        self.send_packet(packet);

        self.sent_encryption_request = true;
    }

    fn handle_encryption_response(&mut self, packet: Box<Packet>) -> Result<(), ()> {
        if !self.sent_encryption_request {
            self.disconnect_login("EncryptionResponse sent before server sent EncryptionRequest");
            return Err(());
        }

        let response = cast_packet::<EncryptionResponse>(&packet);
        let shared_secret = &response.secret;
        let verify_token = &response.verify_token;

        let mut decrypted_shared_secret = vec![0; self.key.size() as usize];
        self.key
            .private_decrypt(
                &shared_secret,
                &mut decrypted_shared_secret,
                rsa::Padding::PKCS1,
            )
            .unwrap();

        let mut decrypted_verify_token = vec![0; self.key.size() as usize];
        self.key
            .private_decrypt(
                &verify_token,
                &mut decrypted_verify_token,
                rsa::Padding::PKCS1,
            )
            .unwrap();

        if decrypted_verify_token[..4] != self.verify_token {
            trace!(
                "Verify token mismatch: received {:?}, sent {:?}",
                &verify_token[..4],
                self.verify_token
            );
            self.disconnect_login("Verify token does not match");
            return Err(());
        }

        let mut key = [0; 16];
        for (i, val) in decrypted_shared_secret[..16].iter().enumerate() {
            key[i] = val.clone();
        }

        self.enable_encryption(key);

        // Send Login Success
        let login_success = LoginSuccess::new(
            self.uuid.to_hyphenated_ref().to_string(),
            self.name.as_ref().unwrap().clone(),
        );
        self.send_packet(login_success);

        Ok(())
    }

    pub fn disconnect_login(&mut self, reason: &str) {
        let packet = DisconnectLogin::new(reason.to_string());
        self.send_packet(packet);
        self.should_disconnect = true;
    }

    fn send_packet<P: Packet + 'static + Send>(&mut self, packet: P) {
        self.packets_to_send.borrow_mut().push(Box::new(packet));
    }

    fn enable_encryption(&mut self, key: [u8; 16]) {
        self.encryption_key = Some(key);
        self.enable_encryption = true;
    }

    fn enable_compression(&mut self, threshold: usize) {
        self.compression_threshold = Some(threshold);
        self.enable_compression = true;
    }

    pub fn packets_to_send(&mut self) -> Vec<Box<Packet>> {
        self.packets_to_send.borrow_mut().drain(..).collect()
    }

    pub fn should_enable_encryption(&mut self) -> Option<[u8; 16]> {
        if self.enable_encryption {
            self.enable_encryption = false;
            Some(self.encryption_key.unwrap())
        } else {
            None
        }
    }

    pub fn should_enable_compression(&mut self) -> Option<usize> {
        if self.enable_compression {
            self.enable_compression = false;
            Some(self.compression_threshold).unwrap()
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
