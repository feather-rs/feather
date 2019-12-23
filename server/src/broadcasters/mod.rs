//! Systems which broadcast packets.
//!
//! There are two types of broadcasters:
//! * Those which broadcast packets to all online clients through `State::broadcast_global()`.
//! * Those which broadcast packets to all clients who can see a given entity through `State::broadcast_entity_update()`.

pub mod movement;
