//! Initial handling of a connection.

use super::{worker::Worker, NewPlayer};
use protocol::{
    packets::{
        client::{HandshakeState, LoginStart, Ping, Request},
        server::{Pong, Response},
    },
    ClientHandshakePacket, ServerStatusPacket,
};

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
pub async fn handle(provider: &mut Worker) -> anyhow::Result<InitialHandling> {
    // Get the handshake packet.
    let handshake = provider.read::<ClientHandshakePacket>().await?;

    let ClientHandshakePacket::Handshake(handshake) = handshake;

    match handshake.next_state {
        HandshakeState::Status => handle_status(provider).await,
        HandshakeState::Login => handle_login(provider).await,
    }
}

async fn handle_status(provider: &mut Worker) -> anyhow::Result<InitialHandling> {
    let _request = provider.read::<Request>().await?;

    // TODO: correctly fill in this information.
    let payload = serde_json::json!({
        "version": {
            "name": "Feather 1.16.2",
            "protocol": 751,
        },
        "players": {
            "max": 100,
            "online": 0
        },
        "description": {
            "text": "Working on it..."
        }
    });
    let response = Response {
        response: serde_json::to_string(&payload)?,
    };
    provider
        .write(&ServerStatusPacket::Response(response))
        .await?;

    if let Ok(ping) = provider.read::<Ping>().await {
        let pong = Pong {
            payload: ping.payload,
        };
        provider.write(&ServerStatusPacket::Pong(pong)).await?;
    }

    Ok(InitialHandling::Disconnect)
}

async fn handle_login(provider: &mut Worker) -> anyhow::Result<InitialHandling> {
    let _login_start = provider.read::<LoginStart>().await?;

    Ok(InitialHandling::Disconnect)
}
