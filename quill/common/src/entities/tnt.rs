use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all tnts:
/// ```no_run
/// use quill::{Game, Position, entities::TntMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TntMarker)>() {
///         println!("Found a tnt with position "tnt"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TntMarker;

pod_component_impl!(TntMarker);

/// Entity wrapper for tnt entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Tnt {
    id: EntityId,
}

wrapper_from_query_impl!(Tnt, TntMarker);
entity_wrapper_impl!(Tnt, TntMarker);

impl crate::HasEntityId for Tnt {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
