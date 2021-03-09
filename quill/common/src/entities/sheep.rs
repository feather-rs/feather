use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all sheeps:
/// ```no_run
/// use quill::{Game, Position, entities::SheepMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SheepMarker)>() {
///         println!("Found a sheep with position "sheep"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SheepMarker;

pod_component_impl!(SheepMarker);

/// Entity wrapper for sheep entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Sheep {
    id: EntityId,
}

wrapper_from_query_impl!(Sheep, SheepMarker);
entity_wrapper_impl!(Sheep, SheepMarker);

impl crate::HasEntityId for Sheep {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
