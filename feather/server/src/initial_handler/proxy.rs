//! Proxy support for BungeeCord and Velocity.

use base::ProfileProperty;
use protocol::packets::client::Handshake;
use uuid::Uuid;

use crate::connection_worker::Worker;

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
pub fn do_bungee_ip_forwarding(handshake: &Handshake) -> anyhow::Result<ProxyData> {
    bungeecord::extract(handshake)
}

pub async fn do_velocity_ip_forwarding(worker: &mut Worker) -> anyhow::Result<ProxyData> {
    velocity::run(worker).await
}
