use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all firework rockets:
/// ```no_run
/// use quill::{Game, Position, entities::FireworkRocketMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FireworkRocketMarker)>() {
///         println!("Found a firework rocket with position "firework rocket"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FireworkRocketMarker;

pod_component_impl!(FireworkRocketMarker);

/// Entity wrapper for firework rocket entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct FireworkRocket {
    id: EntityId,
}

wrapper_from_query_impl!(FireworkRocket, FireworkRocketMarker);
entity_wrapper_impl!(FireworkRocket, FireworkRocketMarker);

impl crate::HasEntityId for FireworkRocket {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
