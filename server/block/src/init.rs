use crate::{chest, ShouldReplace};
use ahash::AHashMap;
use feather_core::blocks::BlockKind;
use feather_core::util::BlockPosition;
use feather_server_types::{BlockEntity, BlockUpdateEvent, EntitySpawnEvent, Game};
use fecs::{EntityBuilder, World};
use once_cell::sync::Lazy;

type BlockEntityCreator = fn(BlockPosition) -> EntityBuilder;

/// Global mapping of blocks which require block entities.
static BLOCK_ENTITY_MAP: Lazy<AHashMap<BlockKind, BlockEntityCreator>> = Lazy::new(|| {
    let mut map: AHashMap<_, fn(BlockPosition) -> EntityBuilder> = AHashMap::new();

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
        // Determine whether we should replace the entity
        // or keep the existing block entity.
        if let Some(should_replace) = world.try_get::<ShouldReplace>(entity).map(|x| x.0) {
            if !should_replace(event.old, event.new) {
                return; // should keep existing block entity; block entities remain unchanged
            }
        }
        game.block_entities.remove(&event.pos);
        game.despawn(entity, world);
    }

    if let Some(init) = BLOCK_ENTITY_MAP.get(&event.new.kind()) {
        // Spawn block entity
        let entity = init(event.pos).build().spawn_in(world);

        game.handle(world, EntitySpawnEvent { entity });
    }
}

#[fecs::event_handler]
pub fn on_block_entity_create_insert_to_map(
    event: &EntitySpawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    if let Some(pos) = world.try_get::<BlockPosition>(event.entity) {
        if world.has::<BlockEntity>(event.entity) {
            game.block_entities.insert(*pos, event.entity);
        }
    }
}
