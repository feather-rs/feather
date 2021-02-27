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

use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey};
use rsa_der as der;

use thiserror::Error;

use feather_core::network::{cast_packet, Packet, PacketStage, PacketType};
use feather_core::text::{Text, TextRoot};

use crate::{PROTOCOL_VERSION, SERVER_VERSION};
use feather_core::network::packets::{
    DisconnectLogin, EncryptionRequest, EncryptionResponse, Handshake, HandshakeState, LoginStart,
    LoginSuccess, Ping, Pong, Request, Response, SetCompression,
};
use feather_server_types::{BanInfo, Config, ProxyMode};
use feather_server_util::name_to_uuid_offline;
use mojang_api::ProfileProperty;
use once_cell::sync::Lazy;
use uuid::Uuid;

/// The key used for symmetric encryption.
pub type Key = [u8; 16];
/// The verify token used to ensure that encryption
/// is working correctly.
type VerifyToken = [u8; 4];

/// The number of bits used for the RSA key.
const RSA_KEY_BITS: usize = 1024;
/// The number of bytes in the shared secret
const SHARED_SECRET_LEN: usize = 128 / 8;

pub static RSA_KEY: Lazy<RSAPrivateKey> = Lazy::new(|| {
    let mut rng = OsRng;
    RSAPrivateKey::new(&mut rng, RSA_KEY_BITS).unwrap()
});

/// An action for the worker thread to execute
/// after `InitialHandler::handle_packet` is called.
pub enum Action {
    EnableCompression(i32),
    EnableEncryption(Key),
    SendPacket(Box<dyn Packet>),
    Disconnect,
    SetStage(PacketStage),
    JoinGame(JoinResult),
}

/// The type returned for when a player has completed the login process.
#[derive(Clone, Debug)]
pub struct JoinResult {
    pub username: Option<String>,
    pub uuid: Uuid,
    pub props: Vec<mojang_api::ProfileProperty>,
}

impl JoinResult {
    /// Creates a `JoinResult` for an offline mode player with the given username.
    ///
    /// The UUID will be computed using Minecraft's method for computing offline mode
    /// UUIDs as a function of the username.
    fn with_username(username: String) -> Self {
        let mut join_result = JoinResult::default();

        join_result.uuid = name_to_uuid_offline(&username);
        join_result.username = Some(username);

        join_result
    }
}

impl Default for JoinResult {
    fn default() -> Self {
        JoinResult {
            username: None,
            uuid: Uuid::new_v4(),
            props: vec![],
        }
    }
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

    /// The verify token generated for this exchange.
    verify_token: VerifyToken,

    /// The server's configuration.
    config: Arc<Config>,
    /// Server bans
    ban_info: Arc<RwLock<BanInfo>>,
    /// The server's player count.
    player_count: Arc<AtomicU32>,
    /// The server's icon, if any was loaded.
    server_icon: Arc<Option<String>>,

    /// The client's IP address.
    client_ip: SocketAddr,

    /// The player info, set to `Some` once
    /// the initial handler is finished and
    /// the player should join.
    info: Option<JoinResult>,

    /// The stage of this initial handler.
    stage: Stage,
}

impl InitialHandler {
    pub fn new(
        config: Arc<Config>,
        ban_info: Arc<RwLock<BanInfo>>,
        player_count: Arc<AtomicU32>,
        server_icon: Arc<Option<String>>,
        client_ip: SocketAddr,
    ) -> Self {
        Self {
            action_queue: vec![],

            key: None,
            compression_threshold: None,

            verify_token: rand::random(),

            config,
            ban_info,
            player_count,
            server_icon,

            client_ip,

            info: None,

            stage: Stage::AwaitHandshake,
        }
    }

