use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all pillagers:
/// ```no_run
/// use quill::{Game, Position, entities::PillagerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PillagerMarker)>() {
///         println!("Found a pillager with position "pillager"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PillagerMarker;

pod_component_impl!(PillagerMarker);

/// Entity wrapper for pillager entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Pillager {
    id: EntityId,
}

wrapper_from_query_impl!(Pillager, PillagerMarker);
entity_wrapper_impl!(Pillager, PillagerMarker);

impl crate::HasEntityId for Pillager {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
