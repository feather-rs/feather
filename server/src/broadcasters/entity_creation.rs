use crate::entity::{CreationPacketCreator, EntityCreateEvent};
use crate::state::State;
use legion::query::Read;
use rayon::prelude::*;
use tonks::{PreparedWorld, QueryAccessor};

/// When an entity is created and has a `CreationPacketCreator`,
/// broadcasts the packet to all online clients.
#[event_handler]
fn broadcast_entity_creation(
    events: &[EntityCreateEvent],
    state: &State,
    accessor: &QueryAccessor<Read<CreationPacketCreator>>,
    world: &mut PreparedWorld,
) {
    events.par_iter().for_each(|event: &EntityCreateEvent| {
        if let Some(accessor) = accessor.find(event.entity) {
            if let Some(packet_creator) = accessor.get_component::<CreationPacketCreator>(world) {
                let packet = packet_creator.get(&accessor, world);
                state.broadcast_global_boxed(packet, None);
            }
        }
    });
}
