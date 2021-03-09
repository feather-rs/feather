use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all dolphins:
/// ```no_run
/// use quill::{Game, Position, entities::DolphinMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &DolphinMarker)>() {
///         println!("Found a dolphin with position "dolphin"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct DolphinMarker;

pod_component_impl!(DolphinMarker);

/// Entity wrapper for dolphin entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Dolphin {
    id: EntityId,
}

wrapper_from_query_impl!(Dolphin, DolphinMarker);
entity_wrapper_impl!(Dolphin, DolphinMarker);

impl crate::HasEntityId for Dolphin {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
