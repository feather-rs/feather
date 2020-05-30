use crate::chest;
use ahash::AHashMap;
use feather_core::blocks::BlockKind;
use feather_core::util::{BlockPosition, Position};
use feather_server_types::{BlockUpdateEvent, EntitySpawnEvent, Game};
use fecs::{EntityBuilder, World};
use once_cell::sync::Lazy;

/// Global mapping of blocks which require blcok entities.
static BLOCK_ENTITY_MAP: Lazy<AHashMap<BlockKind, fn(&Game, BlockPosition) -> EntityBuilder>> =
    Lazy::new(|| {
        let mut map: AHashMap<_, fn(&Game, BlockPosition) -> EntityBuilder> = AHashMap::new();

        map.insert(BlockKind::Chest, chest::create);

        map
    });

/// When a block is created, and there is a block entity kind
/// associated with it, creates the block entity. Additionally,
/// removes any old block entity, if it existed.
#[fecs::event_handler]
pub fn on_block_update_create_block_entity(
    event: &BlockUpdateEvent,
    game: &mut Game,
    world: &mut World,
) {
    if let Some(entity) = game.block_entities.get(&event.pos).copied() {
        // Remove the old entity
        game.block_entities.remove(&event.pos);
        game.despawn(entity, world);
    }

    if let Some(init) = BLOCK_ENTITY_MAP.get(&event.new.kind()) {
        // Spawn block entity
        let entity = init(game, event.pos)
            .with(event.pos)
            .with(Position::from(event.pos))
            .build()
            .spawn_in(world);

        game.block_entities.insert(event.pos, entity);
        game.handle(world, EntitySpawnEvent { entity });
    }
}
