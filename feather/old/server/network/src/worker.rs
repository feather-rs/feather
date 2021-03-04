//! Worker Tokio task.
//!
//! This is responsible for handling connections by
//! both sending and receiving packets.
//!
//! Packet send requests are sent over a channel from the server threads
//! to the worker for any given client.

use crate::initial_handler::{Action, InitialHandler};
use crate::{ListenerToServerMessage, NewClientInfo, ServerToListenerMessage};
use feather_core::anvil::entity::{AnimalData, BaseEntityData};
use feather_core::anvil::player::PlayerData;
use feather_core::network::{MinecraftCodec, Packet, PacketDirection};
use feather_core::util::{Position, Vec3d};
use feather_server_types::{
    BanInfo, Config, PacketBuffers, ServerToWorkerMessage, Uuid, WorkerToServerMessage,
};
use fecs::Entity;
use futures::future::Either;
use futures::SinkExt;
use futures::StreamExt;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;

struct Worker {
    framed: Framed<TcpStream, MinecraftCodec>,
    config: Arc<Config>,
    ip: SocketAddr,
    /// The listener's sender to send the initial `NewClient` message
    /// to the server. Also used to request an entity for the player.
    listener_tx: flume::Sender<ListenerToServerMessage>,
    /// Packet buffers to which we write packets received from the client.
    packet_buffers: Arc<PacketBuffers>,
    /// The channel which will be used by the server thread
    /// to send messages to `rx`.
    server_tx: flume::Sender<ServerToWorkerMessage>,
    /// Channel to receive messages from the server, linked to `server_tx`.
    rx: flume::Receiver<ServerToWorkerMessage>,
    /// The channel which will be used by the server thread
    /// to receive messages from the worker.
    server_rx: Option<flume::Receiver<WorkerToServerMessage>>,
    /// Channel to send messages to the server, linked to `server_rx`.
    tx: flume::Sender<WorkerToServerMessage>,
    /// Initial handler, set to `None` after the player has completed
    /// the login process.
    initial_handler: Option<InitialHandler>,
    /// The entity for the player on the server thread.
    entity: Entity,
}

/// Runs a worker task for the given client.
#[allow(clippy::too_many_arguments)]
pub async fn run_worker(
    stream: TcpStream,
    ip: SocketAddr,
    listener_tx: flume::Sender<ListenerToServerMessage>,
    listener_rx: Arc<Mutex<flume::Receiver<ServerToListenerMessage>>>,
    config: Arc<Config>,
    ban_info: Arc<RwLock<BanInfo>>,
    player_count: Arc<AtomicU32>,
    server_icon: Arc<Option<String>>,
    packet_buffers: Arc<PacketBuffers>,
) {
    let (server_tx, rx) = flume::unbounded();
    let (tx, server_rx) = flume::unbounded();

    let initial_handler = Some(InitialHandler::new(
        Arc::clone(&config),
        Arc::clone(&ban_info),
        Arc::clone(&player_count),
        Arc::clone(&server_icon),
        ip,
    ));

    let codec = MinecraftCodec::new(PacketDirection::Serverbound);
    let framed = Framed::new(stream, codec);

    let entity = request_entity(&listener_tx, &mut *listener_rx.lock().await).await;

    let mut worker = Worker {
        framed,
        ip,
        listener_tx,
        packet_buffers,
        server_tx,
        rx,
        server_rx: Some(server_rx),
        tx,
        initial_handler,
        entity,
        config,
    };

    let msg = match run_worker_impl(&mut worker).await {
        Ok(()) => String::from("client disconnected"),
        Err(e) => format!("{}", e),
    };

    let _ = worker
        .tx
        .send(WorkerToServerMessage::NotifyDisconnected { reason: msg });

    // If server is not aware of connection (and thus
    // will not receive the above message), instruct it
    // to delete the entity. Otherwise, it will delete it
    // through the message above.
    if worker.server_rx.is_some() {
        let _ = worker
            .listener_tx
            .send(ListenerToServerMessage::DeleteEntity(worker.entity));
    }
}

