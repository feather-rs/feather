//! Listener Tokio task.
//!
//! This task listens on a `TcpListener` and accepts
//! connections, spawning worker tasks to handle them,4.

use crate::config::Config;
use crate::io::worker::run_worker;
use crate::io::{ListenerToServerMessage, ServerToListenerMessage};
use crate::packet_buffer::PacketBuffers;
use futures::channel::mpsc;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn run_listener(
    address: SocketAddr,
    tx: crossbeam::Sender<ListenerToServerMessage>,
    rx: mpsc::UnboundedReceiver<ServerToListenerMessage>,
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
                info!("Failed to accept connection: {:?}", e);
                continue;
            }
        };

        info!("Connection received from {}", ip);

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