    /// Notifies this initial handler of a packet
    /// received from the client. After calling this
    /// function, `action_queue` should be called
    /// and the actions should be executed in order.
    pub async fn handle_packet(&mut self, packet: Box<dyn Packet>) {
        if self.stage == Stage::Finished {
            panic!("Called InitialHandler::handle_packet() after completion");
        }

        if let Err(e) = _handle_packet(self, packet).await {
            // Disconnect
            disconnect_login(self, Text::from(e.to_string()));

            let username = self
                .info
                .as_ref()
                .and_then(|info| info.username.as_deref())
                .unwrap_or("unkown");
            log::info!("Player {} disconnected: {}", username, e);
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
async fn _handle_packet(ih: &mut InitialHandler, packet: Box<dyn Packet>) -> Result<(), Error> {
    // Find packet type and forward to correct function
    match packet.ty() {
        PacketType::Handshake => handle_handshake(ih, &cast_packet::<Handshake>(packet))?,
        PacketType::Request => handle_request(ih, &cast_packet::<Request>(packet))?,
        PacketType::Ping => handle_ping(ih, &cast_packet::<Ping>(packet))?,
        PacketType::LoginStart => handle_login_start(ih, &cast_packet::<LoginStart>(packet))?,
        PacketType::EncryptionResponse => {
            handle_encryption_response(ih, &cast_packet::<EncryptionResponse>(packet)).await?
        }
        ty => return Err(Error::InvalidPacket(ty, ih.stage)),
    }

    Ok(())
}

fn handle_handshake(ih: &mut InitialHandler, packet: &Handshake) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitHandshake, packet.ty())?;

    ih.stage = match packet.next_state {
        HandshakeState::Status => {
            ih.action_queue.push(Action::SetStage(PacketStage::Status));
            Stage::AwaitRequest
        }
        HandshakeState::Login => {
            // While status requests can use differing
            // protocol versions, a client
            // needs to have a matching protocol version
            // to log in.
            if packet.protocol_version != PROTOCOL_VERSION {
                return Err(Error::InvalidProtocol(packet.protocol_version));
            }

            // If the server has BungeeCord proxy mode enabled, extract the data that is submitted
            // by BungeeCord if IP forwarding is enabled.
            if ih.config.proxy.proxy_mode == ProxyMode::BungeeCord {
                let bungeecord_data = extract_bungeecord_data(packet)?;
                ih.info = Some(JoinResult {
                    username: None,
                    uuid: bungeecord_data.uuid,
                    props: bungeecord_data.properties,
                });
            }

            ih.action_queue.push(Action::SetStage(PacketStage::Login));
            Stage::AwaitLoginStart
        }
    };

    Ok(())
}

/// Tries to extract the player information that is sent in the `server_address` field of a
/// Handshake packet that originates from a BungeeCord style proxy. This is used to enable IP
/// forwarding for BungeeCord style proxies.
///
/// The server address field should have 4 parts if a client is connecting via BungeeCord. The field
/// has the following format:
///
/// format!("{}\0{}\0{}\0{}", host, address, uuid, mojang_response);
///
/// | Variable        | Definition                                          |
/// |-----------------|-----------------------------------------------------|
/// | Host            | The IP address of the BungeeCord instance           |
/// | Address         | The IP address of the connecting client             |
/// | UUID            | The UUID that is associated to the clients account  |
/// | Mojang response | A JSON formatted version of the `properties` field
/// in [Mojangs response](https://wiki.vg/Protocol_Encryption#Server)       |
fn extract_bungeecord_data(packet: &Handshake) -> Result<BungeeCordData, Error> {
    let bungee_information: Vec<&str> = packet.server_address.split('\0').collect();
    Ok(BungeeCordData::from_vec(&bungee_information)?)
}

#[derive(Debug, PartialEq)]
struct BungeeCordData {
    host: String,
    client: String,
    uuid: Uuid,
    properties: Vec<ProfileProperty>,
}

impl BungeeCordData {
    pub fn from_vec(data: &[&str]) -> Result<Self, Error> {
        if data.len() != 4 {
            return Err(Error::BungeeSpecMismatch("Incorrect length".to_string()));
        }

        let host = (*data.get(0).unwrap()).to_string();
        let client = (*data.get(1).unwrap()).to_string();
        let uuid = Uuid::parse_str(*data.get(2).unwrap())
            .map_err(|e| Error::BungeeSpecMismatch(e.to_string()))?;
        let properties = serde_json::from_str(data.get(3).unwrap())
            .map_err(|e| Error::BungeeSpecMismatch(e.to_string()))?;

        Ok(BungeeCordData {
            host,
            client,
            uuid,
            properties,
        })
    }
}

fn handle_request(ih: &mut InitialHandler, packet: &Request) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitRequest, packet.ty())?;
    let server_icon = (*ih.server_icon).clone().unwrap_or_default();

    // Send response packet
    let mut json = serde_json::json!({
        "version": {
            "name": SERVER_VERSION,
            "protocol": PROTOCOL_VERSION,
        },
        "players": {
            "max": ih.config.server.max_players,
            "online": ih.player_count.load(Ordering::SeqCst),
        },
        "description": {
            "text": ih.config.server.motd,
        },
        "favicon": server_icon,
    });

    // Remove the favicon field if there is no favicon
    if server_icon.is_empty() {
        json.as_object_mut().unwrap().remove("favicon");
    }

    let response = Response {
        json_response: json.to_string(),
    };
    send_packet(ih, response);

    ih.stage = Stage::AwaitPing;

    Ok(())
}

