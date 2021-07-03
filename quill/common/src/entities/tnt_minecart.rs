use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all tnt minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::TntMinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TntMinecartMarker)>() {
///         println!("Found a tnt minecart with position "tnt minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TntMinecartMarker;

pod_component_impl!(TntMinecartMarker);

/// Entity wrapper for tnt minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct TntMinecart {
    id: EntityId,
}

wrapper_from_query_impl!(TntMinecart, TntMinecartMarker);
entity_wrapper_impl!(TntMinecart, TntMinecartMarker);

impl crate::HasEntityId for TntMinecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
