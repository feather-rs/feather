use crate::chunk_logic::ChunkHolders;
use crate::entity::EntityDeleteEvent;
use crate::network::Network;
use crate::state::State;
use feather_core::network::packet::implementation::DestroyEntities;
use legion::query::Read;
use rayon::prelude::*;
use tonks::{PreparedWorld, Query};

/// Broadcasts when an entity is deleted.
#[event_handler]
fn broadcast_entity_deletion(
    events: &[EntityDeleteEvent],
    holders: &ChunkHolders,
    _query: &mut Query<Read<Network>>,
    world: &mut PreparedWorld,
    state: &State,
) {
    events.par_iter().for_each(|event: &EntityDeleteEvent| {
        if let Some(pos) = event.position {
            let chunk = pos.into();

            for entity in holders.holders_for(chunk).unwrap_or(&[]) {
                if let Some(network) = world.get_component::<Network>(*entity) {
                    network.send(DestroyEntities {
                        entity_ids: vec![event.id.0],
                    });
                    state.register_entity_unload(event.entity, *entity);
                }
            }
        }

        // If entity was a player, broadcast PlayerInfo with delete status.
        // TODO: fix
        /*
        if world.get_component::<Player>(event.entity).is_some() {
            let packet = PlayerInfo {
                action: PlayerInfoAction::RemovePlayer,
                uuid: event.uuid,
            };

            state.broadcast_global(packet, None);
        }*/
    });
}
