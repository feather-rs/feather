use crate::network::{send_packet_to_all_players, NetworkComponent};
use feather_core::world::Position;
use specs::{Entities, Entity, Read, ReadStorage, ReaderId, System, SystemData, World};

use crate::entity::VelocityComponent;
use crate::physics::EntityVelocityUpdateEvent;
use crate::util::protocol_velocity;
use feather_core::network::packet::implementation::{
    EntityHeadLook, EntityLook, EntityLookAndRelativeMove, EntityRelativeMove, EntityVelocity,
};
use shrev::EventChannel;

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

/// System for broadcasting when an entity moves.
///
/// This system listens to `EntityMoveEvent`s.
#[derive(Default)]
pub struct EntityMoveBroadcastSystem {
    reader: Option<ReaderId<EntityMoveEvent>>,
}

impl<'a> System<'a> for EntityMoveBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<EntityMoveEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (networks, events, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            broadcast_entity_movement(
                event.entity,
                event.old_pos,
                event.new_pos,
                &networks,
                &entities,
            );
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

/// System for broadcasting when an entity's velocity
/// is updated.
///
/// This system listens to `EntityVelocityUpdateEvent`s.
#[derive(Default)]
pub struct EntityVelocityBroadcastSystem {
    reader: Option<ReaderId<EntityVelocityUpdateEvent>>,
}

impl<'a> System<'a> for EntityVelocityBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, VelocityComponent>,
        Read<'a, EventChannel<EntityVelocityUpdateEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (networks, velocities, events, entities) = data;

        for event in events.read(self.reader.as_mut().unwrap()) {
            let velocity = velocities.get(event.entity).unwrap();
            let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);
            let packet = EntityVelocity {
                entity_id: event.entity.id() as i32,
                velocity_x,
                velocity_y,
                velocity_z,
            };

            send_packet_to_all_players(&networks, &entities, packet, None);
        }
    }

    setup_impl!(reader);
}

/// Broadcasts to all joined players that an entity has moved.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::float_cmp)]
pub fn broadcast_entity_movement(
    entity: Entity,
    old_pos: Position,
    new_pos: Position,
    networks: &ReadStorage<NetworkComponent>,
    entities: &Entities,
) {
    if old_pos == new_pos {
        return;
    }

    let has_moved = old_pos.x != new_pos.x || old_pos.y != new_pos.y || old_pos.z != new_pos.z;
    let has_looked = old_pos.pitch != new_pos.pitch || old_pos.yaw != new_pos.yaw;

    if has_moved {
        let (rx, ry, rz) = calculate_relative_move(old_pos, new_pos);

        if (rx == 0 && ry == 0 && rz == 0) && !has_looked {
            // Because of floating point errors,
            // the physics system may trigger an
            // event when the distance moved is minuscule,
            // which causes jittering on the client.
            // Don't send the packet if it has no effect.
            return;
        }

        if has_looked {
            let packet = EntityLookAndRelativeMove::new(
                entity.id() as i32,
                rx,
                ry,
                rz,
                degrees_to_stops(new_pos.yaw),
                degrees_to_stops(new_pos.pitch),
                new_pos.on_ground,
            );
            send_packet_to_all_players(networks, entities, packet, Some(entity));
        } else {
            let packet = EntityRelativeMove::new(entity.id() as i32, rx, ry, rz, new_pos.on_ground);
            send_packet_to_all_players(networks, entities, packet, Some(entity));
        }
    } else {
        let packet = EntityLook::new(
            entity.id() as i32,
            degrees_to_stops(new_pos.yaw),
            degrees_to_stops(new_pos.pitch),
            new_pos.on_ground,
        );
        send_packet_to_all_players(networks, entities, packet, Some(entity));
    }

    // Entity Head Look also needs to be sent if the entity turned its head
    if has_looked {
        let packet = EntityHeadLook::new(entity.id() as i32, degrees_to_stops(new_pos.yaw));
        send_packet_to_all_players(networks, entities, packet, Some(entity));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityType;
    use crate::testframework as t;
    use feather_core::network::cast_packet;
    use feather_core::network::packet::PacketType;
    use specs::WorldExt;

    #[test]
    fn test_velocity_broadcast_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let entity = t::add_entity(&mut w, EntityType::Item, true);

        let event = EntityVelocityUpdateEvent { entity };
        t::trigger_event(&w, event);

        d.dispatch(&w);
        w.maintain();

        let packet = t::assert_packet_received(&player, PacketType::EntityVelocity);
        let packet = cast_packet::<EntityVelocity>(&*packet);
        assert_eq!(packet.entity_id, entity.id() as i32);
        assert_eq!(packet.velocity_x, 0);
        assert_eq!(packet.velocity_y, 0);
        assert_eq!(packet.velocity_z, 0);
    }
}
