use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all evoker fangss:
/// ```no_run
/// use quill::{Game, Position, entities::EvokerFangsMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EvokerFangsMarker)>() {
///         println!("Found a evoker fangs with position "evoker fangs"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EvokerFangsMarker;

pod_component_impl!(EvokerFangsMarker);

/// Entity wrapper for evoker fangs entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct EvokerFangs {
    id: EntityId,
}

wrapper_from_query_impl!(EvokerFangs, EvokerFangsMarker);
entity_wrapper_impl!(EvokerFangs, EvokerFangsMarker);

impl crate::HasEntityId for EvokerFangs {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
