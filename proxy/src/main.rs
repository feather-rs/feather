use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::num::ParseIntError;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use clap::{ArgEnum, Parser};
use colored::Colorize;
use log::{Level, LevelFilter};
use time::macros::format_description;
use time::OffsetDateTime;

use feather_protocol::codec::CompressionThreshold;
use feather_protocol::packets::client::HandshakeState;
use feather_protocol::{
    ClientHandshakePacket, ClientPacket, ClientPacketCodec, ProtocolState, ServerLoginPacket,
    ServerPacket, ServerPacketCodec, VarInt,
};

/// A simple proxy server that logs transmitted packets
#[derive(Parser)]
#[clap(about, version)]
struct Args {
    /// The Minecraft server address (ip:port)
    #[clap(short = 'a', long)]
    server_address: SocketAddr,
    /// The address that the proxy should listen on (ip:port)
    #[clap(short, long)]
    proxy_address: SocketAddr,
    /// Only log clientside/serverside packets
    #[clap(arg_enum, short, long, default_value_t)]
    side: ConnectionSide,
    /// Show the contents of the packets. `-vv` will also display hexdump
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
    /// Don't log packets with the specified IDs (hexadecimal, comma-separated)
    #[clap(short, long, use_delimiter = true, parse(try_from_str = parse_hex), conflicts_with = "whitelist")]
    blacklist: Option<Vec<u32>>,
    /// Log only packets with the specified IDs (hexadecimal, comma-separated)
    #[clap(short, long, use_delimiter = true, parse(try_from_str = parse_hex), conflicts_with = "blacklist")]
    whitelist: Option<Vec<u32>>,
}

fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

#[derive(ArgEnum, Copy, Clone)]
enum ConnectionSide {
    Client,
    Server,
    Both,
}

impl Default for ConnectionSide {
    fn default() -> Self {
        ConnectionSide::Both
    }
}

fn main() {
    let args: Args = Args::parse();
    fern::Dispatch::new()
        .format(|out, message, record| {
            let level_string = match record.level() {
                Level::Error => record.level().to_string().red(),
                Level::Warn => record.level().to_string().yellow(),
                Level::Info => record.level().to_string().cyan(),
                Level::Debug => record.level().to_string().purple(),
                Level::Trace => record.level().to_string().normal(),
            };
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };
            let datetime: OffsetDateTime =
                OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            out.finish(format_args!(
                "{} {:<5} [{}] {}",
                datetime
                    .format(format_description!(
                        "[year]-[month]-[day] [hour]:[minute]:[second],[subsecond digits:3]"
                    ))
                    .unwrap(),
                level_string,
                target,
                message,
            ));
        })
        .level(match args.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let listener = TcpListener::bind(args.proxy_address).unwrap();

    log::info!("Listening on {}", args.proxy_address);
    while let Ok((client, addr)) = listener.accept() {
        log::info!("Accepting connection from {}", addr);

        let server = match TcpStream::connect(args.server_address) {
            Ok(server) => server,
            Err(err) => {
                log::error!(
                    "failed to connect to server at {}: {}",
                    args.server_address,
                    err
                );
                continue;
            }
        };

        let connection = Arc::new(Mutex::new(Connection {
            username: None,
            client_codec: ClientPacketCodec::new(),
            server_codec: ServerPacketCodec::new(),
        }));

        let client_read = client;
        let client_write = client_read.try_clone().unwrap();
        let server_read = server;
        let server_write = server_read.try_clone().unwrap();

        std::thread::spawn({
            let connection = connection.clone();
            let blacklist = args.blacklist.clone();
            let whitelist = args.whitelist.clone();
            move || match handle_client(
                matches!(args.side, ConnectionSide::Both | ConnectionSide::Client),
                blacklist,
                whitelist,
                &connection,
                client_read,
                server_write,
            ) {
                Ok(()) => {
                    log::info!(
                        "{}: client disconnected",
                        connection
                            .lock()
                            .unwrap()
                            .username
                            .clone()
                            .unwrap_or_default()
                    );
                }
                Err(err) => {
                    log::error!("Client error: {:?}", err);
                }
            }
        });

        std::thread::spawn({
            let connection = connection.clone();
            let blacklist = args.blacklist.clone();
            let whitelist = args.whitelist.clone();
            move || match handle_server(
                matches!(args.side, ConnectionSide::Both | ConnectionSide::Server),
                blacklist,
                whitelist,
                &connection,
                server_read,
                client_write,
            ) {
                Ok(()) => {
                    log::info!(
                        "{}: server disconnected",
                        connection
                            .lock()
                            .unwrap()
                            .username
                            .clone()
                            .unwrap_or_default()
                    )
                }
                Err(err) => {
                    log::error!("Server error: {:?}", err);
                }
            }
        });
    }
}

