use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all chickens:
/// ```no_run
/// use quill::{Game, Position, entities::ChickenMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ChickenMarker)>() {
///         println!("Found a chicken with position "chicken"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ChickenMarker;

pod_component_impl!(ChickenMarker);

/// Entity wrapper for chicken entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Chicken {
    id: EntityId,
}

wrapper_from_query_impl!(Chicken, ChickenMarker);
entity_wrapper_impl!(Chicken, ChickenMarker);

impl crate::HasEntityId for Chicken {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
