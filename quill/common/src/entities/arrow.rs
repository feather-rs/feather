use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all arrows:
/// ```no_run
/// use quill::{Game, Position, entities::ArrowMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ArrowMarker)>() {
///         println!("Found a arrow with position "arrow"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ArrowMarker;

pod_component_impl!(ArrowMarker);

/// Entity wrapper for arrow entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Arrow {
    id: EntityId,
}

wrapper_from_query_impl!(Arrow, ArrowMarker);
entity_wrapper_impl!(Arrow, ArrowMarker);

impl crate::HasEntityId for Arrow {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
