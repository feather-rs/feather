//! Saving of player data files and a system to save
//! player data on disconnect.

use crate::entity::{NamedComponent, PlayerComponent, PositionComponent};
use crate::player::{InventoryComponent, PlayerDisconnectEvent};
use crate::prelude::Config;
use crossbeam::Receiver;
use feather_core::entity::BaseEntityData;
use feather_core::inventory::Inventory;
use feather_core::player_data::{InventorySlot, PlayerData};
use feather_core::{player_data, Gamemode, Position};
use shrev::{EventChannel, ReaderId};
use specs::{Read, ReadStorage, System};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

/// System to save player data upon disconnect.
///
/// This system listens to `PlayerDisconnectEvent`s.
#[derive(Default)]
pub struct PlayerDataSaveSystem {
    reader: Option<ReaderId<PlayerDisconnectEvent>>,
}

impl<'a> System<'a> for PlayerDataSaveSystem {
    type SystemData = (
        Read<'a, Arc<Config>>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, NamedComponent>,
        ReadStorage<'a, InventoryComponent>,
        Read<'a, EventChannel<PlayerDisconnectEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (config, positions, players, nameds, inventories, disconnect_events) = data;

        for event in disconnect_events.read(self.reader.as_mut().unwrap()) {
            let player = players.get(event.player).unwrap();
            save_player_data(
                &config,
                positions.get(event.player).unwrap().current,
                player.gamemode,
                &inventories.get(event.player).unwrap().inventory,
                nameds.get(event.player).unwrap().uuid,
            );
        }
    }

    setup_impl!(reader);
}

/// Saves a player's data.
///
/// This operation is performed asynchronously,
/// and a channel is returned which will receive
/// a message upon completion.
pub fn save_player_data(
    config: &Config,
    position: Position,
    gamemode: Gamemode,
    inventory: &Inventory,
    uuid: Uuid,
) -> Receiver<()> {
    let data = PlayerData {
        entity: BaseEntityData {
            velocity: vec![0.0; 3], // Player velocity has no effect
            position: vec![position.x, position.y, position.z],
            rotation: vec![position.yaw, position.pitch],
        },
        gamemode: gamemode.get_id() as i32,
        inventory: inventory
            .items()
            .iter()
            .enumerate()
            .filter_map(|(index, item)| match item.clone() {
                Some(item) => Some((index, item)),
                None => None,
            })
            .map(|(index, item)| InventorySlot::from_network_index(index, item))
            .collect(),
    };

    // Channel used to communicate with Tokio task
    let (tx, rx) = crossbeam::bounded(1);

    let world_dir = Path::new(&config.world.name).to_owned();

    tokio_executor::blocking::run(move || {
        if let Err(e) = player_data::save_player_data(world_dir.as_path(), uuid, data) {
            error!("Failed to save player data for UUID {}: {:?}", uuid, e);
        } else {
            debug!("Saved player data for UUID {}", uuid);
        }

        let _ = tx.send(()); // Channel could have been dropped, so ignore result
    });

    rx
}
