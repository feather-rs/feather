use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all hopper minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::HopperMinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &HopperMinecartMarker)>() {
///         println!("Found a hopper minecart with position "hopper minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct HopperMinecartMarker;

pod_component_impl!(HopperMinecartMarker);

/// Entity wrapper for hopper minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct HopperMinecart {
    id: EntityId,
}

wrapper_from_query_impl!(HopperMinecart, HopperMinecartMarker);
entity_wrapper_impl!(HopperMinecart, HopperMinecartMarker);

impl crate::HasEntityId for HopperMinecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
