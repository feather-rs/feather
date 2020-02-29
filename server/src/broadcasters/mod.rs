//! Systems which broadcast packets based on events.
//!
//! There are three types of broadcasters:
//! * Those which broadcast packets to all online clients through `State::broadcast_global()`.
//! * Those which broadcast packets to all clients who can see a given entity through `State::broadcast_entity_update()`.
//! * Those which send additional packets, such as equipment, etc. after entity spawning
//! packets have been sent. This is done through `EntitySendEvent`.

mod animation;
mod block;
mod chat;
pub mod entity_creation;
pub mod entity_deletion;
mod inventory;
mod item_collect;
pub mod keepalive;
mod metadata;
pub mod movement;
