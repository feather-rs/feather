use super::{initial_handling, FromWorker, ToWorker, WorkerHandle};
use crate::Server;
use flume::{Receiver, Sender};
use protocol::{MinecraftCodec, Readable, Writeable};
use std::{net::SocketAddr, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// The worker task which handles connections.
pub struct Worker {
    stream: TcpStream,
    addr: SocketAddr,
    write_buffer: Vec<u8>,
    codec: MinecraftCodec,

    to_worker: (Sender<ToWorker>, Receiver<ToWorker>),
    from_worker: (Sender<FromWorker>, Receiver<FromWorker>),

    server: Server,
}

impl Worker {
    pub fn new(stream: TcpStream, addr: SocketAddr, server: &Server) -> Self {
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
        let mut buf = [0u8; 256];
        loop {
            let bytes_read = self.stream.read(&mut buf).await?;
            if let Some(packet) = self.codec.decode(&buf[..bytes_read])? {
                log::trace!("Read packet of type {}", std::any::type_name::<T>());
                return Ok(packet);
            }
        }
    }

    /// Runs the worker. Returns once the connection is lost.
    pub async fn run(mut self) -> anyhow::Result<()> {
        initial_handling::handle(&mut self).await?;
        Ok(())
    }
}
