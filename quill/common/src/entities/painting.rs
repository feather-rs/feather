use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all paintings:
/// ```no_run
/// use quill::{Game, Position, entities::PaintingMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PaintingMarker)>() {
///         println!("Found a painting with position "painting"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PaintingMarker;

pod_component_impl!(PaintingMarker);

/// Entity wrapper for painting entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Painting {
    id: EntityId,
}

wrapper_from_query_impl!(Painting, PaintingMarker);
entity_wrapper_impl!(Painting, PaintingMarker);

impl crate::HasEntityId for Painting {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
