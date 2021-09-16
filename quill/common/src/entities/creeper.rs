use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all creepers:
/// ```no_run
/// use quill::{Game, Position, entities::CreeperMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CreeperMarker)>() {
///         println!("Found a creeper with position "creeper"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CreeperMarker;

pod_component_impl!(CreeperMarker);

/// Entity wrapper for creeper entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Creeper {
    id: EntityId,
}

wrapper_from_query_impl!(Creeper, CreeperMarker);
entity_wrapper_impl!(Creeper, CreeperMarker);

impl crate::HasEntityId for Creeper {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
