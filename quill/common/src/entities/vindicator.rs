use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all vindicators:
/// ```no_run
/// use quill::{Game, Position, entities::VindicatorMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &VindicatorMarker)>() {
///         println!("Found a vindicator with position "vindicator"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct VindicatorMarker;

pod_component_impl!(VindicatorMarker);

/// Entity wrapper for vindicator entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Vindicator {
    id: EntityId,
}

wrapper_from_query_impl!(Vindicator, VindicatorMarker);
entity_wrapper_impl!(Vindicator, VindicatorMarker);

impl crate::HasEntityId for Vindicator {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
