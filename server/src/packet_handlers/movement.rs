use crate::game::Game;
use crate::network::Network;
use feather_core::network::packet::implementation::{
    PlayerLook, PlayerPosition, PlayerPositionAndLookServerbound,
};
use feather_core::Position;
use fecs::{component, IntoQuery, World, Write};

/// System to handle player movement updates.
#[system]
pub fn handle_movement_packets(game: &Game, world: &mut World) {
    <Write<Position>>::query()
        .filter(component::<Network>())
        .par_entities_for_each_mut(world.inner_mut(), |(player, mut position)| {
            let mut position: &mut Position = &mut *position;
            for position_and_look in game.received_for::<PlayerPositionAndLookServerbound>(player) {
                position.x = position_and_look.x;
                position.y = position_and_look.feet_y;
                position.z = position_and_look.z;
                position.pitch = position_and_look.pitch;
                position.yaw = position_and_look.yaw;
                position.on_ground = position_and_look.on_ground;
            }

            for position_update in game.received_for::<PlayerPosition>(player) {
                position.x = position_update.x;
                position.y = position_update.feet_y;
                position.z = position_update.z;
                position.on_ground = position_update.on_ground;
            }

            for look in game.received_for::<PlayerLook>(player) {
                position.pitch = look.pitch;
                position.yaw = look.yaw;
                position.on_ground = look.on_ground;
            }
        });
}
