//! Initial handling of a connection.

use crate::{connection_worker::Worker, favicon::Favicon};
use anyhow::bail;
use base::{ProfileProperty, Text};
use flume::{Receiver, Sender};
use md5::Digest;
use num_bigint::BigInt;
use once_cell::sync::Lazy;
use protocol::{
    codec::CryptKey,
    packets::{
        client::{HandshakeState, Ping},
        server::{
            DisconnectLogin, EncryptionRequest, LoginSuccess, Pong, Response, SetCompression,
        },
    },
    ClientHandshakePacket, ClientLoginPacket, ClientPlayPacket, ClientStatusPacket,
    ServerLoginPacket, ServerPlayPacket, ServerStatusPacket,
};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::convert::TryInto;
use uuid::Uuid;

use self::proxy::ProxyData;

const SERVER_NAME: &str = "Feather 1.16.5";
const PROTOCOL_VERSION: i32 = 754;

mod proxy;

/// Information for a newly connected player.
#[derive(Debug)]
pub struct NewPlayer {
    pub uuid: Uuid,
    pub username: String,
    pub profile: Vec<ProfileProperty>,

    pub received_packets: Receiver<ClientPlayPacket>,
    pub packets_to_send: Sender<ServerPlayPacket>,
}

/// Result of initial handling.
pub enum InitialHandling {
    /// The client should be disconnected (sent when
    /// the connection was just a "status" ping.)
    Disconnect,
    /// We should create a new player.
    Join(NewPlayer),
}

/// Handles a connection until the protocol state is switched to Play;
/// that is, until we send Login Success. Returns the client's information.
pub async fn handle(worker: &mut Worker) -> anyhow::Result<InitialHandling> {
    // Get the handshake packet.
    let handshake = worker.read::<ClientHandshakePacket>().await?;

    let ClientHandshakePacket::Handshake(handshake) = handshake;

    match handshake.next_state {
        HandshakeState::Status => handle_status(worker).await,
        HandshakeState::Login => {
            if handshake.protocol_version < PROTOCOL_VERSION {
                worker
                    .write(ServerLoginPacket::DisconnectLogin(DisconnectLogin {
                        reason: Text::from(
                            "Invalid protocol! The server is running on version 1.16!",
                        )
                        .to_string(),
                    }))
                    .await
                    .ok();
                return Ok(InitialHandling::Disconnect);
            }
            let proxy_data =
                if let Some(crate::options::ProxyMode::Bungeecord) = worker.options().proxy_mode {
                    Some(proxy::do_bungee_ip_forwarding(&handshake)?)
                } else {
                    None
                };
            handle_login(worker, proxy_data).await
        }
    }
}

#[derive(Debug, Serialize)]
struct StatusResponse<'a> {
    version: Version,
    players: Players,
    description: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct Version {
    name: &'static str,
    protocol: i32,
}

#[derive(Debug, Serialize)]
struct Players {
    max: u32,
    online: u32,
}

async fn handle_status(worker: &mut Worker) -> anyhow::Result<InitialHandling> {
    let _request = worker.read::<ClientStatusPacket>().await?;

    let payload = StatusResponse {
        version: Version {
            name: SERVER_NAME,
            protocol: PROTOCOL_VERSION,
        },
        players: Players {
            max: worker.options().max_players,
            online: worker.player_count(),
        },
        description: Text::from(worker.options().motd.clone()),
        favicon: worker
            .options()
            .favicon
            .as_ref()
            .map(Favicon::base64_encoded),
    };
    let response = Response {
        response: serde_json::to_string(&payload)?,
    };
    worker
        .write(&ServerStatusPacket::Response(response))
        .await?;

    match worker.read::<Ping>().await {
        Ok(ping) => {
            let pong = Pong {
                payload: ping.payload,
            };
            worker.write(&ServerStatusPacket::Pong(pong)).await?;
        }
        Err(e) => {
            log::debug!("Didn't receive ping packet from status call: {}", e);
        }
    }

    Ok(InitialHandling::Disconnect)
}

async fn handle_login(
    worker: &mut Worker,
    mut proxy_data: Option<ProxyData>,
) -> anyhow::Result<InitialHandling> {
    let login_start = match worker.read::<ClientLoginPacket>().await? {
        ClientLoginPacket::LoginStart(l) => l,
        _ => bail!("expected login start"),
    };
    log::debug!("{} is logging in", login_start.name);

    // Velocity IP forwarding runs after Login Start is received.
    if let Some(crate::options::ProxyMode::Velocity) = worker.options().proxy_mode {
        proxy_data = Some(proxy::do_velocity_ip_forwarding(worker).await?);
    }

    if worker.options().online_mode {
        enable_encryption(worker, login_start.name).await
    } else {
        let profile = match proxy_data {
            Some(proxy_data) => AuthResponse {
                id: proxy_data.uuid,
                name: login_start.name.clone(),
                properties: proxy_data.profile,
            },
            None => offline_mode_profile(login_start.name),
        };
        finish_login(worker, profile).await
    }
}

