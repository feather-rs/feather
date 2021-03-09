use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all fishing bobbers:
/// ```no_run
/// use quill::{Game, Position, entities::FishingBobberMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FishingBobberMarker)>() {
///         println!("Found a fishing bobber with position "fishing bobber"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FishingBobberMarker;

pod_component_impl!(FishingBobberMarker);

/// Entity wrapper for fishing bobber entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct FishingBobber {
    id: EntityId,
}

wrapper_from_query_impl!(FishingBobber, FishingBobberMarker);
entity_wrapper_impl!(FishingBobber, FishingBobberMarker);

impl crate::HasEntityId for FishingBobber {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
