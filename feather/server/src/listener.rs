use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use flume::Sender;
use tokio::net::{TcpListener, TcpStream};

use crate::{
    connection_worker::Worker, initial_handler::NewPlayer, options::Options,
    player_count::PlayerCount,
};

/// Listens for and accepts incoming connections.
pub struct Listener {
    listener: TcpListener,
    options: Arc<Options>,
    player_count: PlayerCount,
    new_players: Sender<NewPlayer>,
}

impl Listener {
    pub async fn start(
        options: Arc<Options>,
        player_count: PlayerCount,
        new_players: Sender<NewPlayer>,
    ) -> anyhow::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", options.bind_address, options.port))
            .await
            .context("failed to bind to port - maybe a server is already running?")?;

        let listener = Listener {
            listener,
            options,
            player_count,
            new_players,
        };
        tokio::task::spawn(async move {
            listener.run().await;
        });

        Ok(())
    }

    async fn run(mut self) {
        loop {
            if let Ok((stream, addr)) = self.listener.accept().await {
                self.accept(stream, addr).await;
            }
        }
    }

    async fn accept(&mut self, stream: TcpStream, addr: SocketAddr) {
        let worker = Worker::new(
            stream,
            addr,
            Arc::clone(&self.options),
            self.player_count.clone(),
            self.new_players.clone(),
        );
        worker.start();
    }
}