async fn request_entity(
    listener_tx: &flume::Sender<ListenerToServerMessage>,
    listener_rx: &mut flume::Receiver<ServerToListenerMessage>,
) -> Entity {
    let _ = listener_tx.send(ListenerToServerMessage::RequestEntity);

    let recv = listener_rx.next().await.expect("server disconnected");

    match recv {
        ServerToListenerMessage::Entity(entity) => entity,
    }
}

async fn run_worker_impl(worker: &mut Worker) -> anyhow::Result<()> {
    loop {
        let received_message = worker.rx.next();
        let received_packet = worker.framed.next();

        let select = futures::future::select(received_message, received_packet);

        match select.await {
            Either::Left((msg, _)) => {
                if let Some(msg) = msg {
                    handle_server_to_worker_message(worker, msg).await?;
                }
            }
            Either::Right((packet_res, _)) => {
                let packet_res =
                    packet_res.ok_or_else(|| anyhow::anyhow!("client disconnected"))?;

                let packet = packet_res?;

                handle_packet(worker, packet).await?;
            }
        }

        tokio::task::yield_now().await;
    }
}

async fn handle_server_to_worker_message(
    worker: &mut Worker,
    msg: ServerToWorkerMessage,
) -> anyhow::Result<()> {
    match msg {
        ServerToWorkerMessage::SendPacket(packet) => worker.framed.send(packet).await?,
        ServerToWorkerMessage::Disconnect => anyhow::bail!("server requested disconnect"),
    }

    Ok(())
}

async fn handle_packet(worker: &mut Worker, packet: Box<dyn Packet>) -> anyhow::Result<()> {
    if let Some(ref mut ih) = worker.initial_handler {
        ih.handle_packet(packet).await;

        handle_ih_actions(worker).await?;
    } else {
        worker.packet_buffers.push(worker.entity, packet);
    }

    Ok(())
}

async fn handle_ih_actions(worker: &mut Worker) -> anyhow::Result<()> {
    for action in worker
        .initial_handler
        .as_mut()
        .unwrap()
        .actions_to_execute()
    {
        match action {
            Action::SendPacket(packet) => worker.framed.send(packet).await?,
            Action::EnableCompression(threshold) => worker
                .framed
                .codec_mut()
                .enable_compression(threshold as usize),
            Action::EnableEncryption(key) => worker.framed.codec_mut().enable_encryption(key),
            Action::Disconnect => anyhow::bail!("initial handler requested disconnect"),
            Action::SetStage(stage) => worker.framed.codec_mut().set_stage(stage),
            Action::JoinGame(info) => {
                let data = load_player_data(&worker.config, info.uuid).await?;
                let position = data.animal.base.read_position()?;
                let info = NewClientInfo {
                    ip: worker.ip,
                    username: info.username.unwrap_or_else(|| String::from("undefined")),
                    profile: info.props,
                    uuid: info.uuid,
                    data,
                    position,
                    sender: worker.server_tx.clone(),
                    receiver: worker.server_rx.take().unwrap(),
                    entity: worker.entity,
                };

                let _ = worker
                    .listener_tx
                    .send(ListenerToServerMessage::NewClient(info));

                worker.initial_handler = None;
                return Ok(());
            }
        }
    }

    Ok(())
}

const DEFAULT_POSITION: Position = position!(0.0, 70.0, 0.0); // TODO: better calculation

async fn load_player_data(config: &Config, uuid: Uuid) -> Result<PlayerData, anyhow::Error> {
    log::debug!("Loading player data for UUID {}", uuid);
    match feather_core::anvil::player::load_player_data(Path::new(&config.world.name), uuid).await {
        Ok(data) => Ok(data),
        Err(e) => {
            log::debug!(
                "Failed to load player data for {} ({}); creating default data",
                uuid,
                e,
            );

            let data = PlayerData {
                animal: AnimalData::new(
                    BaseEntityData::new(DEFAULT_POSITION, Vec3d::broadcast(0.0)),
                    20.0,
                ),
                gamemode: config.server.default_gamemode.id() as i32,
                inventory: vec![],
                held_item: 0,
            };

            feather_core::anvil::player::save_player_data(
                Path::new(&config.world.name),
                uuid,
                &data,
            )
            .await?;

            Ok(data)
        }
    }
}