fn handle_ping(ih: &mut InitialHandler, packet: &Ping) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitPing, packet.ty())?;

    let pong = Pong {
        payload: packet.payload,
    };
    send_packet(ih, pong);

    // After sending pong, we should disconnect.
    ih.action_queue.push(Action::Disconnect);
    ih.stage = Stage::Finished;

    Ok(())
}

fn handle_login_start(ih: &mut InitialHandler, packet: &LoginStart) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitLoginStart, packet.ty())?;

    if ih.player_count.load(Ordering::Acquire) >= ih.config.server.max_players as u32 {
        disconnect_login(ih, Text::from("Server is full!"));
        return Ok(());
    }

    let (reason, remove_ban) = {
        let ban_info = ih.ban_info.read().unwrap();
        let ip_ban = ban_info.ip_bans.get(&ih.client_ip.ip());

        if let Some(ban) = ip_ban {
            // Expire the ban if it's over.
            if let Some(expires) = ban.expires_after {
                if expires < SystemTime::now() {
                    (None, true)
                } else {
                    (Some(Text::from(ban.reason.clone())), false)
                }
            } else {
                (Some(Text::from(ban.reason.clone())), false)
            }
        } else {
            (None, false)
        }
    };

    if let Some(reason) = reason {
        disconnect_login(ih, reason);
        return Ok(());
    } else if remove_ban {
        ih.ban_info
            .write()
            .unwrap()
            .ip_bans
            .remove(&ih.client_ip.ip());
    }

    // If in online mode, encryption needs to be enabled,
    // and authentication needs to be performed.
    // If not in online mode, the login sequence is
    // already finished, so we can call `finish` after
    // setting the player's info.
    if ih.config.server.online_mode {
        use num_bigint_dig::{BigInt, Sign::Plus};
        // Start enabling encryption
        let der = der::public_key_to_der(
            &BigInt::from_biguint(Plus, RSA_KEY.n().clone()).to_signed_bytes_be(),
            &BigInt::from_biguint(Plus, RSA_KEY.e().clone()).to_signed_bytes_be(),
        );

        let encryption_request = EncryptionRequest {
            server_id: "".to_string(), // Server ID - always empty
            public_key: der,
            verify_token: ih.verify_token.to_vec(),
        };
        send_packet(ih, encryption_request);

        ih.info = Some(JoinResult::with_username(packet.username.clone()));

        ih.stage = Stage::AwaitEncryptionResponse;
    } else {
        let username = packet.username.clone();

        // Check if there is some info about the client available. This can be the case if the
        // handshake is made by an IP forwarding proxy.
        if let Some(info) = ih.info.as_mut() {
            if info.username.is_none() {
                info.username = Some(username);
            }
        } else {
            // Finished - set info and join
            ih.info = Some(JoinResult::with_username(username))
        }

        finish(ih);
    }

    Ok(())
}

