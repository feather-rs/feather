use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all ghasts:
/// ```no_run
/// use quill::{Game, Position, entities::GhastMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &GhastMarker)>() {
///         println!("Found a ghast with position "ghast"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct GhastMarker;

pod_component_impl!(GhastMarker);

/// Entity wrapper for ghast entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Ghast {
    id: EntityId,
}

wrapper_from_query_impl!(Ghast, GhastMarker);
entity_wrapper_impl!(Ghast, GhastMarker);

impl crate::HasEntityId for Ghast {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
