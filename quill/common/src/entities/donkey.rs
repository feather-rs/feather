use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all donkeys:
/// ```no_run
/// use quill::{Game, Position, entities::DonkeyMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &DonkeyMarker)>() {
///         println!("Found a donkey with position "donkey"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct DonkeyMarker;

pod_component_impl!(DonkeyMarker);

/// Entity wrapper for donkey entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Donkey {
    id: EntityId,
}

wrapper_from_query_impl!(Donkey, DonkeyMarker);
entity_wrapper_impl!(Donkey, DonkeyMarker);

impl crate::HasEntityId for Donkey {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