async fn handle_encryption_response(
    ih: &mut InitialHandler,
    packet: &EncryptionResponse,
) -> Result<(), Error> {
    check_stage(ih, Stage::AwaitEncryptionResponse, packet.ty())?;

    // Decrypt verify token + shared secret
    let shared_secret = decrypt_using_rsa(&packet.secret, &RSA_KEY)?;
    if shared_secret.len() != SHARED_SECRET_LEN {
        return Err(Error::BadSecretLength);
    }

    let verify_token = decrypt_using_rsa(&packet.verify_token, &RSA_KEY)?;
    if verify_token.len() != ih.verify_token.len() {
        return Err(Error::VerifyTokenMismatch);
    }

    // Check that verify token matches
    if verify_token.as_slice() != ih.verify_token {
        return Err(Error::VerifyTokenMismatch);
    }

    // Enable encryption
    let mut key = [0u8; SHARED_SECRET_LEN];
    for (i, x) in shared_secret[..SHARED_SECRET_LEN].iter().enumerate() {
        key[i] = *x;
    }

    ih.key = Some(key);
    ih.action_queue
        .push(Action::EnableEncryption(ih.key.unwrap()));

    use num_bigint_dig::{BigInt, Sign::Plus};
    let der = der::public_key_to_der(
        &BigInt::from_biguint(Plus, RSA_KEY.n().clone()).to_signed_bytes_be(),
        &BigInt::from_biguint(Plus, RSA_KEY.e().clone()).to_signed_bytes_be(),
    );

    // This unwrapping can be shorter with the use of .flatten() which will stabilize in Rust 1.40.
    let username = ih
        .info
        .as_ref()
        .map(|x| x.username.as_ref())
        .and_then(|x| x)
        .ok_or(Error::OptionIsNone)?;

    // Perform authentication
    let auth_result = mojang_api::server_auth(
        &mojang_api::server_hash("", ih.key.unwrap(), der.as_slice()),
        username,
    )
    .await;

    match auth_result {
        Ok(auth) => {
            let info = JoinResult {
                username: Some(auth.name),
                uuid: auth.id,
                props: auth.properties,
            };
            ih.info = Some(info);
        }
        Err(e) => return Err(Error::AuthenticationFailed(e)),
    }

    finish(ih);

    Ok(())
}

fn decrypt_using_rsa(data: &[u8], key: &RSAPrivateKey) -> Result<Vec<u8>, Error> {
    let buf = key
        .decrypt(PaddingScheme::PKCS1v15, data)
        .map_err(|_| Error::BadEncryption)?;

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
    assert!(ih.info.as_ref().unwrap().username.is_some());

    // Enable compression if necessary
    let compression_threshold = ih.config.io.compression_threshold;
    if compression_threshold > 0 {
        enable_compression(ih, compression_threshold);
    }

    let info = ih.info.as_ref().unwrap();

    let uuid_str = info.uuid.to_hyphenated_ref().to_string();

    // Make sure they're not UUID banned
    let (reason, remove_ban) = {
        let ban_info = ih.ban_info.read().unwrap();
        let ip_ban = ban_info.uuid_bans.get(&uuid_str);

        if let Some(ban) = ip_ban {
            // Expire the ban if it's over.
            if let Some(expires) = ban.expires_after {
                if expires < SystemTime::now() {
                    (None, true)
                } else {
                    (Some(Text::from(ban.reason.clone())), false)
                }
            } else {
                (Some(Text::from(ban.reason.clone())), false)
            }
        } else {
            (None, false)
        }
    };

    if let Some(reason) = reason {
        disconnect_login(ih, reason);
        return;
    } else if remove_ban {
        ih.ban_info.write().unwrap().uuid_bans.remove(&uuid_str);
    }

    // Send Login Success
    let login_success = LoginSuccess {
        uuid: uuid_str,
        username: info.username.as_ref().unwrap().to_string(),
    };
    send_packet(ih, login_success);
    ih.action_queue.push(Action::SetStage(PacketStage::Play));
    ih.action_queue
        .push(Action::JoinGame(ih.info.clone().unwrap()));
}

/// Enables compression, sending the Set Compression
/// packet.
fn enable_compression(ih: &mut InitialHandler, threshold: i32) {
    ih.compression_threshold = Some(threshold);
    send_packet(ih, SetCompression { threshold });
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
fn disconnect_login(ih: &mut InitialHandler, reason: Text) {
    let packet = DisconnectLogin {
        reason: TextRoot::from(reason).into(),
    };
    send_packet(ih, packet);

    ih.action_queue.push(Action::Disconnect);
}

/// Adds a packet to the internal packet queue.
fn send_packet<P: Packet + 'static>(ih: &mut InitialHandler, packet: P) {
    ih.action_queue.push(Action::SendPacket(Box::new(packet)));
}