struct Connection {
    /// The client's username.
    username: Option<PlayerName>,
    client_codec: ClientPacketCodec,
    server_codec: ServerPacketCodec,
}

impl Connection {
    fn set_state(&mut self, state: ProtocolState) {
        log::info!(
            "{}: switching to state {:?}",
            self.username.clone().unwrap_or_default(),
            state
        );
        self.client_codec.set_state(state);
        self.server_codec.set_state(state);
    }

    fn set_compression(&mut self, threshold: CompressionThreshold) {
        self.client_codec.set_compression(threshold);
        self.server_codec.set_compression(threshold);
    }
}

#[derive(Clone)]
struct PlayerName(String);

impl Default for PlayerName {
    fn default() -> Self {
        PlayerName("<unknown>".to_string())
    }
}

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn handle_client(
    log: bool,
    blacklist: Option<Vec<u32>>,
    whitelist: Option<Vec<u32>>,
    connection: &Arc<Mutex<Connection>>,
    mut client_read: TcpStream,
    mut server_write: TcpStream,
) -> anyhow::Result<()> {
    loop {
        let length = VarInt::read_from(&mut client_read)
            .map(|var_int| var_int.0)
            .unwrap_or_default();
        if length == 0 {
            break;
        }
        let mut buf = vec![0; length as usize];
        client_read.read_exact(&mut buf)?;
        let mut vec = Vec::new();
        VarInt(length).write_to(&mut vec)?;
        vec.extend(buf);
        if vec.is_empty() {
            break;
        }

        let mut connection = connection.lock().unwrap();
        if let Some(packet) = connection
            .client_codec
            .decode(&vec)
            .context("failed to decode client packet")?
        {
            if log && !hide(packet.id(), blacklist.as_ref(), whitelist.as_ref()) {
                log::info!(
                    "{} -> #{:02X}",
                    connection.username.clone().unwrap_or_default(),
                    packet.id()
                );
                log::debug!(
                    "{} -> {:?}",
                    connection.username.clone().unwrap_or_default(),
                    packet
                );
                log::trace!("{}", pretty_hex::pretty_hex(&&vec));
            }

            // Detect state switches.
            if let ClientPacket::Handshake(packet) = packet {
                let state = match packet {
                    ClientHandshakePacket::Handshake(packet) => match packet.next_state {
                        HandshakeState::Login => ProtocolState::Login,
                        HandshakeState::Status => ProtocolState::Status,
                    },
                };
                connection.set_state(state)
            }

            drop(connection);
            // Forward the packet to the server.
            server_write.write_all(&vec)?;
        }
    }
    Ok(())
}

fn handle_server(
    log: bool,
    blacklist: Option<Vec<u32>>,
    whitelist: Option<Vec<u32>>,
    connection: &Arc<Mutex<Connection>>,
    mut server_read: TcpStream,
    mut client_write: TcpStream,
) -> anyhow::Result<()> {
    loop {
        let length = VarInt::read_from(&mut server_read)
            .map(|var_int| var_int.0)
            .unwrap_or_default();
        if length == 0 {
            break;
        }
        let mut buf = vec![0; length as usize];
        server_read.read_exact(&mut buf)?;
        let mut vec = Vec::new();
        VarInt(length).write_to(&mut vec)?;
        vec.extend(buf);
        if vec.is_empty() {
            break;
        }

        let mut connection = connection.lock().unwrap();
        if let Some(packet) = connection
            .server_codec
            .decode(&vec)
            .context("failed to decode client packet")?
        {
            if log && !hide(packet.id(), blacklist.as_ref(), whitelist.as_ref()) {
                log::info!(
                    "{} <- #{:02X}",
                    connection.username.clone().unwrap_or_default(),
                    packet.id()
                );
                log::debug!(
                    "{} <- {:?}",
                    connection.username.clone().unwrap_or_default(),
                    packet
                );
                log::trace!("{}", pretty_hex::pretty_hex(&&vec));
            }

            match packet {
                // Detect state switches
                ServerPacket::Login(ServerLoginPacket::LoginSuccess(packet)) => {
                    connection.username = Some(PlayerName(packet.username));
                    connection.set_state(ProtocolState::Play);
                }
                // Detect SetCompression
                ServerPacket::Login(ServerLoginPacket::SetCompression(packet)) => {
                    connection.set_compression(packet.threshold as CompressionThreshold)
                }
                _ => (),
            }

            drop(connection);
            // Forward the packet to the server.
            client_write.write_all(&vec)?;
        }
    }
    Ok(())
}

fn hide(packet_id: u32, blacklist: Option<&Vec<u32>>, whitelist: Option<&Vec<u32>>) -> bool {
    if let Some(blacklist) = blacklist {
        blacklist.contains(&packet_id)
    } else if let Some(whitelist) = whitelist {
        !whitelist.contains(&packet_id)
    } else {
        false
    }
}
