use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all tridents:
/// ```no_run
/// use quill::{Game, Position, entities::TridentMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TridentMarker)>() {
///         println!("Found a trident with position "trident"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TridentMarker;

pod_component_impl!(TridentMarker);

/// Entity wrapper for trident entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Trident {
    id: EntityId,
}

wrapper_from_query_impl!(Trident, TridentMarker);
entity_wrapper_impl!(Trident, TridentMarker);

impl crate::HasEntityId for Trident {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
