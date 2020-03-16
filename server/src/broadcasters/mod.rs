//! Systems which send packets based on events.
//!
//! There are four types of broadcasters:
//! * Those which broadcast packets to all online clients through `Game::broadcast_global()`.
//! * Those which broadcast packets to all clients who can see a given entity through `Game::broadcast_entity_update()`.
//! * Those which send additional packets, such as equipment, etc. after entity spawning
//! packets have been sent. This is done through `EntitySendEvent`.
//! * Those which just send a packet to a single player.

// mod animation;
// mod block;
// mod chat;
mod entity_creation;
mod entity_deletion;
// mod inventory;
// mod item_collect;
mod keepalive;
// mod metadata;
// pub mod movement;

pub use entity_creation::on_entity_spawn_send_to_clients;
pub use entity_creation::on_player_join_send_existing_entities;
pub use entity_deletion::on_entity_despawn_broadcast_despawn;
pub use keepalive::broadcast_keepalive;
