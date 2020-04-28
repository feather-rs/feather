use feather_core::network::packets::{
    PlayerLook, PlayerPosition, PlayerPositionAndLookServerbound,
};
use feather_core::util::Position;
use feather_server_types::{Network, PacketBuffers};
use fecs::{component, IntoQuery, World, Write};
use std::sync::Arc;

/// System to handle player movement updates.
#[fecs::system]
pub fn handle_movement_packets(world: &mut World, packet_buffers: &Arc<PacketBuffers>) {
    <Write<Position>>::query()
        .filter(component::<Network>())
        .par_entities_for_each_mut(world.inner_mut(), |(player, mut position)| {
            let mut position: &mut Position = &mut *position;
            for position_and_look in
                packet_buffers.received_for::<PlayerPositionAndLookServerbound>(player)
            {
                position.x = position_and_look.x;
                position.y = position_and_look.feet_y;
                position.z = position_and_look.z;
                position.pitch = position_and_look.pitch;
                position.yaw = position_and_look.yaw;
                position.on_ground = position_and_look.on_ground;
            }

            for position_update in packet_buffers.received_for::<PlayerPosition>(player) {
                position.x = position_update.x;
                position.y = position_update.feet_y;
                position.z = position_update.z;
                position.on_ground = position_update.on_ground;
            }

            for look in packet_buffers.received_for::<PlayerLook>(player) {
                position.pitch = look.pitch;
                position.yaw = look.yaw;
                position.on_ground = look.on_ground;
            }
        });
}
