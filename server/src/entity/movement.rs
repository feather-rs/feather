use specs::storage::ComponentEvent;
use specs::{
    BitSet, Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, ReaderId,
    System, WriteStorage,
};

use feather_core::network::packet::implementation::{
    EntityHeadLook, EntityLook, EntityLookAndRelativeMove, EntityRelativeMove, EntityVelocity,
};
use feather_core::world::Position;

use crate::chunk_logic::ChunkHolders;
use crate::entity::{PositionComponent, VelocityComponent};
use crate::network::{send_packet_boxed_to_player, NetworkComponent};
use crate::util::{protocol_velocity, Util};
use feather_core::Packet;
use hashbrown::HashMap;
use smallvec::SmallVec;

/// Component which stores the last known position for any given entity
/// for a player.
///
/// This is used to ensure position remains synced across clients, since
/// relative movement packets are used.
#[derive(Default, Debug)]
pub struct LastKnownPositionComponent(pub HashMap<Entity, Position>);

impl Component for LastKnownPositionComponent {
    type Storage = DenseVecStorage<Self>;
}

/// System for broadcasting when an entity moves.
#[derive(Default)]
pub struct EntityMoveBroadcastSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
    held: BitSet,
}

impl<'a> System<'a> for EntityMoveBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        WriteStorage<'a, LastKnownPositionComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, ChunkHolders>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut last_positions, networks, chunk_holders, entities) = data;

        self.dirty.clear();

        for event in positions.channel().read(&mut self.reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(index) | ComponentEvent::Inserted(index) => {
                    self.dirty.add(*index);
                }
                _ => (),
            }
        }

        for (position, entity, _) in (&positions, &entities, &self.dirty).join() {
            // Populate `self.held` with chunk holders for this entity's chunk
            for entity in chunk_holders
                .holders_for(position.current.chunk_pos())
                .unwrap_or(&[])
            {
                self.held.add(entity.id());
            }

            // For each player which can see this entity's chunk, send a movement update packet.
            for (network, last_positions, _) in (&networks, &mut last_positions, &self.held).join()
            {
                let last_known_position = match last_positions.0.get(&entity) {
                    Some(pos) => pos,
                    None => continue, // Player hasn't yet known this entity
                };

                if let Some(packets) =
                    packet_for_movement_update(entity, *last_known_position, position.current)
                {
                    packets
                        .into_iter()
                        .for_each(|packet| send_packet_boxed_to_player(network, packet));
                }

                last_positions.0.insert(entity, position.current);
            }

            self.held.clear();
        }
    }

    flagged_setup_impl!(PositionComponent, reader);
}

/// Returns the packet needed to notify a client
/// of a position update, from the old position to the new one.
#[allow(clippy::float_cmp)]
pub fn packet_for_movement_update(
    entity: Entity,
    old_pos: Position,
    new_pos: Position,
) -> Option<SmallVec<[Box<dyn Packet>; 2]>> {
    if old_pos == new_pos {
        return None;
    }

    let mut packets = smallvec![];

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
            return None;
        }

        if has_looked {
            let packet: Box<dyn Packet> = Box::new(EntityLookAndRelativeMove::new(
                entity.id() as i32,
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
                entity.id() as i32,
                rx,
                ry,
                rz,
                new_pos.on_ground,
            ));
            packets.push(packet);
        }
    } else {
        let packet: Box<dyn Packet> = Box::new(EntityLook::new(
            entity.id() as i32,
            degrees_to_stops(new_pos.yaw),
            degrees_to_stops(new_pos.pitch),
            new_pos.on_ground,
        ));
        packets.push(packet);
    }

    // Entity Head Look also needs to be sent if the entity turned its head
    if has_looked {
        let packet: Box<dyn Packet> = Box::new(EntityHeadLook::new(
            entity.id() as i32,
            degrees_to_stops(new_pos.yaw),
        ));
        packets.push(packet);
    }

    Some(packets)
}

/// System for broadcasting when an entity's velocity
/// is updated.
#[derive(Default)]
pub struct EntityVelocityBroadcastSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for EntityVelocityBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, VelocityComponent>,
        Read<'a, Util>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (velocities, util, entities) = data;

        self.dirty.clear();

        for event in velocities.channel().read(self.reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(index) | ComponentEvent::Inserted(index) => {
                    self.dirty.add(*index);
                }
                _ => (),
            }
        }

        for (velocity, entity, _) in (&velocities, &entities, &self.dirty).join() {
            let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);
            let packet = EntityVelocity {
                entity_id: entity.id() as i32,
                velocity_x,
                velocity_y,
                velocity_z,
            };

            util.broadcast_entity_update(entity, packet, Some(entity));
        }
    }

    flagged_setup_impl!(VelocityComponent, reader);
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
    use specs::WorldExt;

    use feather_core::network::cast_packet;
    use feather_core::network::packet::PacketType;

    use crate::entity::EntityType;
    use crate::testframework as t;

    use super::*;

    #[test]
    fn test_velocity_broadcast_system() {
        let (mut w, mut d) = t::builder()
            .with(EntityVelocityBroadcastSystem::default(), "")
            .build();

        let player = t::add_player(&mut w);

        let entity = t::add_entity(&mut w, EntityType::Item, false);

        w.write_component::<VelocityComponent>()
            .insert(entity, VelocityComponent(glm::vec3(0.0, 0.0, 0.0)))
            .unwrap();

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
