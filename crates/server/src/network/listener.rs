use super::{worker::Worker, ListenerHandle, NewPlayer};
use crate::Server;
use anyhow::Context;
use async_executor::Executor;
use async_io::Async;
use either::Either;
use flume::{Receiver, Sender};
use futures_lite::future;
use std::{net::SocketAddr, net::TcpListener, net::TcpStream, sync::Arc};

/// The listener task.
pub struct Listener {
    new_players: (Sender<NewPlayer>, Receiver<NewPlayer>),
    shutdown: (Sender<()>, Receiver<()>),

    listener: Async<TcpListener>,
    executor: Arc<Executor<'static>>,

    server: Server,
}

impl Listener {
    /// Creates a new `Listener` which will listen on the provided address.
    pub fn new(
        addr: SocketAddr,
        executor: &Arc<Executor<'static>>,
        server: &Server,
    ) -> anyhow::Result<Self> {
        let listener = std::net::TcpListener::bind(addr).with_context(|| {
            format!("failed to bind to {}. Is the server already running?", addr)
        })?;
        let listener = Async::new(listener)?;

        let new_players = flume::unbounded();
        let shutdown = flume::unbounded();

        Ok(Self {
            new_players,
            shutdown,
            listener,
            executor: Arc::clone(executor),
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
    pub async fn run(self) {
        loop {
            let res = {
                let accept = self.listener.accept();
                let shutdown = self.shutdown.1.recv_async();

                match future::race(async move { Either::Left(accept.await) }, async move {
                    Either::Right(shutdown.await)
                })
                .await
                {
                    Either::Left(accept) => accept,
                    Either::Right(_) => break,
                }
            };

            if let Ok((stream, addr)) = res {
                self.spawn_worker(stream, addr);
            }
        }
    }

    fn spawn_worker(&self, stream: Async<TcpStream>, addr: SocketAddr) {
        let worker = Worker::new(stream, addr, &self.server, self.new_players.0.clone());
        self.executor
            .spawn(async move {
                if let Err(e) = worker.run().await {
                    log::debug!("Connection handling failed: {:?}", e);
                }
            })
            .detach();
    }
}
