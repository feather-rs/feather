//! Listener Tokio task.
//!
//! This task listens on a `TcpListener` and accepts
//! connections, spawning worker tasks to handle them,4.

use crate::worker::run_worker;
use crate::{ListenerToServerMessage, ServerToListenerMessage};
use feather_server_types::{Config, PacketBuffers};
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn run_listener(
    address: SocketAddr,
    tx: flume::Sender<ListenerToServerMessage>,
    rx: flume::Receiver<ServerToListenerMessage>,
    config: Arc<Config>,
    player_count: Arc<AtomicU32>,
    server_icon: Arc<Option<String>>,
    packet_buffers: Arc<PacketBuffers>,
) -> Result<(), io::Error> {
    let mut listener = TcpListener::bind(address).await?;

    let rx = Arc::new(Mutex::new(rx));

    loop {
        let (stream, ip) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                log::info!("Failed to accept connection: {:?}", e);
                continue;
            }
        };

        log::info!("Connection received from {}", ip);

        tokio::spawn(run_worker(
            stream,
            ip,
            tx.clone(),
            Arc::clone(&rx),
            Arc::clone(&config),
            Arc::clone(&player_count),
            Arc::clone(&server_icon),
            Arc::clone(&packet_buffers),
        ));
    }
}
