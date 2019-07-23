//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

use crate::disconnect_player;
use crate::network::{
    send_packet_boxed_to_player, send_packet_to_player, NetworkComponent, PacketQueue,
};
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    EntityLook, EntityLookAndRelativeMove, EntityRelativeMove, PlayerLook, PlayerPosition,
    PlayerPositionAndLookServerbound,
};
use feather_core::network::packet::{Packet, PacketType};
use feather_core::world::Position;
use feather_core::Gamemode;
use specs::storage::BTreeStorage;
use specs::{
    Component, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, VecStorage,
    WriteStorage,
};
use uuid::Uuid;

pub struct PlayerComponent {
    pub profile_properties: Vec<mojang_api::ServerAuthProperty>,
    pub gamemode: Gamemode,
}

impl Component for PlayerComponent {
    type Storage = BTreeStorage<Self>;
}

pub struct EntityComponent {
    pub uuid: Uuid,
    pub display_name: String,
    pub position: Position,
    pub on_ground: bool,
}

impl Component for EntityComponent {
    type Storage = VecStorage<Self>;
}

/// System for handling player update packets,
/// including movement and inventory packets.
pub struct PlayerUpdateSystem;

impl<'a> System<'a> for PlayerUpdateSystem {
    type SystemData = (
        WriteStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, PacketQueue>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ecomps, pcomps, packet_queue, netcomps, entities, lazy) = data;

        // Take movement packets
        let mut packets = vec![];
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPosition));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPositionAndLookServerbound));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerLook));

        // Handle movement packets
        for (player, packet) in packets {
            let ecomp = ecomps.get(player).unwrap();

            let mut has_moved = false;
            let mut has_looked = false;

            // Get position using packet and old position
            let new_pos = match packet.ty() {
                PacketType::PlayerPosition => {
                    has_moved = true;
                    let packet = cast_packet::<PlayerPosition>(&packet);

                    Position::new(
                        packet.x,
                        packet.feet_y,
                        packet.z,
                        ecomp.position.pitch,
                        ecomp.position.yaw,
                    )
                }
                PacketType::PlayerLook => {
                    has_looked = true;
                    let packet = cast_packet::<PlayerLook>(&packet);

                    Position::new(
                        ecomp.position.x,
                        ecomp.position.y,
                        ecomp.position.z,
                        packet.pitch,
                        packet.yaw,
                    )
                }
                PacketType::PlayerPositionAndLookServerbound => {
                    has_moved = true;
                    has_looked = true;
                    let packet = cast_packet::<PlayerPositionAndLookServerbound>(&packet);

                    Position::new(packet.x, packet.feet_y, packet.z, packet.pitch, packet.yaw)
                }
                _ => unreachable!(),
            };

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

/// Broadcasts to all joined players that an entity has moved.
fn broadcast_entity_movement(
    entity: Entity,
    old_pos: Position,
    new_pos: Position,
    has_moved: bool,
    has_looked: bool,
    netcomps: &ReadStorage<NetworkComponent>,
    pcomps: &ReadStorage<PlayerComponent>,
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
            for (net, _, e) in (netcomps, pcomps, entities).join() {
                if entity != e {
                    send_packet_to_player(net, packet.clone());
                }
            }
        } else {
            let packet = EntityRelativeMove::new(entity.id() as i32, rx, ry, rz, true);
            for (net, _, e) in (netcomps, pcomps, entities).join() {
                if entity != e {
                    send_packet_to_player(net, packet.clone());
                }
            }
        }
    } else {
        let packet = EntityLook::new(
            entity.id() as i32,
            degrees_to_stops(new_pos.yaw),
            degrees_to_stops(new_pos.pitch),
            true,
        );
        for (net, _, e) in (netcomps, pcomps, entities).join() {
            if entity != e {
                send_packet_to_player(net, packet.clone());
            }
        }
    }
}

/// Confirms that a player is past the login
/// sequence and has joined the server, disconnecting
/// them and returning `false` if they have not.
pub fn check_player_joined(
    player: Entity,
    pcomps: &ReadStorage<PlayerComponent>,
    lazy: &LazyUpdate,
) -> bool {
    if !pcomps.contains(player) {
        disconnect_player(player, "You have not yet joined!", lazy);
        return false;
    }

    true
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
    ((degs / 360.0) * 256.) as u8
}
