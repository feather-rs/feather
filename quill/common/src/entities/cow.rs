use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all cows:
/// ```no_run
/// use quill::{Game, Position, entities::CowMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CowMarker)>() {
///         println!("Found a cow with position "cow"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CowMarker;

pod_component_impl!(CowMarker);

/// Entity wrapper for cow entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Cow {
    id: EntityId,
}

wrapper_from_query_impl!(Cow, CowMarker);
entity_wrapper_impl!(Cow, CowMarker);

impl crate::HasEntityId for Cow {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
