use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all horses:
/// ```no_run
/// use quill::{Game, Position, entities::HorseMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &HorseMarker)>() {
///         println!("Found a horse with position "horse"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct HorseMarker;

pod_component_impl!(HorseMarker);

/// Entity wrapper for horse entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Horse {
    id: EntityId,
}

wrapper_from_query_impl!(Horse, HorseMarker);
entity_wrapper_impl!(Horse, HorseMarker);

impl crate::HasEntityId for Horse {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
