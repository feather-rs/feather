//! Worker Tokio task.
//!
//! This is responsible for handling connections by
//! both sending and receiving packets.
//!
//! Packet send requests are sent over a channel from the server threads
//! to the worker for any given client.

use crate::config::Config;
use crate::io::initialhandler::{Action, InitialHandler};
use crate::io::{ListenerToServerMessage, NewClientInfo, ServerToWorkerMessage};
use crate::PlayerCount;
use feather_core::network::packet::PacketDirection;
use feather_core::network::serialize::MinecraftCodec;
use futures::{select, StreamExt};
use futures::{FutureExt, SinkExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::codec::Framed;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

/// Runs a worker task for the given client.
pub async fn run_worker(
    mut stream: TcpStream,
    ip: SocketAddr,
    global_sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) -> Result<(), failure::Error> {
    let codec = MinecraftCodec::new(PacketDirection::Serverbound);
    let mut framed = Framed::new(stream, codec);

    let mut initial_handler = Some(InitialHandler::new(config, player_count, server_icon));

    let (mut tx_worker_to_server, rx_worker_to_server) = crossbeam::unbounded();
    let (tx_server_to_worker, mut rx_server_to_worker) = tokio::sync::mpsc::unbounded_channel();
    let mut rx_worker_to_server = Some(rx_worker_to_server);
    let mut future_server_message = rx_server_to_worker.next().fuse();

    loop {
        let mut server_message = None;
        let mut received_packet = None;

        select! {
            msg = future_server_message => server_message = Some(msg),
            packet = framed.next().fuse() => received_packet = Some(packet),
        }

        if let Some(msg) = server_message {
            if let Some(msg) = msg {
                match msg {
                    ServerToWorkerMessage::SendPacket(packet) => framed.send(packet).await?,
                    ServerToWorkerMessage::Disconnect => return Ok(()),
                    _ => unreachable!(),
                }
            }
        }

        if let Some(packet_result) = received_packet {
            if let Some(packet_result) = packet_result {
                match packet_result {
                    Ok(packet) => {
                        if let Some(ih) = initial_handler.as_mut() {
                            let actions = ih.handle_packet(packet).await;

                            for action in actions {
                                match action {
                                    Action::Disconnect => return Ok(()),
                                    Action::SendPacket(packet) => framed.send(packet).await?,
                                    Action::EnableCompression(threshold) => {
                                        if threshold > 0 {
                                            framed
                                                .codec_mut()
                                                .enable_compression(threshold as usize);
                                        }
                                    }
                                    Action::EnableEncryption(key) => {
                                        framed.codec_mut().enable_encryption(key)
                                    }
                                    Action::JoinGame(res) => {
                                        let info = NewClientInfo {
                                            ip,
                                            username: res.username,
                                            profile: res.props,
                                            uuid: res.uuid,
                                            sender: tx_server_to_worker.clone(),
                                            receiver: rx_worker_to_server.take().unwrap(),
                                        };
                                        global_sender
                                            .send(ListenerToServerMessage::NewClient(info))?;
                                        initial_handler = None;
                                    }
                                }
                            }
                        } else {
                            let _ = tx_worker_to_server
                                .send(ServerToWorkerMessage::NotifyPacketReceived(packet));
                        }
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
    }
}
