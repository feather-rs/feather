//! The initial handler is responsible for
//! handling new connections and getting
//! through the login sequence. After login
//! is completed, control is handed over to the server
//! thread, which is responsible for sending chunks/inventory/
//! players and then spawning the player.
//!
//! The initial handler is also responsible for handling
//! server list pings. To do this, it shares an `Arc<AtomicInteger>`
//! representing the player count with the server.
//!
//! The initial handler runs on the IO worker thread.
//! This is done to ensure minimal latency in packet handling,
//! speeding up the login process and making the latency calculation in
//! the server list ping as low as possible.

use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use uuid::Uuid;

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    DisconnectLogin, EncryptionRequest, EncryptionResponse, Handshake, HandshakeState, LoginStart,
    LoginSuccess, Ping, Pong, Request, Response, SetCompression,
};
use feather_core::network::packet::{Packet, PacketType};

use crate::config::Config;
use crate::io::NewClientInfo;
use crate::{PlayerCount, PROTOCOL_VERSION, SERVER_VERSION};

/// The key used for symmetric encryption.
pub type Key = [u8; 16];
/// The verify token used to ensure that encryption
/// is working correctly.
type VerifyToken = [u8; 4];

/// The number of bits used for the RSA key.
const RSA_KEY_BITS: u32 = 1024;
/// The number of bytes in the shared secret
const SHARED_SECRET_LEN: usize = 128 / 8;

/// An action for the worker thread to execute
/// after `InitialHandler::handle_packet` is called.
pub enum Action {
    EnableCompression(i32),
    EnableEncryption(Key),
    SendPacket(Box<dyn Packet>),
    Disconnect,
    JoinGame(JoinResult),
}

/// The type returned for when a player has completed the login process.
#[derive(Clone, Debug)]
pub struct JoinResult {
    pub username: String,
    pub uuid: Uuid,
    pub props: Vec<mojang_api::ServerAuthProperty>,
}

/// An initial handler for a connection.
///
/// When a packet is received from the client this initial
/// handler is registered with, `handle_packet` should be called.
/// This function runs all the necessary code to handle the
/// login sequence or the server list ping.
///
/// The initial handler is able to communicate with the worker
/// implementation by exposing the `actions_to_execute` method,
/// which returns a vector of actions for the worker to execute.
/// These may include, for example, enabling encryption or sending
/// a packet.
pub struct InitialHandler {
    /// A queue of actions to perform. When `actions_to_execute`
    /// is called, the queue is flushed.
    action_queue: Vec<Action>,

    /// If set to a value, indicates that encryption
    /// should be enabled with the given key.
    key: Option<Key>,
    /// If set to a value, indicates that compression
    /// should be enabled with the given threshold.
    compression_threshold: Option<i32>,

    /// The 1024-bit RSA key used for key exchange
    /// with this client.
    rsa: Rsa<Private>,
    /// The verify token generated for this exchange.
    verify_token: VerifyToken,

    /// The server's configuration.
    config: Arc<Config>,
    /// The server's player count.
    player_count: Arc<PlayerCount>,

    /// The username of the player, sent
    /// in Login Start.
    username: Option<String>,

    /// The player info, set to `Some` once
    /// the initial handler is finished and
    /// the player should join.
    info: Option<JoinResult>,

    /// The stage of this initial handler.
    stage: Stage,
}

impl InitialHandler {
    pub fn new(config: Arc<Config>, player_count: Arc<PlayerCount>) -> Self {
        Self {
            action_queue: vec![],

            key: None,
            compression_threshold: None,

            rsa: Rsa::generate(RSA_KEY_BITS).unwrap(),
            verify_token: rand::random(),

            config,
            player_count,

            username: None,

            info: None,

            stage: Stage::AwaitHandshake,
        }
    }

    /// Notifies this initial handler of a packet
    /// received from the client. After calling this
    /// function, `action_queue` should be called
    /// and the actions should be executed in order.
    pub fn handle_packet(&mut self, packet: Box<dyn Packet>) {
        if self.stage == Stage::Finished {
            panic!("Called InitialHandler::handle_packet() after completion");
        }

        if let Err(e) = _handle_packet(self, packet) {
            // Disconnect
            disconnect_login(self, &format!("{}", e));
            info!(
                "Player {} disconnected: {}",
                self.username.as_ref().unwrap_or(&"unknown".to_string()),
                e
            );
        }
    }

