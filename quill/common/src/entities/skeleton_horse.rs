use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all skeleton horses:
/// ```no_run
/// use quill::{Game, Position, entities::SkeletonHorseMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SkeletonHorseMarker)>() {
///         println!("Found a skeleton horse with position "skeleton horse"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SkeletonHorseMarker;

pod_component_impl!(SkeletonHorseMarker);

/// Entity wrapper for skeleton horse entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct SkeletonHorse {
    id: EntityId,
}

wrapper_from_query_impl!(SkeletonHorse, SkeletonHorseMarker);
entity_wrapper_impl!(SkeletonHorse, SkeletonHorseMarker);

impl crate::HasEntityId for SkeletonHorse {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
