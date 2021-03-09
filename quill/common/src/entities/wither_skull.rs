use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all wither skulls:
/// ```no_run
/// use quill::{Game, Position, entities::WitherSkullMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WitherSkullMarker)>() {
///         println!("Found a wither skull with position "wither skull"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WitherSkullMarker;

pod_component_impl!(WitherSkullMarker);

/// Entity wrapper for wither skull entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct WitherSkull {
    id: EntityId,
}

wrapper_from_query_impl!(WitherSkull, WitherSkullMarker);
entity_wrapper_impl!(WitherSkull, WitherSkullMarker);

impl crate::HasEntityId for WitherSkull {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
