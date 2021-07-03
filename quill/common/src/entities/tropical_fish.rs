use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all tropical fishs:
/// ```no_run
/// use quill::{Game, Position, entities::TropicalFishMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TropicalFishMarker)>() {
///         println!("Found a tropical fish with position "tropical fish"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TropicalFishMarker;

pod_component_impl!(TropicalFishMarker);

/// Entity wrapper for tropical fish entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct TropicalFish {
    id: EntityId,
}

wrapper_from_query_impl!(TropicalFish, TropicalFishMarker);
entity_wrapper_impl!(TropicalFish, TropicalFishMarker);

impl crate::HasEntityId for TropicalFish {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
