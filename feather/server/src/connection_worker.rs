use std::{fmt::Debug, io, net::SocketAddr, sync::Arc, time::Duration};

use base::Text;
use flume::{Receiver, Sender};
use futures_lite::FutureExt;
use io::ErrorKind;
use protocol::{
    codec::CryptKey, packets::server::Disconnect, ClientPlayPacket, MinecraftCodec, Readable,
    ServerPlayPacket, Writeable,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    time::timeout,
};

use crate::{
    initial_handler::{InitialHandling, NewPlayer},
    options::Options,
    player_count::PlayerCount,
};

/// Tokio task which handles a connection and processes
/// packets.
///
/// # Lifecycle
/// * A connection is made, and the `Listener` spawns a `Worker`.
/// * Connection goes through initial handling, i.e., the handshake process.
/// * If the connection was not a status ping, then the main server thread
/// is notified of the new connection via a channel.
pub struct Worker {
    reader: Reader,
    writer: Writer,
    options: Arc<Options>,
    player_count: PlayerCount,
    packets_to_send_tx: Sender<ServerPlayPacket>,
    received_packets_rx: Receiver<ClientPlayPacket>,
    new_players: Sender<NewPlayer>,
}

impl Worker {
    pub fn new(
        stream: TcpStream,
        _addr: SocketAddr,
        options: Arc<Options>,
        player_count: PlayerCount,
        new_players: Sender<NewPlayer>,
    ) -> Self {
        let (reader, writer) = stream.into_split();

        let (received_packets_tx, received_packets_rx) = flume::bounded(32);
        let (packets_to_send_tx, packets_to_send_rx) = flume::unbounded();
        let reader = Reader::new(reader, received_packets_tx);
        let writer = Writer::new(writer, packets_to_send_rx);

        Self {
            reader,
            writer,
            options,
            player_count,
            packets_to_send_tx,
            received_packets_rx,
            new_players,
        }
    }

    pub fn start(self) {
        tokio::task::spawn(async move {
            self.run().await;
        });
    }

    async fn run(mut self) {
        let result = crate::initial_handler::handle(&mut self).await;
        match result {
            Ok(result) => self.proceed(result).await,
            Err(e) => log::debug!("Initial handling failed: {:?}", e),
        }
    }

    async fn proceed(mut self, result: InitialHandling) {
        match result {
            InitialHandling::Disconnect => (),
            InitialHandling::Join(new_player) => {
                if self.player_count.try_add_player().is_err() {
                    self.write(ServerPlayPacket::Disconnect(Disconnect {
                        reason: Text::from("The server is full!").to_string(),
                    }))
                    .await
                    .ok();
                    return;
                }

                let username = new_player.username.clone();
                let _ = self.new_players.send_async(new_player).await;
                self.split(username);
            }
        }
    }

    pub fn options(&self) -> &Options {
        &self.options
    }

    pub fn player_count(&self) -> u32 {
        self.player_count.get()
    }

    #[allow(unused)]
    pub fn enable_compression(&mut self, threshold: usize) {
        self.reader.codec.enable_compression(threshold);
        self.writer.codec.enable_compression(threshold);

        log::debug!("Enabled compression");
    }

    pub fn enable_encryption(&mut self, key: CryptKey) {
        self.reader.codec.enable_encryption(key);
        self.writer.codec.enable_encryption(key);

        log::debug!("Enabled encryption");
    }

    pub async fn read<P: Readable>(&mut self) -> anyhow::Result<P> {
        self.reader.read().await
    }

    pub async fn write(&mut self, packet: impl Writeable + Debug) -> anyhow::Result<()> {
        self.writer.write(packet).await
    }

    pub fn split(self, username: String) {
        let Self {
            reader,
            writer,
            player_count,
            ..
        } = self;
        let reader = tokio::task::spawn(async move { reader.run().await });
        let writer = tokio::task::spawn(async move { writer.run().await });

        tokio::task::spawn(async move {
            let result = reader.race(writer).await.expect("task panicked");
            if let Err(e) = result {
                let message = disconnected_message(e);
                log::debug!("{} lost connection: {}", username, message);
            }
            player_count.remove_player();
        });
    }

    pub fn packets_to_send(&self) -> Sender<ServerPlayPacket> {
        self.packets_to_send_tx.clone()
    }

    pub fn received_packets(&self) -> Receiver<ClientPlayPacket> {
        self.received_packets_rx.clone()
    }
}

struct Reader {
    stream: OwnedReadHalf,
    codec: MinecraftCodec,
    buffer: [u8; 512],
    received_packets: Sender<ClientPlayPacket>,
}

impl Reader {
    pub fn new(stream: OwnedReadHalf, received_packets: Sender<ClientPlayPacket>) -> Self {
        Self {
            stream,
            codec: MinecraftCodec::new(),
            buffer: [0; 512],
            received_packets,
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            let packet = self.read::<ClientPlayPacket>().await?;
            let result = self.received_packets.send_async(packet).await;
            if result.is_err() {
                // server dropped connection
                return Ok(());
            }
        }
    }

    pub async fn read<P: Readable>(&mut self) -> anyhow::Result<P> {
        // Keep reading bytes and trying to get the packet.
        loop {
            if let Some(packet) = self.codec.next_packet::<P>()? {
                return Ok(packet);
            }

            let duration = Duration::from_secs(10);
            let read_bytes = timeout(duration, self.stream.read(&mut self.buffer)).await??;
            if read_bytes == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "read 0 bytes").into());
            }

            let bytes = &self.buffer[..read_bytes];
            self.codec.accept(bytes);
        }
    }
}

struct Writer {
    stream: OwnedWriteHalf,
    codec: MinecraftCodec,
    packets_to_send: Receiver<ServerPlayPacket>,
    buffer: Vec<u8>,
}

impl Writer {
    pub fn new(stream: OwnedWriteHalf, packets_to_send: Receiver<ServerPlayPacket>) -> Self {
        Self {
            stream,
            codec: MinecraftCodec::new(),
            packets_to_send,
            buffer: Vec::new(),
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Ok(packet) = self.packets_to_send.recv_async().await {
            self.write(packet).await?;
        }
        Ok(())
    }

    pub async fn write(&mut self, packet: impl Writeable + Debug) -> anyhow::Result<()> {
        self.codec.encode(&packet, &mut self.buffer)?;
        self.stream.write_all(&self.buffer).await?;
        self.buffer.clear();
        Ok(())
    }
}

fn disconnected_message(e: anyhow::Error) -> String {
    if let Some(io_error) = e.downcast_ref::<io::Error>() {
        if io_error.kind() == ErrorKind::UnexpectedEof {
            return "disconnected".to_owned();
        }
    }
    format!("{:?}", e)
}
