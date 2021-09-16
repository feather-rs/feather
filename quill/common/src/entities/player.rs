use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all players:
/// ```no_run
/// use quill::{Game, Position, entities::PlayerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PlayerMarker)>() {
///         println!("Found a player with position "player"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PlayerMarker;

pod_component_impl!(PlayerMarker);

/// Entity wrapper for player entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Player {
    id: EntityId,
}

wrapper_from_query_impl!(Player, PlayerMarker);
entity_wrapper_impl!(Player, PlayerMarker);

impl crate::HasEntityId for Player {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
