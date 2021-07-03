use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all mules:
/// ```no_run
/// use quill::{Game, Position, entities::MuleMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &MuleMarker)>() {
///         println!("Found a mule with position "mule"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct MuleMarker;

pod_component_impl!(MuleMarker);

/// Entity wrapper for mule entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Mule {
    id: EntityId,
}

wrapper_from_query_impl!(Mule, MuleMarker);
entity_wrapper_impl!(Mule, MuleMarker);

impl crate::HasEntityId for Mule {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
