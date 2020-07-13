use feather_core::network::packets::UpdateHealth;
use feather_server_types::{HealthUpdateEvent, Network};
use fecs::World;

/// When a player's health is updated, updates it on the client.
#[fecs::event_handler]
pub fn on_health_update_send(event: &HealthUpdateEvent, world: &mut World) {
    if let Some(network) = world.try_get::<Network>(event.entity) {
        let packet = UpdateHealth {
            health: event.new as f32,
            food: 20,        // todo
            saturation: 5.0, // todo
        };
        network.send(packet);
    }
}
