use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all drowneds:
/// ```no_run
/// use quill::{Game, Position, entities::DrownedMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &DrownedMarker)>() {
///         println!("Found a drowned with position "drowned"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct DrownedMarker;

pod_component_impl!(DrownedMarker);

/// Entity wrapper for drowned entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Drowned {
    id: EntityId,
}

wrapper_from_query_impl!(Drowned, DrownedMarker);
entity_wrapper_impl!(Drowned, DrownedMarker);

impl crate::HasEntityId for Drowned {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
