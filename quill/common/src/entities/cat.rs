use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all cats:
/// ```no_run
/// use quill::{Game, Position, entities::CatMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CatMarker)>() {
///         println!("Found a cat with position "cat"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CatMarker;

pod_component_impl!(CatMarker);

/// Entity wrapper for cat entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Cat {
    id: EntityId,
}

wrapper_from_query_impl!(Cat, CatMarker);
entity_wrapper_impl!(Cat, CatMarker);

impl crate::HasEntityId for Cat {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