    /// Returns a vector of actions to perform.
    pub fn actions_to_execute(&mut self) -> Vec<Action> {
        let mut new_vec = vec![];
        std::mem::swap(&mut new_vec, &mut self.action_queue);

        new_vec
    }
}

/// Handles a packet, returning `Err` if the player
/// should be disconnected.
fn _handle_packet(ih: &mut InitialHandler, packet: Box<dyn Packet>) -> Result<(), Error> {
    // Find packet type and forward to correct function
    match packet.ty() {
        PacketType::Handshake => handle_handshake(ih, cast_packet::<Handshake>(&packet))?,
        PacketType::Request => handle_request(ih, cast_packet::<Request>(&packet))?,
        PacketType::Ping => handle_ping(ih, cast_packet::<Ping>(&packet))?,
        PacketType::LoginStart => handle_login_start(ih, cast_packet::<LoginStart>(&packet))?,
        PacketType::EncryptionResponse => {
            handle_encryption_response(ih, cast_packet::<EncryptionResponse>(&packet))?
        }
        ty => return Err(Error::InvalidPacket(ty, ih.stage)),
    }

    Ok(())
}

fn handle_handshake(ih: &mut InitialHandler, packet: &Handshake) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitHandshake, packet.ty())?;

    ih.stage = match packet.next_state {
        HandshakeState::Status => Stage::AwaitRequest,
        HandshakeState::Login => {
            // While status requests can use differing
            // protocol versions, a client
            // needs to have a matching protocol version
            // to log in.
            if packet.protocol_version != PROTOCOL_VERSION {
                return Err(Error::InvalidProtocol(packet.protocol_version));
            }

            Stage::AwaitLoginStart
        }
    };

    Ok(())
}

fn handle_request(ih: &mut InitialHandler, packet: &Request) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitRequest, packet.ty())?;

    // Send response packet
    let json = json!({
        "version": {
            "name": SERVER_VERSION,
            "protocol": PROTOCOL_VERSION,
        },
        "players": {
            "max": ih.config.server.max_players,
            "online": ih.player_count.0.load(Ordering::SeqCst),
        },
        "description": {
            "text": ih.config.server.motd,
        }
    });

    let response = Response::new(json.to_string());
    send_packet(ih, response);

    ih.stage = Stage::AwaitPing;

    Ok(())
}

fn handle_ping(ih: &mut InitialHandler, packet: &Ping) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitPing, packet.ty())?;

    let pong = Pong::new(packet.payload);
    send_packet(ih, pong);

    // After sending pong, we should disconnect.
    ih.action_queue.push(Action::Disconnect);
    ih.stage = Stage::Finished;

    Ok(())
}

fn handle_login_start(ih: &mut InitialHandler, packet: &LoginStart) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitLoginStart, packet.ty())?;

    ih.username = Some(packet.username.clone());

    // If in online mode, encryption needs to be enabled,
    // and authentication needs to be performed.
    // If not in online mode, the login sequence is
    // already finished, so we can call `finish` after
    // setting the player's info.
    if ih.config.server.online_mode {
        // Start enabling encryption
        let encryption_request = EncryptionRequest::new(
            "".to_string(), // Server ID - always empty
            ih.rsa.public_key_to_der().unwrap(),
            ih.verify_token.to_vec(),
        );
        send_packet(ih, encryption_request);

        ih.stage = Stage::AwaitEncryptionResponse;
    } else {
        // Finished - set info and join
        ih.info = Some(JoinResult {
            username: ih.username.clone().unwrap(),
            uuid: Uuid::new_v4(),
            props: vec![],
        });
        finish(ih);
    }

    Ok(())
}

