use feather_core::entitymeta::EntityMetadata;
use feather_core::network::packets::PacketEntityMetadata;
use feather_core::util::Position;
use feather_server_types::{
    CreationPacketCreator, EntitySendEvent, EntitySpawnEvent, Game, Network, NetworkId,
    PlayerJoinEvent, SpawnPacketCreator,
};
use fecs::{IntoQuery, Read, World};

/// When an entity is created and has a `CreationPacketCreator` and/or `SpawnPacketCreator`,
/// broadcasts the packets to all online clients.
#[fecs::event_handler]
pub fn on_entity_spawn_send_to_clients(
    event: &EntitySpawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    let accessor = world.entity(event.entity).expect("entity does not exist");

    if let Some(creator) = world.try_get::<CreationPacketCreator>(event.entity) {
        let packet = creator.get(&accessor);
        game.broadcast_global_boxed(world, packet, None);
    }
    let mut to_trigger = vec![];

    if let Some(creator) = world.try_get::<SpawnPacketCreator>(event.entity) {
        // Send metadata before spawn packet. Not sure why this works,
        // but if we don't do this, then the client just despawns
        // the entity immediately after sending.
        if let Some(meta) = world.try_get::<EntityMetadata>(event.entity) {
            let packet = PacketEntityMetadata {
                entity_id: world.get::<NetworkId>(event.entity).0,
                metadata: (&*meta).clone(),
            };
            game.broadcast_entity_update(world, packet, event.entity, Some(event.entity));
        }

        // Now send spawn packet: Spawn Object / Spawn Player / Spawn Mob / whatever.
        let packet = creator.get(&accessor);
        game.broadcast_entity_update_boxed(world, packet, event.entity, Some(event.entity));

        let chunk = world.get::<Position>(event.entity).chunk();

        drop(creator);

        // trigger on_entity_send
        for player in game.chunk_holders.holders_for(chunk) {
            if world.try_get::<Network>(*player).is_some() {
                to_trigger.push(*player);
            }
        }
    }

    for client in to_trigger {
        game.handle(
            world,
            EntitySendEvent {
                entity: event.entity,
                client,
            },
        );
    }
}

/// Wehn a player joins, sends existing entities to the player.
///
/// This only handles init packets (PlayerInfo, etc.)—spawn packets
/// are handled by the view update mechanism in `crate::view`.
#[fecs::event_handler]
pub fn on_player_join_send_existing_entities(event: &PlayerJoinEvent, world: &mut World) {
    let network = world.get::<Network>(event.player);
    for (entity, creator) in <Read<CreationPacketCreator>>::query().iter_entities(world.inner()) {
        let accessor = world
            .entity(entity)
            .expect("query yielded entity which does not exist");
        let packet = creator.get(&accessor);
        network.send_boxed(packet);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item;
    use feather_core::items::{Item, ItemStack};
    use feather_core::network::packets::{PlayerInfo, SpawnObject};
    use feather_core::position;
    use feather_test_framework::Test;
    use std::collections::HashSet;

    #[test]
    fn send_on_spawn() {
        let stack = ItemStack::new(Item::Sand, 47);
        let mut test = Test::new();

        let item1 =
            test.entity(item::create(stack, Default::default()).with(position!(0.0, 100.0, 0.0)));
        let player1 = test.player("player1", Position::default());
        let player2 = test.player("player2", position!(234_234.0, 342.0, 23.0));

        test.handle(
            EntitySpawnEvent { entity: item1 },
            on_entity_spawn_send_to_clients,
        );

        let sent = test.sent::<SpawnObject>(player1).unwrap();
        assert_eq!(sent.entity_id, test.id(item1));

        assert!(test.sent::<SpawnObject>(player2).is_none());
    }

    #[test]
    fn send_existing_entities() {
        let mut test = Test::new();

        let player1 = test.player("player1", position!(1000.0, -5.0, 0.0));
        let player2 = test.player("player2", position!(2000.0, 2_138_901.0, 0.0));
        let player3 = test.player("player3", position!(950.0, 255.0, 0.0));

        test.handle(
            PlayerJoinEvent { player: player3 },
            on_player_join_send_existing_entities,
        );

        let mut players_sent = HashSet::new();
        for _ in 0..3 {
            let packet = test.sent::<PlayerInfo>(player3).unwrap();

            players_sent.insert(packet.uuid);
        }

        for expected in &[player1, player2, player3] {
            assert!(players_sent.contains(&test.uuid(*expected)));
        }
    }
}
