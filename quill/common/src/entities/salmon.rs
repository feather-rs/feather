use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all salmons:
/// ```no_run
/// use quill::{Game, Position, entities::SalmonMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SalmonMarker)>() {
///         println!("Found a salmon with position "salmon"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SalmonMarker;

pod_component_impl!(SalmonMarker);

/// Entity wrapper for salmon entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Salmon {
    id: EntityId,
}

wrapper_from_query_impl!(Salmon, SalmonMarker);
entity_wrapper_impl!(Salmon, SalmonMarker);

impl crate::HasEntityId for Salmon {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
