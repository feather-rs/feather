use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all experience orbs:
/// ```no_run
/// use quill::{Game, Position, entities::ExperienceOrbMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ExperienceOrbMarker)>() {
///         println!("Found a experience orb with position "experience orb"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ExperienceOrbMarker;

pod_component_impl!(ExperienceOrbMarker);

/// Entity wrapper for experience orb entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ExperienceOrb {
    id: EntityId,
}

wrapper_from_query_impl!(ExperienceOrb, ExperienceOrbMarker);
entity_wrapper_impl!(ExperienceOrb, ExperienceOrbMarker);

impl crate::HasEntityId for ExperienceOrb {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
