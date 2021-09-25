//! Shutdown behavior.
use anyhow::Context;
use feather_core::network::packets::DisconnectPlay;
use feather_core::text::{TextRoot, TextValue};
use feather_server_chunk::chunk_worker::Request;
use feather_server_chunk::{save_chunk_at, ChunkWorkerHandle};
use feather_server_lighting::LightingWorkerHandle;
use feather_server_types::{tasks, BanInfo, Game, Network, Player};
use fecs::{IntoQuery, Read, World};
use std::sync::{Arc, RwLock};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub fn init(tx: crossbeam::Sender<()>) {
    ctrlc::set_handler(move || tx.try_send(()).unwrap()).unwrap();
}

pub fn disconnect_players(world: &World) -> anyhow::Result<()> {
    <Read<Network>>::query().for_each(world.inner(), |network| {
        let packet = DisconnectPlay {
            reason: TextRoot::from(TextValue::translate(
                "multiplayer.disconnect.server_shutdown",
            ))
            .into(),
        };

        network.send(packet);
    });

    Ok(())
}

pub fn save_chunks(
    game: &Game,
    cworker_handle: &ChunkWorkerHandle,
    world: &World,
) -> anyhow::Result<()> {
    for chunk in game.chunk_map.iter_chunks() {
        let pos = chunk.read().position();
        save_chunk_at(game, world, pos, cworker_handle);
    }

    // Wait for chunk worker to shut down
    let _ = cworker_handle.sender.send(Request::ShutDown);

    while cworker_handle.receiver.recv().is_ok() {}

    Ok(())
}

pub async fn save_level(game: &mut Game) -> anyhow::Result<()> {
    // Sync world time + level time
    let time = game.time.world_age() as i64;
    game.level.time = time;

    let level_path = format!("{}/{}", game.config.world.name, "level.dat");

    let mut file = File::create(&level_path).await?;
    game.level
        .save_to_file(&mut file)
        .await
        .context("failed to save level file")?;

    file.flush().await?;

    Ok(())
}

pub fn save_player_data(game: &Game, world: &World) -> anyhow::Result<()> {
    <Read<Player>>::query().for_each_entities(&world.inner(), |(player, _)| {
        feather_server_chunk::save_player_data(game, world, player);
    });

    Ok(())
}

pub async fn wait_for_task_completion() -> anyhow::Result<()> {
    tasks().wait().await;
    Ok(())
}

pub fn shut_down_workers(_game: &Game, light_handle: &LightingWorkerHandle) -> anyhow::Result<()> {
    let _ = light_handle
        .tx
        .send(feather_server_lighting::Request::ShutDown);

    // wait for disconnect
    let _ = light_handle.shutdown_rx.recv();
    Ok(())
}

pub async fn save_ban_list(ban_info: &Arc<RwLock<BanInfo>>) -> anyhow::Result<()> {
    const PATH: &str = "bans.toml";

    match File::create(PATH).await {
        Ok(mut file) => ban_info.read().unwrap().save_to_file(&mut file).await,
        Err(e) => Err(e.into()),
    }
}