#[derive(Error, Debug, PartialEq)]
enum Error {
    #[error("invalid packet type {0:?} sent at stage {1:?}")]
    InvalidPacket(PacketType, Stage),
    #[error("unsupported protocol version {0:?}")]
    InvalidProtocol(u32),
    #[error("invalid encryption")]
    BadEncryption,
    #[error("verify tokens do not match")]
    VerifyTokenMismatch,
    #[error("shared secret length is not correct")]
    BadSecretLength,
    #[cfg(debug_assertions)]
    #[error("authentication failure: {0:?}")]
    AuthenticationFailed(mojang_api::Error),
    #[cfg(not(debug_assertions))]
    #[error("Failed to verify username!")]
    AuthenticationFailed(mojang_api::Error),
    #[error("received BungeeCord data does not match the specification: {0}")]
    BungeeSpecMismatch(String),
    #[error("option that should not be None was None")]
    /// An Error type than can be used as the error type of using the Try operator on Option
    /// types. In rust-core, this is an unstable feature (issue #42327)
    OptionIsNone,
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
    use feather_core::network::cast_packet;
    use feather_core::network::packets::{
        Handshake, HandshakeState, LoginSuccess, Ping, Pong, Request, Response, SetCompression,
    };
    use feather_core::network::PacketType;

    use crate::PROTOCOL_VERSION;

    use super::*;
    use mojang_api::ProfileProperty;

    #[test]
    fn extract_bungeecord_data_normal() {
        let handshake = Handshake {
           protocol_version: PROTOCOL_VERSION,
           server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
           server_port: 25565,
           next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract_bungeecord_data(&handshake).unwrap(),
            BungeeCordData {
                host: "192.168.1.87".to_string(),
                client: "192.168.1.67".to_string(),
                uuid: Uuid::parse_str("905c7e4fb96b45139645d123225575e2").unwrap(),
                properties: vec![ProfileProperty {
                    name: "textures".to_string(),
                    value: "textures_value".to_string(),
                    signature: "textures_signature".to_string(),
                }],
            }
        );
    }

