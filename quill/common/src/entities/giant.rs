use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all giants:
/// ```no_run
/// use quill::{Game, Position, entities::GiantMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &GiantMarker)>() {
///         println!("Found a giant with position "giant"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct GiantMarker;

pod_component_impl!(GiantMarker);

/// Entity wrapper for giant entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Giant {
    id: EntityId,
}

wrapper_from_query_impl!(Giant, GiantMarker);
entity_wrapper_impl!(Giant, GiantMarker);

impl crate::HasEntityId for Giant {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
