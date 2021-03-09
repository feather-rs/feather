use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all evokers:
/// ```no_run
/// use quill::{Game, Position, entities::EvokerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EvokerMarker)>() {
///         println!("Found a evoker with position "evoker"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EvokerMarker;

pod_component_impl!(EvokerMarker);

/// Entity wrapper for evoker entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Evoker {
    id: EntityId,
}

wrapper_from_query_impl!(Evoker, EvokerMarker);
entity_wrapper_impl!(Evoker, EvokerMarker);

impl crate::HasEntityId for Evoker {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
