use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all bats:
/// ```no_run
/// use quill::{Game, Position, entities::BatMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &BatMarker)>() {
///         println!("Found a bat with position "bat"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct BatMarker;

pod_component_impl!(BatMarker);

/// Entity wrapper for bat entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Bat {
    id: EntityId,
}

wrapper_from_query_impl!(Bat, BatMarker);
entity_wrapper_impl!(Bat, BatMarker);

impl crate::HasEntityId for Bat {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
