use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all zombies:
/// ```no_run
/// use quill::{Game, Position, entities::ZombieMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombieMarker)>() {
///         println!("Found a zombie with position "zombie"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombieMarker;

pod_component_impl!(ZombieMarker);

/// Entity wrapper for zombie entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Zombie {
    id: EntityId,
}

wrapper_from_query_impl!(Zombie, ZombieMarker);
entity_wrapper_impl!(Zombie, ZombieMarker);

impl crate::HasEntityId for Zombie {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
