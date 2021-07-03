use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all area effect clouds:
/// ```no_run
/// use quill::{Game, Position, entities::AreaEffectCloudMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &AreaEffectCloudMarker)>() {
///         println!("Found a area effect cloud with position "area effect cloud"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct AreaEffectCloudMarker;

pod_component_impl!(AreaEffectCloudMarker);

/// Entity wrapper for area effect cloud entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct AreaEffectCloud {
    id: EntityId,
}

wrapper_from_query_impl!(AreaEffectCloud, AreaEffectCloudMarker);
entity_wrapper_impl!(AreaEffectCloud, AreaEffectCloudMarker);

impl crate::HasEntityId for AreaEffectCloud {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
