use super::{worker::Worker, ListenerHandle, NewPlayer};
use crate::Server;
use anyhow::Context;
use flume::{Receiver, Sender};
use future::Either;
use futures_util::future;
use std::{net::SocketAddr, sync::Arc};
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

    server: Server,
}

impl Listener {
    /// Creates a new `Listener` which will listen on the provided address.
    pub fn new(
        addr: SocketAddr,
        runtime: &runtime::Handle,
        server: &Server,
    ) -> anyhow::Result<Self> {
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
            server: Arc::clone(server),
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
            let res = {
                let accept = self.listener.accept();
                let shutdown = self.shutdown.1.recv_async();
                futures_util::pin_mut!(accept);
                futures_util::pin_mut!(shutdown);

                match future::select(accept, shutdown).await {
                    Either::Left((res, _)) => res,
                    Either::Right(_) => {
                        log::info!("Closing listener");
                        return;
                    }
                }
            };

            if let Ok((stream, addr)) = res {
                self.spawn_worker(stream, addr);
            }
        }
    }

    fn spawn_worker(&self, stream: TcpStream, addr: SocketAddr) {
        let worker = Worker::new(stream, addr, &self.server, self.new_players.0.clone());
        self.runtime.spawn(async move {
            if let Err(e) = worker.run().await {
                log::warn!("Connection handling failed: {:?}", e);
            }
        });
    }
}
