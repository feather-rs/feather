use std::time::Duration;

/// Number of updates (ticks) to do per second.
pub const TPS: u32 = 20;
/// The number of milliseconds per tick.
pub const TICK_MILLIS: u32 = 1000 / TPS;
/// The duration of a tick.
pub const TICK_DURATION: Duration = Duration::from_millis(TICK_MILLIS as u64);

/// Default port for Minecraft servers.
pub const DEFAULT_PORT: u16 = 25565;

/// The protocol version number
pub const PROTOCOL_VERSION: i32 = 757;
/// Vanilla server URL
pub const SERVER_DOWNLOAD_URL: &str =
    "https://launcher.mojang.com/v1/objects/125e5adf40c659fd3bce3e66e67a16bb49ecc1b9/server.jar";
/// Minecraft version of this server in format x.y.z (1.16.5, 1.18.1)
pub const VERSION_STRING: &str = "1.18.1";
/// World save version compatible with this version of the server
pub const ANVIL_VERSION: i32 = 2865;
