use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all cods:
/// ```no_run
/// use quill::{Game, Position, entities::CodMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CodMarker)>() {
///         println!("Found a cod with position "cod"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CodMarker;

pod_component_impl!(CodMarker);

/// Entity wrapper for cod entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Cod {
    id: EntityId,
}

wrapper_from_query_impl!(Cod, CodMarker);
entity_wrapper_impl!(Cod, CodMarker);

impl crate::HasEntityId for Cod {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
