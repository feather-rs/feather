use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all end crystals:
/// ```no_run
/// use quill::{Game, Position, entities::EndCrystalMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EndCrystalMarker)>() {
///         println!("Found a end crystal with position "end crystal"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EndCrystalMarker;

pod_component_impl!(EndCrystalMarker);

/// Entity wrapper for end crystal entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct EndCrystal {
    id: EntityId,
}

wrapper_from_query_impl!(EndCrystal, EndCrystalMarker);
entity_wrapper_impl!(EndCrystal, EndCrystalMarker);

impl crate::HasEntityId for EndCrystal {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
