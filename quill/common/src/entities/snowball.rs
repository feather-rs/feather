use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all snowballs:
/// ```no_run
/// use quill::{Game, Position, entities::SnowballMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SnowballMarker)>() {
///         println!("Found a snowball with position "snowball"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SnowballMarker;

pod_component_impl!(SnowballMarker);

/// Entity wrapper for snowball entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Snowball {
    id: EntityId,
}

wrapper_from_query_impl!(Snowball, SnowballMarker);
entity_wrapper_impl!(Snowball, SnowballMarker);

impl crate::HasEntityId for Snowball {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