fn handle_encryption_response(
    ih: &mut InitialHandler,
    packet: &EncryptionResponse,
) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitEncryptionResponse, packet.ty())?;

    // Decrypt verify token + shared secret
    let shared_secret = decrypt_using_rsa(&packet.secret, &ih.rsa)?;
    if shared_secret.len() != SHARED_SECRET_LEN {
        return Err(Error::BadSecretLength);
    }

    let verify_token = decrypt_using_rsa(&packet.verify_token, &ih.rsa)?;
    if verify_token.len() != ih.verify_token.len() {
        return Err(Error::VerifyTokenMismatch);
    }

    // Check that verify token matches
    if verify_token.as_slice() != &ih.verify_token {
        return Err(Error::VerifyTokenMismatch);
    }

    // Enable encryption
    let mut key = [0u8; SHARED_SECRET_LEN];
    for (i, x) in shared_secret[..SHARED_SECRET_LEN].into_iter().enumerate() {
        key[i] = *x;
    }

    ih.key = Some(key);
    ih.action_queue
        .push(Action::EnableEncryption(ih.key.clone().unwrap()));

    // Perform authentication
    let auth_result = mojang_api::server_auth(
        ih.username.as_ref().unwrap(),
        &mojang_api::server_hash(
            "",
            ih.key.clone().unwrap(),
            ih.rsa.public_key_to_der().unwrap().as_slice(),
        ),
    );

    match auth_result {
        Ok(auth) => {
            let info = JoinResult {
                username: auth.name,
                uuid: Uuid::from_str(&auth.id).unwrap(),
                props: auth.properties,
            };
            ih.info = Some(info);
        }
        Err(_) => return Err(Error::AuthenticationFailed),
    }

    finish(ih);

    Ok(())
}

fn decrypt_using_rsa(data: &[u8], key: &Rsa<Private>) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0u8; key.size() as usize];

    let len = key
        .private_decrypt(data, &mut buf, Padding::PKCS1)
        .map_err(|e| Error::BadEncryption)?;

    buf.truncate(len);

    Ok(buf)
}

/// Terminates the login process, sending Set Compression (if necessary)
/// and Login Success.
///
/// Before calling this function, it is expected that:
/// * `info` is set to a valid value
/// * Encryption has been enabled, if necessary
/// * All other login processes have already run
fn finish(ih: &mut InitialHandler) {
    assert!(ih.info.is_some());

    // Enable compression if necessary
    let compression_threshold = ih.config.io.compression_threshold;
    if compression_threshold > 0 {
        enable_compression(ih, compression_threshold);
    }

    let info = ih.info.as_ref().unwrap();

    // Send Login Success
    let login_success = LoginSuccess::new(
        info.uuid.to_hyphenated_ref().to_string(),
        info.username.clone(),
    );
    send_packet(ih, login_success);
    ih.action_queue
        .push(Action::JoinGame(ih.info.clone().unwrap()));
}

/// Enables compression, sending the Set Compression
/// packet.
fn enable_compression(ih: &mut InitialHandler, threshold: i32) {
    ih.compression_threshold = Some(threshold);
    send_packet(ih, SetCompression::new(threshold));
    ih.action_queue.push(Action::EnableCompression(threshold));
}

/// Checks that the initial handler stage matches
/// the expected stage, returning `Err` with a proper
/// error message if not.
fn check_stage(ih: &InitialHandler, expected: Stage, packet_ty: PacketType) -> Result<(), Error> {
    if ih.stage != expected {
        Err(Error::InvalidPacket(packet_ty, ih.stage))
    } else {
        Ok(())
    }
}

/// Disconnects the initial handler, sending
/// a disconnect packet containing the reason.
fn disconnect_login(ih: &mut InitialHandler, reason: &str) {
    ih.action_queue.push(Action::Disconnect);

    let json = json!({
        "text": reason,
    })
    .to_string();

    let packet = DisconnectLogin::new(json);
    send_packet(ih, packet);
}

/// Adds a packet to the internal packet queue.
fn send_packet<P: Packet + 'static>(ih: &mut InitialHandler, packet: P) {
    ih.action_queue.push(Action::SendPacket(Box::new(packet)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Error {
    InvalidPacket(PacketType, Stage),
    InvalidProtocol(u32),
    BadEncryption,
    VerifyTokenMismatch,
    BadSecretLength,
    AuthenticationFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::InvalidPacket(ty, stage) => write!(
                f,
                "Sent invalid packet {:?} at initial handler stage {:?}",
                ty, stage
            )?,
            Error::InvalidProtocol(protocol) => write!(
                f,
                "Invalid protocol version {} - this server is on {}",
                *protocol, PROTOCOL_VERSION
            )?,
            Error::BadEncryption => write!(f, "Failed to decrypt value")?,
            Error::VerifyTokenMismatch => write!(f, "Verify token does not match")?,
            Error::BadSecretLength => write!(f, "Invalid shared secret length")?,
            Error::AuthenticationFailed => write!(f, "Authentication failed")?,
        }

        Ok(())
    }
}

