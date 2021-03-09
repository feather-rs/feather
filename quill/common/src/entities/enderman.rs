use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all endermans:
/// ```no_run
/// use quill::{Game, Position, entities::EndermanMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EndermanMarker)>() {
///         println!("Found a enderman with position "enderman"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EndermanMarker;

pod_component_impl!(EndermanMarker);

/// Entity wrapper for enderman entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Enderman {
    id: EntityId,
}

wrapper_from_query_impl!(Enderman, EndermanMarker);
entity_wrapper_impl!(Enderman, EndermanMarker);

impl crate::HasEntityId for Enderman {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
