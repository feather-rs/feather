//! This module provides systems and components
//! relating to players, including player movement
//! and inventory handling.

use crate::entity::{broadcast_entity_movement, EntityComponent, PlayerComponent};
use crate::network::{NetworkComponent, PacketQueue};
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    PlayerLook, PlayerPosition, PlayerPositionAndLookServerbound,
};
use feather_core::network::packet::{Packet, PacketType};
use feather_core::world::Position;
use specs::{Entities, LazyUpdate, Read, ReadStorage, System, WriteStorage};

/// System for handling player movement
/// packets.
pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, PacketQueue>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ecomps, pcomps, packet_queue, netcomps, entities, _) = data;

        // Take movement packets
        let mut packets = vec![];
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPosition));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPositionAndLookServerbound));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerLook));

        // Handle movement packets
        for (player, packet) in packets {
            let ecomp = ecomps.get(player).unwrap();

            // Get position using packet and old position
            let (new_pos, has_moved, has_looked) = new_pos_from_packet(ecomp.position, packet);

            // Broadcast position update
            broadcast_entity_movement(
                player,
                ecomp.position,
                new_pos,
                has_moved,
                has_looked,
                &netcomps,
                &pcomps,
                &entities,
            );

            // Set new position
            ecomps.get_mut(player).unwrap().position = new_pos;
        }
    }
}

fn new_pos_from_packet(old_pos: Position, packet: Box<Packet>) -> (Position, bool, bool) {
    let mut has_looked = false;
    let mut has_moved = false;

    let pos = match packet.ty() {
        PacketType::PlayerPosition => {
            has_moved = true;
            let packet = cast_packet::<PlayerPosition>(&packet);

            Position::new(
                packet.x,
                packet.feet_y,
                packet.z,
                old_pos.pitch,
                old_pos.yaw,
            )
        }
        PacketType::PlayerLook => {
            has_looked = true;
            let packet = cast_packet::<PlayerLook>(&packet);

            Position::new(old_pos.x, old_pos.y, old_pos.z, packet.pitch, packet.yaw)
        }
        PacketType::PlayerPositionAndLookServerbound => {
            has_moved = true;
            has_looked = true;
            let packet = cast_packet::<PlayerPositionAndLookServerbound>(&packet);

            Position::new(packet.x, packet.feet_y, packet.z, packet.pitch, packet.yaw)
        }
        _ => panic!(),
    };

    (pos, has_moved, has_looked)
}
