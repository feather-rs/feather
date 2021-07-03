use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all hoglins:
/// ```no_run
/// use quill::{Game, Position, entities::HoglinMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &HoglinMarker)>() {
///         println!("Found a hoglin with position "hoglin"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct HoglinMarker;

pod_component_impl!(HoglinMarker);

/// Entity wrapper for hoglin entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Hoglin {
    id: EntityId,
}

wrapper_from_query_impl!(Hoglin, HoglinMarker);
entity_wrapper_impl!(Hoglin, HoglinMarker);

impl crate::HasEntityId for Hoglin {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