/// The stage of an initial handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    AwaitHandshake,
    AwaitRequest,
    AwaitPing,
    AwaitLoginStart,
    AwaitEncryptionResponse,
    Finished,
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use feather_core::network::cast_packet;
    use feather_core::network::packet::implementation::{
        Handshake, HandshakeState, LoginSuccess, Ping, Pong, Request, Response, SetCompression,
    };
    use feather_core::network::packet::PacketType;

    use crate::PROTOCOL_VERSION;

    use super::*;

    #[test]
    fn test_initial_handler_new() {
        let mut ih = ih();

        assert!(ih.actions_to_execute().is_empty());
    }

    #[test]
    fn test_status_ping() {
        let player_count = 24;
        let mut ih = ih_with_player_count(player_count);

        let handshake = Handshake::new(
            PROTOCOL_VERSION,
            "".to_string(), // Unused - server address
            25565,
            HandshakeState::Status,
        );
        ih.handle_packet(Box::new(handshake));

        // Confirm that no packets were sent and the player wasn't disconnected
        assert!(ih.actions_to_execute().is_empty());

        let request = Request::new();
        ih.handle_packet(Box::new(request));

        let actions = ih.actions_to_execute();

        // Confirm that correct response was received
        assert_eq!(actions.len(), 1);

        let _response = actions.first().unwrap();
        match _response {
            Action::SendPacket(_response) => {
                assert_eq!(_response.ty(), PacketType::Response);

                let response = cast_packet::<Response>(&_response);
                let _: serde_json::Value = serde_json::from_str(&response.json_response).unwrap();
            }
            _ => panic!(),
        }

        // Send ping
        let payload = 39842;
        let ping = Ping::new(payload);
        ih.handle_packet(Box::new(ping));

        let mut actions = ih.actions_to_execute();

        assert_eq!(actions.len(), 2);
        let _pong = actions.remove(0);
        match _pong {
            Action::SendPacket(_pong) => {
                assert_eq!(_pong.ty(), PacketType::Pong);
                let pong = cast_packet::<Pong>(&_pong);
                assert_eq!(pong.payload, payload);
            }
            _ => panic!(),
        }

        let disconnect = actions.remove(0);
        match disconnect {
            Action::Disconnect => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_login_sequence() {
        let mut config = Config::default();
        config.server.online_mode = false;
        let mut ih = ih_with_config(config.clone());

        let handshake = Handshake::new(
            PROTOCOL_VERSION,
            "".to_string(), // Unused - server address
            25565,
            HandshakeState::Login,
        );
        ih.handle_packet(Box::new(handshake));

        assert!(ih.actions_to_execute().is_empty());

        let username = "test";
        let login_start = LoginStart::new(username.to_string());
        ih.handle_packet(Box::new(login_start));

        let mut actions = ih.actions_to_execute();
        assert_eq!(actions.len(), 4);

        let _set_compression = actions.remove(0);

        match _set_compression {
            Action::SendPacket(_set_compression) => {
                assert_eq!(_set_compression.ty(), PacketType::SetCompression);

                let set_compression = cast_packet::<SetCompression>(&_set_compression);
                assert_eq!(set_compression.threshold, config.io.compression_threshold);
            }
            _ => panic!(),
        }

        let enable_compression = actions.remove(0);
        match enable_compression {
            Action::EnableCompression(threshold) => {
                assert_eq!(threshold, config.io.compression_threshold);
            }
            _ => panic!(),
        }

        let _login_success = actions.remove(0);

        match _login_success {
            Action::SendPacket(_login_success) => {
                assert_eq!(_login_success.ty(), PacketType::LoginSuccess);

                let login_success = cast_packet::<LoginSuccess>(&_login_success);
                assert_eq!(login_success.username, username.to_string());
            }
            _ => panic!(),
        }

        let join = actions.remove(0);
        match join {
            Action::JoinGame(_) => (),
            _ => panic!(),
        }
    }

    fn ih() -> InitialHandler {
        InitialHandler::new(
            Arc::new(Config::default()),
            Arc::new(PlayerCount(AtomicUsize::new(0))),
        )
    }

    fn ih_with_player_count(count: usize) -> InitialHandler {
        InitialHandler::new(
            Arc::new(Config::default()),
            Arc::new(PlayerCount(AtomicUsize::new(count))),
        )
    }

    fn ih_with_config(config: Config) -> InitialHandler {
        InitialHandler::new(Arc::new(config), Arc::new(PlayerCount(AtomicUsize::new(0))))
    }
}
