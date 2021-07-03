use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all villagers:
/// ```no_run
/// use quill::{Game, Position, entities::VillagerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &VillagerMarker)>() {
///         println!("Found a villager with position "villager"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct VillagerMarker;

pod_component_impl!(VillagerMarker);

/// Entity wrapper for villager entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Villager {
    id: EntityId,
}

wrapper_from_query_impl!(Villager, VillagerMarker);
entity_wrapper_impl!(Villager, VillagerMarker);

impl crate::HasEntityId for Villager {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
