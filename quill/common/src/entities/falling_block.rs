use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all falling blocks:
/// ```no_run
/// use quill::{Game, Position, entities::FallingBlockMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FallingBlockMarker)>() {
///         println!("Found a falling block with position "falling block"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FallingBlockMarker;

pod_component_impl!(FallingBlockMarker);

/// Entity wrapper for falling block entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct FallingBlock {
    id: EntityId,
}

wrapper_from_query_impl!(FallingBlock, FallingBlockMarker);
entity_wrapper_impl!(FallingBlock, FallingBlockMarker);

impl crate::HasEntityId for FallingBlock {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
