use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all zombie horses:
/// ```no_run
/// use quill::{Game, Position, entities::ZombieHorseMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZombieHorseMarker)>() {
///         println!("Found a zombie horse with position "zombie horse"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZombieHorseMarker;

pod_component_impl!(ZombieHorseMarker);

/// Entity wrapper for zombie horse entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ZombieHorse {
    id: EntityId,
}

wrapper_from_query_impl!(ZombieHorse, ZombieHorseMarker);
entity_wrapper_impl!(ZombieHorse, ZombieHorseMarker);

impl crate::HasEntityId for ZombieHorse {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
