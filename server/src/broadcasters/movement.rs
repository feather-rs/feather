//! Broadcasting of movement updates.

use crate::chunk_logic::ChunkHolders;
use crate::entity::{EntityId, EntityMoveEvent};
use crate::network::Network;
use crate::util::{calculate_relative_move, degrees_to_stops};
use feather_core::network::packet::implementation::{
    EntityHeadLook, EntityLook, EntityLookAndRelativeMove, EntityRelativeMove,
};
use feather_core::{Packet, Position};
use hashbrown::HashMap;
use legion::entity::Entity;
use legion::query::{Read, Write};
use smallvec::SmallVec;
use tonks::{PreparedWorld, Query};

/// Component containing the last sent positions of all entities for a given client.
/// This component is used to determine
/// the relative movement for an entity.
#[derive(Default)]
pub struct LastKnownPositions(pub HashMap<Entity, Position>);

/// System to broadcast when an entity moves.
#[event_handler]
fn broadcast_move(
    events: &[EntityMoveEvent],
    _query: &mut Query<(
        Read<Network>,
        Read<Position>,
        Write<LastKnownPositions>,
        Read<EntityId>,
    )>,
    world: &mut PreparedWorld,
    chunk_holders: &ChunkHolders,
) {
    events.iter().for_each(|event: &EntityMoveEvent| {
        // Find position of entity.
        let pos = *world.get_component::<Position>(event.entity).unwrap();

        // Find clients which can see the entity.
        let chunk = pos.chunk_pos();
        let clients = chunk_holders.holders_for(chunk).unwrap_or(&[]);

        let entity_id = world.get_component::<EntityId>(event.entity).unwrap().0;

        // For each client, send the position update relative to the client's last known
        // position for the entity. If no `LastKnownPositions` entry exists for the entity,
        // then the entity has not yet been sent to the client, so we do not send a position
        // update. (When an entity is spawned on a client, the `LastKnownPositions` entry
        // is inserted with the starting position.)
        clients.iter().copied().for_each(|client: Entity| {
            // Don't sent player's position to themself
            if client == event.entity {
                return;
            }

            if !world.is_alive(client) {
                return;
            }

            let mut last_known_positions =
                unsafe { world.get_component_mut_unchecked::<LastKnownPositions>(client) }.unwrap();
            if let Some(old_pos) = last_known_positions.0.get_mut(&event.entity) {
                let packets = packets_for_movement_update(entity_id, *old_pos, pos);

                let network = world.get_component::<Network>(client).unwrap();

                packets.into_iter().for_each(|packet| {
                    network.send_boxed(packet);
                });

                // Update last known position.
                *old_pos = pos;
            }
        });
    });
}

/// Returns the packet needed to notify a client
/// of a position update, from the old position to the new one.
#[allow(clippy::float_cmp)]
fn packets_for_movement_update(
    entity_id: i32,
    old_pos: Position,
    new_pos: Position,
) -> SmallVec<[Box<dyn Packet>; 2]> {
    if old_pos == new_pos {
        return smallvec![];
    }

    let mut packets = smallvec![];

    let has_moved = old_pos.x != new_pos.x || old_pos.y != new_pos.y || old_pos.z != new_pos.z;
    let has_looked = old_pos.pitch != new_pos.pitch
        || old_pos.yaw != new_pos.yaw
        || old_pos.on_ground != new_pos.on_ground;

    if has_moved {
        let (rx, ry, rz) = calculate_relative_move(old_pos, new_pos);

        if (rx == 0 && ry == 0 && rz == 0) && !has_looked {
            // Because of floating point errors,
            // the physics system may trigger an
            // event when the distance moved is minuscule,
            // which causes jittering on the client.
            // Don't send the packet if it has no effect.
            return smallvec![];
        }

        if has_looked {
            let packet: Box<dyn Packet> = Box::new(EntityLookAndRelativeMove::new(
                entity_id,
                rx,
                ry,
                rz,
                degrees_to_stops(new_pos.yaw),
                degrees_to_stops(new_pos.pitch),
                new_pos.on_ground,
            ));
            packets.push(packet);
        } else {
            let packet: Box<dyn Packet> = Box::new(EntityRelativeMove::new(
                entity_id,
                rx,
                ry,
                rz,
                new_pos.on_ground,
            ));
            packets.push(packet);
        }
    } else {
        let packet: Box<dyn Packet> = Box::new(EntityLook::new(
            entity_id,
            degrees_to_stops(new_pos.yaw),
            degrees_to_stops(new_pos.pitch),
            new_pos.on_ground,
        ));
        packets.push(packet);
    }

    // Entity Head Look also needs to be sent if the entity turned its head
    if has_looked {
        let packet: Box<dyn Packet> = Box::new(EntityHeadLook::new(
            entity_id,
            degrees_to_stops(new_pos.yaw),
        ));
        packets.push(packet);
    }

    packets
}
