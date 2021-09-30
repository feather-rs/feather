use common::{Game, Window};
use ecs::{SysResult, SystemExecutor};
use quill_common::events::InventoryUpdateEvent;

use crate::Server;
use quill_common::components::ClientId;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(inventory_change);
}

fn inventory_change(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (event, window)) in game.ecs.query::<(&InventoryUpdateEvent, &Window)>().iter() {
        let client = server
            .clients
            .get(*game.ecs.get::<ClientId>(entity).unwrap())
            .unwrap();
        for slot in &event.0 {
            client.send_slot_item(
                0,
                *slot as i16,
                window
                    .item(*slot)
                    .ok()
                    .map(|item| item.clone())
                    .unwrap_or_default(),
            )
        }
    }
    Ok(())
}
