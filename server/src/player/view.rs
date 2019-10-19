//! This module implements creating and destroying
//! entities on the client when a player moves.
//!
//! When a player crosses chunk boundaries, the following
//! takes place:
//! * We send a `Destroy Entities` packet containing all
//! entities which are no longer within the view distance.
//! * We spawn an entity on the client for every entity
//! which is now within the view distance.
//!
//! This is handled by `ViewUpdateSystem`, which listens
//! to `ChunkCrossEvent`s.

use crate::config::Config;
use crate::entity::ChunkEntities;
use crate::lazy::LazyUpdateExt;
use crate::network::{send_packet_to_player, NetworkComponent};
use crate::player::movement::ChunkCrossEvent;
use feather_core::network::packet::implementation::DestroyEntities;
use shrev::EventChannel;
use specs::{LazyUpdate, Read, ReadStorage, ReaderId, System};
use std::sync::Arc;

/// System for updating entities visible
/// by the client.
#[derive(Default)]
pub struct ViewUpdateSystem {
    reader: Option<ReaderId<ChunkCrossEvent>>,
}

impl<'a> System<'a> for ViewUpdateSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<ChunkCrossEvent>>,
        Read<'a, ChunkEntities>,
        Read<'a, Arc<Config>>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (networks, cross_events, chunk_entities, config, lazy) = data;

        for event in cross_events.read(self.reader.as_mut().unwrap()) {
            // Find new and old entities.
            let old_entities =
                chunk_entities.entites_within_view_distance(event.old, config.server.view_distance);
            let new_entities =
                chunk_entities.entites_within_view_distance(event.new, config.server.view_distance);

            let mut to_destroy = vec![];

            // Compute entities which are only present in one of the sets.
            // If an entity is only present in `old_entities` and not `new_entities`,
            // it should be destroyed on the client.
            // If an entity is only present in `new_entities`, it should be spawned.
            for entity in old_entities.symmetric_difference(&new_entities) {
                if *entity == event.player {
                    continue;
                }

                if old_entities.contains(entity) {
                    // Entity is in `old_entities` but not in `new_entities`.
                    // Destroy it. If the entity is a player, also destroy this player
                    // on the client.
                    to_destroy.push(entity.id() as i32);

                    if let Some(network) = networks.get(*entity) {
                        let packet = DestroyEntities {
                            entity_ids: vec![event.player.id() as i32],
                        };
                        send_packet_to_player(network, packet);
                    }
                } else {
                    // Entity is in `new_entities` but not in `old_entities`.
                    // Spawn it. If the entity is a player, also send this player
                    // to that entity.
                    lazy.send_entity_to_player(event.player, *entity);

                    if networks.get(*entity).is_some() {
                        lazy.send_entity_to_player(*entity, event.player);
                    }
                }
            }

            if !to_destroy.is_empty() {
                let packet = DestroyEntities {
                    entity_ids: to_destroy,
                };
                send_packet_to_player(&networks.get(event.player).unwrap(), packet);
            }
        }
    }

    setup_impl!(reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{item, PositionComponent};
    use crate::testframework as t;
    use feather_core::network::cast_packet;
    use feather_core::network::packet::implementation::{SpawnObject, SpawnPlayer};
    use feather_core::{ChunkPosition, Item, ItemStack, PacketType};
    use hashbrown::HashSet;
    use specs::{Builder, WorldExt};

    #[test]
    fn test_view_update_system() {
        let (mut world, mut dispatcher) = t::builder()
            .with(ViewUpdateSystem::default(), "view")
            .build();

        let player_chunk = ChunkPosition::new(0, 0);

        let player1 = t::add_player_without_holder(&mut world);
        let player2 = t::add_player_without_holder(&mut world);

        let entity1 = item::create(
            &world.fetch(),
            &world.fetch(),
            ItemStack::new(Item::Stone, 0),
            0,
        )
        .with(PositionComponent::default())
        .build();
        let entity2 = item::create(
            &world.fetch(),
            &world.fetch(),
            ItemStack::new(Item::Stone, 0),
            0,
        )
        .with(PositionComponent::default())
        .build();
        let entity3 = item::create(
            &world.fetch(),
            &world.fetch(),
            ItemStack::new(Item::Stone, 0),
            0,
        )
        .with(PositionComponent::default())
        .build();
        let entity4 = item::create(
            &world.fetch(),
            &world.fetch(),
            ItemStack::new(Item::Stone, 0),
            0,
        )
        .with(PositionComponent::default())
        .build();

        let mut config = Config::default();
        config.server.view_distance = 4;
        world.insert(Arc::new(config));

        {
            let mut chunk_entities = world.fetch_mut::<ChunkEntities>();
            chunk_entities.add_to_chunk(player_chunk, player1.entity);
            chunk_entities.add_to_chunk(player_chunk, player2.entity);
            chunk_entities.add_to_chunk(ChunkPosition::new(3, -3), entity1);
            chunk_entities.add_to_chunk(player_chunk, entity2);
            chunk_entities.add_to_chunk(ChunkPosition::new(4, -3), entity3);
            chunk_entities.add_to_chunk(ChunkPosition::new(100, 103), entity4);
        }

        let event = ChunkCrossEvent {
            player: player1.entity,
            old: ChunkPosition::new(100, 103),
            new: player_chunk,
        };
        t::trigger_event(&world, event);

        world.maintain();
        dispatcher.dispatch(&world);
        world.maintain();
        dispatcher.dispatch(&world);
        world.maintain();

        let packets = t::received_packets(&player1, None);

        let mut received_spawns = HashSet::new();

        for packet in packets {
            dbg!(packet.ty());
            match packet.ty() {
                PacketType::DestroyEntities => {
                    let packet = cast_packet::<DestroyEntities>(&*packet);
                    let destroyed = packet.entity_ids.iter().cloned().collect::<HashSet<_>>();
                    assert_eq!(destroyed.len(), 1);
                    assert!(destroyed.contains(&(entity4.id() as i32)));
                }
                PacketType::SpawnObject => {
                    let packet = cast_packet::<SpawnObject>(&*packet);
                    received_spawns.insert(packet.entity_id);
                }
                PacketType::SpawnPlayer => {
                    let packet = cast_packet::<SpawnPlayer>(&*packet);
                    received_spawns.insert(packet.entity_id);
                }
                _ => (),
            }
        }

        println!("{:?}", received_spawns);
        assert_eq!(received_spawns.len(), 3);
        assert!(received_spawns.contains(&(entity1.id() as i32)));
        assert!(received_spawns.contains(&(entity2.id() as i32)));
        assert!(received_spawns.contains(&(player2.entity.id() as i32)));

        // Confirm that `player1` was sent to `player2`.
        let packets = t::received_packets(&player2, None);
        assert_eq!(packets.len(), 2); // One for Spawn Player, one for metadata

        let packet = packets
            .into_iter()
            .find(|packet| packet.ty() == PacketType::SpawnPlayer)
            .unwrap();
        let packet = cast_packet::<SpawnPlayer>(&*packet);
        assert_eq!(packet.entity_id, player1.entity.id() as i32);
    }
}
