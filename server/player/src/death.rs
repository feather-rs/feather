use entity::drops::drop_item;
use feather_core::util::Position;
use feather_server_types::{EntityDeathEvent, Game, Inventory, Player};
use fecs::World;

/// Scatters a player's items when they die.
#[fecs::event_handler]
pub fn on_player_death_scatter_inventory(
    event: &EntityDeathEvent,
    game: &mut Game,
    world: &mut World,
) {
    if !world.has::<Player>(event.entity) {
        return;
    }

    let inventory = world.get::<Inventory>(event.entity);
    let pos = *world.get::<Position>(event.entity);

    // Remove items and drop on ground
    let items_to_spawn = inventory
        .iter_mut()
        .filter_map(|mut item| item.take())
        .collect::<Vec<_>>();

    drop(inventory);

    for item in items_to_spawn {
        drop_item(game, world, item, pos);
    }
}
