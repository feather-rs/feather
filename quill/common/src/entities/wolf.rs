use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all wolfs:
/// ```no_run
/// use quill::{Game, Position, entities::WolfMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WolfMarker)>() {
///         println!("Found a wolf with position "wolf"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WolfMarker;

pod_component_impl!(WolfMarker);

/// Entity wrapper for wolf entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Wolf {
    id: EntityId,
}

wrapper_from_query_impl!(Wolf, WolfMarker);
entity_wrapper_impl!(Wolf, WolfMarker);

impl crate::HasEntityId for Wolf {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
