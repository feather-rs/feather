use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all skeletons:
/// ```no_run
/// use quill::{Game, Position, entities::SkeletonMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SkeletonMarker)>() {
///         println!("Found a skeleton with position "skeleton"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SkeletonMarker;

pod_component_impl!(SkeletonMarker);

/// Entity wrapper for skeleton entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Skeleton {
    id: EntityId,
}

wrapper_from_query_impl!(Skeleton, SkeletonMarker);
entity_wrapper_impl!(Skeleton, SkeletonMarker);

impl crate::HasEntityId for Skeleton {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
