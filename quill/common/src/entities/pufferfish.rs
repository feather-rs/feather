use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all pufferfishs:
/// ```no_run
/// use quill::{Game, Position, entities::PufferfishMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PufferfishMarker)>() {
///         println!("Found a pufferfish with position "pufferfish"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PufferfishMarker;

pod_component_impl!(PufferfishMarker);

/// Entity wrapper for pufferfish entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Pufferfish {
    id: EntityId,
}

wrapper_from_query_impl!(Pufferfish, PufferfishMarker);
entity_wrapper_impl!(Pufferfish, PufferfishMarker);

impl crate::HasEntityId for Pufferfish {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
