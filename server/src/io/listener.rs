//! Listener Tokio task.
//!
//! This task listens on a `TcpListener` and accepts
//! connections, spawning worker tasks to handle them,4.

use crate::config::Config;
use crate::io::worker::run_worker;
use crate::io::ListenerToServerMessage;
use crate::PlayerCount;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io;
use tokio::net::TcpListener;

pub async fn run_listener(
    address: SocketAddr,
    sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) -> Result<(), io::Error> {
    let mut listener = TcpListener::bind(address).await?;

    loop {
        let (stream, ip) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                debug!("Failed to accept connection: {:?}", e);
                continue;
            }
        };

        debug!("Connection received from {}", ip);

        tokio::spawn(run_worker(
            stream,
            ip,
            sender.clone(),
            Arc::clone(&config),
            Arc::clone(&player_count),
            Arc::clone(&server_icon),
        ));
    }
}
