use super::{initial_handling, FromWorker, NewPlayer, ToWorker, WorkerHandle};
use crate::Server;
use anyhow::bail;
use flume::{Receiver, Sender};
use future::Either;
use futures_util::future;
use initial_handling::InitialHandling;
use protocol::{ClientPlayPacket, MinecraftCodec, Readable, Writeable};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::TcpStream;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

/// The worker task which handles connections.
pub struct Worker {
    stream: TcpStream,
    addr: SocketAddr,
    write_buffer: Vec<u8>,
    codec: MinecraftCodec,

    to_worker: (Sender<ToWorker>, Receiver<ToWorker>),
    from_worker: (Sender<FromWorker>, Receiver<FromWorker>),
    new_players: Sender<NewPlayer>,

    server: Server,
}

impl Worker {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        server: &Server,
        new_players: Sender<NewPlayer>,
    ) -> Self {
        let to_worker = flume::bounded(64);
        let from_worker = flume::bounded(64);
        let codec = MinecraftCodec::new();
        Self {
            stream,
            addr,
            codec,
            write_buffer: Vec::new(),
            to_worker,
            from_worker,
            new_players,
            server: Arc::clone(server),
        }
    }

    /// Creates a new handle to this worker.
    pub fn handle(&self) -> WorkerHandle {
        WorkerHandle {
            sender: self.to_worker.0.clone(),
            receiver: self.from_worker.1.clone(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    pub fn codec(&mut self) -> &mut MinecraftCodec {
        &mut self.codec
    }

    /// Writes a packet to the stream.
    pub async fn write<T>(&mut self, packet: &T) -> anyhow::Result<()>
    where
        T: Writeable,
    {
        log::trace!("Writing packet of type {}", std::any::type_name::<T>());
        self.codec.encode(packet, &mut self.write_buffer);
        self.stream.write_all(&self.write_buffer).await?;
        self.write_buffer.clear();
        Ok(())
    }

    /// Reads a packet from the stream.
    pub async fn read<T>(&mut self) -> anyhow::Result<T>
    where
        T: Readable,
    {
        read(&mut self.stream, &mut self.codec).await
    }

    /// Runs the worker. Returns once the connection is lost.
    pub async fn run(mut self) -> anyhow::Result<()> {
        match initial_handling::handle(&mut self).await? {
            InitialHandling::Disconnect => return Ok(()),
            InitialHandling::Join(player) => self.new_players.send_async(player).await?,
        }

        if let Err(e) = self.run_internal().await {
            log::debug!("Worker disconnected: {:?}", e);
            self.from_worker
                .0
                .send_async(FromWorker::Failed {
                    message: e.to_string(),
                })
                .await?;
        }
        Ok(())
    }

    async fn run_internal(&mut self) -> anyhow::Result<()> {
        loop {
            // Either send a packet or receive one.
            let action = {
                let message = self.to_worker.1.recv_async();
                let recv_packet = timeout(
                    Duration::from_secs(10),
                    read::<ClientPlayPacket>(&mut self.stream, &mut self.codec),
                );
                futures_util::pin_mut!(message);
                futures_util::pin_mut!(recv_packet);

                match future::select(message, recv_packet).await {
                    Either::Left((message, _)) => Either::Left(message),
                    Either::Right((recv_packet, _)) => Either::Right(recv_packet),
                }
            };

            match action {
                Either::Left(message) => match message? {
                    ToWorker::SendPacket(packet) => self.write(&packet).await?,
                    ToWorker::Disconnect => return Ok(()),
                },
                Either::Right(packet) => {
                    dbg!(&packet);
                    self.from_worker
                        .0
                        .send_async(FromWorker::PacketReceived(packet??))
                        .await?;
                }
            }
        }
    }
}

async fn read<T>(stream: &mut TcpStream, codec: &mut MinecraftCodec) -> anyhow::Result<T>
where
    T: Readable,
{
    let mut buf = [0u8; 256];
    loop {
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            bail!("end of stream");
        }

        if let Some(packet) = codec.decode(&buf[..bytes_read])? {
            log::trace!("Read packet of type {}", std::any::type_name::<T>());
            return Ok(packet);
        }
    }
}
