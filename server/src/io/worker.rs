//! Worker Tokio task.
//!
//! This is responsible for handling connections by
//! both sending and receiving packets.
//!
//! Packet send requests are sent over a channel from the server threads
//! to the worker for any given client.

use crate::config::Config;
use crate::io::initial_handler::{Action, InitialHandler};
use crate::io::{ListenerToServerMessage, NewClientInfo, ServerToWorkerMessage};
use crate::PlayerCount;
use feather_core::network::codec::MinecraftCodec;
use feather_core::network::packet::PacketDirection;
use feather_core::player_data::PlayerData;
use futures::{select, StreamExt};
use futures::{FutureExt, SinkExt};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use uuid::Uuid;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to read player data")]
    PlayerData,
}

/// Runs a worker task for the given client.
pub async fn run_worker(
    stream: TcpStream,
    ip: SocketAddr,
    global_sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
) {
    let (tx_worker_to_server, rx_worker_to_server) = crossbeam::unbounded();

    let msg = match _run_worker(
        stream,
        ip,
        global_sender,
        config,
        player_count,
        server_icon,
        tx_worker_to_server.clone(),
        rx_worker_to_server.clone(),
    )
    .await
    {
        Ok(()) => "normal disconnect".to_string(),
        Err(e) => format!("{}", e),
    };

    let _ = tx_worker_to_server.send(ServerToWorkerMessage::NotifyDisconnect(msg));
}

#[allow(clippy::too_many_arguments)]
async fn _run_worker(
    stream: TcpStream,
    ip: SocketAddr,
    global_sender: crossbeam::Sender<ListenerToServerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
    server_icon: Arc<Option<String>>,
    tx_worker_to_server: crossbeam::Sender<ServerToWorkerMessage>,
    rx_worker_to_server: crossbeam::Receiver<ServerToWorkerMessage>,
) -> Result<(), failure::Error> {
    let codec = MinecraftCodec::new(PacketDirection::Serverbound);

    let mut framed = Framed::new(stream, codec);

    let mut initial_handler = Some(InitialHandler::new(
        Arc::clone(&config),
        player_count,
        server_icon,
    ));

    let (tx_server_to_worker, mut rx_server_to_worker) = futures::channel::mpsc::unbounded();
    let mut rx_worker_to_server = Some(rx_worker_to_server);

    loop {
        let mut server_message = None;
        let mut received_packet = None;

        select! {
            msg = rx_server_to_worker.next().fuse() => server_message = Some(msg),
            packet = tokio::time::timeout(Duration::from_millis(10000), framed.next()).fuse() => received_packet = Some(packet),
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
            if let Some(packet_result) = packet_result? {
                match packet_result {
                    Ok(packet) => {
                        if let Some(ih) = initial_handler.as_mut() {
                            ih.handle_packet(packet).await;
                            let actions = ih.actions_to_execute();

                            for action in actions {
                                match action {
                                    Action::Disconnect => return Ok(()),
                                    Action::SendPacket(packet) => framed.send(packet).await?,
                                    Action::EnableCompression(threshold) => {
                                        if threshold > 0 {
                                            trace!(
                                                "Enabling compression with threshold {}",
                                                threshold
                                            );
                                            framed
                                                .codec_mut()
                                                .enable_compression(threshold as usize);
                                        }
                                    }
                                    Action::EnableEncryption(key) => {
                                        trace!("Enabling encryption");
                                        framed.codec_mut().enable_encryption(key)
                                    }
                                    Action::SetStage(stage) => framed.codec_mut().set_stage(stage),
                                    Action::JoinGame(res) => {
                                        // let data = load_player_data(&config, res.uuid).await?;
                                        let data = PlayerData::default();
                                        let info = NewClientInfo {
                                            ip,
                                            username: res.username.ok_or(Error::PlayerData)?,
                                            profile: res.props,
                                            uuid: res.uuid,
                                            sender: tx_server_to_worker.clone(),
                                            receiver: rx_worker_to_server.take().unwrap(),
                                            /*position: data
                                            .entity
                                            .read_position()
                                            .ok_or_else(|| Error::PlayerData)?,*/
                                            position: position!(0.0, 80.0, 0.0),
                                            data,
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
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

async fn load_player_data(config: &Config, uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    feather_core::player_data::load_player_data(Path::new(&config.world.name), uuid).await
}
