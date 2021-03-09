use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all silverfishs:
/// ```no_run
/// use quill::{Game, Position, entities::SilverfishMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SilverfishMarker)>() {
///         println!("Found a silverfish with position "silverfish"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SilverfishMarker;

pod_component_impl!(SilverfishMarker);

/// Entity wrapper for silverfish entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Silverfish {
    id: EntityId,
}

wrapper_from_query_impl!(Silverfish, SilverfishMarker);
entity_wrapper_impl!(Silverfish, SilverfishMarker);

impl crate::HasEntityId for Silverfish {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
