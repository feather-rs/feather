use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all squids:
/// ```no_run
/// use quill::{Game, Position, entities::SquidMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &SquidMarker)>() {
///         println!("Found a squid with position "squid"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct SquidMarker;

pod_component_impl!(SquidMarker);

/// Entity wrapper for squid entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Squid {
    id: EntityId,
}

wrapper_from_query_impl!(Squid, SquidMarker);
entity_wrapper_impl!(Squid, SquidMarker);

impl crate::HasEntityId for Squid {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
