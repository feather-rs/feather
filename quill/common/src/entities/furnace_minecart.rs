use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all furnace minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::FurnaceMinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &FurnaceMinecartMarker)>() {
///         println!("Found a furnace minecart with position "furnace minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct FurnaceMinecartMarker;

pod_component_impl!(FurnaceMinecartMarker);

/// Entity wrapper for furnace minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct FurnaceMinecart {
    id: EntityId,
}

wrapper_from_query_impl!(FurnaceMinecart, FurnaceMinecartMarker);
entity_wrapper_impl!(FurnaceMinecart, FurnaceMinecartMarker);

impl crate::HasEntityId for FurnaceMinecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
