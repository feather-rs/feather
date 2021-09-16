use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all witchs:
/// ```no_run
/// use quill::{Game, Position, entities::WitchMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WitchMarker)>() {
///         println!("Found a witch with position "witch"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WitchMarker;

pod_component_impl!(WitchMarker);

/// Entity wrapper for witch entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Witch {
    id: EntityId,
}

wrapper_from_query_impl!(Witch, WitchMarker);
entity_wrapper_impl!(Witch, WitchMarker);

impl crate::HasEntityId for Witch {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
