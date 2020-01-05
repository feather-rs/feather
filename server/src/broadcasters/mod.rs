//! Systems which broadcast packets based on events.
//!
//! There are two types of broadcasters:
//! * Those which broadcast packets to all online clients through `State::broadcast_global()`.
//! * Those which broadcast packets to all clients who can see a given entity through `State::broadcast_entity_update()`.

pub mod entity_creation;
pub mod entity_deletion;
pub mod keepalive;
pub mod movement;
