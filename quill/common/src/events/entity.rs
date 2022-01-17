use serde::{Deserialize, Serialize};

/// Triggered when a player joins the `Game`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerJoinEvent;

/// Triggered when an entity is removed from the world.
///
/// The entity will remain alive for one tick after it is
/// destroyed to allow systems to observe this event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRemoveEvent;

/// Triggered when an entity is added into the world.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityCreateEvent;
