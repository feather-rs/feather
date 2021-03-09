use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all spiders:
/// ```no_run
/// use quill::{Game, Position, entities::SpiderMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SpiderMarker)>() {
///         println!("Found a spider with position "spider"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SpiderMarker;

pod_component_impl!(SpiderMarker);

/// Entity wrapper for spider entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Spider {
    id: EntityId,
}

wrapper_from_query_impl!(Spider, SpiderMarker);
entity_wrapper_impl!(Spider, SpiderMarker);

impl crate::HasEntityId for Spider {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
