use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all leash knots:
/// ```no_run
/// use quill::{Game, Position, entities::LeashKnotMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LeashKnotMarker)>() {
///         println!("Found a leash knot with position "leash knot"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LeashKnotMarker;

pod_component_impl!(LeashKnotMarker);

/// Entity wrapper for leash knot entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct LeashKnot {
    id: EntityId,
}

wrapper_from_query_impl!(LeashKnot, LeashKnotMarker);
entity_wrapper_impl!(LeashKnot, LeashKnotMarker);

impl crate::HasEntityId for LeashKnot {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