    #[test]
    fn extract_bungeecord_data_too_short() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2"
                .to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract_bungeecord_data(&handshake).err().unwrap(),
            Error::BungeeSpecMismatch("Incorrect length".to_string())
        );
    }

    #[test]
    fn extract_bungeecord_data_too_long() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0a\0b"
                .to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract_bungeecord_data(&handshake).err().unwrap(),
            Error::BungeeSpecMismatch("Incorrect length".to_string())
        );
    }

    #[test]
    fn extract_bungeecord_data_localhost_host_ip() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "localhost\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract_bungeecord_data(&handshake).unwrap(),
            BungeeCordData {
                host: "localhost".to_string(),
                client: "192.168.1.67".to_string(),
                uuid: Uuid::parse_str("905c7e4fb96b45139645d123225575e2").unwrap(),
                properties: vec![ProfileProperty {
                    name: "textures".to_string(),
                    value: "textures_value".to_string(),
                    signature: "textures_signature".to_string(),
                }],
            }
        );
    }

    #[test]
    fn extract_bungeecord_data_localhost_client_ip() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0localhost\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract_bungeecord_data(&handshake).unwrap(),
            BungeeCordData {
                host: "192.168.1.87".to_string(),
                client: "localhost".to_string(),
                uuid: Uuid::parse_str("905c7e4fb96b45139645d123225575e2").unwrap(),
                properties: vec![ProfileProperty {
                    name: "textures".to_string(),
                    value: "textures_value".to_string(),
                    signature: "textures_signature".to_string(),
                }],
            }
        );
    }

    #[test]
    fn extract_bungeecord_data_invalid_uuid() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\005c7e4fb9675e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        let error = extract_bungeecord_data(&handshake).err().unwrap();
        if let Error::BungeeSpecMismatch(e) = error {
            assert!(e.contains("invalid length"));
        } else {
            panic!();
        }
    }

    #[test]
    fn extract_bungeecord_data_invalid_properties() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"sinature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        let error = extract_bungeecord_data(&handshake).err().unwrap();
        if let Error::BungeeSpecMismatch(e) = error {
            assert!(e.contains("missing field `signature`"));
        } else {
            panic!();
        }
    }

    #[test]
    fn test_initial_handler_new() {
        let mut ih = ih();

        assert!(ih.actions_to_execute().is_empty());
    }

    #[tokio::test]
    async fn test_status_ping() {
        let player_count = 24;
        let mut ih = ih_with_player_count(player_count);

        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: String::default(), // Unused - server address
            server_port: 25565,
            next_state: HandshakeState::Status,
        };
        ih.handle_packet(Box::new(handshake)).await;

        // Confirm that stage was switched and no other actions were performed
        let actions = ih.actions_to_execute();
        assert_eq!(actions.len(), 1);
        match actions.first().unwrap() {
            Action::SetStage(stage) => assert_eq!(*stage, PacketStage::Status),
            _ => panic!(),
        }

        let request = Request {};
        ih.handle_packet(Box::new(request)).await;

        let mut actions = ih.actions_to_execute();

        // Confirm that correct response was received
        assert_eq!(actions.len(), 1);

        let response = actions.remove(0);
        match response {
            Action::SendPacket(response) => {
                assert_eq!(response.ty(), PacketType::Response);

                let response = cast_packet::<Response>(response);
                let _: serde_json::Value = serde_json::from_str(&response.json_response).unwrap();
            }
            _ => panic!(),
        }

        // Send ping
        let payload = 39842;
        let ping = Ping { payload };
        ih.handle_packet(Box::new(ping)).await;

        let mut actions = ih.actions_to_execute();

        assert_eq!(actions.len(), 2);
        let pong = actions.remove(0);
        match pong {
            Action::SendPacket(pong) => {
                assert_eq!(pong.ty(), PacketType::Pong);
                let pong = cast_packet::<Pong>(pong);
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

    #[tokio::test]
    async fn test_login_sequence() {
        let mut config = Config::default();
        config.server.online_mode = false;
        let mut ih = ih_with_config(config.clone());

        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: String::default(), // Unused - server address
            server_port: 25565,
            next_state: HandshakeState::Login,
        };
        ih.handle_packet(Box::new(handshake)).await;

        let actions = ih.actions_to_execute();
        assert_eq!(actions.len(), 1);
        match actions.first().unwrap() {
            Action::SetStage(stage) => assert_eq!(*stage, PacketStage::Login),
            _ => panic!(),
        }

        let username = "test";
        let login_start = LoginStart {
            username: String::from(username),
        };
        ih.handle_packet(Box::new(login_start)).await;

        let mut actions = ih.actions_to_execute();
        assert_eq!(actions.len(), 5);

        let set_compression = actions.remove(0);

        match set_compression {
            Action::SendPacket(set_compression) => {
                assert_eq!(set_compression.ty(), PacketType::SetCompression);

                let set_compression = cast_packet::<SetCompression>(set_compression);
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

        let login_success = actions.remove(0);

        match login_success {
            Action::SendPacket(login_success) => {
                assert_eq!(login_success.ty(), PacketType::LoginSuccess);

                let login_success = cast_packet::<LoginSuccess>(login_success);
                assert_eq!(login_success.username, username.to_string());
            }
            _ => panic!(),
        }

        match actions.remove(0) {
            Action::SetStage(stage) => assert_eq!(stage, PacketStage::Play),
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
            Arc::new(RwLock::new(BanInfo::default())),
            Arc::new(AtomicU32::new(0)),
            Arc::new(Some(String::from("test"))),
            "127.0.0.1:8080".parse().unwrap(),
        )
    }

    fn ih_with_player_count(count: u32) -> InitialHandler {
        InitialHandler::new(
            Arc::new(Config::default()),
            Arc::new(RwLock::new(BanInfo::default())),
            Arc::new(AtomicU32::new(count)),
            Arc::new(Some(String::from("test"))),
            "127.0.0.1:8080".parse().unwrap(),
        )
    }

    fn ih_with_config(config: Config) -> InitialHandler {
        InitialHandler::new(
            Arc::new(config),
            Arc::new(RwLock::new(BanInfo::default())),
            Arc::new(AtomicU32::new(0)),
            Arc::new(Some(String::from("test"))),
            "127.0.0.1:8080".parse().unwrap(),
        )
    }
}
