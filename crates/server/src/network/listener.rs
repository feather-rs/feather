use super::{worker::Worker, ListenerHandle, NewPlayer};
use anyhow::Context;
use flume::{Receiver, Sender};
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    runtime,
};

/// The listener task.
pub struct Listener {
    new_players: (Sender<NewPlayer>, Receiver<NewPlayer>),
    shutdown: (Sender<()>, Receiver<()>),

    listener: TcpListener,
    runtime: runtime::Handle,
}

impl Listener {
    /// Creates a new `Listener` which will listen on the provided address.
    pub fn new(addr: SocketAddr, runtime: &runtime::Handle) -> anyhow::Result<Self> {
        let listener = std::net::TcpListener::bind(addr).with_context(|| {
            format!("failed to bind to {}. Is the server already running?", addr)
        })?;
        let listener = tokio::net::TcpListener::from_std(listener)?;

        let new_players = flume::unbounded();
        let shutdown = flume::unbounded();

        Ok(Self {
            new_players,
            shutdown,
            listener,
            runtime: runtime.clone(),
        })
    }

    /// Creates a new handle to this listener.
    pub fn handle(&self) -> ListenerHandle {
        ListenerHandle {
            shutdown: self.shutdown.0.clone(),
            new_players: self.new_players.1.clone(),
        }
    }

    /// Runs the listener. Returns after the shutdown channel was notified.
    pub async fn run(mut self) {
        loop {
            if let Ok((stream, addr)) = self.listener.accept().await {
                log::debug!("Accepting connection from {}", addr);
                self.spawn_worker(stream, addr);
            }
        }
    }

    fn spawn_worker(&self, stream: TcpStream, addr: SocketAddr) {
        let worker = Worker::new(stream, addr);
        self.runtime.spawn(async move {
            if let Err(e) = worker.run().await {
                log::warn!("Connection handling failed: {:?}", e);
            }
        });
    }
}
