use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all pandas:
/// ```no_run
/// use quill::{Game, Position, entities::PandaMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PandaMarker)>() {
///         println!("Found a panda with position "panda"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PandaMarker;

pod_component_impl!(PandaMarker);

/// Entity wrapper for panda entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Panda {
    id: EntityId,
}

wrapper_from_query_impl!(Panda, PandaMarker);
entity_wrapper_impl!(Panda, PandaMarker);

impl crate::HasEntityId for Panda {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
