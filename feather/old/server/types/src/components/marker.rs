//! "Marker" components: zero-sized structs which
//! can be used to filter for specific entities
//! in queries.

/// Marks an entity as a block entity (sometimes called a "tile entity")
pub struct BlockEntity;

/// Zero-sized marker component used to mark players.
pub struct Player;

/// A player is in a gamemode where they may take damage.
pub struct CanTakeDamage;

/// A player is in a gamemode where they may instantly break blocks.
pub struct CanInstaBreak;

/// A player is allowed to break blocks.
pub struct CanBreak;

/// Marks that a player has teleported and
/// we should force-update the client's
/// position.
///
/// Only necessary for players.
pub struct Teleported;

/// Whether an entity is able to respawn.
///
/// If this component is added, then the entity
/// will not be removed from the `World` when it dies.
pub struct CanRespawn;

/// Component for players who are currently on the respawn screen.
///
/// Players with this component _should not be affected by gameplay actions_.
/// They should not collect items, take damage, etc.
pub struct Dead;
