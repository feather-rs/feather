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
use tokio::net::{TcpListener, TcpStream};

pub async fn run_listener(
    address: SocketAddr,
    sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) -> Result<(), io::Error> {
    let mut listener = TcpListener::bind(address).await?;

    loop {
        let (stream, ip) = listener.accept().await?;

        tokio::spawn(handle_connection(
            stream,
            ip,
            sender.clone(),
            Arc::clone(&config),
            Arc::clone(&player_count),
            Arc::clone(&server_icon),
        ));
    }
}

async fn handle_connection(
    stream: TcpStream,
    ip: SocketAddr,
    sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) {
    let _ = run_worker(stream, ip, sender, config, player_count, server_icon).await;
}
