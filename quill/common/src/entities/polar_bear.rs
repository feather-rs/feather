use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all polar bears:
/// ```no_run
/// use quill::{Game, Position, entities::PolarBearMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PolarBearMarker)>() {
///         println!("Found a polar bear with position "polar bear"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PolarBearMarker;

pod_component_impl!(PolarBearMarker);

/// Entity wrapper for polar bear entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct PolarBear {
    id: EntityId,
}

wrapper_from_query_impl!(PolarBear, PolarBearMarker);
entity_wrapper_impl!(PolarBear, PolarBearMarker);

impl crate::HasEntityId for PolarBear {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
