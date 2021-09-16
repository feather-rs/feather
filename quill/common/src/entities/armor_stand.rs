use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all armor stands:
/// ```no_run
/// use quill::{Game, Position, entities::ArmorStandMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ArmorStandMarker)>() {
///         println!("Found a armor stand with position "armor stand"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ArmorStandMarker;

pod_component_impl!(ArmorStandMarker);

/// Entity wrapper for armor stand entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ArmorStand {
    id: EntityId,
}

wrapper_from_query_impl!(ArmorStand, ArmorStandMarker);
entity_wrapper_impl!(ArmorStand, ArmorStandMarker);

impl crate::HasEntityId for ArmorStand {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
