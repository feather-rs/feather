use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all potions:
/// ```no_run
/// use quill::{Game, Position, entities::PotionMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PotionMarker)>() {
///         println!("Found a potion with position "potion"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PotionMarker;

pod_component_impl!(PotionMarker);

/// Entity wrapper for potion entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Potion {
    id: EntityId,
}

wrapper_from_query_impl!(Potion, PotionMarker);
entity_wrapper_impl!(Potion, PotionMarker);

impl crate::HasEntityId for Potion {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
