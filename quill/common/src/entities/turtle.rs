use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all turtles:
/// ```no_run
/// use quill::{Game, Position, entities::TurtleMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TurtleMarker)>() {
///         println!("Found a turtle with position "turtle"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TurtleMarker;

pod_component_impl!(TurtleMarker);

/// Entity wrapper for turtle entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Turtle {
    id: EntityId,
}

wrapper_from_query_impl!(Turtle, TurtleMarker);
entity_wrapper_impl!(Turtle, TurtleMarker);

impl crate::HasEntityId for Turtle {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
