use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all items:
/// ```no_run
/// use quill::{Game, Position, entities::ItemMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ItemMarker)>() {
///         println!("Found a item with position "item"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ItemMarker;

pod_component_impl!(ItemMarker);

/// Entity wrapper for item entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Item {
    id: EntityId,
}

wrapper_from_query_impl!(Item, ItemMarker);
entity_wrapper_impl!(Item, ItemMarker);

impl crate::HasEntityId for Item {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
