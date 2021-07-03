use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all ravagers:
/// ```no_run
/// use quill::{Game, Position, entities::RavagerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &RavagerMarker)>() {
///         println!("Found a ravager with position "ravager"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct RavagerMarker;

pod_component_impl!(RavagerMarker);

/// Entity wrapper for ravager entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Ravager {
    id: EntityId,
}

wrapper_from_query_impl!(Ravager, RavagerMarker);
entity_wrapper_impl!(Ravager, RavagerMarker);

impl crate::HasEntityId for Ravager {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
