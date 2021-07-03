use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all dragon fireballs:
/// ```no_run
/// use quill::{Game, Position, entities::DragonFireballMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &DragonFireballMarker)>() {
///         println!("Found a dragon fireball with position "dragon fireball"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct DragonFireballMarker;

pod_component_impl!(DragonFireballMarker);

/// Entity wrapper for dragon fireball entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct DragonFireball {
    id: EntityId,
}

wrapper_from_query_impl!(DragonFireball, DragonFireballMarker);
entity_wrapper_impl!(DragonFireball, DragonFireballMarker);

impl crate::HasEntityId for DragonFireball {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
