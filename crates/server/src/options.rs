use base::DEFAULT_PORT;

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
}

impl Default for Options {
    fn default() -> Self {
        Self {
            port: 25569,
            bind_address: "0.0.0.0".to_owned(),
            favicon: None,
            motd: "A Feather server".to_owned(),
            online_mode: false,
            view_distance: 8,
        }
    }
}
