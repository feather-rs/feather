use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all endermites:
/// ```no_run
/// use quill::{Game, Position, entities::EndermiteMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EndermiteMarker)>() {
///         println!("Found a endermite with position "endermite"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EndermiteMarker;

pod_component_impl!(EndermiteMarker);

/// Entity wrapper for endermite entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Endermite {
    id: EntityId,
}

wrapper_from_query_impl!(Endermite, EndermiteMarker);
entity_wrapper_impl!(Endermite, EndermiteMarker);

impl crate::HasEntityId for Endermite {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
