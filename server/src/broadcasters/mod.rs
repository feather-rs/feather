//! Systems which send packets based on events.
//!
//! There are four types of broadcasters:
//! * Those which broadcast packets to all online clients through `Game::broadcast_global()`.
//! * Those which broadcast packets to all clients who can see a given entity through `Game::broadcast_entity_update()`.
//! * Those which send additional packets, such as equipment, etc. after entity spawning
//! packets have been sent. This is done through `EntitySendEvent`.
//! * Those which just send a packet to a single player.

mod animation;
mod block;
// mod chat;
mod entity_creation;
mod entity_deletion;
mod inventory;
mod item_collect;
mod keepalive;
mod metadata;
mod movement;

pub use self::inventory::{
    on_entity_send_send_equipment, on_inventory_update_broadcast_equipment_update,
    on_inventory_update_send_set_slot,
};
pub use animation::on_player_animation_broadcast_animation;
pub use block::on_block_update_broadcast;
pub use entity_creation::on_entity_spawn_send_to_clients;
pub use entity_creation::on_player_join_send_existing_entities;
pub use entity_deletion::on_entity_despawn_broadcast_despawn;
pub use item_collect::on_item_collect_broadcast;
pub use keepalive::broadcast_keepalive;
pub use metadata::on_entity_send_send_metadata;
pub use movement::{
    broadcast_movement, broadcast_velocity, on_entity_client_remove_update_last_known_positions,
    on_entity_send_update_last_known_positions, LastKnownPositions,
};
