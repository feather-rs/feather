//! "Marker" components: zero-sized structs which
//! can be used to filter for specific entities
//! in queries.

/// Zero-sized marker component used to mark players.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;
