use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all snow golems:
/// ```no_run
/// use quill::{Game, Position, entities::SnowGolemMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SnowGolemMarker)>() {
///         println!("Found a snow golem with position "snow golem"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SnowGolemMarker;

pod_component_impl!(SnowGolemMarker);

/// Entity wrapper for snow golem entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct SnowGolem {
    id: EntityId,
}

wrapper_from_query_impl!(SnowGolem, SnowGolemMarker);
entity_wrapper_impl!(SnowGolem, SnowGolemMarker);

impl crate::HasEntityId for SnowGolem {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
