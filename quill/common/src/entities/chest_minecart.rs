use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all chest minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::ChestMinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ChestMinecartMarker)>() {
///         println!("Found a chest minecart with position "chest minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ChestMinecartMarker;

pod_component_impl!(ChestMinecartMarker);

/// Entity wrapper for chest minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ChestMinecart {
    id: EntityId,
}

wrapper_from_query_impl!(ChestMinecart, ChestMinecartMarker);
entity_wrapper_impl!(ChestMinecart, ChestMinecartMarker);

impl crate::HasEntityId for ChestMinecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
