//! Proxy support for BungeeCord and Velocity.

use base::ProfileProperty;
use protocol::packets::client::Handshake;
use uuid::Uuid;

use crate::{connection_worker::Worker, options::ProxyMode};

mod bungeecord;
mod velocity;

/// IP forwarding data received from the proxy.
#[derive(Debug, PartialEq)]
pub struct ProxyData {
    /// IP address of the proxy.
    pub host: String,
    /// IP address of the client.
    pub client: String,
    /// Client UUID.
    pub uuid: Uuid,
    /// Client profile properties (skin).
    pub profile: Vec<ProfileProperty>,
}

/// Runs proxy forwarding and returns the client's `ProxyData`.
pub async fn do_ip_forwarding(
    worker: &mut Worker,
    mode: ProxyMode,
    handshake: &Handshake,
) -> anyhow::Result<ProxyData> {
    match mode {
        ProxyMode::Bungeecord => bungeecord::extract(handshake),
        ProxyMode::Velocity => todo!(),
    }
}
