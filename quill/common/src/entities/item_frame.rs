use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all item frames:
/// ```no_run
/// use quill::{Game, Position, entities::ItemFrameMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ItemFrameMarker)>() {
///         println!("Found a item frame with position "item frame"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ItemFrameMarker;

pod_component_impl!(ItemFrameMarker);

/// Entity wrapper for item frame entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ItemFrame {
    id: EntityId,
}

wrapper_from_query_impl!(ItemFrame, ItemFrameMarker);
entity_wrapper_impl!(ItemFrame, ItemFrameMarker);

impl crate::HasEntityId for ItemFrame {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
