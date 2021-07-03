use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all iron golems:
/// ```no_run
/// use quill::{Game, Position, entities::IronGolemMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &IronGolemMarker)>() {
///         println!("Found a iron golem with position "iron golem"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct IronGolemMarker;

pod_component_impl!(IronGolemMarker);

/// Entity wrapper for iron golem entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct IronGolem {
    id: EntityId,
}

wrapper_from_query_impl!(IronGolem, IronGolemMarker);
entity_wrapper_impl!(IronGolem, IronGolemMarker);

impl crate::HasEntityId for IronGolem {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
