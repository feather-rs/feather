use feather_core::{
    network::{
        packets::{DestroyEntities, PlayerInfo, PlayerInfoAction, UpdateBlockEntity},
        Packet,
    },
    util::BlockPosition,
};
use feather_server_types::{BlockEntity, EntityDespawnEvent, Game, NetworkId, Player, Uuid};
use fecs::{Entity, World};

/// Broadcasts when an entity is deleted.
#[fecs::event_handler]
pub fn on_entity_despawn_broadcast_despawn(
    event: &EntityDespawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    let packet = packet_to_despawn(event.entity, world);

    game.broadcast_entity_update_boxed(world, packet, event.entity, Some(event.entity));

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

fn packet_to_despawn(entity: Entity, world: &World) -> Box<dyn Packet> {
    // For normal entities, use Destroy Entities.
    // For block entities, send Update Block Entity
    // with a single TAG_END to remove the block entity
    // at the position.

    if world.has::<BlockEntity>(entity) {
        Box::new(UpdateBlockEntity {
            location: *world.get::<BlockPosition>(entity),
            ..Default::default()
        })
    } else {
        Box::new(DestroyEntities {
            entity_ids: vec![world.get::<NetworkId>(entity).0],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item;
    use feather_core::items::ItemStack;
    use feather_core::util::Position;
    use feather_test_framework::Test;

    #[test]
    fn broadcast_despawn() {
        let mut test = Test::new();

        let player = test.player("", Position::default());
        let player_far_away = test.player("faraway", position!(0.0, 0.0, 10000.0));
        let item =
            test.entity(item::create(ItemStack::default(), 0).with(position!(10.0, 64.0, 0.0)));

        test.handle(
            EntityDespawnEvent { entity: item },
            on_entity_despawn_broadcast_despawn,
        );

        let packet = test.sent::<DestroyEntities>(player).unwrap();
        assert_eq!(packet.entity_ids, vec![test.id(item)]);
        assert!(test.sent::<PlayerInfo>(player).is_none());

        assert!(test.sent::<DestroyEntities>(player_far_away).is_none());
        assert!(test.sent::<PlayerInfo>(player_far_away).is_none());

        let player2 = test.player("", position!(45.0, -324.0, 16.8));
        test.handle(
            EntityDespawnEvent { entity: player2 },
            on_entity_despawn_broadcast_despawn,
        );

        let packet = test.sent::<DestroyEntities>(player).unwrap();
        assert_eq!(packet.entity_ids, vec![test.id(player2)]);

        // player_far_away should receive because PlayerInfo is broadcasted globally
        let packets = [
            test.sent::<PlayerInfo>(player).unwrap(),
            test.sent::<PlayerInfo>(player_far_away).unwrap(),
        ];

        for packet in &packets {
            assert_eq!(packet.uuid, test.uuid(player2));
            assert_eq!(packet.action, PlayerInfoAction::RemovePlayer);
        }
    }
}
