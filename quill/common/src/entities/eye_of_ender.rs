use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all eye of enders:
/// ```no_run
/// use quill::{Game, Position, entities::EyeOfEnderMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EyeOfEnderMarker)>() {
///         println!("Found a eye of ender with position "eye of ender"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EyeOfEnderMarker;

pod_component_impl!(EyeOfEnderMarker);

/// Entity wrapper for eye of ender entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct EyeOfEnder {
    id: EntityId,
}

wrapper_from_query_impl!(EyeOfEnder, EyeOfEnderMarker);
entity_wrapper_impl!(EyeOfEnder, EyeOfEnderMarker);

impl crate::HasEntityId for EyeOfEnder {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
