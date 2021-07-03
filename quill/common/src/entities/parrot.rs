use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all parrots:
/// ```no_run
/// use quill::{Game, Position, entities::ParrotMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ParrotMarker)>() {
///         println!("Found a parrot with position "parrot"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ParrotMarker;

pod_component_impl!(ParrotMarker);

/// Entity wrapper for parrot entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Parrot {
    id: EntityId,
}

wrapper_from_query_impl!(Parrot, ParrotMarker);
entity_wrapper_impl!(Parrot, ParrotMarker);

impl crate::HasEntityId for Parrot {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
