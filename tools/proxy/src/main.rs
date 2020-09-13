use std::net::{SocketAddr, TcpListener, TcpStream};

use anyhow::{bail, Context};
use argh::FromArgs;
use async_executor::Executor;
use async_io::{block_on, Async};
use either::Either;
use feather_protocol::{
    packets::client::HandshakeState, ClientHandshakePacket, ClientPacket, ClientPacketCodec,
    ProtocolState, ServerLoginPacket, ServerPacket, ServerPacketCodec,
};
use futures_lite::FutureExt;
use futures_lite::{AsyncReadExt, AsyncWriteExt};
use simple_logger::SimpleLogger;

/// A proxy for debugging and inspecting the Minecraft protocol.
#[derive(Debug, FromArgs)]
struct Args {
    /// the port to listen on. Must be different from
    /// the vanilla server's port.
    #[argh(option, short = 'p')]
    port: u16,
    /// the port of the server to proxy to.
    #[argh(option, short = 's')]
    server_port: u16,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    let args: Args = argh::from_env();

    let addr = format!("127.0.0.1:{}", args.port);
    let listener = TcpListener::bind(&addr)
        .with_context(|| format!("failed to bind to port {}", args.port))?;
    let mut listener = Async::new(listener)?;

    log::info!("Listening on {}", addr);

    let executor = Executor::new();
    block_on(executor.run(async {
        accept_connections(&executor, &mut listener, args.server_port).await;
    }));

    Ok(())
}

async fn accept_connections(
    executor: &Executor,
    listener: &mut Async<TcpListener>,
    server_port: u16,
) {
    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(con) => con,
            Err(e) => {
                log::error!("Failed to accept connection: {}", e);
                continue;
            }
        };
        log::info!("Accepting connection from {}", addr);

        executor
            .spawn(async move {
                if let Err(e) = handle_connection(stream, server_port).await {
                    log::error!("Connection dropped: {:?}", e);
                }
            })
            .detach();
    }
}

struct Connection {
    /// The client's username.
    username: String,

    client_buffer: [u8; 256],
    client_codec: ClientPacketCodec,
    client: Async<TcpStream>,

    server_buffer: [u8; 256],
    server_codec: ServerPacketCodec,
    server: Async<TcpStream>,
}

impl Connection {
    fn set_state(&mut self, state: ProtocolState) {
        log::info!("{}: switching to state {:?}", self.username, state);
        self.client_codec.set_state(state);
        self.server_codec.set_state(state);
    }
}

const MAX_PACKET_DISPLAY: usize = 4000;

async fn handle_connection(client: Async<TcpStream>, server_port: u16) -> anyhow::Result<()> {
    let server =
        Async::<TcpStream>::connect(format!("127.0.0.1:{}", server_port).parse::<SocketAddr>()?)
            .await
            .with_context(|| format!("failed to connect to server at localhost:{}", server_port))?;

    let mut connection = Connection {
        username: String::from("<unknown>"),

        client_buffer: [0; 256],
        client_codec: ClientPacketCodec::new(),
        client,

        server_buffer: [0; 256],
        server_codec: ServerPacketCodec::new(),
        server,
    };

    let mut buffer = Vec::new();

    loop {
        let client = &mut connection.client;
        let client_buffer = &mut connection.client_buffer;
        let server = &mut connection.server;
        let server_buffer = &mut connection.server_buffer;

        // Attempt to read data from either the server or the client.
        let recv_client = async { Either::Left(client.read(client_buffer).await) };
        let recv_server = async { Either::Right(server.read(server_buffer).await) };
        match recv_client.race(recv_server).await {
            Either::Left(client) => {
                let mut bytes_read = client?;
                if bytes_read == 0 {
                    bail!("disconnected from client");
                }

                while let Some(packet) = connection
                    .client_codec
                    .decode(&connection.client_buffer[..bytes_read])
                    .context("failed to decode client packet")?
                {
                    bytes_read = 0;
                    let mut packet_str = format!("{:?}", packet);
                    if packet_str.len() > MAX_PACKET_DISPLAY {
                        packet_str = format!("{} <snip>", &packet_str[..MAX_PACKET_DISPLAY]);
                    }
                    log::info!("client @ {}: {}", connection.username, packet_str);

                    // Attempt to detect state switches.
                    if let ClientPacket::Handshake(packet) = &packet {
                        let state = match packet {
                            ClientHandshakePacket::Handshake(packet) => match packet.next_state {
                                HandshakeState::Login => ProtocolState::Login,
                                HandshakeState::Status => ProtocolState::Status,
                            },
                        };
                        connection.set_state(state)
                    }

                    // Forward the packet to the server.
                    connection.client_codec.encode(&packet, &mut buffer);
                    connection.server.write_all(&buffer).await?;
                    buffer.clear();
                }
            }
            Either::Right(server) => {
                let mut bytes_read = server?;
                if bytes_read == 9 {
                    bail!("disconnected from server");
                }

                while let Some(packet) = connection
                    .server_codec
                    .decode(&connection.server_buffer[..bytes_read])
                    .context("failed to decode server packet")?
                {
                    bytes_read = 0;
                    let mut packet_str = format!("{:?}", packet);
                    if packet_str.len() > MAX_PACKET_DISPLAY {
                        packet_str = format!("{} <snip>", &packet_str[..MAX_PACKET_DISPLAY]);
                    }
                    log::info!("server @ {}: {}", connection.username, packet_str);

                    // Attempt to detect state switches.
                    if let ServerPacket::Login(ServerLoginPacket::LoginSuccess(packet)) = &packet {
                        connection.username = packet.username.clone();
                        connection.set_state(ProtocolState::Play);
                    }

                    // Forward the packet to the client.
                    connection.server_codec.encode(&packet, &mut buffer);
                    connection.client.write_all(&buffer).await?;
                    buffer.clear();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_args() {
        let args = Args::from_args(&["minecraft-proxy"], &["-p", "25565", "-s", "25566"]).unwrap();
        assert_eq!(args.port, 25565);
        assert_eq!(args.server_port, 25566);
    }

    #[test]
    fn parse_cli_args_long_port() {
        let args = Args::from_args(
            &["minecraft-proxy"],
            &["--port", "25565", "--server-port", "25566"],
        )
        .unwrap();
        assert_eq!(args.port, 25565);
        assert_eq!(args.server_port, 25566);
    }
}
