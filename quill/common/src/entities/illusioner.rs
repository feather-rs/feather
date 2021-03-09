use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all illusioners:
/// ```no_run
/// use quill::{Game, Position, entities::IllusionerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &IllusionerMarker)>() {
///         println!("Found a illusioner with position "illusioner"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct IllusionerMarker;

pod_component_impl!(IllusionerMarker);

/// Entity wrapper for illusioner entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Illusioner {
    id: EntityId,
}

wrapper_from_query_impl!(Illusioner, IllusionerMarker);
entity_wrapper_impl!(Illusioner, IllusionerMarker);

impl crate::HasEntityId for Illusioner {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
