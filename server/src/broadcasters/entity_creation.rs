use crate::entity::{CreationPacketCreator, SpawnPacketCreator};
use crate::game::Game;
use crate::network::Network;
use crate::BumpVec;
use feather_core::Position;
use fecs::{Entity, IntoQuery, Read, World};

/// When an entity is created and has a `CreationPacketCreator` and/or `SpawnPacketCreator`,
/// broadcasts the packets to all online clients.
pub fn on_entity_spawn_send_to_clients(game: &mut Game, world: &mut World, entity: Entity) {
    let accessor = world.entity(entity).expect("entity does not exist");

    if let Some(creator) = world.try_get::<CreationPacketCreator>(entity) {
        let packet = creator.get(&accessor);
        game.broadcast_global_boxed(world, packet, None);
    }
    let mut to_trigger = BumpVec::new_in(game.bump());

    if let Some(creator) = world.try_get::<SpawnPacketCreator>(entity) {
        let packet = creator.get(&accessor);
        game.broadcast_entity_update_boxed(world, packet, entity, Some(entity));

        let chunk = world.get::<Position>(entity).chunk();

        drop(creator);

        // trigger on_entity_send
        for player in game.chunk_holders.holders_for(chunk) {
            if world.try_get::<Network>(*player).is_some() {
                to_trigger.push(*player);
            }
        }
    }

    for client in to_trigger {
        game.on_entity_send(world, entity, client);
    }
}

/// Wehn a player joins, sends existing entities to the player.
///
/// This only handles init packets (PlayerInfo, etc.)—spawn packets
/// are handled by the view update mechanism in `crate::view`.
pub fn on_player_join_send_existing_entities(world: &mut World, player: Entity) {
    let network = world.get::<Network>(player);
    for (entity, creator) in <Read<CreationPacketCreator>>::query().iter_entities(world.inner()) {
        let accessor = world
            .entity(entity)
            .expect("query yielded entity which does not exist");
        let packet = creator.get(&accessor);
        network.send_boxed(packet);
    }
}
