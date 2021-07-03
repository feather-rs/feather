use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all eggs:
/// ```no_run
/// use quill::{Game, Position, entities::EggMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EggMarker)>() {
///         println!("Found a egg with position "egg"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EggMarker;

pod_component_impl!(EggMarker);

/// Entity wrapper for egg entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Egg {
    id: EntityId,
}

wrapper_from_query_impl!(Egg, EggMarker);
entity_wrapper_impl!(Egg, EggMarker);

impl crate::HasEntityId for Egg {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
