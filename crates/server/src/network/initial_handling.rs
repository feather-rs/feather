//! Initial handling of a connection.

use super::{worker::Worker, NewPlayer};
use anyhow::bail;
use base::{ProfileProperty, Text};
use num_bigint::BigInt;
use once_cell::sync::Lazy;
use protocol::{
    codec::CryptKey,
    packets::{
        client::{HandshakeState, Ping},
        server::{EncryptionRequest, LoginSuccess, Pong, Response},
    },
    ClientHandshakePacket, ClientLoginPacket, ClientStatusPacket, ServerLoginPacket,
    ServerStatusPacket,
};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKeyParts, RSAPrivateKey};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::{convert::TryInto, sync::atomic::Ordering};
use uuid::Uuid;

const SERVER_NAME: &str = "Feather 1.16.3";
const PROTOCOL_VERSION: i32 = 753;

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
        HandshakeState::Login => handle_login(worker).await,
    }
}

#[derive(Debug, Serialize)]
struct StatusResponse<'a> {
    version: Version,
    players: Players,
    description: &'a Text,
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
    max: i32,
    online: usize,
}

async fn handle_status(worker: &mut Worker) -> anyhow::Result<InitialHandling> {
    let _request = worker.read::<ClientStatusPacket>().await?;

    // TODO: correctly fill in this information.
    let payload = StatusResponse {
        version: Version {
            name: SERVER_NAME,
            protocol: PROTOCOL_VERSION,
        },
        players: Players {
            max: worker.server().config.server.max_players,
            online: worker.server().player_count.load(Ordering::SeqCst),
        },
        description: &worker.server().config.server.motd,
        favicon: worker.server().icon.as_ref().map(String::as_str),
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

async fn handle_login(worker: &mut Worker) -> anyhow::Result<InitialHandling> {
    let login_start = match worker.read::<ClientLoginPacket>().await? {
        ClientLoginPacket::LoginStart(l) => l,
        _ => bail!("expected login start"),
    };
    log::debug!("{} is logging in", login_start.name);

    let config = &worker.server().config;
    if config.server.online_mode {
        enable_encryption(worker, login_start.name).await
    } else {
        let profile = compute_offline_mode_profile(login_start.name);
        finish_login(worker, profile).await
    }
}

fn compute_offline_mode_profile(username: String) -> AuthResponse {
    // TODO: correct offline mode handling
    AuthResponse {
        name: username,
        id: Uuid::new_v4(),
        properties: Vec::new(),
    }
}

const RSA_BITS: usize = 1024;

/// Cached RSA key used by this server instance.
static RSA_KEY: Lazy<RSAPrivateKey> =
    Lazy::new(|| RSAPrivateKey::new(&mut OsRng, RSA_BITS).expect("failed to create RSA key"));
static RSA_KEY_ENCODED: Lazy<Vec<u8>> = Lazy::new(|| {
    rsa_der::public_key_to_der(&RSA_KEY.n().to_bytes_be(), &RSA_KEY.e().to_bytes_be())
});

async fn enable_encryption(
    worker: &mut Worker,
    username: String,
) -> anyhow::Result<InitialHandling> {
    log::debug!("Authenticating {}", username);
    let shared_secret = do_encryption_handshake(worker).await?;
    worker.codec().enable_encryption(shared_secret);

    let response = authenticate(worker, shared_secret, username).await?;

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

async fn authenticate(
    worker: &Worker,
    shared_secret: CryptKey,
    username: String,
) -> anyhow::Result<AuthResponse> {
    let server_hash = compute_server_hash(shared_secret);

    let response: AuthResponse = worker
        .server()
        .blocking_pool
        .spawn(async move {
            let url = format!(
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
            username, server_hash
        );
            let response = ureq::get(&url).call();

            serde_json::from_reader(response.into_reader())
        })
        .await?;

    Ok(response)
}

fn compute_server_hash(shared_secret: CryptKey) -> String {
    let mut hasher = Sha1::new();
    hasher.update(b""); // server ID - always empty
    hasher.update(&shared_secret);
    hasher.update(&RSA_KEY_ENCODED);
    hexdigest(&hasher.digest().bytes())
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
    let success = LoginSuccess {
        uuid: response.id,
        username: response.name.clone(),
    };
    worker
        .write(&ServerLoginPacket::LoginSuccess(success))
        .await?;

    let new_player = NewPlayer {
        addr: worker.addr(),
        username: response.name.into(),
        uuid: response.id,
        worker: worker.handle(),
    };
    Ok(InitialHandling::Join(new_player))
}
