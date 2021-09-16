use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all boats:
/// ```no_run
/// use quill::{Game, Position, entities::BoatMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &BoatMarker)>() {
///         println!("Found a boat with position "boat"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct BoatMarker;

pod_component_impl!(BoatMarker);

/// Entity wrapper for boat entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Boat {
    id: EntityId,
}

wrapper_from_query_impl!(Boat, BoatMarker);
entity_wrapper_impl!(Boat, BoatMarker);

impl crate::HasEntityId for Boat {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
