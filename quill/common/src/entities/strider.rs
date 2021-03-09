use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all striders:
/// ```no_run
/// use quill::{Game, Position, entities::StriderMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &StriderMarker)>() {
///         println!("Found a strider with position "strider"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct StriderMarker;

pod_component_impl!(StriderMarker);

/// Entity wrapper for strider entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Strider {
    id: EntityId,
}

wrapper_from_query_impl!(Strider, StriderMarker);
entity_wrapper_impl!(Strider, StriderMarker);

impl crate::HasEntityId for Strider {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
