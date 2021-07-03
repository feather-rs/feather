use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all ender dragons:
/// ```no_run
/// use quill::{Game, Position, entities::EnderDragonMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EnderDragonMarker)>() {
///         println!("Found a ender dragon with position "ender dragon"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EnderDragonMarker;

pod_component_impl!(EnderDragonMarker);

/// Entity wrapper for ender dragon entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct EnderDragon {
    id: EntityId,
}

wrapper_from_query_impl!(EnderDragon, EnderDragonMarker);
entity_wrapper_impl!(EnderDragon, EnderDragonMarker);

impl crate::HasEntityId for EnderDragon {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
