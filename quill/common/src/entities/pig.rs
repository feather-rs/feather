use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all pigs:
/// ```no_run
/// use quill::{Game, Position, entities::PigMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PigMarker)>() {
///         println!("Found a pig with position "pig"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PigMarker;

pod_component_impl!(PigMarker);

/// Entity wrapper for pig entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Pig {
    id: EntityId,
}

wrapper_from_query_impl!(Pig, PigMarker);
entity_wrapper_impl!(Pig, PigMarker);

impl crate::HasEntityId for Pig {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
