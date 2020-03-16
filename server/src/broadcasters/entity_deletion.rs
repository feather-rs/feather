use crate::entity::EntityId;
use crate::game::Game;
use crate::player::Player;
use feather_core::network::packet::implementation::{
    DestroyEntities, PlayerInfo, PlayerInfoAction,
};
use fecs::{Entity, World};
use uuid::Uuid;

/// Broadcasts when an entity is deleted.
pub fn on_entity_despawn_broadcast_despawn(game: &mut Game, world: &mut World, entity: Entity) {
    let id = world.get::<EntityId>(entity).0;
    let packet = DestroyEntities {
        entity_ids: vec![id],
    };

    game.broadcast_entity_update(world, packet, entity, Some(entity));

    // If the entity was a player, send Player Info to
    // remove them from the tablist.
    if world.has::<Player>(entity) {
        let uuid = *world.get::<Uuid>(entity);
        let packet = PlayerInfo {
            action: PlayerInfoAction::RemovePlayer,
            uuid,
        };

        game.broadcast_global(world, packet, None);
    }
}
