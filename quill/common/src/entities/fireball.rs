use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all fireballs:
/// ```no_run
/// use quill::{Game, Position, entities::FireballMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FireballMarker)>() {
///         println!("Found a fireball with position "fireball"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FireballMarker;

pod_component_impl!(FireballMarker);

/// Entity wrapper for fireball entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Fireball {
    id: EntityId,
}

wrapper_from_query_impl!(Fireball, FireballMarker);
entity_wrapper_impl!(Fireball, FireballMarker);

impl crate::HasEntityId for Fireball {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
