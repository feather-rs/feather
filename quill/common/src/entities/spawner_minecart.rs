use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all spawner minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::SpawnerMinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SpawnerMinecartMarker)>() {
///         println!("Found a spawner minecart with position "spawner minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SpawnerMinecartMarker;

pod_component_impl!(SpawnerMinecartMarker);

/// Entity wrapper for spawner minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct SpawnerMinecart {
    id: EntityId,
}

wrapper_from_query_impl!(SpawnerMinecart, SpawnerMinecartMarker);
entity_wrapper_impl!(SpawnerMinecart, SpawnerMinecartMarker);

impl crate::HasEntityId for SpawnerMinecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
