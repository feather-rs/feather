use config::Config;
use simple_logger::SimpleLogger;
use std::{
    fs::File,
    sync::{atomic::AtomicUsize, Arc},
};
use tokio::runtime;

mod config;
mod network;

/// Shared server state. Stored as a resource.
pub type Server = Arc<ServerInner>;

pub struct ServerInner {
    /// The number of online players.
    pub player_count: AtomicUsize,
    /// The server configuration.
    pub config: Config,
    /// The server icon, base64-encoded.
    pub icon: Option<String>,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();
    let runtime = runtime::Builder::new()
        .threaded_scheduler()
        .enable_io()
        .build()?;

    let config = Config::load_from_file(&mut File::open("config.toml")?)?;
    let server = Arc::new(ServerInner {
        player_count: AtomicUsize::new(0),
        config,
        icon: None,
    });

    runtime.enter(|| {
        let listener = network::Listener::new(
            "127.0.0.1:25565".parse().unwrap(),
            runtime.handle(),
            &server,
        )?;
        runtime.handle().block_on(async move {
            listener.run().await;
        });
        Result::<(), anyhow::Error>::Ok(())
    })?;

    Ok(())
}
