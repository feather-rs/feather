use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all zombified piglins:
/// ```no_run
/// use quill::{Game, Position, entities::ZombifiedPiglinMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombifiedPiglinMarker)>() {
///         println!("Found a zombified piglin with position "zombified piglin"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombifiedPiglinMarker;

pod_component_impl!(ZombifiedPiglinMarker);

/// Entity wrapper for zombified piglin entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ZombifiedPiglin {
    id: EntityId,
}

wrapper_from_query_impl!(ZombifiedPiglin, ZombifiedPiglinMarker);
entity_wrapper_impl!(ZombifiedPiglin, ZombifiedPiglinMarker);

impl crate::HasEntityId for ZombifiedPiglin {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
