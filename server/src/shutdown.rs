//! Shutdown behavior.
use crate::chunk_worker::Request;
use crate::game::Game;
use crate::network::Network;
use crate::{lighting, save};
use crossbeam::Sender;
use feather_core::level;
use feather_core::level::save_level_file;
use feather_core::network::packet::implementation::DisconnectPlay;
use fecs::{IntoQuery, Read, World};
use std::fs::File;

pub fn init(tx: Sender<()>) {
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
}

pub fn disconnect_players(world: &World) {
    <Read<Network>>::query().for_each(world.inner(), |network| {
        let packet = DisconnectPlay {
            reason: json!({
                "text": "Server closed"
            })
            .to_string(),
        };

        network.send(packet);
    })
}

pub fn save_chunks(game: &Game, world: &World) {
    for chunk in game.chunk_map.iter_chunks() {
        let pos = chunk.read().position();
        save::save_chunk_at(game, world, pos);
    }

    // Wait for chunk worker to shut down
    let _ = game.chunk_worker_handle.sender.send(Request::ShutDown);

    while let Ok(_) = game.chunk_worker_handle.receiver.recv() {}
}

pub fn save_level(game: &mut Game) {
    // Sync world time + level time
    let time = game.time.world_age() as i64;
    game.level.time = time;

    let level_path = format!("{}/{}", game.config.world.name, "level.dat");

    let root = level::Root {
        data: game.level.clone(),
    };
    save_level_file(&root, &mut File::create(&level_path).unwrap())
        .expect("Failed to save level file");
}

pub fn save_player_data(_world: &World) {}

pub fn shut_down_workers(game: &Game) {
    let _ = game
        .lighting_worker_handle
        .tx
        .send(lighting::Request::ShutDown);

    // wait for disconnect
    let _ = game.lighting_worker_handle.shutdown_rx.recv();
}
