//! Assorted utility functions.

use crate::entity::{EntityDeleteEvent, EntityId, Name};
use crate::io::ServerToWorkerMessage;
use crate::network::Network;
use crate::state::State;
use feather_core::Position;
use legion::entity::Entity;
use std::borrow::Cow;
use uuid::Uuid;

/// Calculates the relative move fields
/// as used in the Entity Relative Move packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (i16, i16, i16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as i16;
    let y = ((current.y * 32.0 - old.y * 32.0) * 128.0) as i16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as i16;
    (x, y, z)
}

/// Converts degrees to stops as used in the protocol.
pub fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.0) as u8
}

/// Disconnects a player.
pub fn disconnect_player(state: &State, player: Entity, reason: impl Into<Cow<'static, str>>) {
    let reason = reason.into();

    state.exec_with_scheduler(move |world, scheduler| {
        {
            let username = world.get_component::<Name>(player).unwrap();
            info!("Disconnecting player {}: {}", username.0, reason);

            let network = world.get_component::<Network>(player).unwrap();
            network
                .sender
                .unbounded_send(ServerToWorkerMessage::Disconnect)
                .unwrap();

            let position = *world.get_component::<Position>(player).unwrap();
            let id = *world.get_component::<EntityId>(player).unwrap();
            let uuid = *world.get_component::<Uuid>(player).unwrap();

            scheduler.trigger(EntityDeleteEvent {
                entity: player,
                position: Some(position),
                id,
                uuid,
            });

            let event = EntityDeleteEvent {
                entity: player,
                position: Some(position),
                id,
                uuid,
            };
            scheduler.trigger(event);
        }

        world.delete(player);
    });
}
