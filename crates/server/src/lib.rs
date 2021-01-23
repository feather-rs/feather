use std::sync::Arc;

use listener::Listener;

mod connection_worker;
mod favicon;
mod initial_handler;
mod listener;
mod options;

pub use options::Options;

/// A Minecraft server.
///
/// Call [`register`] to register the server
/// with a [`Game`](common::Game). This will
/// cause the server to serve the game to players.
///
/// Uses asynchronous IO with Tokio.
pub struct Server {
    options: Arc<Options>,
}

impl Server {
    /// Starts a server with the given `Options`.
    pub async fn bind(options: Options) -> anyhow::Result<Self> {
        let options = Arc::new(options);
        Listener::start(Arc::clone(&options)).await?;
        Ok(Self { options })
    }
}
