use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all mooshrooms:
/// ```no_run
/// use quill::{Game, Position, entities::MooshroomMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &MooshroomMarker)>() {
///         println!("Found a mooshroom with position "mooshroom"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct MooshroomMarker;

pod_component_impl!(MooshroomMarker);

/// Entity wrapper for mooshroom entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Mooshroom {
    id: EntityId,
}

wrapper_from_query_impl!(Mooshroom, MooshroomMarker);
entity_wrapper_impl!(Mooshroom, MooshroomMarker);

impl crate::HasEntityId for Mooshroom {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
