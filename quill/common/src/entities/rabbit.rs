use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all rabbits:
/// ```no_run
/// use quill::{Game, Position, entities::RabbitMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &RabbitMarker)>() {
///         println!("Found a rabbit with position "rabbit"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct RabbitMarker;

pod_component_impl!(RabbitMarker);

/// Entity wrapper for rabbit entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Rabbit {
    id: EntityId,
}

wrapper_from_query_impl!(Rabbit, RabbitMarker);
entity_wrapper_impl!(Rabbit, RabbitMarker);

impl crate::HasEntityId for Rabbit {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
