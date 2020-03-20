//! Shutdown behavior.
use crate::chunk_worker::Request;
use crate::game::Game;
use crate::save;
use crossbeam::Sender;
use fecs::World;

pub fn init(tx: Sender<()>) {
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
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

pub fn save_level(_world: &World) {}

pub fn save_player_data(_world: &World) {}
