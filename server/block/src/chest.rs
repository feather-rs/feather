use feather_core::util::BlockPosition;
use feather_server_entity::drops::drop_item;
use feather_server_types::{BlockUpdateEvent, BumpVec, Game, Inventory};
use fecs::{EntityBuilder, World};

/// Marker component for chests.
pub struct Chest;

/// Creates a chest.
pub fn create(_game: &Game, _pos: BlockPosition) -> EntityBuilder {
    EntityBuilder::new()
        .with(Chest)
        .with(Inventory::chest(false)) // TODO: handle large chests
}

/// When a chest is despawned, drops its contents.
#[fecs::event_handler]
pub fn on_chest_break_drop_contents(event: &BlockUpdateEvent, game: &mut Game, world: &mut World) {
    if let Some(entity) = game.block_entities.get(&event.pos).copied() {
        if !world.has::<Chest>(entity) {
            return;
        }

        let items = BumpVec::from_iter_in(
            world
                .get::<Inventory>(entity)
                .iter_mut()
                .filter_map(|mut guard| guard.take()),
            game.bump(),
        );
        for item in items {
            drop_item(game, world, item, event.pos.position());
        }
    }
}
