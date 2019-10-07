//! Shutdown behavior.

use crate::chunk_logic::ChunkWorkerHandle;
use crate::config::Config;
use crate::entity::{NamedComponent, PlayerComponent, PositionComponent};
use crate::player;
use crate::player::InventoryComponent;
use crate::time::Time;
use crate::{chunkworker, entity};
use crossbeam::Sender;
use feather_core::level::{save_level_file, LevelData, Root};
use feather_core::prelude::ChunkMap;
use specs::{Join, World, WorldExt};
use std::fs::File;
use std::sync::Arc;

pub fn init(tx: Sender<()>) {
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
}

pub fn save_chunks(world: &mut World) {
    let mut chunk_map = world.fetch_mut::<ChunkMap>();
    let handle = world.fetch::<ChunkWorkerHandle>();
    let count = entity::save_chunks(&mut chunk_map, &world.fetch(), &world.fetch());

    // Need to call `world.maintain()` for lazy chunk saving
    // to take effect
    world.maintain();

    handle.sender.send(chunkworker::Request::ShutDown).unwrap();

    let mut saved = 0;
    // Wait for chunks to finish saving
    while let Ok(msg) = handle.receiver.recv() {
        if let chunkworker::Reply::SavedChunk(_) = msg {
            saved += 1;
        }

        if saved == count {
            break;
        }
    }

    assert!(
        count == saved,
        "didn't save all chunks: {} != {}",
        count,
        saved
    );
}

pub fn save_level(world: &World) {
    let mut level = (*world.fetch::<LevelData>()).clone();

    // Sync world time + level time
    let time = world.fetch::<Time>();
    level.time = time.0 as i64;

    let config = world.fetch::<Arc<Config>>();

    let level_path = format!("{}/{}", config.world.name, "level.dat");

    let root = Root { data: level };
    save_level_file(&root, &mut File::create(&level_path).unwrap())
        .expect("Failed to save level file");
}

pub fn save_player_data(world: &World) {
    let config = world.fetch::<Arc<Config>>();

    let positions = world.read_component::<PositionComponent>();
    let nameds = world.read_component::<NamedComponent>();
    let players = world.read_component::<PlayerComponent>();
    let inventories = world.read_component::<InventoryComponent>();

    let mut channels = vec![];

    for (position, named, player, inventory) in (&positions, &nameds, &players, &inventories).join()
    {
        let rx = player::save_player_data(
            &config,
            position.current,
            player.gamemode,
            &inventory.inventory,
            named.uuid,
        );
        channels.push(rx);
    }

    // Wait for saving to complete
    channels.into_iter().for_each(|rx| rx.recv().unwrap());
}
