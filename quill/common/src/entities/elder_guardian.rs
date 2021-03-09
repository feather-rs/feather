use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all elder guardians:
/// ```no_run
/// use quill::{Game, Position, entities::ElderGuardianMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ElderGuardianMarker)>() {
///         println!("Found a elder guardian with position "elder guardian"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ElderGuardianMarker;

pod_component_impl!(ElderGuardianMarker);

/// Entity wrapper for elder guardian entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ElderGuardian {
    id: EntityId,
}

wrapper_from_query_impl!(ElderGuardian, ElderGuardianMarker);
entity_wrapper_impl!(ElderGuardian, ElderGuardianMarker);

impl crate::HasEntityId for ElderGuardian {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
