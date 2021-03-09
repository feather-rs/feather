use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all phantoms:
/// ```no_run
/// use quill::{Game, Position, entities::PhantomMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &PhantomMarker)>() {
///         println!("Found a phantom with position "phantom"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct PhantomMarker;

pod_component_impl!(PhantomMarker);

/// Entity wrapper for phantom entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Phantom {
    id: EntityId,
}

wrapper_from_query_impl!(Phantom, PhantomMarker);
entity_wrapper_impl!(Phantom, PhantomMarker);

impl crate::HasEntityId for Phantom {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
