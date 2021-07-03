use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all bees:
/// ```no_run
/// use quill::{Game, Position, entities::BeeMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &BeeMarker)>() {
///         println!("Found a bee with position "bee"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct BeeMarker;

pod_component_impl!(BeeMarker);

/// Entity wrapper for bee entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Bee {
    id: EntityId,
}

wrapper_from_query_impl!(Bee, BeeMarker);
entity_wrapper_impl!(Bee, BeeMarker);

impl crate::HasEntityId for Bee {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
