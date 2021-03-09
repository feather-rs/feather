use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all withers:
/// ```no_run
/// use quill::{Game, Position, entities::WitherMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WitherMarker)>() {
///         println!("Found a wither with position "wither"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WitherMarker;

pod_component_impl!(WitherMarker);

/// Entity wrapper for wither entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Wither {
    id: EntityId,
}

wrapper_from_query_impl!(Wither, WitherMarker);
entity_wrapper_impl!(Wither, WitherMarker);

impl crate::HasEntityId for Wither {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