fn offline_mode_profile(username: String) -> AuthResponse {
    // TODO: correct offline mode handling
    AuthResponse {
        id: offline_mode_uuid(&username),
        name: username,
        properties: Vec::new(),
    }
}

fn offline_mode_uuid(username: &str) -> Uuid {
    // See: https://gist.github.com/games647/2b6a00a8fc21fd3b88375f03c9e2e603
    let mut hasher = md5::Md5::default();
    hasher.update(format!("OfflinePlayer:{}", username).as_bytes());
    let hash = hasher.finalize();

    let mut builder = uuid::Builder::from_bytes(hash.try_into().unwrap());

    builder
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Md5);

    builder.build()
}

const RSA_BITS: usize = 1024;

/// Cached RSA key used by this server instance.
static RSA_KEY: Lazy<RsaPrivateKey> =
    Lazy::new(|| RsaPrivateKey::new(&mut OsRng, RSA_BITS).expect("failed to create RSA key"));
static RSA_KEY_ENCODED: Lazy<Vec<u8>> = Lazy::new(|| {
    rsa_der::public_key_to_der(&RSA_KEY.n().to_bytes_be(), &RSA_KEY.e().to_bytes_be())
});

async fn enable_encryption(
    worker: &mut Worker,
    username: String,
) -> anyhow::Result<InitialHandling> {
    log::debug!("Authenticating {}", username);
    let shared_secret = do_encryption_handshake(worker).await?;
    worker.enable_encryption(shared_secret);

    let response = authenticate(shared_secret, username).await?;

    finish_login(worker, response).await
}

async fn do_encryption_handshake(worker: &mut Worker) -> anyhow::Result<CryptKey> {
    let verify_token: [u8; 16] = rand::random();
    let request = EncryptionRequest {
        server_id: String::new(), // always empty
        public_key: RSA_KEY_ENCODED.clone(),
        verify_token: verify_token.to_vec(),
    };
    worker
        .write(&ServerLoginPacket::EncryptionRequest(request))
        .await?;

    let response = match worker.read::<ClientLoginPacket>().await? {
        ClientLoginPacket::EncryptionResponse(r) => r,
        _ => bail!("expected encryption response"),
    };

    // Decrypt shared secret and verify token.
    let shared_secret = RSA_KEY.decrypt(PaddingScheme::PKCS1v15Encrypt, &response.shared_secret)?;
    let received_verify_token =
        RSA_KEY.decrypt(PaddingScheme::PKCS1v15Encrypt, &response.verify_token)?;

    if received_verify_token != verify_token {
        bail!("verify tokens do not match");
    }

    Ok((&shared_secret[..]).try_into()?)
}

#[derive(Debug, Deserialize)]
struct AuthResponse {
    id: Uuid,
    name: String,
    properties: Vec<ProfileProperty>,
}

async fn authenticate(shared_secret: CryptKey, username: String) -> anyhow::Result<AuthResponse> {
    let server_hash = compute_server_hash(shared_secret);

    let response: AuthResponse = tokio::task::spawn_blocking(move || {
        let url = format!(
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
            username, server_hash
        );
        let response = ureq::get(&url).call()?;

        Result::<AuthResponse, anyhow::Error>::Ok(response.into_json()?)
    })
    .await??;

    Ok(response)
}

fn compute_server_hash(shared_secret: CryptKey) -> String {
    let mut hasher = Sha1::new();
    hasher.update(b""); // server ID - always empty
    hasher.update(&shared_secret);
    hasher.update(&*RSA_KEY_ENCODED);
    hexdigest(hasher.finalize().as_slice())
}

// Non-standard hex digest used by Minecraft.
fn hexdigest(bytes: &[u8]) -> String {
    let bigint = BigInt::from_signed_bytes_be(bytes);
    format!("{:x}", bigint)
}

async fn finish_login(
    worker: &mut Worker,
    response: AuthResponse,
) -> anyhow::Result<InitialHandling> {
    enable_compression(worker).await?;

    let success = LoginSuccess {
        uuid: response.id,
        username: response.name.clone(),
    };
    worker
        .write(&ServerLoginPacket::LoginSuccess(success))
        .await?;

    let new_player = NewPlayer {
        username: response.name,
        uuid: response.id,
        profile: response.properties,
        received_packets: worker.received_packets(),
        packets_to_send: worker.packets_to_send(),
    };
    log::debug!("Completed initial handling for {}", new_player.username);
    Ok(InitialHandling::Join(new_player))
}

async fn enable_compression(worker: &mut Worker) -> anyhow::Result<()> {
    if let Some(threshold) = worker.options().compression_threshold {
        let packet = ServerLoginPacket::SetCompression(SetCompression {
            threshold: threshold as i32,
        });
        worker.write(&packet).await?;
        worker.enable_compression(threshold);
    }
    Ok(())
}
