use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all piglins:
/// ```no_run
/// use quill::{Game, Position, entities::PiglinMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PiglinMarker)>() {
///         println!("Found a piglin with position "piglin"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PiglinMarker;

pod_component_impl!(PiglinMarker);

/// Entity wrapper for piglin entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Piglin {
    id: EntityId,
}

wrapper_from_query_impl!(Piglin, PiglinMarker);
entity_wrapper_impl!(Piglin, PiglinMarker);

impl crate::HasEntityId for Piglin {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
