use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all small fireballs:
/// ```no_run
/// use quill::{Game, Position, entities::SmallFireballMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SmallFireballMarker)>() {
///         println!("Found a small fireball with position "small fireball"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SmallFireballMarker;

pod_component_impl!(SmallFireballMarker);

/// Entity wrapper for small fireball entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct SmallFireball {
    id: EntityId,
}

wrapper_from_query_impl!(SmallFireball, SmallFireballMarker);
entity_wrapper_impl!(SmallFireball, SmallFireballMarker);

impl crate::HasEntityId for SmallFireball {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
