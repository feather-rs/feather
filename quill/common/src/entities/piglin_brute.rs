use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all piglin brutes:
/// ```no_run
/// use quill::{Game, Position, entities::PiglinBruteMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PiglinBruteMarker)>() {
///         println!("Found a piglin brute with position "piglin brute"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PiglinBruteMarker;

pod_component_impl!(PiglinBruteMarker);

/// Entity wrapper for piglin brute entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct PiglinBrute {
    id: EntityId,
}

wrapper_from_query_impl!(PiglinBrute, PiglinBruteMarker);
entity_wrapper_impl!(PiglinBrute, PiglinBruteMarker);

impl crate::HasEntityId for PiglinBrute {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
