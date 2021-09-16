use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all zombie villagers:
/// ```no_run
/// use quill::{Game, Position, entities::ZombieVillagerMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombieVillagerMarker)>() {
///         println!("Found a zombie villager with position "zombie villager"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombieVillagerMarker;

pod_component_impl!(ZombieVillagerMarker);

/// Entity wrapper for zombie villager entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ZombieVillager {
    id: EntityId,
}

wrapper_from_query_impl!(ZombieVillager, ZombieVillagerMarker);
entity_wrapper_impl!(ZombieVillager, ZombieVillagerMarker);

impl crate::HasEntityId for ZombieVillager {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
