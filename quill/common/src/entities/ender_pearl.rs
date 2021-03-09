use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all ender pearls:
/// ```no_run
/// use quill::{Game, Position, entities::EnderPearlMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EnderPearlMarker)>() {
///         println!("Found a ender pearl with position "ender pearl"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EnderPearlMarker;

pod_component_impl!(EnderPearlMarker);

/// Entity wrapper for ender pearl entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct EnderPearl {
    id: EntityId,
}

wrapper_from_query_impl!(EnderPearl, EnderPearlMarker);
entity_wrapper_impl!(EnderPearl, EnderPearlMarker);

impl crate::HasEntityId for EnderPearl {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
