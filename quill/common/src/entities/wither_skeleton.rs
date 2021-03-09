use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all wither skeletons:
/// ```no_run
/// use quill::{Game, Position, entities::WitherSkeletonMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WitherSkeletonMarker)>() {
///         println!("Found a wither skeleton with position "wither skeleton"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WitherSkeletonMarker;

pod_component_impl!(WitherSkeletonMarker);

/// Entity wrapper for wither skeleton entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct WitherSkeleton {
    id: EntityId,
}

wrapper_from_query_impl!(WitherSkeleton, WitherSkeletonMarker);
entity_wrapper_impl!(WitherSkeleton, WitherSkeletonMarker);

impl crate::HasEntityId for WitherSkeleton {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
