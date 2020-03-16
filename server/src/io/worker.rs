//! Worker Tokio task.
//!
//! This is responsible for handling connections by
//! both sending and receiving packets.
//!
//! Packet send requests are sent over a channel from the server threads
//! to the worker for any given client.

use crate::config::Config;
use crate::io::initial_handler::{Action, InitialHandler};
use crate::io::{
    ListenerToServerMessage, NewClientInfo, ServerToListenerMessage, ServerToWorkerMessage,
    WorkerToServerMessage,
};
use crate::packet_buffer::PacketBuffers;
use feather_core::network::codec::MinecraftCodec;
use feather_core::network::packet::PacketDirection;
use feather_core::player_data::PlayerData;
use feather_core::Packet;
use fecs::Entity;
use futures::channel::mpsc;
use futures::future::Either;
use futures::SinkExt;
use futures::StreamExt;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;
use uuid::Uuid;

struct Worker {
    framed: Framed<TcpStream, MinecraftCodec>,
    ip: SocketAddr,
    /// The listener's sender to send the initial `NewClient` message
    /// to the server. Also used to request an entity for the player.
    listener_tx: crossbeam::Sender<ListenerToServerMessage>,
    /// Packet buffers to which we write packets received from the client.
    packet_buffers: Arc<PacketBuffers>,
    /// The channel which will be used by the server thread
    /// to send messages to `rx`.
    server_tx: mpsc::UnboundedSender<ServerToWorkerMessage>,
    /// Channel to receive messages from the server, linked to `server_tx`.
    rx: mpsc::UnboundedReceiver<ServerToWorkerMessage>,
    /// The channel which will be used by the server thread
    /// to receive messages from the worker.
    server_rx: crossbeam::Receiver<WorkerToServerMessage>,
    /// Channel to send messages to the sserver, linked to `server_rx`.
    tx: crossbeam::Sender<WorkerToServerMessage>,
    /// Initial handler, set to `None` after the player has completed
    /// the login process.
    initial_handler: Option<InitialHandler>,
    /// The entity for the player on the server thread.
    entity: Entity,
}

/// Runs a worker task for the given client.
pub async fn run_worker(
    stream: TcpStream,
    ip: SocketAddr,
    listener_tx: crossbeam::Sender<ListenerToServerMessage>,
    listener_rx: Arc<Mutex<mpsc::UnboundedReceiver<ServerToListenerMessage>>>,
    config: Arc<Config>,
    player_count: Arc<AtomicU32>,
    server_icon: Arc<Option<String>>,
    packet_buffers: Arc<PacketBuffers>,
) {
    let (server_tx, rx) = mpsc::unbounded();
    let (tx, server_rx) = crossbeam::unbounded();

    let initial_handler = Some(InitialHandler::new(
        Arc::clone(&config),
        Arc::clone(&player_count),
        Arc::clone(&server_icon),
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
        server_rx,
        tx,
        initial_handler,
        entity,
    };

    let msg = match run_worker_impl(&mut worker).await {
        Ok(()) => String::from("client disconnected"),
        Err(e) => format!("{}", e),
    };

    let _ = worker
        .tx
        .send(WorkerToServerMessage::NotifyDisconnected { reason: msg });
}

async fn request_entity(
    listener_tx: &crossbeam::Sender<ListenerToServerMessage>,
    listener_rx: &mut mpsc::UnboundedReceiver<ServerToListenerMessage>,
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
                let packet_res = packet_res.ok_or(anyhow::anyhow!("client disconnected"))?;

                let packet = packet_res?;

                handle_packet(worker, packet).await?;
            }
        }
    }
}

async fn handle_server_to_worker_message(
    worker: &mut Worker,
    msg: ServerToWorkerMessage,
) -> anyhow::Result<()> {
    match msg {
        ServerToWorkerMessage::SendPacket(packet) => worker.framed.send(packet).await?,
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
                let info = NewClientInfo {
                    ip: worker.ip,
                    username: info.username.unwrap_or(String::from("undefined")),
                    profile: info.props,
                    uuid: info.uuid,
                    data: Default::default(),            // TODO
                    position: position!(0.0, 70.0, 0.0), // TODO
                    sender: worker.server_tx.clone(),
                    receiver: worker.server_rx.clone(),
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

#[allow(dead_code)] // TODO
async fn load_player_data(config: &Config, uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    feather_core::player_data::load_player_data(Path::new(&config.world.name), uuid).await
}
