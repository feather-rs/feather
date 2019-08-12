use crate::network::{send_packet_to_all_players, NetworkComponent};
use feather_core::world::Position;
use specs::{Entities, Entity, ReadStorage};

use feather_core::network::packet::implementation::{
    EntityHeadLook, EntityLook, EntityLookAndRelativeMove, EntityRelativeMove,
};

/// Event triggered when an entity moves.
///
/// This event is triggered *after* the entity's
/// position is updated.
#[derive(Debug, Clone)]
pub struct EntityMoveEvent {
    pub entity: Entity,
    pub old_pos: Position,
    pub new_pos: Position,
}

/// Broadcasts to all joined players that an entity has moved.
#[allow(clippy::too_many_arguments)]
pub fn broadcast_entity_movement(
    entity: Entity,
    old_pos: Position,
    new_pos: Position,
    has_moved: bool,
    has_looked: bool,
    netcomps: &ReadStorage<NetworkComponent>,
    entities: &Entities,
) {
    assert!(has_moved || has_looked);

    if has_moved {
        let (rx, ry, rz) = calculate_relative_move(old_pos, new_pos);

        if has_looked {
            let packet = EntityLookAndRelativeMove::new(
                entity.id() as i32,
                rx,
                ry,
                rz,
                degrees_to_stops(new_pos.yaw),
                degrees_to_stops(new_pos.pitch),
                true,
            );
            send_packet_to_all_players(netcomps, entities, packet, Some(entity));
        } else {
            let packet = EntityRelativeMove::new(entity.id() as i32, rx, ry, rz, true);
            send_packet_to_all_players(netcomps, entities, packet, Some(entity));
        }
    } else {
        let packet = EntityLook::new(
            entity.id() as i32,
            degrees_to_stops(new_pos.yaw),
            degrees_to_stops(new_pos.pitch),
            true,
        );
        send_packet_to_all_players(netcomps, entities, packet, Some(entity));
    }

    // Entity Head Look also needs to be sent if the entity turned its head
    if has_looked {
        let packet = EntityHeadLook::new(entity.id() as i32, degrees_to_stops(new_pos.yaw));
        send_packet_to_all_players(netcomps, entities, packet, Some(entity));
    }
}

/// Calculates the relative move fields
/// as used in the Entity Relative Move packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (i16, i16, i16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as i16;
    let y = ((current.y * 32.0 - old.y * 32.0) * 128.0) as i16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as i16;
    (x, y, z)
}

pub fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.0) as u8
}
