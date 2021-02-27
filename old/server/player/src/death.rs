//! Handles when a player dies.
//!
//! Player deaths have some annoying properties which makes a correct implementation difficult.
//! Most notably, a dead player (one who is currently on the respawn screen) has no physical
//! presence within the world, despite them still being connected to the server and existing
//! in that sense.

use entity::drops::drop_item;
use feather_core::util::Position;
use feather_server_types::{Dead, EntityDeathEvent, Game, Inventory, InventoryUpdateEvent, Player};
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
    let slots_to_update = inventory
        .enumerate()
        .filter_map(|(index, slot)| slot.map(|_| index));
    let event = InventoryUpdateEvent {
        entity: event.entity,
        slots: slots_to_update.collect(),
    };

    let items_to_spawn = inventory
        .iter_mut()
        .filter_map(|mut item| item.take())
        .collect::<Vec<_>>();

    drop(inventory);

    for item in items_to_spawn {
        drop_item(game, world, item, pos);
    }

    game.handle(world, event);
}

/// Adds the `Dead` component to a player when they die to
/// avoid causing them to be physically interacted with.
///
/// The component will be removed once the user clicks the respawn
/// button.
#[fecs::event_handler]
pub fn on_player_death_mark_dead(event: &EntityDeathEvent, world: &mut World) {
    if world.has::<Player>(event.entity) {
        world.add(event.entity, Dead).unwrap();
    }
}
