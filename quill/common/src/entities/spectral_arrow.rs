use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all spectral arrows:
/// ```no_run
/// use quill::{Game, Position, entities::SpectralArrowMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SpectralArrowMarker)>() {
///         println!("Found a spectral arrow with position "spectral arrow"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SpectralArrowMarker;

pod_component_impl!(SpectralArrowMarker);

/// Entity wrapper for spectral arrow entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct SpectralArrow {
    id: EntityId,
}

wrapper_from_query_impl!(SpectralArrow, SpectralArrowMarker);
entity_wrapper_impl!(SpectralArrow, SpectralArrowMarker);

impl crate::HasEntityId for SpectralArrow {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
