use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all husks:
/// ```no_run
/// use quill::{Game, Position, entities::HuskMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &HuskMarker)>() {
///         println!("Found a husk with position "husk"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct HuskMarker;

pod_component_impl!(HuskMarker);

/// Entity wrapper for husk entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Husk {
    id: EntityId,
}

wrapper_from_query_impl!(Husk, HuskMarker);
entity_wrapper_impl!(Husk, HuskMarker);

impl crate::HasEntityId for Husk {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
