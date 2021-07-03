use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all ocelots:
/// ```no_run
/// use quill::{Game, Position, entities::OcelotMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &OcelotMarker)>() {
///         println!("Found a ocelot with position "ocelot"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct OcelotMarker;

pod_component_impl!(OcelotMarker);

/// Entity wrapper for ocelot entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Ocelot {
    id: EntityId,
}

wrapper_from_query_impl!(Ocelot, OcelotMarker);
entity_wrapper_impl!(Ocelot, OcelotMarker);

impl crate::HasEntityId for Ocelot {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
