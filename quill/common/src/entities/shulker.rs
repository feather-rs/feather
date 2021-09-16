use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all shulkers:
/// ```no_run
/// use quill::{Game, Position, entities::ShulkerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ShulkerMarker)>() {
///         println!("Found a shulker with position "shulker"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ShulkerMarker;

pod_component_impl!(ShulkerMarker);

/// Entity wrapper for shulker entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Shulker {
    id: EntityId,
}

wrapper_from_query_impl!(Shulker, ShulkerMarker);
entity_wrapper_impl!(Shulker, ShulkerMarker);

impl crate::HasEntityId for Shulker {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
