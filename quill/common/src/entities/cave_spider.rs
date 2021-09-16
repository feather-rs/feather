use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all cave spiders:
/// ```no_run
/// use quill::{Game, Position, entities::CaveSpiderMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CaveSpiderMarker)>() {
///         println!("Found a cave spider with position "cave spider"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CaveSpiderMarker;

pod_component_impl!(CaveSpiderMarker);

/// Entity wrapper for cave spider entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct CaveSpider {
    id: EntityId,
}

wrapper_from_query_impl!(CaveSpider, CaveSpiderMarker);
entity_wrapper_impl!(CaveSpider, CaveSpiderMarker);

impl crate::HasEntityId for CaveSpider {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
