use base::Gamemode;

use crate::favicon::Favicon;

/// Options for building a [`Server`](crate::Server).
#[derive(Debug, Clone)]
pub struct Options {
    /// Port to listen on.
    pub port: u16,
    /// Addresses to bind to.
    pub bind_address: String,

    /// The server favicon.
    pub favicon: Option<Favicon>,
    /// The server MOTD.
    pub motd: String,

    /// Whether the server should authenticate players.
    pub online_mode: bool,

    /// The maximum view distance, which determines
    /// how far players can see.
    pub view_distance: u32,

    /// Maximum number of players to allow on the server.
    pub max_players: u32,

    /// The default gamemode for new players.
    pub default_gamemode: Gamemode,

    /// Proxy IP forwarding mode
    pub proxy_mode: Option<ProxyMode>,
    // HMAC key used with Velocity IP forwarding.
    pub velocity_secret: String,

    /// Packet size threshold at which to compress data
    pub compression_threshold: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProxyMode {
    Bungeecord,
    Velocity,
}
