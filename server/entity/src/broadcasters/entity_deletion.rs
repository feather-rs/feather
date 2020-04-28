use feather_core::network::packets::{DestroyEntities, Player, PlayerInfo, PlayerInfoAction};
use feather_server_types::{EntityDespawnEvent, EntityId, Game, Uuid};
use fecs::World;

/// Broadcasts when an entity is deleted.
#[fecs::event_handler]
pub fn on_entity_despawn_broadcast_despawn(
    event: &EntityDespawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    let id = world.get::<EntityId>(event.entity).0;
    let packet = DestroyEntities {
        entity_ids: vec![id],
    };

    game.broadcast_entity_update(world, packet, event.entity, Some(event.entity));

    // If the entity was a player, send Player Info to
    // remove them from the tablist.
    if world.has::<Player>(event.entity) {
        let uuid = *world.get::<Uuid>(event.entity);
        let packet = PlayerInfo {
            action: PlayerInfoAction::RemovePlayer,
            uuid,
        };

        game.broadcast_global(world, packet, None);
    }
}
