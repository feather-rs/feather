use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::MinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &MinecartMarker)>() {
///         println!("Found a minecart with position "minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct MinecartMarker;

pod_component_impl!(MinecartMarker);

/// Entity wrapper for minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Minecart {
    id: EntityId,
}

wrapper_from_query_impl!(Minecart, MinecartMarker);
entity_wrapper_impl!(Minecart, MinecartMarker);

impl crate::HasEntityId for Minecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
