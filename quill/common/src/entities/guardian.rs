use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all guardians:
/// ```no_run
/// use quill::{Game, Position, entities::GuardianMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &GuardianMarker)>() {
///         println!("Found a guardian with position "guardian"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct GuardianMarker;

pod_component_impl!(GuardianMarker);

/// Entity wrapper for guardian entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Guardian {
    id: EntityId,
}

wrapper_from_query_impl!(Guardian, GuardianMarker);
entity_wrapper_impl!(Guardian, GuardianMarker);

impl crate::HasEntityId for Guardian {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
