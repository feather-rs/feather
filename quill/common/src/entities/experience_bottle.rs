use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all experience bottles:
/// ```no_run
/// use quill::{Game, Position, entities::ExperienceBottleMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ExperienceBottleMarker)>() {
///         println!("Found a experience bottle with position "experience bottle"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ExperienceBottleMarker;

pod_component_impl!(ExperienceBottleMarker);

/// Entity wrapper for experience bottle entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ExperienceBottle {
    id: EntityId,
}

wrapper_from_query_impl!(ExperienceBottle, ExperienceBottleMarker);
entity_wrapper_impl!(ExperienceBottle, ExperienceBottleMarker);

impl crate::HasEntityId for ExperienceBottle {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
