use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all foxs:
/// ```no_run
/// use quill::{Game, Position, entities::FoxMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FoxMarker)>() {
///         println!("Found a fox with position "fox"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FoxMarker;

pod_component_impl!(FoxMarker);

/// Entity wrapper for fox entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Fox {
    id: EntityId,
}

wrapper_from_query_impl!(Fox, FoxMarker);
entity_wrapper_impl!(Fox, FoxMarker);

impl crate::HasEntityId for Fox {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
