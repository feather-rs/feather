use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all blazes:
/// ```no_run
/// use quill::{Game, Position, entities::BlazeMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &BlazeMarker)>() {
///         println!("Found a blaze with position "blaze"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct BlazeMarker;

pod_component_impl!(BlazeMarker);

/// Entity wrapper for blaze entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Blaze {
    id: EntityId,
}

wrapper_from_query_impl!(Blaze, BlazeMarker);
entity_wrapper_impl!(Blaze, BlazeMarker);

impl crate::HasEntityId for Blaze {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
