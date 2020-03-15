use crate::chunk_logic::ChunkHolders;
use crate::entity::{CreationPacketCreator, EntityCreateEvent, SpawnPacketCreator};
use crate::network::Network;
use crate::player::PlayerJoinEvent;
use crate::state::State;
use feather_core::{ChunkPosition, Position};
use legion::query::Read;
use rayon::prelude::*;
use tonks::{PreparedWorld, Query, QueryAccessor};

/// When an entity is created and has a `CreationPacketCreator` and/or `SpawnPacketCreator`,
/// broadcasts the packets to all online clients.
#[event_handler]
fn broadcast_entity_creation(
    events: &[EntityCreateEvent],
    state: &State,
    accessor1: &QueryAccessor<Read<CreationPacketCreator>>,
    accessor2: &QueryAccessor<Read<SpawnPacketCreator>>,
    _query: &mut Query<(
        Read<Position>,
        Read<crate::metadata::Metadata>,
        Read<crate::entity::EntityId>,
        Read<crate::network::Network>,
    )>,
    world: &mut PreparedWorld,
    holders: &ChunkHolders,
) {
    events.par_iter().for_each(|event: &EntityCreateEvent| {
        if let Some(accessor) = accessor1.find(event.entity) {
            if let Some(packet_creator) = accessor.get_component::<CreationPacketCreator>(world) {
                let packet = packet_creator.get(&accessor, world);
                state.broadcast_global_boxed(packet, None);
            }
        }

        if let Some(accessor) = accessor2.find(event.entity) {
            if let Some(packet_creator) = accessor.get_component::<SpawnPacketCreator>(world) {
                let packet = packet_creator.get(&accessor, world);
                // state.broadcast_entity_update_boxed(event.entity, packet, Some(event.entity));

                // state.broadcast_entity_update_boxed(event.entity, packet, Some(event.entity));
                if let Some(meta) = world.get_component::<crate::metadata::Metadata>(event.entity) {
                    let chunk: ChunkPosition =
                        (*world.get_component::<Position>(event.entity).unwrap()).into();
                    for entity in holders.holders_for(chunk).unwrap_or(&[]) {
                        if let Some(network) =
                            world.get_component::<crate::network::Network>(*entity)
                        {
                            use feather_core::network::packet::implementation::PacketEntityMetadata;
                            network.send_boxed(packet.box_clone());
                            let entity_id = world
                                .get_component::<crate::entity::EntityId>(event.entity)
                                .unwrap()
                                .0;
                            let packet = PacketEntityMetadata {
                                entity_id,
                                metadata: meta.to_full_raw_metadata(),
                            };
                            network.send(packet);
                        }
                    }
                }
            }
        }

        // Register entity sends
        let chunk = (*world.get_component::<Position>(event.entity).unwrap()).into();
        for entity in holders.holders_for(chunk).unwrap_or(&[]) {
            state.register_entity_send(event.entity, *entity);
        }
    });
}

/// Wehn a player joins, sends existing entities to the player.
///
/// This only handles init packets (PlayerInfo, etc.)—spawn packets
/// are handled by the view update mechanism.
#[event_handler]
fn broadcast_existing_entities(
    events: &[PlayerJoinEvent],
    accessor: &QueryAccessor<Read<CreationPacketCreator>>,
    query: &mut Query<Read<CreationPacketCreator>>,
    _query2: &mut Query<Read<Network>>,
    world: &mut PreparedWorld,
    state: &State,
) {
    // TODO: change to par_iter when legion implements immutable queries
    events.iter().for_each(|event: &PlayerJoinEvent| {
        // Send init packets for all entities with a `CreationPacketCreator`.
        let network = world.get_component::<Network>(event.player).unwrap();

        query.par_entities_for_each_immutable(world, |(entity, packet_creator)| {
            if let Some(accessor) = accessor.find(entity) {
                let packet = packet_creator.get(&accessor, world);
                network.send_boxed(packet);
                state.register_entity_send(entity, event.player);
            }
        });
    });
}
