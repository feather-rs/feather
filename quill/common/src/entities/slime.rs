use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all slimes:
/// ```no_run
/// use quill::{Game, Position, entities::SlimeMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SlimeMarker)>() {
///         println!("Found a slime with position "slime"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SlimeMarker;

pod_component_impl!(SlimeMarker);

/// Entity wrapper for slime entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Slime {
    id: EntityId,
}

wrapper_from_query_impl!(Slime, SlimeMarker);
entity_wrapper_impl!(Slime, SlimeMarker);

impl crate::HasEntityId for Slime {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
